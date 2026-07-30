//! MS011: Randomness Misuse.

use crate::{rules::resolve_confidence, rules::resolve_severity, Rule};
use moveshield_config::Config;
use moveshield_core::models::{
    Confidence, Evidence, EvidenceKind, Finding, RuleCategory, Severity,
};
use moveshield_knowledge::KnowledgeStore;

/// MS011: Randomness Misuse.
/// Detects usage of predictable seeds or transaction values (like tx_context::digest or epoch_time) for randomness.
pub struct MS011;

impl Rule for MS011 {
    fn id(&self) -> &'static str {
        "MS011"
    }

    fn name(&self) -> &'static str {
        "Randomness Misuse"
    }

    fn description(&self) -> &'static str {
        "Detects usage of insecure or predictable sources of randomness (such as digest or epoch_time)."
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
        Some("CWE-337") // Predictable Seed in Pseudo-Random Number Generator (PRNG)
    }

    fn references(&self) -> Vec<&'static str> {
        vec!["https://docs.sui.io/concepts/sui-move-concepts/randomness"]
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
                    let has_predictable_source = func.body_text.contains("tx_context::digest")
                        || func.body_text.contains("digest(")
                        || func.body_text.contains("epoch_time")
                        || func
                            .calls
                            .iter()
                            .any(|c| c.contains("digest") || c.contains("epoch_time"));

                    let is_lottery_or_gaming = func.name.contains("random")
                        || func.name.contains("rand")
                        || func.name.contains("lottery")
                        || func.name.contains("game")
                        || func.name.contains("flip")
                        || func.name.contains("roll")
                        || func.name.contains("spin");

                    if has_predictable_source && is_lottery_or_gaming {
                        let evidence = vec![
                            Evidence {
                                kind: EvidenceKind::MatchedPattern,
                                message: "Predictable seed / transaction value used in a gaming or random selection function.".to_string(),
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
                                "Randomness Misuse in function '{}': uses a predictable source of randomness.",
                                func.name
                            ),
                            explanation: "Using predictable on-chain values (like transaction digests, clock epoch times, or block metadata) for randomness allows validators or sophisticated users to predict or manipulate results, compromising games, lotteries, and random distributions.".to_string(),
                            evidence,
                            suggested_fix: "Utilize Sui's official Random module (sui::random) or secure off-chain VRFs (Verifiable Random Functions) instead of on-chain predictable variables.".to_string(),
                            recommendation: format!(
                                "Replace predictable on-chain variables in function '{}' with the official 'sui::random::Random' object to secure randomness.",
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
