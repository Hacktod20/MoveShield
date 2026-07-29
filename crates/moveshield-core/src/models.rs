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

/// Represents an engineering finding/violation produced by the rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    /// Rule identifier (e.g., "MS001").
    pub rule_id: String,
    /// Category of the rule.
    pub category: RuleCategory,
    /// Severity of the finding.
    pub severity: Severity,
    /// Confidence in the finding.
    pub confidence: Confidence,
    /// Location in the code where the issue was found.
    pub location: Location,
    /// Quick summary message.
    pub message: String,
    /// Detailed explanation of the finding and why it matters.
    pub explanation: String,
    /// Suggested fix or remediation.
    pub suggested_fix: String,
    /// Reference links (documentation, security guides, etc.).
    pub references: Vec<String>,
}
