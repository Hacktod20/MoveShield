//! Reusable engineering knowledge store.

use moveshield_core::models::Project;
use moveshield_graph::CallGraph;
use moveshield_ir::{resolve_type_path, FunctionSemantic, MoveIR, TransferEvent};

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
}
