//! Reusable engineering knowledge store.

use moveshield_core::models::Project;
use moveshield_graph::CallGraph;

/// Represents computed engineering knowledge (facts and relationship graphs).
pub struct KnowledgeStore {
    /// The Call Graph computed from the MoveIR.
    pub call_graph: CallGraph,
    /// The canonical CDM Project containing modules, functions, and structs.
    pub project: Project,
}

impl KnowledgeStore {
    /// Creates a new KnowledgeStore with the given Call Graph and Project.
    pub fn new(call_graph: CallGraph, project: Project) -> Self {
        Self {
            call_graph,
            project,
        }
    }
}
