//! Compiler Adapter trait and Development Parser fallback implementation.

use moveshield_core::errors::{Error, Result};
use moveshield_core::models::{
    AstNode, Field, Function, Location, Module, Package, Project, Struct, UseMember, UseStatement,
};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::discovery::ProjectDiscovery;

/// Trait defining the stable boundary for compiler adapters.
pub trait CompilerAdapter {
    /// Compiles or discovers a Move project and converts it to the Core Data Model.
    fn compile(&self, path: &Path) -> Result<Project>;
}

/// A lightweight "Development Parser" fallback implementing CompilerAdapter.
/// This exists solely as a temporary fallback when the official toolchain is missing.
pub struct DevelopmentParserAdapter;

impl CompilerAdapter for DevelopmentParserAdapter {
    fn compile(&self, path: &Path) -> Result<Project> {
        let (package_name, addresses, source_files) =
            ProjectDiscovery::discover(path).map_err(Error::Compiler)?;

        let mut modules = Vec::new();

        for file_path in source_files {
            let relative_file = file_path
                .strip_prefix(path)
                .unwrap_or(&file_path)
                .to_path_buf();

            let content = fs::read_to_string(&file_path).map_err(Error::Io)?;

            let file_modules = parse_move_file(&content, &relative_file, &addresses)?;
            modules.extend(file_modules);
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

/// Strips all single-line and multi-line comments from the Move source code.
fn strip_comments(src: &str) -> String {
    let chars: Vec<char> = src.chars().collect();
    let mut result = String::with_capacity(src.len());
    let mut i = 0;
    let mut block_depth = 0;
    let mut in_string = false;
    let mut escaped = false;

    while i < chars.len() {
        if block_depth > 0 {
            if i + 1 < chars.len() && chars[i] == '/' && chars[i + 1] == '*' {
                block_depth += 1;
                result.push(' ');
                result.push(' ');
                i += 2;
            } else if i + 1 < chars.len() && chars[i] == '*' && chars[i + 1] == '/' {
                block_depth -= 1;
                result.push(' ');
                result.push(' ');
                i += 2;
            } else {
                let c = chars[i];
                result.push(if c == '\n' { '\n' } else { ' ' });
                i += 1;
            }
        } else if in_string {
            let c = chars[i];
            result.push(c);
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_string = false;
            }
            i += 1;
        } else if chars[i] == '"' {
            in_string = true;
            result.push(chars[i]);
            i += 1;
        } else if i + 1 < chars.len() && chars[i] == '/' && chars[i + 1] == '/' {
            i += 2;
            while i < chars.len() && chars[i] != '\n' {
                result.push(' ');
                i += 1;
            }
            if i < chars.len() {
                result.push('\n');
                i += 1;
            }
        } else if i + 1 < chars.len() && chars[i] == '/' && chars[i + 1] == '*' {
            block_depth = 1;
            result.push(' ');
            result.push(' ');
            i += 2;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}

/// Helper to determine if a specific word exists at the current index with word boundaries.
fn is_word_at(src: &[char], idx: usize, word: &str) -> bool {
    if idx + word.len() > src.len() {
        return false;
    }
    for (i, c) in word.chars().enumerate() {
        if src[idx + i] != c {
            return false;
        }
    }
    if idx > 0 {
        let prev = src[idx - 1];
        if prev.is_alphanumeric() || prev == '_' {
            return false;
        }
    }
    if idx + word.len() < src.len() {
        let next = src[idx + word.len()];
        if next.is_alphanumeric() || next == '_' {
            return false;
        }
    }
    true
}

/// Finds matching curly braces `{` and `}` starting from start_idx.
fn find_matching_block(chars: &[char], start_idx: usize) -> Option<(usize, usize)> {
    let mut depth = 0usize;
    let mut first_brace = None;
    let mut in_string = false;
    let mut escaped = false;

    for (idx, &c) in chars.iter().enumerate().skip(start_idx) {
        if in_string {
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_string = false;
            }
        } else if c == '"' {
            in_string = true;
        } else if c == '{' {
            if depth == 0 {
                first_brace = Some(idx);
            }
            depth += 1;
        } else if c == '}' && depth > 0 {
            depth -= 1;
            if depth == 0 {
                if let Some(start) = first_brace {
                    return Some((start + 1, idx));
                }
            }
        }
    }
    None
}

/// Parses a Move file and extracts modules, functions, and structs.
fn parse_move_file(
    content: &str,
    relative_path: &Path,
    addresses: &HashMap<String, String>,
) -> Result<Vec<Module>> {
    let stripped = strip_comments(content);
    let chars: Vec<char> = stripped.chars().collect();
    let mut modules = Vec::new();

    let mut i = 0;
    while i < chars.len() {
        if is_word_at(&chars, i, "module") {
            // Found a module declaration! Let's parse the module identifier.
            let mut name_start = i + 6;
            while name_start < chars.len() && chars[name_start].is_whitespace() {
                name_start += 1;
            }
            let mut name_end = name_start;
            while name_end < chars.len()
                && (chars[name_end].is_alphanumeric()
                    || chars[name_end] == '_'
                    || chars[name_end] == ':')
            {
                name_end += 1;
            }

            if name_end > name_start {
                let raw_ident: String = chars[name_start..name_end].iter().collect();
                let parts: Vec<&str> = raw_ident.split("::").collect();
                let (address, module_name) = if parts.len() == 2 {
                    (parts[0].to_string(), parts[1].to_string())
                } else {
                    ("0x0".to_string(), parts[0].to_string())
                };

                // Resolve address if it matches a named address
                let resolved_address = addresses.get(&address).cloned().unwrap_or(address);

                // Find the module brace block
                if let Some((block_start, block_end)) = find_matching_block(&chars, name_end) {
                    let module_block = &chars[block_start..block_end];
                    let (functions, structs, use_statements) =
                        parse_module_contents(module_block, relative_path, &module_name)?;

                    modules.push(Module {
                        name: module_name,
                        address: resolved_address,
                        source_file: relative_path.to_path_buf(),
                        functions,
                        structs,
                        use_statements,
                    });

                    i = block_end + 1;
                    continue;
                }
            }
        }
        i += 1;
    }

    Ok(modules)
}

/// Parses the contents of a module to extract functions, structs, and use statements.
fn parse_module_contents(
    chars: &[char],
    relative_path: &Path,
    current_module: &str,
) -> Result<(Vec<Function>, Vec<Struct>, Vec<UseStatement>)> {
    let mut functions = Vec::new();
    let mut structs = Vec::new();
    let mut use_statements = Vec::new();

    let mut i = 0;
    while i < chars.len() {
        // Look for use statements
        if is_word_at(chars, i, "use") {
            let mut end = i + 3;
            while end < chars.len() && chars[end] != ';' {
                end += 1;
            }
            if end < chars.len() {
                let use_str: String = chars[i + 3..end].iter().collect();
                if let Some(use_stmt) = parse_use_statement(&use_str) {
                    use_statements.push(use_stmt);
                }
                i = end + 1;
                continue;
            }
        }

        // Look for structs
        if is_word_at(chars, i, "struct") {
            let mut name_start = i + 6;
            while name_start < chars.len() && chars[name_start].is_whitespace() {
                name_start += 1;
            }
            let mut name_end = name_start;
            while name_end < chars.len()
                && (chars[name_end].is_alphanumeric() || chars[name_end] == '_')
            {
                name_end += 1;
            }

            if name_end > name_start {
                let struct_name: String = chars[name_start..name_end].iter().collect();

                // Find matching brace block for the struct fields
                if let Some((block_start, block_end)) = find_matching_block(chars, name_end) {
                    let fields_block: String = chars[block_start..block_end].iter().collect();
                    let fields = parse_struct_fields(&fields_block);

                    // Scan abilities before `{`
                    let pre_block_str: String = chars[name_end..block_start - 1].iter().collect();
                    let abilities = parse_abilities(&pre_block_str);

                    structs.push(Struct {
                        name: struct_name,
                        abilities,
                        fields,
                    });

                    i = block_end + 1;
                    continue;
                }
            }
        }

        // Look for functions
        if is_word_at(chars, i, "fun") {
            // Determine visibility by checking keywords before 'fun'
            let mut visibility = "private".to_string();
            let mut check_idx = i;
            let mut words_before = Vec::new();

            // Collect up to 4 words before 'fun' to identify modifiers like 'public', 'entry', 'public(friend)'
            let mut temp_chars = Vec::new();
            while check_idx > 0 && words_before.len() < 4 {
                check_idx -= 1;
                let c = chars[check_idx];
                if c.is_alphanumeric() || c == '_' || c == '(' || c == ')' {
                    temp_chars.push(c);
                } else if !temp_chars.is_empty() {
                    let word: String = temp_chars.iter().rev().collect();
                    words_before.push(word);
                    temp_chars.clear();
                }
                if c == ';' || c == '}' || c == '{' {
                    break;
                }
            }
            if !temp_chars.is_empty() {
                let word: String = temp_chars.iter().rev().collect();
                words_before.push(word);
            }

            if words_before.contains(&"public(friend)".to_string())
                || (words_before.contains(&"public".to_string())
                    && words_before.contains(&"friend".to_string()))
            {
                visibility = "public(friend)".to_string();
            } else if words_before.contains(&"public".to_string()) {
                visibility = "public".to_string();
            } else if words_before.contains(&"entry".to_string()) {
                visibility = "entry".to_string();
            }

            let mut name_start = i + 3;
            while name_start < chars.len() && chars[name_start].is_whitespace() {
                name_start += 1;
            }
            let mut name_end = name_start;
            while name_end < chars.len()
                && (chars[name_end].is_alphanumeric() || chars[name_end] == '_')
            {
                name_end += 1;
            }

            if name_end > name_start {
                let fun_name: String = chars[name_start..name_end].iter().collect();

                // Find the brace block or check if it's native (ends with semicolon)
                let mut scan_idx = name_end;
                let mut found_brace = false;
                let mut ended_with_semicolon = false;
                while scan_idx < chars.len() {
                    if chars[scan_idx] == '{' {
                        found_brace = true;
                        break;
                    } else if chars[scan_idx] == ';' {
                        ended_with_semicolon = true;
                        break;
                    }
                    scan_idx += 1;
                }

                // Extract parameter strings from raw characters inside (...)
                let mut params_start = None;
                let mut params_end = None;
                let mut depth = 0;
                #[allow(clippy::needless_range_loop)]
                for idx in name_end..scan_idx {
                    if chars[idx] == '(' {
                        if depth == 0 {
                            params_start = Some(idx + 1);
                        }
                        depth += 1;
                    } else if chars[idx] == ')' {
                        depth -= 1;
                        if depth == 0 {
                            params_end = Some(idx);
                            break;
                        }
                    }
                }

                let mut parameters = Vec::new();
                if let (Some(ps), Some(pe)) = (params_start, params_end) {
                    let params_str: String = chars[ps..pe].iter().collect();
                    for part in params_str.split(',') {
                        let trimmed = part.trim();
                        if !trimmed.is_empty() {
                            parameters.push(trimmed.to_string());
                        }
                    }
                }

                if found_brace {
                    if let Some((block_start, block_end)) = find_matching_block(chars, scan_idx) {
                        let body_chars = &chars[block_start..block_end];
                        let complexity = calculate_complexity(body_chars);
                        let calls = extract_calls(body_chars, current_module);
                        let body_text: String = body_chars.iter().collect();
                        let ast = Some(AstNode::Block(parse_statements(&body_text)));

                        functions.push(Function {
                            name: fun_name,
                            visibility,
                            location: Location {
                                file: relative_path.to_path_buf(),
                                start_line: 1, // Simple line tracking fallback
                                start_column: 1,
                                end_line: 1,
                                end_column: 1,
                            },
                            complexity,
                            calls,
                            parameters,
                            body_text,
                            ast,
                        });

                        i = block_end + 1;
                        continue;
                    }
                } else if ended_with_semicolon {
                    functions.push(Function {
                        name: fun_name,
                        visibility,
                        location: Location {
                            file: relative_path.to_path_buf(),
                            start_line: 1,
                            start_column: 1,
                            end_line: 1,
                            end_column: 1,
                        },
                        complexity: 1,
                        calls: Vec::new(),
                        parameters,
                        body_text: String::new(),
                        ast: None,
                    });
                    i = scan_idx + 1;
                    continue;
                }
            }
        }
        i += 1;
    }

    Ok((functions, structs, use_statements))
}

/// Parses struct abilities (e.g., "has key, store")
fn parse_abilities(src: &str) -> Vec<String> {
    let mut abilities = Vec::new();
    if let Some(idx) = src.find("has") {
        let list_str = &src[idx + 3..];
        for part in list_str.split(',') {
            let clean = part.trim();
            if !clean.is_empty() {
                abilities.push(clean.to_string());
            }
        }
    }
    abilities
}

/// Parses simple field declarations inside a struct body
fn parse_struct_fields(src: &str) -> Vec<Field> {
    let mut fields = Vec::new();
    for line in src.lines() {
        let clean_line = line.trim();
        if clean_line.is_empty() || clean_line.starts_with("//") {
            continue;
        }
        let parts: Vec<&str> = clean_line.split(':').collect();
        if parts.len() == 2 {
            let name = parts[0].trim().to_string();
            let mut type_str = parts[1].trim().to_string();
            if type_str.ends_with(',') {
                type_str.pop();
            }
            fields.push(Field {
                name,
                type_str: type_str.trim().to_string(),
            });
        }
    }
    fields
}

/// Calculates standard cyclomatic complexity of a function body.
fn calculate_complexity(body: &[char]) -> usize {
    let mut complexity = 1;
    let mut i = 0;
    while i < body.len() {
        if is_word_at(body, i, "if") || is_word_at(body, i, "while") || is_word_at(body, i, "loop")
        {
            complexity += 1;
        } else if i + 1 < body.len()
            && ((body[i] == '&' && body[i + 1] == '&') || (body[i] == '|' && body[i + 1] == '|'))
        {
            complexity += 1;
            i += 1;
        }
        i += 1;
    }
    complexity
}

/// Extracts all function calls inside a function body.
fn extract_calls(body: &[char], current_module: &str) -> Vec<String> {
    let mut calls = Vec::new();
    let keywords = vec![
        "if", "else", "while", "loop", "return", "abort", "assert", "let", "mut", "fun", "struct",
        "public", "entry",
    ];

    let mut i = 0;
    while i < body.len() {
        if body[i].is_alphabetic() || body[i] == '_' {
            // Extract the potential identifier (can include alphanumeric, _, and :)
            let mut end = i;
            while end < body.len()
                && (body[end].is_alphanumeric() || body[end] == '_' || body[end] == ':')
            {
                end += 1;
            }

            let raw_ident: String = body[i..end].iter().collect();

            // Check if followed by '(' to qualify as a function call
            let mut scan = end;
            while scan < body.len() && body[scan].is_whitespace() {
                scan += 1;
            }

            if scan < body.len() && body[scan] == '(' {
                // Ensure it's not a keyword
                if !keywords.contains(&raw_ident.as_str()) {
                    let qualified = if raw_ident.contains("::") {
                        raw_ident.clone()
                    } else {
                        format!("{}::{}", current_module, raw_ident)
                    };
                    if !calls.contains(&qualified) {
                        calls.push(qualified);
                    }
                }
            }
            i = end;
            continue;
        }
        i += 1;
    }
    calls
}

/// Parses module-level `use` statements.
fn parse_use_statement(src: &str) -> Option<UseStatement> {
    let clean = src.trim();
    if clean.is_empty() {
        return None;
    }

    // Check if there are curly braces for nested members
    if let Some(brace_start) = clean.find('{') {
        let prefix = clean[..brace_start].trim();
        // Remove trailing :: if any
        let prefix = prefix.strip_suffix("::").unwrap_or(prefix).trim();

        let brace_end = clean.find('}')?;
        let members_str = &clean[brace_start + 1..brace_end];

        let prefix_parts: Vec<&str> = prefix.split("::").collect();
        let (address, module) = if prefix_parts.len() == 1 {
            (None, prefix_parts[0].to_string())
        } else if prefix_parts.len() == 2 {
            (
                Some(prefix_parts[0].to_string()),
                prefix_parts[1].to_string(),
            )
        } else {
            (
                Some(prefix_parts[0].to_string()),
                prefix_parts[1..].join("::"),
            )
        };

        let mut members = Vec::new();
        for m_part in members_str.split(',') {
            let m_clean = m_part.trim();
            if m_clean.is_empty() {
                continue;
            }
            if m_clean.contains("as") {
                let parts: Vec<&str> = m_clean.split("as").collect();
                if parts.len() == 2 {
                    members.push(UseMember {
                        name: parts[0].trim().to_string(),
                        alias: Some(parts[1].trim().to_string()),
                    });
                }
            } else {
                members.push(UseMember {
                    name: m_clean.to_string(),
                    alias: None,
                });
            }
        }

        return Some(UseStatement {
            address,
            module,
            members,
        });
    }

    // Single item use statement (e.g., use sui::transfer; or use sui::tx_context::TxContext;)
    let mut alias = None;
    let mut main_path = clean;
    if clean.contains("as") {
        let parts: Vec<&str> = clean.split("as").collect();
        if parts.len() == 2 {
            main_path = parts[0].trim();
            alias = Some(parts[1].trim().to_string());
        }
    }

    let parts: Vec<&str> = main_path.split("::").collect();
    if parts.is_empty() {
        return None;
    }

    if parts.len() == 1 {
        Some(UseStatement {
            address: None,
            module: parts[0].to_string(),
            members: vec![],
        })
    } else if parts.len() == 2 {
        let mod_name = parts[1].to_string();
        let use_members = if let Some(ref a) = alias {
            vec![UseMember {
                name: "Self".to_string(),
                alias: Some(a.clone()),
            }]
        } else {
            vec![]
        };
        Some(UseStatement {
            address: Some(parts[0].to_string()),
            module: mod_name,
            members: use_members,
        })
    } else {
        let address = Some(parts[0].to_string());
        let module = parts[1].to_string();
        let member_name = parts[2..].join("::");
        Some(UseStatement {
            address,
            module,
            members: vec![UseMember {
                name: member_name,
                alias,
            }],
        })
    }
}

/// Parses function body into a list of AstNodes representing statements.
fn parse_statements(src: &str) -> Vec<AstNode> {
    let mut statements = Vec::new();
    let chars: Vec<char> = src.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        // Skip whitespace
        while i < chars.len() && chars[i].is_whitespace() {
            i += 1;
        }
        if i >= chars.len() {
            break;
        }

        // Check for 'let' keyword
        if is_word_at(&chars, i, "let") {
            let mut end = i + 3;
            let mut depth = 0;
            while end < chars.len() {
                if chars[end] == '(' || chars[end] == '{' {
                    depth += 1;
                } else if chars[end] == ')' || chars[end] == '}' {
                    depth -= 1;
                } else if chars[end] == ';' && depth == 0 {
                    break;
                }
                end += 1;
            }
            if end < chars.len() {
                let let_stmt: String = chars[i..end].iter().collect();
                if let Some(node) = parse_let_statement(&let_stmt) {
                    statements.push(node);
                }
                i = end + 1;
                continue;
            }
        }

        // Check for 'if' keyword
        if is_word_at(&chars, i, "if") {
            let mut scan = i + 2;
            while scan < chars.len() && chars[scan].is_whitespace() {
                scan += 1;
            }
            let mut cond_end = scan;
            while cond_end < chars.len() && chars[cond_end] != '{' {
                cond_end += 1;
            }
            if cond_end < chars.len() {
                let cond_str: String = chars[scan..cond_end].iter().collect();
                let cond_node = parse_expression(&cond_str);

                if let Some((then_start, then_end)) = find_matching_block(&chars, cond_end) {
                    let then_str: String = chars[then_start..then_end].iter().collect();
                    let then_branch = AstNode::Block(parse_statements(&then_str));

                    let mut else_scan = then_end + 1;
                    while else_scan < chars.len() && chars[else_scan].is_whitespace() {
                        else_scan += 1;
                    }
                    let mut else_branch = None;
                    if else_scan < chars.len() && is_word_at(&chars, else_scan, "else") {
                        let mut else_block_start = else_scan + 4;
                        while else_block_start < chars.len()
                            && chars[else_block_start].is_whitespace()
                        {
                            else_block_start += 1;
                        }
                        if else_block_start < chars.len() {
                            if chars[else_block_start] == '{' {
                                if let Some((eb_start, eb_end)) =
                                    find_matching_block(&chars, else_block_start)
                                {
                                    let else_str: String = chars[eb_start..eb_end].iter().collect();
                                    else_branch =
                                        Some(Box::new(AstNode::Block(parse_statements(&else_str))));
                                    statements.push(AstNode::If {
                                        condition: Box::new(cond_node),
                                        then_branch: Box::new(then_branch),
                                        else_branch,
                                    });
                                    i = eb_end + 1;
                                    continue;
                                }
                            } else if is_word_at(&chars, else_block_start, "if") {
                                let sub_statements = parse_statements(
                                    &chars[else_block_start..].iter().collect::<String>(),
                                );
                                if !sub_statements.is_empty() {
                                    else_branch = Some(Box::new(sub_statements[0].clone()));
                                }
                                statements.push(AstNode::If {
                                    condition: Box::new(cond_node),
                                    then_branch: Box::new(then_branch),
                                    else_branch,
                                });
                                i = chars.len();
                                continue;
                            }
                        }
                    }

                    statements.push(AstNode::If {
                        condition: Box::new(cond_node),
                        then_branch: Box::new(then_branch),
                        else_branch,
                    });
                    i = then_end + 1;
                    continue;
                }
            }
        }

        // Check for 'while' keyword
        if is_word_at(&chars, i, "while") {
            let mut scan = i + 5;
            while scan < chars.len() && chars[scan].is_whitespace() {
                scan += 1;
            }
            let mut cond_end = scan;
            while cond_end < chars.len() && chars[cond_end] != '{' {
                cond_end += 1;
            }
            if cond_end < chars.len() {
                let cond_str: String = chars[scan..cond_end].iter().collect();
                let cond_node = parse_expression(&cond_str);

                if let Some((body_start, body_end)) = find_matching_block(&chars, cond_end) {
                    let body_str: String = chars[body_start..body_end].iter().collect();
                    let body_node = AstNode::Block(parse_statements(&body_str));

                    statements.push(AstNode::While {
                        condition: Box::new(cond_node),
                        body: Box::new(body_node),
                    });
                    i = body_end + 1;
                    continue;
                }
            }
        }

        // Check for 'loop' keyword
        if is_word_at(&chars, i, "loop") {
            let mut scan = i + 4;
            while scan < chars.len() && chars[scan].is_whitespace() {
                scan += 1;
            }
            if scan < chars.len() && chars[scan] == '{' {
                if let Some((body_start, body_end)) = find_matching_block(&chars, scan) {
                    let body_str: String = chars[body_start..body_end].iter().collect();
                    let body_node = AstNode::Block(parse_statements(&body_str));

                    statements.push(AstNode::Loop {
                        body: Box::new(body_node),
                    });
                    i = body_end + 1;
                    continue;
                }
            }
        }

        // Default statement: parse up to semicolon
        let mut end = i;
        let mut depth = 0;
        while end < chars.len() {
            if chars[end] == '(' || chars[end] == '{' {
                depth += 1;
            } else if chars[end] == ')' || chars[end] == '}' {
                depth -= 1;
            } else if chars[end] == ';' && depth == 0 {
                break;
            }
            end += 1;
        }

        if end > i {
            let stmt_str: String = chars[i..end].iter().collect();
            let clean_stmt = stmt_str.trim();
            if !clean_stmt.is_empty() {
                if clean_stmt.contains('=') && !clean_stmt.contains("==") {
                    if let Some(node) = parse_assignment_statement(clean_stmt) {
                        statements.push(node);
                    } else {
                        statements.push(parse_expression(clean_stmt));
                    }
                } else {
                    statements.push(parse_expression(clean_stmt));
                }
            }
            i = end + 1;
        } else {
            i += 1;
        }
    }

    statements
}

/// Parses local let statement.
fn parse_let_statement(src: &str) -> Option<AstNode> {
    let clean = src.trim().strip_prefix("let")?.trim();
    let mut is_mut = false;
    let mut rest = clean;
    if let Some(stripped) = clean.strip_prefix("mut") {
        is_mut = true;
        rest = stripped.trim();
    }

    let mut name_end = 0;
    let chars: Vec<char> = rest.chars().collect();
    while name_end < chars.len() {
        let c = chars[name_end];
        if c == ':' || c == '=' || c.is_whitespace() {
            break;
        }
        name_end += 1;
    }

    if name_end == 0 {
        return None;
    }

    let name: String = chars[0..name_end].iter().collect();
    let mut rest_str = chars[name_end..].iter().collect::<String>();
    rest_str = rest_str.trim().to_string();

    let mut type_annotation = None;
    if rest_str.starts_with(':') {
        let mut type_end = 0;
        let rest_chars: Vec<char> = rest_str.chars().collect();
        while type_end < rest_chars.len() && rest_chars[type_end] != '=' {
            type_end += 1;
        }
        let parsed_type = rest_chars[1..type_end]
            .iter()
            .collect::<String>()
            .trim()
            .to_string();
        type_annotation = Some(parsed_type);
        if type_end < rest_chars.len() {
            rest_str = rest_chars[type_end..].iter().collect::<String>();
        } else {
            rest_str = String::new();
        }
    }

    let mut initializer = None;
    if let Some(stripped) = rest_str.strip_prefix('=') {
        let expr_str = stripped.trim();
        initializer = Some(Box::new(parse_expression(expr_str)));
    }

    Some(AstNode::Let {
        name,
        is_mut,
        type_annotation,
        initializer,
    })
}

/// Parses an assignment.
fn parse_assignment_statement(src: &str) -> Option<AstNode> {
    let parts: Vec<&str> = src.splitn(2, '=').collect();
    if parts.len() == 2 {
        let target = parts[0].trim().to_string();
        let val_str = parts[1].trim();
        return Some(AstNode::Assign {
            target,
            value: Box::new(parse_expression(val_str)),
        });
    }
    None
}

/// Parses nested expression structures.
fn parse_expression(src: &str) -> AstNode {
    let clean = src.trim();
    if clean.is_empty() {
        return AstNode::Expression(String::new());
    }

    if clean.ends_with(')') {
        if let Some(paren_idx) = clean.find('(') {
            let func_name = clean[..paren_idx].trim();
            if func_name
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == ':')
            {
                let args_str = &clean[paren_idx + 1..clean.len() - 1];
                let mut arguments = Vec::new();

                let mut current_arg = String::new();
                let mut depth = 0;
                for c in args_str.chars() {
                    if c == '(' || c == '{' || c == '[' {
                        depth += 1;
                        current_arg.push(c);
                    } else if c == ')' || c == '}' || c == ']' {
                        depth -= 1;
                        current_arg.push(c);
                    } else if c == ',' && depth == 0 {
                        arguments.push(parse_expression(&current_arg));
                        current_arg.clear();
                    } else {
                        current_arg.push(c);
                    }
                }
                if !current_arg.trim().is_empty() {
                    arguments.push(parse_expression(&current_arg));
                }

                return AstNode::Call {
                    function_name: func_name.to_string(),
                    arguments,
                };
            }
        }
    }

    if clean.chars().next().is_some_and(|c| c.is_numeric())
        || clean == "true"
        || clean == "false"
        || clean.starts_with("0x")
    {
        return AstNode::Literal(clean.to_string());
    }

    if clean
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == ':')
    {
        return AstNode::Identifier(clean.to_string());
    }

