//! MoveShield Testing Harness crate.
//!
//! Exposes utility routines, regression harnesses, and mocks
//! to consistently validate static analysis, configuration, and reporting logic.

/// Runs the complete suite of MoveShield integration tests.
pub fn run_test_suite() {}

#[cfg(test)]
mod tests {
    use moveshield_compiler::{CompilerAdapter, DevelopmentParserAdapter};
    use moveshield_config::Config;
    use moveshield_graph::CallGraph;
    use moveshield_ir::MoveIR;
    use moveshield_knowledge::KnowledgeStore;
    use std::collections::HashSet;
    use std::path::Path;

    #[test]
    fn test_integration_vulnerable_fixtures() {
        let path = Path::new("../../tests/integration/fixtures/vulnerable");
        let adapter = DevelopmentParserAdapter;
        let project = adapter.compile(path).expect("Failed to compile vulnerable fixtures");

        let ir = MoveIR::from_project(&project);
        let call_graph = CallGraph::build(ir);
        let store = KnowledgeStore::new(call_graph, project);
        let config = Config::default_config();

        let rules = moveshield_rules::all_rules();
        let mut triggered_rules = HashSet::new();
        let mut total_findings = 0;

        for rule in &rules {
            let findings = rule.run(&store, &config);
            println!("Rule {}: found {} findings", rule.id(), findings.len());
            for finding in &findings {
                println!("  Finding message: {}", finding.message);
                triggered_rules.insert(rule.id().to_string());
            }
            total_findings += findings.len();
        }

        // Verify that EVERY SINGLE RULE triggers in the vulnerable package!
        let expected_rules = vec![
            "MS001", "MS002", "MS003", "MS004", "MS005", "MS006", "MS007", "MS008", "MS009",
            "MS010", "MS011", "MS012", "MS013", "MS014", "MS015",
        ];

        for rule_id in &expected_rules {
            assert!(
                triggered_rules.contains(*rule_id),
                "Expected rule {} to trigger in vulnerable fixtures, but it did not!",
                rule_id
            );
        }

        assert!(total_findings >= expected_rules.len());
    }

    #[test]
    fn test_integration_safe_fixtures() {
        let path = Path::new("../../tests/integration/fixtures/safe");
        let adapter = DevelopmentParserAdapter;
        let project = adapter.compile(path).expect("Failed to compile safe fixtures");

        let ir = MoveIR::from_project(&project);
        let call_graph = CallGraph::build(ir);
        let store = KnowledgeStore::new(call_graph, project);
        let config = Config::default_config();

        let rules = moveshield_rules::all_rules();
        let mut total_findings = 0;

        for rule in &rules {
            let findings = rule.run(&store, &config);
            for finding in &findings {
                println!("SAFE FIXTURES FAILURE - Triggered rule: {} with message: {}", rule.id(), finding.message);
            }
            total_findings += findings.len();
        }

        // Verify that NO security/architectural findings trigger in the safe package!
        assert_eq!(
            total_findings, 0,
            "Expected 0 findings in safe fixtures, but found {}!",
            total_findings
        );
    }
}
