//! Reusable engineering knowledge store.

use moveshield_graph::CallGraph;

/// Represents computed engineering knowledge (facts and relationship graphs).
pub struct KnowledgeStore {
    /// The Call Graph computed from the MoveIR.
    pub call_graph: CallGraph,
}

impl KnowledgeStore {
    /// Creates a new KnowledgeStore with the given Call Graph.
    pub fn new(call_graph: CallGraph) -> Self {
        Self { call_graph }
    }
}
