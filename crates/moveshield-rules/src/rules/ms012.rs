//! MS012: Gas Inefficiencies.

use crate::{rules::resolve_confidence, rules::resolve_severity, Rule};
use moveshield_config::Config;
use moveshield_core::models::{
    Confidence, Evidence, EvidenceKind, Finding, RuleCategory, Severity,
};
use moveshield_knowledge::KnowledgeStore;

/// MS012: Gas Inefficiencies.
/// Identify expensive or unnecessary operations, such as nested loops or heavy vector operations inside loop bodies.
pub struct MS012;

impl Rule for MS012 {
    fn id(&self) -> &'static str {
        "MS012"
    }

    fn name(&self) -> &'static str {
        "Gas Inefficiencies"
    }

    fn description(&self) -> &'static str {
        "Identifies expensive operations (such as nested loops or heavy vector operations) that can cause out-of-gas aborts."
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Performance
    }

    fn default_severity(&self) -> Severity {
        Severity::Informational
    }

    fn default_confidence(&self) -> Confidence {
        Confidence::High
    }

    fn cwe(&self) -> Option<&'static str> {
        Some("CWE-400") // Uncontrolled Resource Consumption
    }

    fn references(&self) -> Vec<&'static str> {
        vec!["https://docs.sui.io/concepts/sui-move-concepts/gas"]
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
                    if func.visibility != "public" && func.visibility != "entry" {
                        continue;
                    }

                    // Check for nested loops or heavy vector operations inside loop bodies
                    let has_while = func.body_text.contains("while");
                    let has_loop = func.body_text.contains("loop");
                    let has_heavy_vec = func.body_text.contains("vector::push_back")
                        || func.body_text.contains("vector::remove")
                        || func.body_text.contains("vector::insert");

                    // Trigger if a public/entry function has a loop and does heavy vector operations
                    if (has_while || has_loop) && has_heavy_vec {
                        let evidence = vec![
                            Evidence {
                                kind: EvidenceKind::MatchedPattern,
                                message: "Loop structure combined with dynamic vector resizing (push_back/remove/insert) in a public function.".to_string(),
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
                                "Possible Gas Inefficiency in function '{}': loop contains dynamic vector operations.",
                                func.name
                            ),
                            explanation: "Dynamically resizing vectors (e.g. vector::push_back, insert, remove) inside loops can consume substantial gas. Since public/entry functions run on-chain, uncontrolled gas consumption could hit the max block gas limit, rendering transactions un-executable.".to_string(),
                            evidence,
                            suggested_fix: "Pre-allocate vector capacity, keep the loop size strictly bounded, or move heavy elements management off-chain.".to_string(),
                            recommendation: format!(
                                "Optimize the dynamic vector operations in loop bodies inside function '{}' to reduce transaction gas overhead.",
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
