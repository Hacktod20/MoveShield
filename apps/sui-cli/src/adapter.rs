//! Adapter for the official Sui Move source parser.

use move_compiler::command_line::compiler::{Compiler, PASS_PARSER};
use move_compiler::diagnostics::report_diagnostics_to_buffer_with_mapped_files;
use move_compiler::editions::{Edition, Flavor};
use move_compiler::parser::ast::{
    self, Definition, Exp, Exp_, FunctionBody_, ModuleDefinition, ModuleMember, Sequence,
    SequenceItem_, StructFields,
};
use move_compiler::shared::{files::MappedFiles, PackageConfig};
use move_ir_types::location::Loc;
use moveshield_compiler::{CompilerAdapter, ProjectDiscovery};
use moveshield_core::errors::{Error, Result};
use moveshield_core::models::{Field, Function, Location, Module, Package, Project, Struct};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Parses Sui Move sources with the official compiler parser and converts its AST to MoveShield's
/// analysis model. This runs the compiler's parser pass; it does not type-check the package.
pub struct SuiCompilerAdapter;

impl CompilerAdapter for SuiCompilerAdapter {
    fn compile(&self, path: &Path) -> Result<Project> {
        let (package_name, addresses, source_files) =
            ProjectDiscovery::discover(path).map_err(Error::Compiler)?;
        let compiler_root = path.canonicalize().map_err(Error::Io)?;
        let manifest_path = path.join("Move.toml");
        let manifest = fs::read_to_string(&manifest_path).map_err(Error::Io)?;
        let manifest: toml::Value =
            toml::from_str(&manifest).map_err(|error| Error::Compiler(error.to_string()))?;

        let edition = if let Some(edition) = manifest
            .get("package")
            .and_then(|package| package.get("edition"))
            .and_then(toml::Value::as_str)
        {
            edition
                .parse::<Edition>()
                .map_err(|error| Error::Compiler(format!("Invalid Move edition: {error}")))?
        } else {
            Edition::default()
        };
        let package_config = PackageConfig {
            flavor: Flavor::Sui,
            edition,
            ..PackageConfig::default()
        };

        let target_files = source_files
            .iter()
            .map(|file| {
                let relative_file = file.strip_prefix(path).map_err(|error| {
                    Error::Compiler(format!(
                        "Discovered source {} is outside package {}: {error}",
                        file.display(),
                        path.display()
                    ))
                })?;
                compiler_root
                    .join(relative_file)
                    .to_str()
                    .map(str::to_owned)
                    .ok_or_else(|| {
                        Error::Compiler(format!(
                            "Sui compiler cannot read non-Unicode source path {}",
                            file.display()
                        ))
                    })
            })
            .collect::<Result<Vec<_>>>()?;
        let compiler = Compiler::from_files(
            None,
            target_files,
            Vec::<String>::new(),
            BTreeMap::<String, move_compiler::shared::NumericalAddress>::new(),
        )
        .set_default_config(package_config);
        let (mapped_files, result) = compiler
            .run::<PASS_PARSER>()
            .map_err(|error| Error::Compiler(error.to_string()))?;
        let parser = result.map_err(|(pass, diagnostics)| {
            Error::Compiler(format!(
                "Sui Move parser failed at compiler pass {pass}:\n{}",
                String::from_utf8_lossy(&report_diagnostics_to_buffer_with_mapped_files(
                    &mapped_files,
                    diagnostics,
                    false,
                ))
            ))
        })?;
        if parser.compilation_env().has_errors() {
            let diagnostics = parser.compilation_env().take_final_diags();
            return Err(Error::Compiler(format!(
                "Sui Move parser reported errors:\n{}",
                String::from_utf8_lossy(&report_diagnostics_to_buffer_with_mapped_files(
                    &mapped_files,
                    diagnostics,
                    false,
                ))
            )));
        }
        let (_, ast) = parser.into_ast();

        let mut modules = Vec::new();
        for package in ast.source_definitions {
            match package.def {
                Definition::Module(module) => modules.push(convert_module(
                    module,
                    None,
                    &mapped_files,
                    &compiler_root,
                    &addresses,
                )?),
                Definition::Address(address) => {
                    for module in address.modules {
                        modules.push(convert_module(
                            module,
                            Some(&address.addr),
                            &mapped_files,
                            &compiler_root,
                            &addresses,
                        )?);
                    }
                }
            }
        }

        let package = Package {
            name: package_name.clone(),
            path: path.to_path_buf(),
            modules,
        };
        Ok(Project {
            name: package_name,
            root_path: path.to_path_buf(),
            packages: vec![package],
        })
    }
}

