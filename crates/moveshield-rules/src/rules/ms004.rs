//! MS004: Shared Object Misuse.

use crate::{rules::resolve_confidence, rules::resolve_severity, Rule};
use moveshield_config::Config;
use moveshield_core::models::{
    Confidence, Evidence, EvidenceKind, Finding, RuleCategory, Severity,
};
use moveshield_knowledge::KnowledgeStore;

/// MS004: Shared Object Misuse.
/// Detects improper mutation of shared objects.
pub struct MS004;

impl Rule for MS004 {
    fn id(&self) -> &'static str {
        "MS004"
    }

    fn name(&self) -> &'static str {
        "Shared Object Misuse"
    }

    fn description(&self) -> &'static str {
        "Detects public or entry functions that mutate shared objects without any capability-based access control."
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
        vec!["https://docs.sui.io/concepts/sui-move-concepts/object-ownership/shared"]
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

                    let mut mutable_shared_params = Vec::new();
                    let has_capability =
                        store.has_capability_param(&module.address, &module.name, &func.name);

                    for param in &func.parameters {
                        let is_mut = param.contains("&mut");
                        let is_shared_target = param.contains("Pool")
                            || param.contains("Store")
                            || param.contains("State")
                            || param.contains("Shared")
                            || param.contains("Config")
                            || param.contains("Registry")
                            || param.contains("Object")
                            || param.contains("Treasury");

                        if is_mut && is_shared_target {
                            mutable_shared_params.push(param.clone());
                        }
                    }

                    if !mutable_shared_params.is_empty() && !has_capability {
                        let mut evidence = Vec::new();
                        for param in &mutable_shared_params {
                            evidence.push(Evidence {
                                kind: EvidenceKind::MatchedObject,
                                message: format!("Mutable shared object parameter lacking capability control: '{}'", param),
                                snippet: Some(format!("fun {}(...)", func.name)),
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
                                "Shared Object Misuse in function '{}': shared object is mutated without checking a capability.",
                                func.name
                            ),
                            explanation: "Shared objects are accessible by anyone. Public or entry functions that mutate shared objects without any Capability-based authorization can be called by malicious actors to manipulate or drain the contract state.".to_string(),
                            evidence,
                            suggested_fix: "Add a Capability parameter (e.g. &AdminCap) to the function signature to restrict mutation to authorized callers, or use explicit sender checks.".to_string(),
                            recommendation: format!(
                                "Restrict function '{}' by adding a required admin capability parameter (e.g. &AdminCap) to authorize callers before modifying the shared object.",
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
