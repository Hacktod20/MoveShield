//! MS009: Treasury Misconfiguration.

use crate::{rules::resolve_confidence, rules::resolve_severity, Rule};
use moveshield_config::Config;
use moveshield_core::models::{
    Confidence, Evidence, EvidenceKind, Finding, RuleCategory, Severity,
};
use moveshield_knowledge::KnowledgeStore;

/// MS009: Treasury Misconfiguration.
/// Detects insecure treasury management patterns (such as transferring or storing TreasuryCap without validation).
pub struct MS009;

impl Rule for MS009 {
    fn id(&self) -> &'static str {
        "MS009"
    }

    fn name(&self) -> &'static str {
        "Treasury Misconfiguration"
    }

    fn description(&self) -> &'static str {
        "Detects insecure treasury management patterns or unrestricted transfers of TreasuryCap."
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
        vec!["https://docs.sui.io/concepts/sui-move-concepts/coin"]
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
                    let has_treasury_cap =
                        func.parameters.iter().any(|p| p.contains("TreasuryCap"));
                    let is_public_or_entry =
                        func.visibility == "public" || func.visibility == "entry";

                    let performs_transfer = func.calls.iter().any(|c| c.contains("transfer"));
                    let body_has_transfer = func.body_text.contains("transfer::")
                        || func.body_text.contains("public_transfer");

                    if has_treasury_cap
                        && is_public_or_entry
                        && (performs_transfer || body_has_transfer)
                    {
                        let evidence = vec![
                            Evidence {
                                kind: EvidenceKind::MatchedCapability,
                                message: "Function accepts TreasuryCap parameter and performs object transfer/unrestricted movement.".to_string(),
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
                                "Treasury Misconfiguration in function '{}': TreasuryCap is transferred or exposed.",
                                func.name
                            ),
                            explanation: "The TreasuryCap is a highly sensitive capability that controls the minting and burning of a custom Coin in Sui. Publicly transferring, returning, or storing the TreasuryCap in a shared/unrestricted state compromises the token supply's security.".to_string(),
                            evidence,
                            suggested_fix: "Keep the TreasuryCap strictly within trusted admin ownership and do not expose transfer functions for it publicly.".to_string(),
                            recommendation: format!(
                                "Restructure function '{}' to ensure that TreasuryCap cannot be leaked, transferred, or exposed to unauthorized addresses.",
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