fn convert_module(
    module: ModuleDefinition,
    inherited_address: Option<&ast::LeadingNameAccess>,
    mapped_files: &MappedFiles,
    project_root: &Path,
    addresses: &std::collections::HashMap<String, String>,
) -> Result<Module> {
    let raw_address = module
        .address
        .as_ref()
        .or(inherited_address)
        .map(|address| match address.value {
            ast::LeadingNameAccess_::AnonymousAddress(value) => value.to_string(),
            ast::LeadingNameAccess_::GlobalAddress(name) | ast::LeadingNameAccess_::Name(name) => {
                name.value.to_string()
            }
        })
        .unwrap_or_else(|| "0x0".to_string());
    let address = addresses.get(&raw_address).cloned().unwrap_or(raw_address);
    let name = module.name.0.value.to_string();
    let source_file = relative_path(mapped_files, &module.loc, project_root)?;
    let mut functions = Vec::new();
    let mut structs = Vec::new();
    let mut use_statements = Vec::new();

    for member in module.members {
        match member {
            ModuleMember::Function(function) => {
                let body_text = source_text(mapped_files, &function.body.loc)?.to_string();
                let mut metrics = AstMetrics {
                    complexity: 1,
                    ..AstMetrics::default()
                };
                if let FunctionBody_::Defined(sequence) = &function.body.value {
                    metrics.visit_sequence(sequence);
                }
                let parameters = function
                    .signature
                    .parameters
                    .iter()
                    .map(|(_, name, ty)| {
                        let name = name.0.value.to_string();
                        let ty = source_text(mapped_files, &ty.loc)?;
                        Ok(format!("{name}: {}", ty.trim()))
                    })
                    .collect::<Result<Vec<_>>>()?;
                let visibility = if function.entry.is_some() {
                    "entry"
                } else {
                    match function.visibility {
                        ast::Visibility::Public(_) => "public",
                        ast::Visibility::Friend(_) => "public(friend)",
                        ast::Visibility::Package(_) => "public(package)",
                        ast::Visibility::Internal => "private",
                    }
                };
                functions.push(Function {
                    name: function.name.0.value.to_string(),
                    visibility: visibility.to_string(),
                    location: convert_location(mapped_files, &function.loc, project_root)?,
                    complexity: metrics.complexity,
                    calls: metrics.calls,
                    parameters,
                    body_text,
                    ast: None,
                });
            }
            ModuleMember::Struct(structure) => {
                let fields = match structure.fields {
                    StructFields::Named(fields) => fields
                        .into_iter()
                        .map(|(_, field, ty)| {
                            Ok(Field {
                                name: field.0.value.to_string(),
                                type_str: source_text(mapped_files, &ty.loc)
                                    .map(str::trim)
                                    .map(str::to_string)?,
                            })
                        })
                        .collect::<Result<Vec<_>>>()?,
                    StructFields::Positional(fields) => fields
                        .into_iter()
                        .enumerate()
                        .map(|(index, (_, ty))| {
                            Ok(Field {
                                name: index.to_string(),
                                type_str: source_text(mapped_files, &ty.loc)
                                    .map(str::trim)
                                    .map(str::to_string)?,
                            })
                        })
                        .collect::<Result<Vec<_>>>()?,
                    StructFields::Native(_) => Vec::new(),
                };
                structs.push(Struct {
                    name: structure.name.0.value.to_string(),
                    abilities: structure
                        .abilities
                        .iter()
                        .map(|ability| ability.value.to_string())
                        .collect(),
                    fields,
                });
            }
            ModuleMember::Use(use_decl) => {
                use_statements.extend(convert_use(&use_decl.use_, addresses));
            }
            ModuleMember::Enum(_)
            | ModuleMember::Friend(_)
            | ModuleMember::Constant(_)
            | ModuleMember::Spec(_) => {}
        }
    }

    Ok(Module {
        name,
        address,
        source_file,
        functions,
        structs,
        use_statements,
    })
}

fn convert_use(
    use_decl: &ast::Use,
    addresses: &std::collections::HashMap<String, String>,
) -> Vec<moveshield_core::models::UseStatement> {
    match use_decl {
        ast::Use::ModuleUse(module_ident, module_use) => {
            vec![convert_module_use(
                leading_address(&module_ident.value.address, addresses),
                module_ident.value.module.0.value.to_string(),
                module_use,
            )]
        }
        ast::Use::NestedModuleUses(address, modules) => modules
            .iter()
            .map(|(module, module_use)| {
                convert_module_use(
                    leading_address(address, addresses),
                    module.0.value.to_string(),
                    module_use,
                )
            })
            .collect(),
        ast::Use::Fun { .. } | ast::Use::Partial { .. } => Vec::new(),
    }
}

