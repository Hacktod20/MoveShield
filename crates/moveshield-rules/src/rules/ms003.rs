//! MS003: Capability Leakage.

use crate::{rules::resolve_confidence, rules::resolve_severity, Rule};
use moveshield_config::Config;
use moveshield_core::models::{
    Confidence, Evidence, EvidenceKind, Finding, RuleCategory, Severity,
};
use moveshield_ir::ObjectState;
use moveshield_knowledge::KnowledgeStore;

/// MS003: Capability Leakage.
/// Detects when AdminCap or similar privileged capabilities leave trusted ownership.
pub struct MS003;

impl Rule for MS003 {
    fn id(&self) -> &'static str {
        "MS003"
    }

    fn name(&self) -> &'static str {
        "Capability Leakage"
    }

    fn description(&self) -> &'static str {
        "Detects when AdminCap or similar privileged capabilities are insecurely transferred or leaked."
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
                    let has_cap_param =
                        store.has_capability_param(&module.address, &module.name, &func.name);
                    let is_public_or_entry =
                        func.visibility == "public" || func.visibility == "entry";

                    let transfers = store.get_transfers(&module.address, &module.name, &func.name);

                    let mut leaks_cap = false;
                    for tr in &transfers {
                        if store.is_capability(&tr.asset_type) {
                            if let Some(state) = store.get_variable_state(
                                &module.address,
                                &module.name,
                                &func.name,
                                &tr.asset_name,
                            ) {
                                if state == ObjectState::Transferred {
                                    leaks_cap = true;
                                }
                            }
                        }
                    }

                    if has_cap_param && is_public_or_entry && leaks_cap {
                        let mut evidence = Vec::new();
                        for param in &func.parameters {
                            if param.contains("Cap") || param.contains("Capability") {
                                evidence.push(Evidence {
                                    kind: EvidenceKind::MatchedCapability,
                                    message: format!(
                                        "Privileged capability parameter: '{}'",
                                        param
                                    ),
                                    snippet: Some(format!("fun {}(...)", func.name)),
                                    line: Some(func.location.start_line),
                                    column: Some(func.location.start_column),
                                });
                            }
                        }

                        for tr in &transfers {
                            if store.is_capability(&tr.asset_type) {
                                evidence.push(Evidence {
                                    kind: EvidenceKind::MatchedCall,
                                    message: format!(
                                        "Function performs object transfer on the capability '{}' of type '{}' to recipient '{}'.",
                                        tr.asset_name, tr.asset_type, tr.recipient
                                    ),
                                    snippet: Some(func.body_text.clone()),
                                    line: Some(func.location.start_line),
                                    column: Some(func.location.start_column),
                                });
                            }
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
                                "Possible Capability Leakage in function '{}': privileged capability is insecurely transferred.",
                                func.name
                            ),
                            explanation: "Privileged capabilities represent administrative access. Transferring or exposing capabilities through public/entry functions without strict controls can allow malicious actors to acquire control of the smart contract.".to_string(),
                            evidence,
                            suggested_fix: "Avoid transferring capabilities directly. Enforce strict recipient checks or keep capabilities inside non-transferable module-controlled logic.".to_string(),
                            recommendation: format!(
                                "Audit the capability transfer inside function '{}'. Ensure that only trusted addresses can receive this capability and avoid exposing transfer logic publicly.",
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
