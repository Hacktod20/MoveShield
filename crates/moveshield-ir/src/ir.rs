//! Move Intermediate Representation (MoveIR) for engineering analysis.

use moveshield_core::models::{AstNode, Location, Project, UseStatement};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Semantic representation of a project inside MoveIR.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveProject {
    /// Name of the project.
    pub name: String,
    /// Modules in the project, indexed by qualified name ("address::module_name").
    pub modules: HashMap<String, MoveModule>,
}

/// Semantic representation of a module in MoveIR.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveModule {
    /// Name of the module.
    pub name: String,
    /// Address of the module.
    pub address: String,
    /// Functions defined in this module.
    pub functions: HashMap<String, MoveFunction>,
    /// Module-level use statements.
    pub use_statements: Vec<UseStatement>,
}

/// Semantic representation of a function in MoveIR.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveFunction {
    /// Name of the function.
    pub name: String,
    /// Visibility of the function.
    pub visibility: String,
    /// Source location.
    pub location: Location,
    /// Cyclomatic complexity score.
    pub complexity: usize,
    /// Function calls made by this function.
    pub calls: Vec<String>,
    /// Parameters of the function.
    pub parameters: Vec<String>,
    /// The body text of the function.
    pub body_text: String,
    /// Rich AST structure parsed from the function body.
    pub ast: Option<AstNode>,
}

/// The main representation of MoveIR.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveIR {
    /// The processed MoveProject.
    pub project: MoveProject,
    /// Pre-analyzed function semantics mapping (fq_name -> FunctionSemantic).
    pub function_semantics: HashMap<String, FunctionSemantic>,
}

