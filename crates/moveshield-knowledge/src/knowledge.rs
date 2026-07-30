//! Reusable engineering knowledge store.

use moveshield_core::models::Project;
use moveshield_graph::{CallGraph, ControlFlowGraph};
use moveshield_ir::{resolve_type_path, FunctionSemantic, MoveIR, ObjectState, TransferEvent};

/// Represents computed engineering knowledge (facts and relationship graphs).
pub struct KnowledgeStore {
    /// The Call Graph computed from the MoveIR.
    pub call_graph: CallGraph,
    /// The canonical CDM Project containing modules, functions, and structs.
    pub project: Project,
    /// The computed MoveIR containing the semantic analysis layer.
    pub ir: MoveIR,
}

impl KnowledgeStore {
    /// Creates a new KnowledgeStore with the given Call Graph and Project.
    pub fn new(call_graph: CallGraph, project: Project) -> Self {
        let ir = MoveIR::from_project(&project);
        Self {
            call_graph,
            project,
            ir,
        }
    }

    /// Gets the pre-analyzed semantic layer for a specific function.
    pub fn get_function_semantic(
        &self,
        address: &str,
        module: &str,
        function: &str,
    ) -> Option<&FunctionSemantic> {
        let fq_name = format!("{}::{}::{}", address, module, function);
        self.ir.function_semantics.get(&fq_name)
    }

    /// Resolves a symbol within a module.
    pub fn resolve_symbol(&self, address: &str, module: &str, symbol: &str) -> String {
        let module_key = format!("{}::{}", address, module);
        if let Some(m) = self.ir.project.modules.get(&module_key) {
            resolve_type_path(symbol, &m.name, &m.address, &m.use_statements)
        } else {
            symbol.to_string()
        }
    }

    /// Checks if a type represents a Capability.
    pub fn is_capability(&self, type_str: &str) -> bool {
        type_str.contains("Cap") || type_str.contains("Capability")
    }

    /// Checks if a function's parameters has any capability type.
    pub fn has_capability_param(&self, address: &str, module: &str, function: &str) -> bool {
        self.get_function_semantic(address, module, function)
            .is_some_and(|sem| sem.has_capability_param)
    }

    /// Checks if a function has a TxContext parameter.
    pub fn has_tx_context_param(&self, address: &str, module: &str, function: &str) -> bool {
        self.get_function_semantic(address, module, function)
            .is_some_and(|sem| sem.has_tx_context_param)
    }

    /// Checks if a function has a Signer parameter.
    pub fn has_signer_param(&self, address: &str, module: &str, function: &str) -> bool {
        self.get_function_semantic(address, module, function)
            .is_some_and(|sem| sem.has_signer_param)
    }

    /// Returns list of asset or capability transfer events inside a function.
    pub fn get_transfers(&self, address: &str, module: &str, function: &str) -> Vec<TransferEvent> {
        self.get_function_semantic(address, module, function)
            .map_or(Vec::new(), |sem| sem.transfers.clone())
    }

    /// Returns the resolved calls made inside a function.
    pub fn get_resolved_calls(&self, address: &str, module: &str, function: &str) -> Vec<String> {
        self.get_function_semantic(address, module, function)
            .map_or(Vec::new(), |sem| sem.resolved_calls.clone())
    }

    /// Returns the Control Flow Graph (CFG) for a specific function.
    pub fn get_cfg(&self, address: &str, module: &str, function: &str) -> Option<ControlFlowGraph> {
        let module_key = format!("{}::{}", address, module);
        let m = self.ir.project.modules.get(&module_key)?;
        let func = m.functions.get(function)?;
        Some(ControlFlowGraph::build(&func.ast))
    }

    /// Verifies if a function has an access control guard, either on itself or propagated from callers (1-level).
    pub fn is_access_controlled(&self, address: &str, module: &str, function: &str) -> bool {
        // Direct checks
        if self.has_capability_param(address, module, function)
            || self.has_tx_context_param(address, module, function)
            || self.has_signer_param(address, module, function)
        {
            return true;
        }

        // Interprocedural 1-level callers check
        let fq_name = format!("{}::{}::{}", address, module, function);
        let callers = self.call_graph.get_callers(&fq_name);
        if !callers.is_empty() {
            let mut all_callers_guarded = true;
            for caller in callers {
                let parts: Vec<&str> = caller.split("::").collect();
                if parts.len() == 3 {
                    if !self.has_capability_param(parts[0], parts[1], parts[2])
                        && !self.has_tx_context_param(parts[0], parts[1], parts[2])
                        && !self.has_signer_param(parts[0], parts[1], parts[2])
                    {
                        all_callers_guarded = false;
                        break;
                    }
                } else {
                    all_callers_guarded = false;
                    break;
                }
            }
            if all_callers_guarded {
                return true;
            }
        }

        false
    }

    /// Gets the semantic lifecycle state of a variable or its original alias in a function.
    pub fn get_variable_state(
        &self,
        address: &str,
        module: &str,
        function: &str,
        var_name: &str,
    ) -> Option<ObjectState> {
        let sem = self.get_function_semantic(address, module, function)?;
        // Resolve alias recursively first
        let mut current = var_name.to_string();
        while let Some(parent) = sem.aliases.get(&current) {
            current = parent.clone();
        }
        sem.object_states.get(&current).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use moveshield_core::models::{AstNode, Location, Module, Package, Project};
    use std::path::PathBuf;

    #[test]
    fn test_knowledge_store_interprocedural() {
        // Prepare dummy project structure
        // Module with secure and insecure functions
        let project = Project {
            name: "test_project".to_string(),
            root_path: PathBuf::from("."),
            packages: vec![Package {
                name: "test_pkg".to_string(),
                path: PathBuf::from("."),
                modules: vec![Module {
                    name: "test_mod".to_string(),
                    address: "0x42".to_string(),
                    source_file: PathBuf::from("test.move"),
                    use_statements: Vec::new(),
                    structs: Vec::new(),
                    functions: vec![
                        moveshield_core::models::Function {
                            name: "caller_secure".to_string(),
                            visibility: "public".to_string(),
                            location: Location {
                                file: PathBuf::from("test.move"),
                                start_line: 1,
                                start_column: 1,
                                end_line: 1,
                                end_column: 1,
                            },
                            complexity: 1,
                            calls: vec!["test_mod::callee".to_string()],
                            parameters: vec!["cap: &AdminCap".to_string()],
                            body_text: "test_mod::callee()".to_string(),
                            ast: Some(AstNode::Block(vec![AstNode::Call {
                                function_name: "test_mod::callee".to_string(),
                                arguments: Vec::new(),
                            }])),
                        },
                        moveshield_core::models::Function {
                            name: "callee".to_string(),
                            visibility: "private".to_string(),
                            location: Location {
                                file: PathBuf::from("test.move"),
                                start_line: 5,
                                start_column: 1,
                                end_line: 5,
                                end_column: 1,
                            },
                            complexity: 1,
                            calls: Vec::new(),
                            parameters: Vec::new(),
                            body_text: "".to_string(),
                            ast: Some(AstNode::Block(Vec::new())),
                        },
                    ],
                }],
            }],
        };

        let ir = MoveIR::from_project(&project);
        let cg = CallGraph::build(ir);
        let store = KnowledgeStore::new(cg, project);

        // Directly checks caller
        assert!(store.is_access_controlled("0x42", "test_mod", "caller_secure"));
        // Interprocedurally checks callee (since only caller_secure calls it, and caller_secure is guarded)
        assert!(store.is_access_controlled("0x42", "test_mod", "callee"));
    }
}