fn convert_module_use(
    address: String,
    module: String,
    module_use: &ast::ModuleUse,
) -> moveshield_core::models::UseStatement {
    let members = match module_use {
        ast::ModuleUse::Module(alias) => alias
            .iter()
            .map(|name| moveshield_core::models::UseMember {
                name: "Self".to_string(),
                alias: Some(name.0.value.to_string()),
            })
            .collect(),
        ast::ModuleUse::Members(imports) => imports
            .iter()
            .map(|(name, alias)| moveshield_core::models::UseMember {
                name: name.value.to_string(),
                alias: alias.map(|alias| alias.value.to_string()),
            })
            .collect(),
        ast::ModuleUse::Partial { .. } => Vec::new(),
    };
    moveshield_core::models::UseStatement {
        address: Some(address),
        module,
        members,
    }
}

fn leading_address(
    address: &ast::LeadingNameAccess,
    addresses: &std::collections::HashMap<String, String>,
) -> String {
    let raw = match address.value {
        ast::LeadingNameAccess_::AnonymousAddress(value) => value.to_string(),
        ast::LeadingNameAccess_::GlobalAddress(name) | ast::LeadingNameAccess_::Name(name) => {
            name.value.to_string()
        }
    };
    addresses.get(&raw).cloned().unwrap_or(raw)
}

fn relative_path(mapped_files: &MappedFiles, loc: &Loc, root: &Path) -> Result<PathBuf> {
    let path = mapped_files
        .file_name_mapping()
        .get(&loc.file_hash())
        .ok_or_else(|| {
            Error::Compiler("Sui compiler returned a source location without a file".to_string())
        })?;
    path.strip_prefix(root)
        .map(Path::to_path_buf)
        .map_err(|error| {
            Error::Compiler(format!(
                "Sui compiler source {} is outside package {}: {error}",
                path.display(),
                root.display()
            ))
        })
}

fn source_text<'a>(mapped_files: &'a MappedFiles, loc: &Loc) -> Result<&'a str> {
    mapped_files
        .source_of_loc_opt(loc)
        .ok_or_else(|| Error::Compiler("Sui compiler returned an unmapped source span".to_string()))
}

fn convert_location(mapped_files: &MappedFiles, loc: &Loc, root: &Path) -> Result<Location> {
    let span = mapped_files.position_opt(loc).ok_or_else(|| {
        Error::Compiler("Sui compiler could not map a source location".to_string())
    })?;
    Ok(Location {
        file: relative_path(mapped_files, loc, root)?,
        start_line: span.start.user_line(),
        start_column: span.start.user_column(),
        end_line: span.end.user_line(),
        end_column: span.end.user_column(),
    })
}

#[derive(Default)]
struct AstMetrics {
    complexity: usize,
    calls: Vec<String>,
}

impl AstMetrics {
    fn visit_sequence(&mut self, sequence: &Sequence) {
        for item in &sequence.1 {
            match &item.value {
                SequenceItem_::Seq(expression) | SequenceItem_::Bind(_, _, expression) => {
                    self.visit_exp(expression);
                }
                SequenceItem_::Declare(_, _) => {}
            }
        }
        if let Some(expression) = sequence.3.as_ref() {
            self.visit_exp(expression);
        }
    }

