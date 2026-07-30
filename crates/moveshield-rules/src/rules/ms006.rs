//! MS006: Timestamp / Clock Misuse.

use crate::{rules::resolve_confidence, rules::resolve_severity, Rule};
use moveshield_config::Config;
use moveshield_core::models::{
    Confidence, Evidence, EvidenceKind, Finding, RuleCategory, Severity,
};
use moveshield_knowledge::KnowledgeStore;

/// MS006: Timestamp / Clock Misuse.
/// Detects incorrect assumptions about Clock usage (such as relying on tx_context::epoch_time).
pub struct MS006;

impl Rule for MS006 {
    fn id(&self) -> &'static str {
        "MS006"
    }

    fn name(&self) -> &'static str {
        "Timestamp / Clock Misuse"
    }

    fn description(&self) -> &'static str {
        "Detects usage of insecure epoch_time instead of the standard sui::clock::Clock."
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Security
    }

    fn default_severity(&self) -> Severity {
        Severity::Medium
    }

    fn default_confidence(&self) -> Confidence {
        Confidence::High
    }

    fn cwe(&self) -> Option<&'static str> {
        Some("CWE-829") // Inclusion of Functionality from Untrusted Control Sphere
    }

    fn references(&self) -> Vec<&'static str> {
        vec!["https://docs.sui.io/concepts/sui-move-concepts/time"]
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
                    let uses_epoch_time = func.body_text.contains("epoch_time")
                        || func.body_text.contains("epoch_timestamp")
                        || func.calls.iter().any(|c| c.contains("epoch_time"));

                    let has_clock_param = func.parameters.iter().any(|p| p.contains("Clock"));

                    if uses_epoch_time && !has_clock_param {
                        let evidence = vec![
                            Evidence {
                                kind: EvidenceKind::MatchedPattern,
                                message: "Found epoch_time or epoch_timestamp without Clock parameter validation.".to_string(),
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
                                "Insecure timestamp usage in function '{}': relies on epoch time without Clock.",
                                func.name
                            ),
                            explanation: "Transaction epoch_time represents the start of the epoch, not the current time, and is highly coarse/unreliable. For real-time applications, validators can manipulate details, and epoch-relative timing has low granularity. Developers should instead require a read of the official sui::clock::Clock object.".to_string(),
                            evidence,
                            suggested_fix: "Pass the &sui::clock::Clock object as a parameter and call sui::clock::timestamp_ms(clock) to get a secure timestamp.".to_string(),
                            recommendation: format!(
                                "Replace insecure 'epoch_time' usage inside function '{}' with standard '&sui::clock::Clock' object access.",
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
