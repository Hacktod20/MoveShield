//! MS001: Cyclomatic Complexity Rule.

use crate::{rules::resolve_confidence, rules::resolve_severity, Rule};
use moveshield_config::Config;
use moveshield_core::models::{
    Confidence, Evidence, EvidenceKind, Finding, RuleCategory, Severity,
};
use moveshield_knowledge::KnowledgeStore;

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

    fn description(&self) -> &'static str {
        "Detects functions with high cyclomatic complexity that are hard to maintain and test."
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Maintainability
    }

    fn default_severity(&self) -> Severity {
        Severity::Medium
    }

    fn default_confidence(&self) -> Confidence {
        Confidence::High
    }

    fn cwe(&self) -> Option<&'static str> {
        None
    }

    fn references(&self) -> Vec<&'static str> {
        vec!["https://docs.moveshield.com/rules/MS001"]
    }

    fn version(&self) -> &'static str {
        "0.1.0"
    }

    fn run(&self, store: &KnowledgeStore, config: &Config) -> Vec<Finding> {
        let mut findings = Vec::new();

        if !config.is_rule_enabled(self.id()) {
            return findings;
        }

        // Retrieve rule-specific configuration or fall back to defaults
        let max_complexity = if let Some(ms001_cfg) = &config.rules.ms001 {
            ms001_cfg.max_complexity
        } else {
            10
        };

        let severity = resolve_severity(self.id(), self.default_severity(), config);
        let confidence = resolve_confidence(self.id(), self.default_confidence(), config);

        // Iterate through all modules and functions in the project
        for package in &store.project.packages {
            for module in &package.modules {
                for func in &module.functions {
                    if func.complexity > max_complexity {
                        let evidence = vec![Evidence {
                            kind: EvidenceKind::MatchedFunction,
                            message: format!(
                                "Function '{}' has cyclomatic complexity of {}.",
                                func.name, func.complexity
                            ),
                            snippet: Some(format!("fun {} ...", func.name)),
                            line: Some(func.location.start_line),
                            column: Some(func.location.start_column),
                        }];

                        findings.push(Finding {
                            rule_id: self.id().to_string(),
                            title: self.name().to_string(),
                            category: self.category(),
                            severity,
                            confidence,
                            location: func.location.clone(),
                            module_name: module.name.clone(),
                            function_name: func.name.clone(),
                            message: format!(
                                "Function '{}' has high cyclomatic complexity ({}/{}).",
                                func.name, func.complexity, max_complexity
                            ),
                            explanation: "High cyclomatic complexity indicates that a function has too many decision paths. This makes the code harder to understand, test, maintain, and increases the likelihood of security and logical bugs.".to_string(),
                            evidence,
                            suggested_fix: "Refactor the function by breaking it down into smaller, simpler helper functions. Extract nested branches or conditions into separate logic blocks.".to_string(),
                            recommendation: format!(
                                "Reduce the cyclomatic complexity of '{}' below {} by refactoring nested logical branches (if/while/loop) into smaller, single-purpose helper functions.",
                                func.name, max_complexity
                            ),
                            references: self.references().iter().map(|r| r.to_string()).collect(),
                        });
                    }
                }
            }
        }

        findings
    }
}
