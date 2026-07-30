//! MS010: Object Ownership Violations.

use crate::{rules::resolve_confidence, rules::resolve_severity, Rule};
use moveshield_config::Config;
use moveshield_core::models::{
    Confidence, Evidence, EvidenceKind, Finding, Location, RuleCategory, Severity,
};
use moveshield_knowledge::KnowledgeStore;
use std::collections::HashSet;

/// MS010: Object Ownership Violations.
/// Detects wrapped objects (key structs nested inside other key structs) which might violate standard ownership.
pub struct MS010;

impl Rule for MS010 {
    fn id(&self) -> &'static str {
        "MS010"
    }

    fn name(&self) -> &'static str {
        "Object Ownership Violations"
    }

    fn description(&self) -> &'static str {
        "Detects wrapped objects (nested key structs) that may lead to ownership inconsistencies."
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
        Some("CWE-284") // Improper Access Control
    }

    fn references(&self) -> Vec<&'static str> {
        vec!["https://docs.sui.io/concepts/sui-move-concepts/object-ownership/wrapped"]
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
                // 1. Identify all defined structs with 'key' ability (representing objects)
                let mut key_structs = HashSet::new();
                for s in &module.structs {
                    if s.abilities.iter().any(|a| a == "key") {
                        key_structs.insert(s.name.clone());
                    }
                }

                // 2. Scan each key struct's fields for references to other key structs
                for s in &module.structs {
                    if !s.abilities.iter().any(|a| a == "key") {
                        continue;
                    }

                    for field in &s.fields {
                        // Check if the field type matches any of the defined key structs
                        let wraps_key_object = key_structs.iter().any(|key_name| {
                            field.type_str.contains(key_name)
                        });

                        if wraps_key_object {
                            let loc = Location {
                                file: module.source_file.clone(),
                                start_line: 1,
                                start_column: 1,
                                end_line: 1,
                                end_column: 1,
                            };

                            let evidence = vec![
                                Evidence {
                                    kind: EvidenceKind::MatchedObject,
                                    message: format!(
                                        "Struct '{}' (key ability) wraps field '{}' of type '{}' (which also has key ability).",
                                        s.name, field.name, field.type_str
                                    ),
                                    snippet: Some(format!("struct {} ...", s.name)),
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
                                    "Object Ownership Violation: struct '{}' wraps another key-ability object '{}'.",
                                    s.name, field.type_str
                                ),
                                explanation: "Wrapping key-ability structs directly inside other key-ability structs obscures object ownership in Sui, making nested objects inaccessible directly by ID and potentially causing inconsistent state updates or rendering the nested object un-transferable.".to_string(),
                                evidence,
                                suggested_fix: "Consider using dynamic fields/dynamic object fields or ID-based references (e.g. UID or address) to establish ownership/parent-child relationships instead of direct wrapping.".to_string(),
                                recommendation: format!(
                                    "Refactor struct '{}' to reference the child object '{}' by ID or via dynamic object fields rather than direct structural wrapping.",
                                    s.name, field.type_str
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
