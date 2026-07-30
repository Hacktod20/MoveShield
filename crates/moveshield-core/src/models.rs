//! Shared domain models for the Core Data Model (CDM).

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Severity of a finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Informational,
}

/// Confidence level of a finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Confidence {
    High,
    Medium,
    Low,
}

/// Category of a rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuleCategory {
    Security,
    Performance,
    Architecture,
    Maintainability,
    Documentation,
    Style,
}

/// Represents a source location in a file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct Location {
    /// File path relative to the workspace root.
    pub file: PathBuf,
    /// Starting line number (1-based).
    pub start_line: usize,
    /// Starting column number (1-based).
    pub start_column: usize,
    /// Ending line number (1-based).
    pub end_line: usize,
    /// Ending column number (1-based).
    pub end_column: usize,
}

/// Represents a scanned project (canonical CDM root).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    /// Name of the project.
    pub name: String,
    /// Path to the project root.
    pub root_path: PathBuf,
    /// Packages in the project.
    pub packages: Vec<Package>,
}

/// Represents a Move package.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    /// Name of the package.
    pub name: String,
    /// Path to the package directory.
    pub path: PathBuf,
    /// Modules inside the package.
    pub modules: Vec<Module>,
}

/// Represents a module-level `use` statement in Move.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UseStatement {
    /// Optional address of the imported module (e.g., "sui" or "0x2").
    pub address: Option<String>,
    /// Module being imported (e.g., "transfer").
    pub module: String,
    /// Specific member or members imported (e.g., "public_transfer", or empty/Self for whole module).
    pub members: Vec<UseMember>,
}

/// Represents a specific imported member under a `use` statement.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UseMember {
    /// Name of the member or "Self".
    pub name: String,
    /// Optional local alias (e.g., `use sui::tx_context::TxContext as Ctx;`).
    pub alias: Option<String>,
}

/// Represents a parsed AST node inside a function body.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AstNode {
    /// Variable binding (e.g. `let x = 5;` or `let mut x: u64 = 10;`).
    Let {
        name: String,
        is_mut: bool,
        type_annotation: Option<String>,
        initializer: Option<Box<AstNode>>,
    },
    /// Assignment / mutation statement (e.g. `x = y;` or `state.value = val;`).
    Assign { target: String, value: Box<AstNode> },
    /// A function call or method invocation (e.g. `transfer::public_transfer(cap, recipient)`).
    Call {
        function_name: String,
        arguments: Vec<AstNode>,
    },
    /// Variable/symbol identifier.
    Identifier(String),
    /// A raw literal (e.g. `5`, `0x2`, `true`).
    Literal(String),
    /// Code block containing a list of sub-nodes.
    Block(Vec<AstNode>),
    /// If conditional branch.
    If {
        condition: Box<AstNode>,
        then_branch: Box<AstNode>,
        else_branch: Option<Box<AstNode>>,
    },
    /// While loop.
    While {
        condition: Box<AstNode>,
        body: Box<AstNode>,
    },
    /// Infinite loop.
    Loop { body: Box<AstNode> },
    /// Unparsed / generic fallback expression string.
    Expression(String),
}

/// Represents a Move module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    /// Name of the module.
    pub name: String,
    /// Address of the module (e.g. "0x0", "0x2", "0x123...").
    pub address: String,
    /// Source file path.
    pub source_file: PathBuf,
    /// Functions defined in this module.
    pub functions: Vec<Function>,
    /// Structs defined in this module.
    pub structs: Vec<Struct>,
    /// Module-level import/use declarations.
    #[serde(default)]
    pub use_statements: Vec<UseStatement>,
}

/// Represents a Move function.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    /// Name of the function.
    pub name: String,
    /// Visibility of the function ("public", "public(friend)", "entry", or "private").
    pub visibility: String,
    /// Source location.
    pub location: Location,
    /// Cyclomatic complexity of the function.
    pub complexity: usize,
    /// List of function names that this function calls.
    /// Format: "module_name::function_name" or "address::module_name::function_name".
    pub calls: Vec<String>,
    /// Parameters of the function (e.g. "cap: &UpgradeCap").
    pub parameters: Vec<String>,
    /// The body text of the function.
    pub body_text: String,
    /// Rich AST structure parsed from the function body.
    pub ast: Option<AstNode>,
}

/// Represents a Move struct.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Struct {
    /// Name of the struct.
    pub name: String,
    /// Abilities of the struct (e.g. key, store, copy, drop).
    pub abilities: Vec<String>,
    /// Fields of the struct.
    pub fields: Vec<Field>,
}

/// Represents a field in a Move struct.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    /// Name of the field.
    pub name: String,
    /// Type of the field as a string representation.
    pub type_str: String,
}

/// Type of evidence matched by a rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceKind {
    MatchedFunction,
    MatchedCall,
    MatchedPattern,
    MatchedCapability,
    MatchedObject,
    MatchedExpression,
    CallPath,
}

/// Represents evidence supporting a finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Evidence {
    /// The type of evidence.
    pub kind: EvidenceKind,
    /// A descriptive message of what was matched.
    pub message: String,
    /// Optional code snippet or symbol.
    pub snippet: Option<String>,
    /// Optional starting line number (1-based).
    pub line: Option<usize>,
    /// Optional starting column number (1-based).
    pub column: Option<usize>,
}

/// Represents an engineering finding/violation produced by the rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    /// Rule identifier (e.g., "MS001").
    pub rule_id: String,
    /// User-friendly title of the finding.
    pub title: String,
    /// Category of the rule.
    pub category: RuleCategory,
    /// Severity of the finding.
    pub severity: Severity,
    /// Confidence in the finding.
    pub confidence: Confidence,
    /// Location in the code where the issue was found.
    pub location: Location,
    /// The name of the module where the issue occurred.
    pub module_name: String,
    /// The name of the function where the issue occurred.
    pub function_name: String,
    /// Quick summary message.
    pub message: String,
    /// Detailed explanation of the finding and why it matters.
    pub explanation: String,
    /// List of evidence supporting the finding.
    pub evidence: Vec<Evidence>,
    /// Suggested fix or remediation.
    pub suggested_fix: String,
    /// Actionable remediation recommendation.
    pub recommendation: String,
    /// Reference links (documentation, security guides, etc.).
    pub references: Vec<String>,
}
