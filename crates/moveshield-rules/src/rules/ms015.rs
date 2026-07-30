//! MS015: Duplicate Objects.

use crate::{rules::resolve_confidence, rules::resolve_severity, Rule};
use moveshield_config::Config;
use moveshield_core::models::{
    Confidence, Evidence, EvidenceKind, Finding, RuleCategory, Severity,
};
use moveshield_knowledge::KnowledgeStore;

/// MS015: Duplicate Objects.
/// Detects duplicate object creations or conflicting initialization patterns (multiple object::new calls in a single function).
pub struct MS015;

impl Rule for MS015 {
    fn id(&self) -> &'static str {
        "MS015"
    }

    fn name(&self) -> &'static str {
        "Duplicate Objects"
    }

    fn description(&self) -> &'static str {
        "Detects multiple calls to object::new within a single function block, which may indicate duplicate object creation."
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Architecture
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
        vec!["https://docs.sui.io/concepts/sui-move-concepts/objects"]
    }

    fn version(&self) -> &'static str {
        "0.1.0"
    }

    fn run(&self, store: &KnowledgeStore, config: &Config) -> Vec<Finding> {
        let mut findings = Vec::new();

        if !config.is_rule_enabled(self.id()) {
            return findings;
        }

        let severity = resolve_severity(self.id(), self.default_severity(), config);
        let confidence = resolve_confidence(self.id(), self.default_confidence(), config);

        for package in &store.project.packages {
            for module in &package.modules {
                for func in &module.functions {
                    // Count occurrences of object::new in the function body
                    let occurrences = func.body_text.matches("object::new").count();

                    if occurrences >= 2 {
                        let evidence = vec![
                            Evidence {
                                kind: EvidenceKind::MatchedPattern,
                                message: format!("Found {} calls to 'object::new' in function body.", occurrences),
                                snippet: Some(func.body_text.clone()),
                                line: Some(func.location.start_line),
                                column: Some(func.location.start_column),
                            }
                        ];

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
                                "Duplicate object creation detected in function '{}': multiple object::new calls.",
                                func.name
                            ),
                            explanation: "Creating multiple distinct objects of identical structural or semantic purpose within a single transaction path can lead to bloated state, conflicting parent-child ownership, or logical inconsistencies during runtime initialization.".to_string(),
                            evidence,
                            suggested_fix: "Refactor the initialization logic to consolidate object creation, or ensure each object has a distinct, well-defined single-purpose ownership model.".to_string(),
                            recommendation: format!(
                                "Consolidate the multiple 'object::new' calls inside function '{}' to prevent redundant object footprint and storage inflation.",
                                func.name
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
