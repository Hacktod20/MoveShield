//! Control Flow Graph (CFG) representation and construction.

use moveshield_core::models::AstNode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Kind of a node in the Control Flow Graph.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CfgNodeKind {
    /// Function entrance.
    Entry,
    /// A single statement (e.g., Let, Assign, Call, or Expression).
    Statement(AstNode),
    /// A branch condition (e.g., condition of an If or While statement).
    BranchCondition(AstNode),
    /// Dummy node representing a loop/branch junction or exit.
    Nop,
    /// Normal return from function.
    Exit,
    /// Explicit execution abort.
    Abort,
}

/// A node in the Control Flow Graph.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CfgNode {
    /// Unique identifier for this node.
    pub id: usize,
    /// The specific kind of CFG node.
    pub kind: CfgNodeKind,
}

/// Kind of an edge in the Control Flow Graph.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CfgEdgeKind {
    /// Default sequential execution flow.
    Sequential,
    /// Path taken when a conditional branch is true.
    TrueBranch,
    /// Path taken when a conditional branch is false.
    FalseBranch,
    /// Back-edge in a loop.
    LoopBack,
    /// Unconditional jump/transfer of control.
    Unconditional,
}

/// A directed edge connecting two CFG nodes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CfgEdge {
    /// Origin node ID.
    pub from: usize,
    /// Destination node ID.
    pub to: usize,
    /// Edge type.
    pub kind: CfgEdgeKind,
}

/// Representation of a Control Flow Graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlFlowGraph {
    /// Nodes in the CFG, mapped by their unique ID.
    pub nodes: HashMap<usize, CfgNode>,
    /// Directed edges in the CFG.
    pub edges: Vec<CfgEdge>,
    /// The starting node ID of the CFG.
    pub entry_id: usize,
    /// The exiting node ID representing normal return.
    pub exit_id: usize,
    /// The abort node ID representing failure exit.
    pub abort_id: usize,
}

impl ControlFlowGraph {
    /// Builds a Control Flow Graph from a function AST.
    pub fn build(body_ast: &Option<AstNode>) -> Self {
        let mut builder = CfgBuilder::new();
        let entry_id = builder.entry_id;
        let mut active_tails = vec![entry_id];

        if let Some(ref ast) = body_ast {
            active_tails = builder.build_node(ast, active_tails);
        }

        // Connect remaining active tails to normal exit
        for tail in active_tails {
            builder.add_edge(tail, builder.exit_id, CfgEdgeKind::Sequential);
        }

        ControlFlowGraph {
            nodes: builder.nodes,
            edges: builder.edges,
            entry_id,
            exit_id: builder.exit_id,
            abort_id: builder.abort_id,
        }
    }

    /// Helper to find all nodes reachable from the entry node.
    pub fn get_reachable_nodes(&self) -> Vec<usize> {
        let mut reachable = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut queue = vec![self.entry_id];

        while let Some(node_id) = queue.pop() {
            if visited.insert(node_id) {
                reachable.push(node_id);
                for edge in &self.edges {
                    if edge.from == node_id {
                        queue.push(edge.to);
                    }
                }
            }
        }

        reachable
    }
}

struct CfgBuilder {
    nodes: HashMap<usize, CfgNode>,
    edges: Vec<CfgEdge>,
    next_id: usize,
    loop_stack: Vec<(usize, usize)>, // (continue_node_id, break_node_id)
    entry_id: usize,
    exit_id: usize,
    abort_id: usize,
}

impl CfgBuilder {
    fn new() -> Self {
        let mut nodes = HashMap::new();

        let entry_node = CfgNode {
            id: 0,
            kind: CfgNodeKind::Entry,
        };
        nodes.insert(0, entry_node);

        let exit_node = CfgNode {
            id: 1,
            kind: CfgNodeKind::Exit,
        };
        nodes.insert(1, exit_node);

        let abort_node = CfgNode {
            id: 2,
            kind: CfgNodeKind::Abort,
        };
        nodes.insert(2, abort_node);

        Self {
            nodes,
            edges: Vec::new(),
            next_id: 3,
            loop_stack: Vec::new(),
            entry_id: 0,
            exit_id: 1,
            abort_id: 2,
        }
    }