    AstNode::Expression(clean.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_comments() {
        let src = "module foo { // this is a comment\n /* block comment */ fun bar() {} }";
        let stripped = strip_comments(src);
        assert!(!stripped.contains("comment"));
        assert!(stripped.contains("module foo"));
        assert!(stripped.contains("fun bar"));
    }

    #[test]
    fn comments_inside_strings_and_nested_comments_are_preserved_correctly() {
        let src = r#"module foo { /* outer /* nested */ done */ fun bar() { let s = "/* not a comment */ // still text"; } }"#;
        let stripped = strip_comments(src);
        assert!(stripped.contains(r#""/* not a comment */ // still text""#));
        assert!(!stripped.contains("nested"));

        let chars: Vec<char> = stripped.chars().collect();
        let module_block = find_matching_block(&chars, 0).unwrap();
        assert_eq!(chars[module_block.1], '}');
    }

    #[test]
    fn matching_blocks_ignore_braces_in_strings() {
        let chars: Vec<char> = r#"{ let s = "}"; { } }"#.chars().collect();
        let (body_start, body_end) = find_matching_block(&chars, 0).unwrap();
        assert_eq!(
            chars[body_start..body_end].iter().collect::<String>(),
            r#" let s = "}"; { } "#
        );
    }

    #[test]
    fn test_calculate_complexity() {
        let body_chars: Vec<char> = "if (true) { while (false) { loop { if (x && y || z) {} } } }"
            .chars()
            .collect();
        // 1 (start) + if (1) + while (1) + loop (1) + if (1) + && (1) + || (1) = 7
        let complexity = calculate_complexity(&body_chars);
        assert_eq!(complexity, 7);
    }

    #[test]
    fn test_extract_calls() {
        let body_chars: Vec<char> = "clean::calculate_fee(10); foo(20); if (true) { bar(); }"
            .chars()
            .collect();
        let calls = extract_calls(&body_chars, "my_module");
        assert!(calls.contains(&"clean::calculate_fee".to_string()));
        assert!(calls.contains(&"my_module::foo".to_string()));
        assert!(calls.contains(&"my_module::bar".to_string()));
        assert!(!calls.contains(&"if".to_string()));
    }

    #[test]
    fn test_parse_use_statements() {
        let use_single = parse_use_statement("sui::transfer").unwrap();
        assert_eq!(use_single.module, "transfer");
        assert_eq!(use_single.address, Some("sui".to_string()));

        let use_nested = parse_use_statement("sui::tx_context::{Self, TxContext as Ctx}").unwrap();
        assert_eq!(use_nested.module, "tx_context");
        assert_eq!(use_nested.members.len(), 2);
        assert_eq!(use_nested.members[0].name, "Self");
        assert_eq!(use_nested.members[1].name, "TxContext");
        assert_eq!(use_nested.members[1].alias, Some("Ctx".to_string()));
    }

    #[test]
    fn test_parse_statements() {
        let body = "let mut x: u64 = 5; x = 10; transfer::public_transfer(x, recipient);";
        let stmts = parse_statements(body);
        assert_eq!(stmts.len(), 3);
        match &stmts[0] {
            AstNode::Let {
                name,
                is_mut,
                type_annotation,
                initializer,
            } => {
                assert_eq!(name, "x");
                assert!(is_mut);
                assert_eq!(type_annotation.as_deref(), Some("u64"));
                assert!(initializer.is_some());
            }
            _ => panic!("Expected Let"),
        }

        match &stmts[1] {
            AstNode::Assign { target, value } => {
                assert_eq!(target, "x");
                assert_eq!(**value, AstNode::Literal("10".to_string()));
            }
            _ => panic!("Expected Assign"),
        }

        match &stmts[2] {
            AstNode::Call {
                function_name,
                arguments,
            } => {
                assert_eq!(function_name, "transfer::public_transfer");
                assert_eq!(arguments.len(), 2);
            }
            _ => panic!("Expected Call"),
        }
    }
}
