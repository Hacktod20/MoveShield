//! MoveShield Finding Engine for standardizing and deduplicating findings.

use moveshield_core::models::Finding;

/// Finding Engine that normalizes and orders findings deterministically.
pub struct FindingEngine;

impl FindingEngine {
    /// Creates a new FindingEngine instance.
    pub fn new() -> Self {
        Self
    }

    /// Normalizes, deduplicates, and deterministically sorts findings before they are passed to the Report Engine.
    pub fn process(&self, mut findings: Vec<Finding>) -> Vec<Finding> {
        // Deduplicate findings by exact match
        let mut unique_findings = Vec::new();
        for finding in findings.drain(..) {
            if !unique_findings.iter().any(|f: &Finding| {
                f.rule_id == finding.rule_id
                    && f.location == finding.location
                    && f.message == finding.message
            }) {
                unique_findings.push(finding);
            }
        }

        // Deterministically sort findings by rule_id, file path, line number, then column
        unique_findings.sort_by(|a, b| {
            a.rule_id.cmp(&b.rule_id)
                .then_with(|| a.location.file.cmp(&b.location.file))
                .then_with(|| a.location.start_line.cmp(&b.location.start_line))
                .then_with(|| a.location.start_column.cmp(&b.location.start_column))
        });

        unique_findings
    }
}