    fn visit_exp(&mut self, expression: &Exp) {
        match &expression.value {
            Exp_::Call(function, arguments) => {
                let name = function.value.to_string();
                if !self.calls.contains(&name) {
                    self.calls.push(name);
                }
                for argument in &arguments.value {
                    self.visit_exp(argument);
                }
            }
            Exp_::IfElse(condition, then_branch, else_branch) => {
                self.complexity += 1;
                self.visit_exp(condition);
                self.visit_exp(then_branch);
                if let Some(branch) = else_branch {
                    self.visit_exp(branch);
                }
            }
            Exp_::While(condition, body) => {
                self.complexity += 1;
                self.visit_exp(condition);
                self.visit_exp(body);
            }
            Exp_::Loop(body) => {
                self.complexity += 1;
                self.visit_exp(body);
            }
            Exp_::Match(subject, arms) => {
                self.complexity += arms.value.len().saturating_sub(1);
                self.visit_exp(subject);
                for arm in &arms.value {
                    if let Some(guard) = &arm.value.guard {
                        self.visit_exp(guard);
                    }
                    self.visit_exp(&arm.value.rhs);
                }
            }
            Exp_::Move(_, inner)
            | Exp_::Copy(_, inner)
            | Exp_::Parens(inner)
            | Exp_::Dereference(inner)
            | Exp_::UnaryExp(_, inner)
            | Exp_::Borrow(_, inner)
            | Exp_::DotUnresolved(_, inner)
            | Exp_::Labeled(_, inner)
            | Exp_::Abort(Some(inner))
            | Exp_::Return(_, Some(inner))
            | Exp_::Break(_, Some(inner)) => self.visit_exp(inner),
            Exp_::Pack(_, fields) => {
                for (_, value) in fields {
                    self.visit_exp(value);
                }
            }
            Exp_::Vector(_, _, values) => {
                for value in &values.value {
                    self.visit_exp(value);
                }
            }
            Exp_::ExpList(values) => {
                for value in values {
                    self.visit_exp(value);
                }
            }
            Exp_::Block(sequence) => self.visit_sequence(sequence),
            Exp_::Lambda(_, _, body) => self.visit_exp(body),
            Exp_::Quant(_, bindings, _, condition, body) => {
                for binding in &bindings.value {
                    self.visit_exp(&binding.value.1);
                }
                if let Some(condition) = condition {
                    self.visit_exp(condition);
                }
                self.visit_exp(body);
            }
            Exp_::Assign(left, right) => {
                self.visit_exp(left);
                self.visit_exp(right);
            }
            Exp_::BinopExp(left, operator, right) => {
                self.visit_exp(left);
                self.visit_exp(right);
                if matches!(operator.value, ast::BinOp_::And | ast::BinOp_::Or) {
                    self.complexity += 1;
                }
            }
            Exp_::Dot(base, _, _) => self.visit_exp(base),
            Exp_::DotCall(base, _, _, _, _, arguments) => {
                self.visit_exp(base);
                for argument in &arguments.value {
                    self.visit_exp(argument);
                }
            }
            Exp_::Index(base, indices) => {
                self.visit_exp(base);
                for index in &indices.value {
                    self.visit_exp(index);
                }
            }
            Exp_::Cast(value, _) | Exp_::Annotate(value, _) => self.visit_exp(value),
            Exp_::Value(_)
            | Exp_::Name(_)
            | Exp_::Unit
            | Exp_::Abort(None)
            | Exp_::Return(_, None)
            | Exp_::Break(_, None)
            | Exp_::Continue(_)
            | Exp_::Spec(_)
            | Exp_::UnresolvedError => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn official_parser_builds_models_and_source_locations() {
        let example = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../example");
        let project = SuiCompilerAdapter
            .compile(&example)
            .expect("official Sui parser should parse the bundled example");

        let modules = &project.packages[0].modules;
        assert_eq!(modules.len(), 2);
        let module = modules
            .iter()
            .find(|module| module.name == "complex")
            .expect("example should contain complex");
        assert_eq!(module.address, "0x42");
        assert_eq!(module.use_statements.len(), 1);
        assert_eq!(module.use_statements[0].address.as_deref(), Some("0x42"));
        assert_eq!(module.use_statements[0].module, "clean");
        let function = modules
            .iter()
            .flat_map(|module| &module.functions)
            .find(|function| function.name == "transfer")
            .expect("example should contain transfer");
        assert_eq!(
            function.location.file,
            PathBuf::from("sources/complex.move")
        );
        assert_eq!(function.location.start_line, 6);
        assert_eq!(function.complexity, 12);
        assert!(function.calls.contains(&"clean::calculate_fee".to_string()));
    }

    #[test]
    fn invalid_move_syntax_returns_compiler_diagnostics() {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock is before UNIX epoch")
            .as_nanos();
        let package = std::env::temp_dir().join(format!(
            "moveshield-sui-parser-test-{}-{timestamp}",
            std::process::id()
        ));
        let sources = package.join("sources");
        fs::create_dir_all(&sources).expect("failed to create test package");
        fs::write(
            package.join("Move.toml"),
            "[package]\nname = \"invalid\"\nversion = \"0.0.1\"\n",
        )
        .expect("failed to write test manifest");
        fs::write(
            sources.join("invalid.move"),
            "module 0x42::invalid { public fun broken() { @@@ } }\n",
        )
        .expect("failed to write invalid Move source");

        let error = SuiCompilerAdapter
            .compile(&package)
            .expect_err("invalid syntax must fail the official parser");
        let message = error.to_string();
        assert!(
            message.contains("Sui Move parser reported errors"),
            "{message}"
        );
        assert!(message.contains("invalid.move"), "{message}");
        fs::remove_dir_all(package).expect("failed to remove test package");
    }
}
