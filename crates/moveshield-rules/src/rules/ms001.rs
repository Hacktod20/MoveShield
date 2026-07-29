//! MS001: Cyclomatic Complexity Rule.

use moveshield_core::models::{Finding, Confidence, Severity, RuleCategory};
use moveshield_knowledge::KnowledgeStore;
use moveshield_config::Config;
use crate::Rule;

/// MS001: Cyclomatic Complexity Rule.
/// Detects functions with high cyclomatic complexity that are hard to maintain and test.
pub struct MS001;

impl Rule for MS001 {
    fn id(&self) -> &'static str {
        "MS001"
    }

    fn name(&self) -> &'static str {
        "Cyclomatic Complexity"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Maintainability
    }

    fn run(&self, store: &KnowledgeStore, config: &Config) -> Vec<Finding> {
        let mut findings = Vec::new();

        // Retrieve rule-specific configuration or fall back to defaults
        let (enabled, max_complexity) = if let Some(ms001_cfg) = &config.rules.ms001 {
            (ms001_cfg.enabled, ms001_cfg.max_complexity)
        } else {
            (true, 10)
        };

        if !enabled {
            return findings;
        }

        // Iterate through all functions in the Call Graph
        for (fq_id, func) in &store.call_graph.function_metadata {
            if func.complexity > max_complexity {
                findings.push(Finding {
                    rule_id: self.id().to_string(),
                    category: self.category(),
                    severity: Severity::Medium,
                    confidence: Confidence::High,
                    location: func.location.clone(),
                    message: format!(
                        "Function '{}' has high cyclomatic complexity ({}). The maximum limit is {}.",
                        fq_id, func.complexity, max_complexity
                    ),
                    explanation: "High cyclomatic complexity indicates that a function has too many decision paths. This makes the code harder to understand, test, maintain, and increases the likelihood of security and logical bugs.".to_string(),
                    suggested_fix: "Refactor the function by breaking it down into smaller, simpler helper functions. Extract nested branches or conditions into separate logic blocks.".to_string(),
                    references: vec![
                        "https://docs.moveshield.com/rules/MS001".to_string()
                    ],
                });
            }
        }

        findings
    }
}