impl MoveIR {
    /// Constructs MoveIR semantic model from the Core Data Model (CDM) Project.
    pub fn from_project(project: &Project) -> Self {
        let mut modules = HashMap::new();
        let mut function_semantics = HashMap::new();

        for package in &project.packages {
            for module in &package.modules {
                let module_key = format!("{}::{}", module.address, module.name);

                let mut functions = HashMap::new();
                for function in &module.functions {
                    let move_fn = MoveFunction {
                        name: function.name.clone(),
                        visibility: function.visibility.clone(),
                        location: function.location.clone(),
                        complexity: function.complexity,
                        calls: function.calls.clone(),
                        parameters: function.parameters.clone(),
                        body_text: function.body_text.clone(),
                        ast: function.ast.clone(),
                    };
                    functions.insert(function.name.clone(), move_fn);
                }

                let move_mod = MoveModule {
                    name: module.name.clone(),
                    address: module.address.clone(),
                    functions,
                    use_statements: module.use_statements.clone(),
                };

                modules.insert(module_key, move_mod);
            }
        }

        // Run semantic analysis across all modules and functions
        for (module_key, module) in &modules {
            for (fn_name, func) in &module.functions {
                let fq_name = format!("{}::{}", module_key, fn_name);
                let sem = FunctionSemantic::analyze(func, module);
                function_semantics.insert(fq_name, sem);
            }
        }

        let move_project = MoveProject {
            name: project.name.clone(),
            modules,
        };

        Self {
            project: move_project,
            function_semantics,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use moveshield_core::models::{AstNode, Location, UseMember, UseStatement};
    use std::path::PathBuf;

    #[test]
    fn test_resolve_type_path() {
        let use_statements = vec![
            UseStatement {
                address: Some("sui".to_string()),
                module: "transfer".to_string(),
                members: vec![],
            },
            UseStatement {
                address: Some("sui".to_string()),
                module: "tx_context".to_string(),
                members: vec![UseMember {
                    name: "TxContext".to_string(),
                    alias: Some("Ctx".to_string()),
                }],
            },
        ];

        // test fully qualified
        assert_eq!(
            resolve_type_path("0x2::coin::Coin", "my_module", "0x42", &use_statements),
            "0x2::coin::Coin"
        );

        // test partially qualified via import
        assert_eq!(
            resolve_type_path(
                "transfer::public_transfer",
                "my_module",
                "0x42",
                &use_statements
            ),
            "sui::transfer::public_transfer"
        );

        // test alias member
        assert_eq!(
            resolve_type_path("Ctx", "my_module", "0x42", &use_statements),
            "sui::tx_context::TxContext"
        );

        // test fallback to local module
        assert_eq!(
            resolve_type_path("MyStruct", "my_module", "0x42", &use_statements),
            "0x42::my_module::MyStruct"
        );
    }

    #[test]
    fn test_analyze_function_semantic() {
        let module = MoveModule {
            name: "my_module".to_string(),
            address: "0x42".to_string(),
            functions: HashMap::new(),
            use_statements: vec![UseStatement {
                address: Some("sui".to_string()),
                module: "transfer".to_string(),
                members: vec![],
            }],
        };

        // fun test_flow(cap: AdminCap, recipient: address) {
        //     let mut x = cap;
        //     transfer::public_transfer(x, recipient);
        // }
        let ast = AstNode::Block(vec![
            AstNode::Let {
                name: "x".to_string(),
                is_mut: true,
                type_annotation: None,
                initializer: Some(Box::new(AstNode::Identifier("cap".to_string()))),
            },
            AstNode::Call {
                function_name: "transfer::public_transfer".to_string(),
                arguments: vec![
                    AstNode::Identifier("x".to_string()),
                    AstNode::Identifier("recipient".to_string()),
                ],
            },
        ]);

        let func = MoveFunction {
            name: "test_flow".to_string(),
            visibility: "public".to_string(),
            location: Location {
                file: PathBuf::from("test.move"),
                start_line: 1,
                start_column: 1,
                end_line: 1,
                end_column: 1,
            },
            complexity: 1,
            calls: vec!["transfer::public_transfer".to_string()],
            parameters: vec![
                "cap: AdminCap".to_string(),
                "recipient: address".to_string(),
            ],
            body_text: "".to_string(),
            ast: Some(ast),
        };

        let semantic = FunctionSemantic::analyze(&func, &module);

        assert_eq!(semantic.fq_name, "0x42::my_module::test_flow");
        assert!(semantic.has_capability_param);
        assert_eq!(
            semantic.resolved_calls,
            vec!["sui::transfer::public_transfer".to_string()]
        );

        // Assert data-flow correctly tracked asset type from 'cap' (parameter) to 'x' (local) to 'public_transfer' (sink)
        assert_eq!(semantic.transfers.len(), 1);
        assert_eq!(semantic.transfers[0].asset_name, "x");
        assert_eq!(
            semantic.transfers[0].asset_type,
            "0x42::my_module::AdminCap"
        );
        assert_eq!(semantic.transfers[0].recipient, "recipient");
    }
}

/// Resolves a type path using the module's imports and aliases.
pub fn resolve_type_path(
    name: &str,
    current_module: &str,
    current_address: &str,
    use_statements: &[UseStatement],
) -> String {
    let clean = name.trim();
    let parts: Vec<&str> = clean.split("::").collect();
    if parts.len() == 3 {
        return clean.to_string();
    }

    if parts.len() == 2 {
        let mod_prefix = parts[0];
        let type_name = parts[1];
        for stmt in use_statements {
            if stmt.module == mod_prefix {
                let addr = stmt.address.as_deref().unwrap_or("0x2");
                return format!("{}::{}::{}", addr, stmt.module, type_name);
            }
        }
        return format!("{}::{}::{}", current_address, mod_prefix, type_name);
    }

    // Check if there is an import matching this type name
    for stmt in use_statements {
        for member in &stmt.members {
            if member.name == clean || member.alias.as_deref() == Some(clean) {
                let addr = stmt.address.as_deref().unwrap_or("0x2");
                let real_name = if member.name == "Self" {
                    &stmt.module
                } else {
                    &member.name
                };
                return format!("{}::{}::{}", addr, stmt.module, real_name);
            }
        }
    }

    format!("{}::{}::{}", current_address, current_module, clean)
}

/// Represents an asset or capability transfer event inside a function.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TransferEvent {
    /// Variable name being transferred.
    pub asset_name: String,
    /// Resolved fully-qualified type of the asset.
    pub asset_type: String,
    /// Recipient address/variable name.
    pub recipient: String,
    /// Source location.
    pub location: Option<Location>,
}

/// Pre-analyzed function semantics including resolved symbols, types, and data-flow transfers.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FunctionSemantic {
    /// Fully qualified function name: address::module::name
    pub fq_name: String,
    /// Parameter names and raw type annotations
    pub parameters: Vec<(String, String)>,
    /// Resolved parameter types
    pub param_types: HashMap<String, String>,
    /// Local variables and inferred types
    pub local_types: HashMap<String, String>,
    /// Calls made by this function with resolved target name
    pub resolved_calls: Vec<String>,
    /// Asset transfer events inside this function
    pub transfers: Vec<TransferEvent>,
    /// Access control attributes
    pub has_capability_param: bool,
    pub has_tx_context_param: bool,
    pub has_signer_param: bool,
}

impl FunctionSemantic {
    /// Runs semantic analysis on a Move function body.
    pub fn analyze(func: &MoveFunction, module: &MoveModule) -> Self {
        let fq_name = format!("{}::{}::{}", module.address, module.name, func.name);
        let mut parameters = Vec::new();
        let mut param_types = HashMap::new();
        let mut local_types = HashMap::new();
        let mut resolved_calls = Vec::new();
        let mut transfers = Vec::new();

        let mut has_capability_param = false;
        let mut has_tx_context_param = false;
        let mut has_signer_param = false;

        // Parse function parameters
        for param_str in &func.parameters {
            let parts: Vec<&str> = param_str.splitn(2, ':').collect();
            if parts.len() == 2 {
                let p_name = parts[0].trim().to_string();
                let p_type_raw = parts[1].trim().to_string();
                parameters.push((p_name.clone(), p_type_raw.clone()));

                let p_type_resolved = resolve_type_path(
                    &p_type_raw,
                    &module.name,
                    &module.address,
                    &module.use_statements,
                );
                param_types.insert(p_name.clone(), p_type_resolved.clone());

                if p_type_raw.contains("Cap") || p_type_raw.contains("Capability") {
                    has_capability_param = true;
                }
                if p_type_raw.contains("TxContext") {
                    has_tx_context_param = true;
                }
                if p_type_raw.contains("signer") {
                    has_signer_param = true;
                }
            }
        }

        // Perform AST traversal and data-flow analysis
        if let Some(ref ast) = func.ast {
            let mut state = DataFlowState {
                module,
                param_types: &param_types,
                local_types: &mut local_types,
                resolved_calls: &mut resolved_calls,
                transfers: &mut transfers,
            };
            state.analyze_node(ast, &func.location);
        } else {
            for call in &func.calls {
                resolved_calls.push(resolve_type_path(
                    call,
                    &module.name,
                    &module.address,
                    &module.use_statements,
                ));
            }
        }

        Self {
            fq_name,
            parameters,
            param_types,
            local_types,
            resolved_calls,
            transfers,
            has_capability_param,
            has_tx_context_param,
            has_signer_param,
        }
    }
}

