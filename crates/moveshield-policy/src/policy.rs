//! MoveShield Policy Engine for filtering and resolving policy overrides.

use moveshield_core::models::Finding;
use moveshield_config::Config;

/// Policy Engine to apply organizational filters and override severities.
pub struct PolicyEngine;

impl PolicyEngine {
    /// Creates a new PolicyEngine instance.
    pub fn new() -> Self {
        Self
    }

    /// Filters findings and applies overrides according to policy configuration.
    pub fn filter_and_apply(&self, findings: Vec<Finding>, _config: &Config) -> Vec<Finding> {
        // Currently, all findings are permitted without filtering in Milestone 1
        findings
    }
}