    fn create_node(&mut self, kind: CfgNodeKind) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.nodes.insert(id, CfgNode { id, kind });
        id
    }

    fn add_edge(&mut self, from: usize, to: usize, kind: CfgEdgeKind) {
        self.edges.push(CfgEdge { from, to, kind });
    }

    fn build_node(&mut self, node: &AstNode, current_tails: Vec<usize>) -> Vec<usize> {
        match node {
            AstNode::Block(statements) => {
                let mut tails = current_tails;
                for stmt in statements {
                    tails = self.build_node(stmt, tails);
                }
                tails
            }
            AstNode::If {
                condition,
                then_branch,
                else_branch,
            } => {
                // 1. Create condition node
                let cond_id = self.create_node(CfgNodeKind::BranchCondition((**condition).clone()));
                for tail in current_tails {
                    self.add_edge(tail, cond_id, CfgEdgeKind::Sequential);
                }

                // 2. Build True Branch
                let then_tails = self.build_node(then_branch, vec![cond_id]);
                // Set correct edge kind for the condition -> then branch start
                for edge in self.edges.iter_mut() {
                    if edge.from == cond_id
                        && edge.to != self.exit_id
                        && edge.to != self.abort_id
                        && edge.kind == CfgEdgeKind::Sequential
                    {
                        edge.kind = CfgEdgeKind::TrueBranch;
                    }
                }

                // 3. Build False Branch
                let mut else_tails = Vec::new();
                if let Some(ref eb) = else_branch {
                    else_tails = self.build_node(eb, vec![cond_id]);
                    // Set correct edge kind for the condition -> else branch start
                    let mut found_else_edge = false;
                    for edge in self.edges.iter_mut() {
                        if edge.from == cond_id && edge.kind == CfgEdgeKind::Sequential {
                            edge.kind = CfgEdgeKind::FalseBranch;
                            found_else_edge = true;
                            break;
                        }
                    }
                    if !found_else_edge {
                        // If no new edge was added directly from cond_id as sequential (maybe nested block or empty/nop),
                        // we'll explicitly tag conditional edges later or ensure a false branch edge exists.
                    }
                } else {
                    else_tails.push(cond_id);
                }

                // Any edge coming directly out of cond_id that isn't TrueBranch or Unconditional
                // must be FalseBranch. Let's make sure edges are correctly labelled.
                for edge in self.edges.iter_mut() {
                    if edge.from == cond_id && edge.kind == CfgEdgeKind::Sequential {
                        edge.kind = CfgEdgeKind::FalseBranch;
                    }
                }

                // 4. Merge tails
                let mut merged_tails = then_tails;
                for et in else_tails {
                    if !merged_tails.contains(&et) {
                        merged_tails.push(et);
                    }
                }
                merged_tails
            }
            AstNode::While { condition, body } => {
                let cond_id = self.create_node(CfgNodeKind::BranchCondition((**condition).clone()));
                let exit_nop_id = self.create_node(CfgNodeKind::Nop);

                for tail in current_tails {
                    self.add_edge(tail, cond_id, CfgEdgeKind::Sequential);
                }

                self.loop_stack.push((cond_id, exit_nop_id));

                // Build body with incoming FalseBranch or TrueBranch? TrueBranch flows to body.
                let body_incoming = vec![cond_id];
                let body_tails = self.build_node(body, body_incoming);

                // Fix condition -> body edge kind to TrueBranch
                for edge in self.edges.iter_mut() {
                    if edge.from == cond_id
                        && edge.to != exit_nop_id
                        && edge.kind == CfgEdgeKind::Sequential
                    {
                        edge.kind = CfgEdgeKind::TrueBranch;
                    }
                }

                // Body tails flow back to the condition node
                for bt in body_tails {
                    self.add_edge(bt, cond_id, CfgEdgeKind::LoopBack);
                }

                // Condition false branch flows to loop exit nop
                self.add_edge(cond_id, exit_nop_id, CfgEdgeKind::FalseBranch);

                self.loop_stack.pop();

                vec![exit_nop_id]
            }
            AstNode::Loop { body } => {
                let header_id = self.create_node(CfgNodeKind::Nop);
                let exit_nop_id = self.create_node(CfgNodeKind::Nop);

                for tail in current_tails {
                    self.add_edge(tail, header_id, CfgEdgeKind::Sequential);
                }

                self.loop_stack.push((header_id, exit_nop_id));

                let body_tails = self.build_node(body, vec![header_id]);

                for bt in body_tails {
                    self.add_edge(bt, header_id, CfgEdgeKind::LoopBack);
                }

                self.loop_stack.pop();

                vec![exit_nop_id]
            }
            _ => {
                // Standard statement. Check if it's return, abort, break, continue
                if is_return(node) {
                    let return_node_id = self.create_node(CfgNodeKind::Statement(node.clone()));
                    for tail in current_tails {
                        self.add_edge(tail, return_node_id, CfgEdgeKind::Sequential);
                    }
                    self.add_edge(return_node_id, self.exit_id, CfgEdgeKind::Unconditional);
                    Vec::new()
                } else if is_abort(node) {
                    let abort_node_id = self.create_node(CfgNodeKind::Statement(node.clone()));
                    for tail in current_tails {
                        self.add_edge(tail, abort_node_id, CfgEdgeKind::Sequential);
                    }
                    self.add_edge(abort_node_id, self.abort_id, CfgEdgeKind::Unconditional);
                    Vec::new()
                } else if is_break(node) {
                    let break_node_id = self.create_node(CfgNodeKind::Statement(node.clone()));
                    for tail in current_tails {
                        self.add_edge(tail, break_node_id, CfgEdgeKind::Sequential);
                    }
                    if let Some(&(_, break_target)) = self.loop_stack.last() {
                        self.add_edge(break_node_id, break_target, CfgEdgeKind::Unconditional);
                    }
                    Vec::new()
                } else if is_continue(node) {
                    let continue_node_id = self.create_node(CfgNodeKind::Statement(node.clone()));
                    for tail in current_tails {
                        self.add_edge(tail, continue_node_id, CfgEdgeKind::Sequential);
                    }
                    if let Some(&(continue_target, _)) = self.loop_stack.last() {
                        self.add_edge(continue_node_id, continue_target, CfgEdgeKind::LoopBack);
                    }
                    Vec::new()
                } else {
                    let stmt_id = self.create_node(CfgNodeKind::Statement(node.clone()));
                    for tail in current_tails {
                        self.add_edge(tail, stmt_id, CfgEdgeKind::Sequential);
                    }
                    vec![stmt_id]
                }
            }
        }
    }
}

