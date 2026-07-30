//! MS014: Dead Public Functions.

use crate::{rules::resolve_confidence, rules::resolve_severity, Rule};
use moveshield_config::Config;
use moveshield_core::models::{
    Confidence, Evidence, EvidenceKind, Finding, RuleCategory, Severity,
};
use moveshield_knowledge::KnowledgeStore;

/// MS014: Dead Public Functions.
/// Detects public functions that are never referenced or called by other functions.
pub struct MS014;

impl Rule for MS014 {
    fn id(&self) -> &'static str {
        "MS014"
    }

    fn name(&self) -> &'static str {
        "Dead Public Functions"
    }

    fn description(&self) -> &'static str {
        "Detects public functions that are never called or referenced within the project."
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Architecture
    }

    fn default_severity(&self) -> Severity {
        Severity::Medium
    }

    fn default_confidence(&self) -> Confidence {
        Confidence::High
    }

    fn cwe(&self) -> Option<&'static str> {
        Some("CWE-561") // Dead Code
    }

    fn references(&self) -> Vec<&'static str> {
        vec!["https://docs.moveshield.com/rules/MS014"]
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

        // Retrieve unused function list from the Call Graph
        let unused_fqs = store.call_graph.get_unused_functions();

        for package in &store.project.packages {
            for module in &package.modules {
                for func in &module.functions {
                    let fq_id = format!("{}::{}::{}", module.address, module.name, func.name);
                    let alt_fq_id = format!("{}::{}", module.name, func.name);

                    if unused_fqs.contains(&fq_id) || unused_fqs.contains(&alt_fq_id) {
                        // Ensure it's a public function (not private, not entry, not init)
                        if func.visibility != "public" || func.name == "init" {
                            continue;
                        }

                        let evidence = vec![
                            Evidence {
                                kind: EvidenceKind::CallPath,
                                message: format!("Function '{}' is defined as public but has 0 incoming calls in the Call Graph.", func.name),
                                snippet: Some(format!("public fun {}(...)", func.name)),
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
                                "Dead Public Function detected: '{}' is never called.",
                                func.name
                            ),
                            explanation: "Public functions that are never called inside the project's source code represent dead/unreachable code. While library crates may intentionally expose unused public functions, in application modules they increase deployment size, waste gas, and make code maintenance harder.".to_string(),
                            evidence,
                            suggested_fix: "Remove the public function, reduce its visibility to private (friend) if it's not needed outside, or ensure it is correctly integrated and tested.".to_string(),
                            recommendation: format!(
                                "Review function '{}' in module '{}'. If it is not a library-level API, remove it or reduce its visibility.",
                                func.name, module.name
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
