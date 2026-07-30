//! MS013: Unused Capabilities.

use crate::{rules::resolve_confidence, rules::resolve_severity, Rule};
use moveshield_config::Config;
use moveshield_core::models::{
    Confidence, Evidence, EvidenceKind, Finding, Location, RuleCategory, Severity,
};
use moveshield_knowledge::KnowledgeStore;
use std::collections::HashSet;

/// MS013: Unused Capabilities.
/// Detects custom capabilities that are defined in a module but never used as a parameter input in any function.
pub struct MS013;

impl Rule for MS013 {
    fn id(&self) -> &'static str {
        "MS013"
    }

    fn name(&self) -> &'static str {
        "Unused Capabilities"
    }

    fn description(&self) -> &'static str {
        "Detects custom capabilities defined in a module but never used as a parameter in any of its functions."
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
                // 1. Identify all defined structs representing capabilities (ending in Cap or Capability)
                let mut defined_caps = HashSet::new();
                for s in &module.structs {
                    if s.name.ends_with("Cap") || s.name.ends_with("Capability") {
                        defined_caps.insert(s.name.clone());
                    }
                }

                // 2. Scan all function parameter lists in the module to see which caps are used
                let mut used_caps = HashSet::new();
                for func in &module.functions {
                    for param in &func.parameters {
                        for cap in &defined_caps {
                            if param.contains(cap) {
                                used_caps.insert(cap.clone());
                            }
                        }
                    }
                }

                // 3. For any defined cap that is never used in function parameters, emit finding
                for cap in &defined_caps {
                    if !used_caps.contains(cap) {
                        let loc = Location {
                            file: module.source_file.clone(),
                            start_line: 1,
                            start_column: 1,
                            end_line: 1,
                            end_column: 1,
                        };

                        let evidence = vec![
                            Evidence {
                                kind: EvidenceKind::MatchedCapability,
                                message: format!("Capability struct '{}' defined but never utilized as a parameter input in this module.", cap),
                                snippet: Some(format!("struct {} ...", cap)),
                                line: Some(1),
                                column: Some(1),
                            }
                        ];

                        findings.push(Finding {
                            rule_id: self.id().to_string(),
                            title: self.name().to_string(),
                            category: self.category(),
                            severity,
                            confidence,
                            location: loc,
                            module_name: module.name.clone(),
                            function_name: "".to_string(),
                            message: format!(
                                "Unused Capability '{}' defined in module '{}'.",
                                cap, module.name
                            ),
                            explanation: "Defining administrative/privileged capability structs that are never validated in any function indicates dead code or incomplete access control setups. This can confuse developers and lead to security misconfigurations.".to_string(),
                            evidence,
                            suggested_fix: "Remove the unused capability struct, or implement functions that utilize this capability to perform restricted/admin-gated operations.".to_string(),
                            recommendation: format!(
                                "Delete the unused capability struct '{}' in module '{}' or use it to gate privileged functions.",
                                cap, module.name
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