fn is_return(node: &AstNode) -> bool {
    match node {
        AstNode::Expression(s) => s.starts_with("return"),
        AstNode::Identifier(s) => s == "return",
        _ => false,
    }
}

fn is_abort(node: &AstNode) -> bool {
    match node {
        AstNode::Expression(s) => {
            s.starts_with("abort") || s.starts_with("assert!") || s.contains("abort")
        }
        AstNode::Identifier(s) => s == "abort",
        AstNode::Call { function_name, .. } => {
            function_name == "abort" || function_name == "assert" || function_name.contains("abort")
        }
        _ => false,
    }
}

fn is_break(node: &AstNode) -> bool {
    match node {
        AstNode::Expression(s) => s == "break",
        AstNode::Identifier(s) => s == "break",
        _ => false,
    }
}

fn is_continue(node: &AstNode) -> bool {
    match node {
        AstNode::Expression(s) => s == "continue",
        AstNode::Identifier(s) => s == "continue",
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cfg_sequential() {
        // let x = 5; let y = 10;
        let ast = Some(AstNode::Block(vec![
            AstNode::Let {
                name: "x".to_string(),
                is_mut: false,
                type_annotation: None,
                initializer: Some(Box::new(AstNode::Literal("5".to_string()))),
            },
            AstNode::Let {
                name: "y".to_string(),
                is_mut: false,
                type_annotation: None,
                initializer: Some(Box::new(AstNode::Literal("10".to_string()))),
            },
        ]));

        let cfg = ControlFlowGraph::build(&ast);
        assert_eq!(cfg.nodes.len(), 5); // Entry (0), Exit (1), Abort (2), Let x (3), Let y (4)
        assert_eq!(cfg.edges.len(), 3); // 0 -> 3 -> 4 -> 1

        let reachable = cfg.get_reachable_nodes();
        assert!(reachable.contains(&0));
        assert!(reachable.contains(&3));
        assert!(reachable.contains(&4));
        assert!(reachable.contains(&1));
    }

    #[test]
    fn test_cfg_if_else() {
        // if (cond) { x = 1; } else { x = 2; }
        let ast = Some(AstNode::Block(vec![AstNode::If {
            condition: Box::new(AstNode::Identifier("cond".to_string())),
            then_branch: Box::new(AstNode::Block(vec![AstNode::Let {
                name: "x".to_string(),
                is_mut: false,
                type_annotation: None,
                initializer: Some(Box::new(AstNode::Literal("1".to_string()))),
            }])),
            else_branch: Some(Box::new(AstNode::Block(vec![AstNode::Let {
                name: "x".to_string(),
                is_mut: false,
                type_annotation: None,
                initializer: Some(Box::new(AstNode::Literal("2".to_string()))),
            }]))),
        }]));

        let cfg = ControlFlowGraph::build(&ast);
        // Condition (3), Then (4), Else (5)
        // Entry(0) -> Cond(3) -> TrueBranch -> Then(4) -> Exit(1)
        //                      -> FalseBranch -> Else(5) -> Exit(1)
        assert_eq!(cfg.nodes.len(), 6);

        // Find TrueBranch and FalseBranch edges
        let true_edge = cfg
            .edges
            .iter()
            .find(|e| e.kind == CfgEdgeKind::TrueBranch)
            .unwrap();
        let false_edge = cfg
            .edges
            .iter()
            .find(|e| e.kind == CfgEdgeKind::FalseBranch)
            .unwrap();
        assert_eq!(true_edge.from, 3);
        assert_eq!(false_edge.from, 3);
    }

    #[test]
    fn test_cfg_while_loop() {
        // while (cond) { x = 1; continue; break; }
        let ast = Some(AstNode::Block(vec![AstNode::While {
            condition: Box::new(AstNode::Identifier("cond".to_string())),
            body: Box::new(AstNode::Block(vec![
                AstNode::Let {
                    name: "x".to_string(),
                    is_mut: false,
                    type_annotation: None,
                    initializer: Some(Box::new(AstNode::Literal("1".to_string()))),
                },
                AstNode::Expression("continue".to_string()),
                AstNode::Expression("break".to_string()),
            ])),
        }]));

        let cfg = ControlFlowGraph::build(&ast);

        // Verify we have TrueBranch, FalseBranch, LoopBack, and Unconditional edges
        let has_loopback = cfg.edges.iter().any(|e| e.kind == CfgEdgeKind::LoopBack);
        let has_unconditional = cfg
            .edges
            .iter()
            .any(|e| e.kind == CfgEdgeKind::Unconditional);
        assert!(has_loopback);
        assert!(has_unconditional);
    }
}
