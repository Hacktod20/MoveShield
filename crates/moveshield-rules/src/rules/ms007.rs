//! MS007: Dynamic Field Risks.

use crate::{rules::resolve_confidence, rules::resolve_severity, Rule};
use moveshield_config::Config;
use moveshield_core::models::{
    Confidence, Evidence, EvidenceKind, Finding, RuleCategory, Severity,
};
use moveshield_knowledge::KnowledgeStore;

/// MS007: Dynamic Field Risks.
/// Detects unsafe use of dynamic fields.
pub struct MS007;

impl Rule for MS007 {
    fn id(&self) -> &'static str {
        "MS007"
    }

    fn name(&self) -> &'static str {
        "Dynamic Field Risks"
    }

    fn description(&self) -> &'static str {
        "Detects potentially unsafe addition of dynamic fields without proper existence checks or authorization."
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
        Some("CWE-252") // Unchecked Return Value or unvalidated dynamic resource
    }

    fn references(&self) -> Vec<&'static str> {
        vec!["https://docs.sui.io/concepts/sui-move-concepts/dynamic-fields"]
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
                    let adds_field = func.body_text.contains("dynamic_field::add")
                        || func.body_text.contains("dynamic_object_field::add")
                        || func.calls.iter().any(|c| {
                            c.contains("dynamic_field::add")
                                || c.contains("dynamic_object_field::add")
                        });

                    let checks_existence = func.body_text.contains("exists_")
                        || func.body_text.contains("exists(")
                        || func.body_text.contains("has_field")
                        || func
                            .calls
                            .iter()
                            .any(|c| c.contains("exists") || c.contains("has_field"));

                    let has_capability = func
                        .parameters
                        .iter()
                        .any(|p| p.contains("Cap") || p.contains("Capability"));

                    if adds_field && !checks_existence && !has_capability {
                        let evidence = vec![
                            Evidence {
                                kind: EvidenceKind::MatchedPattern,
                                message: "Calling dynamic field addition without checking if the field already exists or validating a capability.".to_string(),
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
                                "Unsafe use of dynamic fields in function '{}': field is added without checking existence.",
                                func.name
                            ),
                            explanation: "Adding a dynamic field that already exists will cause a runtime transaction abort in Sui. To avoid denial of service or unexpected aborts, ensure you check whether the field exists before adding it, or restrict additions to authorized admins.".to_string(),
                            evidence,
                            suggested_fix: "Add an existence check using dynamic_field::exists_ (or dynamic_object_field::exists_) before calling add(), or ensure only authorized callers can trigger the addition.".to_string(),
                            recommendation: format!(
                                "Introduce a defensive existence check using 'dynamic_field::exists_' in function '{}' prior to adding the dynamic field.",
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
