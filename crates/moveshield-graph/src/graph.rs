//! Call Graph construction and traversal engine.

use moveshield_ir::{MoveFunction, MoveIR};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Represents a Call Graph built from MoveIR.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallGraph {
    /// Mapping from caller (fully-qualified ID) to its callees (fully-qualified IDs).
    pub callees: HashMap<String, Vec<String>>,
    /// Mapping from callee (fully-qualified ID) to its callers (fully-qualified IDs).
    pub callers: HashMap<String, Vec<String>>,
    /// All defined function fully-qualified IDs in the project.
    pub all_functions: HashSet<String>,
    /// Metadata about each function.
    pub function_metadata: HashMap<String, MoveFunction>,
}

impl CallGraph {
    /// Builds an immutable CallGraph from MoveIR.
    pub fn build(ir: MoveIR) -> Self {
        let mut all_functions = HashSet::new();
        let mut function_metadata = HashMap::new();

        // 1. Collect all defined functions with their fully qualified IDs.
        for (module_key, module) in &ir.project.modules {
            for (fn_name, function) in &module.functions {
                let fq_id = format!("{}::{}", module_key, fn_name);
                all_functions.insert(fq_id.clone());
                function_metadata.insert(fq_id.clone(), function.clone());
            }
        }

        let mut callees: HashMap<String, Vec<String>> = HashMap::new();
        let mut callers: HashMap<String, Vec<String>> = HashMap::new();

        // Initialize maps
        for fq_id in &all_functions {
            callees.insert(fq_id.clone(), Vec::new());
            callers.insert(fq_id.clone(), Vec::new());
        }

        // 2. Resolve every call edge and populate callees and callers maps.
        for (module_key, module) in &ir.project.modules {
            for (fn_name, function) in &module.functions {
                let caller_fq_id = format!("{}::{}", module_key, fn_name);

                for raw_call in &function.calls {
                    if let Some(callee_fq_id) =
                        resolve_call(&module.address, &module.name, raw_call, &all_functions, &ir)
                    {
                        // Avoid adding duplicate call edges
                        let out_list = callees.entry(caller_fq_id.clone()).or_default();
                        if !out_list.contains(&callee_fq_id) {
                            out_list.push(callee_fq_id.clone());
                        }

                        let in_list = callers.entry(callee_fq_id.clone()).or_default();
                        if !in_list.contains(&caller_fq_id) {
                            in_list.push(caller_fq_id.clone());
                        }
                    }
                }
            }
        }

        Self {
            callees,
            callers,
            all_functions,
            function_metadata,
        }
    }

