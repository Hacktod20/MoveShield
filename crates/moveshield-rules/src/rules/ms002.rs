//! MS002: Public Transfer Detection.

use crate::{rules::resolve_confidence, rules::resolve_severity, Rule};
use moveshield_config::Config;
use moveshield_core::models::{
    Confidence, Evidence, EvidenceKind, Finding, RuleCategory, Severity,
};
use moveshield_knowledge::KnowledgeStore;

/// MS002: Public Transfer Detection.
/// Detects unrestricted transfer patterns (e.g. transfer::public_transfer or transfer::public_share_object).
pub struct MS002;

impl Rule for MS002 {
    fn id(&self) -> &'static str {
        "MS002"
    }

    fn name(&self) -> &'static str {
        "Public Transfer Detection"
    }

    fn description(&self) -> &'static str {
        "Detects unrestricted object transfer patterns (public_transfer or public_share_object)."
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Security
    }

    fn default_severity(&self) -> Severity {
        Severity::High
    }

    fn default_confidence(&self) -> Confidence {
        Confidence::High
    }

    fn cwe(&self) -> Option<&'static str> {
        Some("CWE-284") // Improper Access Control
    }

    fn references(&self) -> Vec<&'static str> {
        vec!["https://docs.sui.io/concepts/sui-move-concepts/object-ownership/address-owned"]
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
                    // Exclude standard init function
                    if func.name == "init" {
                        continue;
                    }

                    let mut matched_calls = Vec::new();
                    let resolved_calls =
                        store.get_resolved_calls(&module.address, &module.name, &func.name);

                    for call in resolved_calls {
                        if call == "sui::transfer::public_transfer"
                            || call == "sui::transfer::public_share_object"
                        {
                            matched_calls.push(call);
                        }
                    }

                    if !matched_calls.is_empty() {
                        let mut evidence = Vec::new();
                        for call in &matched_calls {
                            evidence.push(Evidence {
                                kind: EvidenceKind::MatchedCall,
                                message: format!(
                                    "Found call to unrestricted transfer function: '{}'",
                                    call
                                ),
                                snippet: Some(func.body_text.clone()),
                                line: Some(func.location.start_line),
                                column: Some(func.location.start_column),
                            });
                        }

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
                                "Unrestricted transfer pattern detected in function '{}' calling standard public transfer methods.",
                                func.name
                            ),
                            explanation: "Using public_transfer or public_share_object bypasses module-defined transfer rules. For restricted or custom-ownership assets, transfer restrictions must be explicitly enforced inside custom transfer functions rather than exposing unrestricted transfer options.".to_string(),
                            evidence,
                            suggested_fix: "Remove direct calls to public_transfer / public_share_object. Instead, enforce custom business rules or utilize custom transfer/wrap functions restricted to authorized callers.".to_string(),
                            recommendation: format!(
                                "Enforce custom business logic inside function '{}' and replace unrestricted 'public_transfer' calls with module-gated transfer checks.",
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
