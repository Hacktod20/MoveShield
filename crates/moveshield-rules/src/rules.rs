//! MoveShield Rule Engine interface.

use moveshield_config::Config;
use moveshield_core::models::{Finding, RuleCategory};
use moveshield_knowledge::KnowledgeStore;

/// Base trait that every static analysis rule must implement.
pub trait Rule {
    /// Unique identifier of the rule, e.g., "MS001".
    fn id(&self) -> &'static str;

    /// User-friendly name of the rule.
    fn name(&self) -> &'static str;

    /// Category of the rule.
    fn category(&self) -> RuleCategory;

    /// Evaluates the rule against the computed KnowledgeStore and Config.
    fn run(&self, store: &KnowledgeStore, config: &Config) -> Vec<Finding>;
}

pub mod ms001;
pub use ms001::MS001;
