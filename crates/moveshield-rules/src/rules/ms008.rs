//! MS008: Upgrade Policy Issues.

use crate::{rules::resolve_confidence, rules::resolve_severity, Rule};
use moveshield_config::Config;
use moveshield_core::models::{
    Confidence, Evidence, EvidenceKind, Finding, RuleCategory, Severity,
};
use moveshield_knowledge::KnowledgeStore;

/// MS008: Upgrade Policy Issues.
/// Detects missing or weak package upgrade restrictions.
pub struct MS008;

impl Rule for MS008 {
    fn id(&self) -> &'static str {
        "MS008"
    }

    fn name(&self) -> &'static str {
        "Upgrade Policy Issues"
    }

    fn description(&self) -> &'static str {
        "Detects missing or weak package upgrade restrictions (e.g. missing UpgradeCap gating)."
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
        Some("CWE-285") // Improper Authorization
    }

    fn references(&self) -> Vec<&'static str> {
        vec!["https://docs.sui.io/concepts/sui-move-concepts/packages/custom-upgrade-policy"]
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
                    let is_upgrade_func = func.name.contains("upgrade")
                        || func.name.contains("authorize_upgrade")
                        || func.name.contains("publish");

                    let is_public_or_entry = func.visibility == "public" || func.visibility == "entry";

                    let has_upgrade_cap = func.parameters.iter().any(|p| p.contains("UpgradeCap"));

                    if is_upgrade_func && is_public_or_entry && !has_upgrade_cap {
                        let evidence = vec![
                            Evidence {
                                kind: EvidenceKind::MatchedFunction,
                                message: format!("Publicly accessible upgrade-related function: '{}'", func.name),
                                snippet: Some(format!("fun {}(...)", func.name)),
                                line: Some(func.location.start_line),
                                column: Some(func.location.start_column),
                            },
                            Evidence {
                                kind: EvidenceKind::MatchedPattern,
                                message: "Missing 'UpgradeCap' validation in signature.".to_string(),
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
                                "Upgrade Policy Issue in function '{}': upgrade logic is not restricted by UpgradeCap.",
                                func.name
                            ),
                            explanation: "Package upgrades in Sui are privileged actions. If custom upgrade or package authorization functions do not require the standard UpgradeCap (or proper admin capability check), malicious actors could hijack the upgrade process to deploy malicious code.".to_string(),
                            evidence,
                            suggested_fix: "Ensure your custom upgrade/publishing logic accepts and validates a reference or ownership of the official 'sui::package::UpgradeCap'.".to_string(),
                            recommendation: format!(
                                "Incorporate a required 'UpgradeCap' parameter into the signature of function '{}' to secure package publishing.",
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