    /// Returns the list of functions that the given function calls.
    pub fn get_callees(&self, caller: &str) -> &[String] {
        self.callees
            .get(caller)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Returns the list of functions that call the given function.
    pub fn get_callers(&self, callee: &str) -> &[String] {
        self.callers
            .get(callee)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Checks if a function is recursive (contains a path to itself).
    pub fn is_recursive(&self, func: &str) -> bool {
        let mut visited = HashSet::new();
        let mut queue = Vec::new();

        // Initialize with direct callees
        if let Some(direct_callees) = self.callees.get(func) {
            for callee in direct_callees {
                if callee == func {
                    return true;
                }
                queue.push(callee);
            }
        }

        while let Some(current) = queue.pop() {
            if current == func {
                return true;
            }
            if visited.insert(current) {
                if let Some(next_callees) = self.callees.get(current) {
                    for callee in next_callees {
                        queue.push(callee);
                    }
                }
            }
        }

        false
    }

    /// Returns all functions identified as "entry" functions.
    pub fn get_entry_points(&self) -> Vec<String> {
        let mut entries = Vec::new();
        for fq_id in &self.all_functions {
            if let Some(func) = self.function_metadata.get(fq_id) {
                if func.visibility == "entry" {
                    entries.push(fq_id.clone());
                }
            }
        }
        entries
    }

    /// Returns any defined functions that are never called (incoming callers is empty).
    pub fn get_unused_functions(&self) -> Vec<String> {
        let mut unused = Vec::new();
        for fq_id in &self.all_functions {
            if let Some(incoming) = self.callers.get(fq_id) {
                if incoming.is_empty() {
                    // For Milestone 1, we also verify if it's not an entry point
                    if let Some(func) = self.function_metadata.get(fq_id) {
                        if func.visibility != "entry" && func.name != "init" {
                            unused.push(fq_id.clone());
                        }
                    }
                }
            }
        }
        unused
    }
}

/// Resolves a raw call target string into a fully qualified function ID ("address::module::function").
fn resolve_call(
    caller_address: &str,
    caller_module: &str,
    target_raw: &str,
    all_functions: &HashSet<String>,
    ir: &MoveIR,
) -> Option<String> {
    // 1. Exact match in all functions
    if all_functions.contains(target_raw) {
        return Some(target_raw.to_string());
    }

    let parts: Vec<&str> = target_raw.split("::").collect();

    // 2. Local call e.g., "foo" -> "address::module::foo"
    if parts.len() == 1 {
        let fq_id = format!("{}::{}::{}", caller_address, caller_module, target_raw);
        if all_functions.contains(&fq_id) {
            return Some(fq_id);
        }
    }

    // 3. Partially qualified call, e.g., "module_name::foo"
    if parts.len() == 2 {
        let module_name = parts[0];
        let fn_name = parts[1];

        // Let's search if there's a module matching module_name in the project
        for (module_key, module) in &ir.project.modules {
            if module.name == module_name {
                let fq_id = format!("{}::{}", module_key, fn_name);
                if all_functions.contains(&fq_id) {
                    return Some(fq_id);
                }
            }
        }
    }

    // 4. Fully qualified call with named address, e.g., "named_addr::module_name::foo"
    if parts.len() == 3 {
        // Since the compiler/adapter resolves named addresses, let's search if we find a match
        // by looking at modules with matching name and function name
        let module_name = parts[1];
        let fn_name = parts[2];
        for (module_key, module) in &ir.project.modules {
            if module.name == module_name {
                let fq_id = format!("{}::{}", module_key, fn_name);
                if all_functions.contains(&fq_id) {
                    return Some(fq_id);
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use moveshield_core::models::Location;
    use moveshield_ir::{MoveFunction, MoveIR, MoveModule, MoveProject};

    #[test]
    fn test_call_graph_basic() {
        let mut clean_fns = HashMap::new();

        // Define clean::get_base_fee (no calls)
        clean_fns.insert(
            "get_base_fee".to_string(),
            MoveFunction {
                name: "get_base_fee".to_string(),
                visibility: "public".to_string(),
                location: Location {
                    file: std::path::PathBuf::from("clean.move"),
                    start_line: 1,
                    start_column: 1,
                    end_line: 1,
                    end_column: 1,
                },
                complexity: 1,
                calls: Vec::new(),
            },
        );

        let mut complex_fns = HashMap::new();

        // Define complex::transfer (calls clean::get_base_fee, self recursive)
        complex_fns.insert(
            "transfer".to_string(),
            MoveFunction {
                name: "transfer".to_string(),
                visibility: "entry".to_string(),
                location: Location {
                    file: std::path::PathBuf::from("complex.move"),
                    start_line: 1,
                    start_column: 1,
                    end_line: 1,
                    end_column: 1,
                },
                complexity: 12,
                calls: vec![
                    "clean::get_base_fee".to_string(),
                    "complex::transfer".to_string(), // recursive!
                ],
            },
        );

        let mut modules = HashMap::new();
        modules.insert(
            "0x42::clean".to_string(),
            MoveModule {
                name: "clean".to_string(),
                address: "0x42".to_string(),
                functions: clean_fns,
            },
        );
        modules.insert(
            "0x42::complex".to_string(),
            MoveModule {
                name: "complex".to_string(),
                address: "0x42".to_string(),
                functions: complex_fns,
            },
        );

        let ir = MoveIR {
            project: MoveProject {
                name: "test_project".to_string(),
                modules,
            },
        };

        let graph = CallGraph::build(ir);

        // Verify total defined functions
        assert_eq!(graph.all_functions.len(), 2);
        assert!(graph.all_functions.contains("0x42::complex::transfer"));
        assert!(graph.all_functions.contains("0x42::clean::get_base_fee"));

        // Verify recursive detection
        assert!(graph.is_recursive("0x42::complex::transfer"));
        assert!(!graph.is_recursive("0x42::clean::get_base_fee"));

        // Verify entry points
        let entries = graph.get_entry_points();
        assert!(entries.contains(&"0x42::complex::transfer".to_string()));
    }
}
