//! Compiler Adapter trait and Development Parser fallback implementation.

use moveshield_core::errors::{Error, Result};
use moveshield_core::models::{Field, Function, Location, Module, Package, Project, Struct};
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
    let mut result = String::new();
    let chars: Vec<char> = src.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if i + 1 < chars.len() && chars[i] == '/' && chars[i + 1] == '/' {
            i += 2;
            while i < chars.len() && chars[i] != '\n' {
                i += 1;
            }
        } else if i + 1 < chars.len() && chars[i] == '/' && chars[i + 1] == '*' {
            i += 2;
            while i + 1 < chars.len() && !(chars[i] == '*' && chars[i + 1] == '/') {
                i += 1;
            }
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
    let mut depth = 0;
    let mut first_brace = None;
    for (idx, &c) in chars.iter().enumerate().skip(start_idx) {
        if c == '{' {
            if depth == 0 {
                first_brace = Some(idx);
            }
            depth += 1;
        } else if c == '}' {
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
                    let (functions, structs) =
                        parse_module_contents(module_block, relative_path, &module_name)?;

                    modules.push(Module {
                        name: module_name,
                        address: resolved_address,
                        source_file: relative_path.to_path_buf(),
                        functions,
                        structs,
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

/// Parses the contents of a module to extract functions and structs.
fn parse_module_contents(
    chars: &[char],
    relative_path: &Path,
    current_module: &str,
) -> Result<(Vec<Function>, Vec<Struct>)> {
    let mut functions = Vec::new();
    let mut structs = Vec::new();

    let mut i = 0;
    while i < chars.len() {
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

                if found_brace {
                    if let Some((block_start, block_end)) = find_matching_block(chars, scan_idx) {
                        let body_chars = &chars[block_start..block_end];
                        let complexity = calculate_complexity(body_chars);
                        let calls = extract_calls(body_chars, current_module);

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
                    });
                    i = scan_idx + 1;
                    continue;
                }
            }
        }
        i += 1;
    }

    Ok((functions, structs))
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
}