struct DataFlowState<'a> {
    module: &'a MoveModule,
    param_types: &'a HashMap<String, String>,
    local_types: &'a mut HashMap<String, String>,
    resolved_calls: &'a mut Vec<String>,
    transfers: &'a mut Vec<TransferEvent>,
}

impl<'a> DataFlowState<'a> {
    fn analyze_node(&mut self, node: &AstNode, loc: &Location) {
        match node {
            AstNode::Block(statements) => {
                for stmt in statements {
                    self.analyze_node(stmt, loc);
                }
            }
            AstNode::Let {
                name,
                initializer,
                type_annotation,
                ..
            } => {
                let resolved_type = if let Some(ref ta) = type_annotation {
                    resolve_type_path(
                        ta,
                        &self.module.name,
                        &self.module.address,
                        &self.module.use_statements,
                    )
                } else if let Some(ref init) = initializer {
                    self.infer_node_type(init)
                } else {
                    "Unknown".to_string()
                };

                self.local_types.insert(name.clone(), resolved_type);

                if let Some(ref init) = initializer {
                    self.analyze_node(init, loc);
                }
            }
            AstNode::Assign { target, value } => {
                let val_type = self.infer_node_type(value);
                self.local_types.insert(target.clone(), val_type);
                self.analyze_node(value, loc);
            }
            AstNode::Call {
                function_name,
                arguments,
            } => {
                let fq_call = resolve_type_path(
                    function_name,
                    &self.module.name,
                    &self.module.address,
                    &self.module.use_statements,
                );
                self.resolved_calls.push(fq_call.clone());

                let is_transfer = fq_call == "sui::transfer::public_transfer"
                    || fq_call == "sui::transfer::transfer"
                    || fq_call == "sui::transfer::public_share_object"
                    || fq_call == "sui::transfer::share_object";

                if is_transfer && !arguments.is_empty() {
                    let asset_node = &arguments[0];
                    let asset_name = match asset_node {
                        AstNode::Identifier(name) => name.clone(),
                        _ => "expression".to_string(),
                    };
                    let asset_type = self.infer_node_type(asset_node);

                    let recipient = if arguments.len() > 1 {
                        match &arguments[1] {
                            AstNode::Identifier(name) => name.clone(),
                            AstNode::Literal(lit) => lit.clone(),
                            _ => "unknown".to_string(),
                        }
                    } else {
                        "unknown".to_string()
                    };

                    self.transfers.push(TransferEvent {
                        asset_name,
                        asset_type,
                        recipient,
                        location: Some(loc.clone()),
                    });
                }

                for arg in arguments {
                    self.analyze_node(arg, loc);
                }
            }
            AstNode::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.analyze_node(condition, loc);
                self.analyze_node(then_branch, loc);
                if let Some(ref eb) = else_branch {
                    self.analyze_node(eb, loc);
                }
            }
            AstNode::While { condition, body } => {
                self.analyze_node(condition, loc);
                self.analyze_node(body, loc);
            }
            AstNode::Loop { body } => {
                self.analyze_node(body, loc);
            }
            _ => {}
        }
    }

    fn infer_node_type(&self, node: &AstNode) -> String {
        match node {
            AstNode::Identifier(name) => {
                if let Some(t) = self.local_types.get(name) {
                    return t.clone();
                }
                if let Some(t) = self.param_types.get(name) {
                    return t.clone();
                }
                "Unknown".to_string()
            }
            AstNode::Literal(lit) => {
                if lit == "true" || lit == "false" {
                    "bool".to_string()
                } else if lit.starts_with("0x") {
                    "address".to_string()
                } else {
                    "u64".to_string()
                }
            }
            AstNode::Call { function_name, .. } => {
                let fq_call = resolve_type_path(
                    function_name,
                    &self.module.name,
                    &self.module.address,
                    &self.module.use_statements,
                );
                if fq_call == "sui::object::new" {
                    return "sui::object::UID".to_string();
                }
                if fq_call == "sui::tx_context::sender" {
                    return "address".to_string();
                }
                "Unknown".to_string()
            }
            _ => "Unknown".to_string(),
        }
    }
}
