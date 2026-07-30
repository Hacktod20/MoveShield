//! MS005: Missing Access Control.

use crate::{rules::resolve_confidence, rules::resolve_severity, Rule};
use moveshield_config::Config;
use moveshield_core::models::{
    Confidence, Evidence, EvidenceKind, Finding, RuleCategory, Severity,
};
use moveshield_knowledge::KnowledgeStore;

/// MS005: Missing Access Control.
/// Detects public entry functions that perform sensitive operations without privilege validation.
pub struct MS005;

impl Rule for MS005 {
    fn id(&self) -> &'static str {
        "MS005"
    }

    fn name(&self) -> &'static str {
        "Missing Access Control"
    }

    fn description(&self) -> &'static str {
        "Detects sensitive public or entry functions that completely lack access control validations (e.g., capabilities)."
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Security
    }

    fn default_severity(&self) -> Severity {
        Severity::Critical
    }

    fn default_confidence(&self) -> Confidence {
        Confidence::High
    }

    fn cwe(&self) -> Option<&'static str> {
        Some("CWE-285") // Improper Authorization
    }

    fn references(&self) -> Vec<&'static str> {
        vec!["https://docs.sui.io/concepts/sui-move-concepts/capabilities"]
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

                    // Exclude standard init function
                    if func.name == "init" {
                        continue;
                    }

                    // Check if the function name or body suggests a sensitive operation
                    let is_sensitive_name = func.name.contains("mint")
                        || func.name.contains("burn")
                        || func.name.contains("withdraw")
                        || func.name.contains("upgrade")
                        || func.name.contains("liquidate")
                        || func.name.contains("set_admin")
                        || func.name.contains("pause");

                    let calls_sensitive_ops = func.calls.iter().any(|c| {
                        c.contains("mint")
                            || c.contains("burn")
                            || c.contains("withdraw")
                            || c.contains("upgrade")
                    });

                    if is_sensitive_name || calls_sensitive_ops {
                        // Check if function signature contains a Capability parameter or a validation context
                        let has_capability_or_ctx = func.parameters.iter().any(|p| {
                            p.contains("Cap")
                                || p.contains("Capability")
                                || p.contains("TxContext")
                                || p.contains("signer")
                        });

                        if !has_capability_or_ctx {
                            let evidence = vec![
                                Evidence {
                                    kind: EvidenceKind::MatchedFunction,
                                    message: format!("Sensitive function visibility is public or entry: '{}'", func.name),
                                    snippet: Some(format!("fun {}(...)", func.name)),
                                    line: Some(func.location.start_line),
                                    column: Some(func.location.start_column),
                                },
                                Evidence {
                                    kind: EvidenceKind::MatchedPattern,
                                    message: "Missing 'Cap', 'Capability', 'TxContext' or 'signer' parameter.".to_string(),
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
                                    "Missing Access Control in sensitive function '{}'.",
                                    func.name
                                ),
                                explanation: "Publicly accessible functions that perform sensitive state modifications (such as minting, burning, or changing admin settings) must be strictly gated. Lacking proper capability verification or transaction context allows any user to perform privileged actions.".to_string(),
                                evidence,
                                suggested_fix: "Introduce a Capability parameter (e.g. &AdminCap) or add custom sender verification utilizing TxContext.".to_string(),
                                recommendation: format!(
                                    "Add access control to function '{}' by requiring a Capability or performing sender address verification inside the function body.",
                                    func.name
                                ),
                                references: self.references().iter().map(|r| r.to_string()).collect(),
                            });
                        }
                    }
                }
            }
        }

        findings
    }
}
