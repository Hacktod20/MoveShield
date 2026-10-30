use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

/// Root configuration struct for MoveShield.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// Schema version of the config file.
    #[serde(default = "default_version")]
    pub version: u32,
    /// Rule configurations.
    #[serde(default)]
    pub rules: RulesConfig,
}

/// A generic configuration per-rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuleConfig {
    /// Whether this rule is enabled.
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    /// Optional severity override for findings produced by this rule.
    pub severity_override: Option<String>,
    /// Optional confidence override for findings produced by this rule.
    pub confidence_override: Option<String>,
    /// Optional max cyclomatic complexity for MS001, or general threshold parameter.
    pub max_complexity: Option<usize>,
}

impl Default for RuleConfig {
    fn default() -> Self {
        Self {
            enabled: default_enabled(),
            severity_override: None,
            confidence_override: None,
            max_complexity: None,
        }
    }
}

/// Rules section inside config.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RulesConfig {
    /// Configuration for MS001 (Cyclomatic Complexity).
    #[serde(rename = "MS001")]
    pub ms001: Option<MS001Config>,
    /// Catch-all generic configurations for other rules (MS002 - MS015).
    #[serde(flatten)]
    pub other_rules: BTreeMap<String, RuleConfig>,
}

/// Specific configuration for MS001 (Cyclomatic Complexity).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MS001Config {
    /// Whether this rule is enabled.
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    /// Maximum allowed cyclomatic complexity.
    #[serde(default = "default_max_complexity")]
    pub max_complexity: usize,
    /// Optional severity override for findings produced by this rule.
    pub severity_override: Option<String>,
    /// Optional confidence override for findings produced by this rule.
    pub confidence_override: Option<String>,
}

impl Default for MS001Config {
    fn default() -> Self {
        Self {
            enabled: default_enabled(),
            max_complexity: default_max_complexity(),
            severity_override: None,
            confidence_override: None,
        }
    }
}

fn default_version() -> u32 {
    1
}

fn default_enabled() -> bool {
    true
}

fn default_max_complexity() -> usize {
    10
}

impl Config {
    /// Create a default configuration.
    pub fn default_config() -> Self {
        Self {
            version: 1,
            rules: RulesConfig {
                ms001: Some(MS001Config::default()),
                other_rules: BTreeMap::new(),
            },
        }
    }

    /// Checks if a rule is enabled.
    pub fn is_rule_enabled(&self, rule_id: &str) -> bool {
        if rule_id == "MS001" {
            self.rules.ms001.as_ref().map(|c| c.enabled).unwrap_or(true)
        } else {
            self.rules
                .other_rules
                .get(rule_id)
                .map(|c| c.enabled)
                .unwrap_or(true)
        }
    }

    /// Validates supported schema versions and configured rule metadata.
    pub fn validate(&self) -> Result<(), String> {
        if self.version != 1 {
            return Err(format!(
                "Unsupported configuration version {}; supported version is 1",
                self.version
            ));
        }

        if let Some(config) = &self.rules.ms001 {
            validate_overrides(
                "MS001",
                config.severity_override.as_deref(),
                config.confidence_override.as_deref(),
            )?;
        }

        for (rule_id, config) in &self.rules.other_rules {
            if rule_id.len() != 5
                || !rule_id.starts_with("MS")
                || !rule_id[2..].bytes().all(|byte| byte.is_ascii_digit())
            {
                return Err(format!(
                    "Invalid rule identifier '{}'; expected an identifier such as MS002",
                    rule_id
                ));
            }
            validate_overrides(
                rule_id,
                config.severity_override.as_deref(),
                config.confidence_override.as_deref(),
            )?;
        }

        Ok(())
    }

    /// Load config from a file path.
    pub fn load_from_path(path: &Path) -> Result<Self, String> {
        if !path.exists() {
            return Err(format!("Configuration file does not exist: {:?}", path));
        }
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read configuration file: {}", e))?;
        let config: Config = toml::from_str(&content)
            .map_err(|e| format!("Failed to parse TOML configuration: {}", e))?;
        config.validate()?;
        Ok(config)
    }

    /// Load from a project directory, defaulting only when no config file exists.
    pub fn load_from_dir(dir: &Path) -> Result<Self, String> {
        let path = dir.join("moveshield.toml");
        if path.exists() {
            Self::load_from_path(&path)
        } else {
            Ok(Self::default_config())
        }
    }
}

fn validate_overrides(
    rule_id: &str,
    severity: Option<&str>,
    confidence: Option<&str>,
) -> Result<(), String> {
    if let Some(severity) = severity {
        if !matches!(
            severity.to_ascii_lowercase().as_str(),
            "critical" | "high" | "medium" | "low" | "informational" | "info"
        ) {
            return Err(format!(
                "Invalid severity override '{}' for {}; expected critical, high, medium, low, or informational",
                severity, rule_id
            ));
        }
    }

    if let Some(confidence) = confidence {
        if !matches!(
            confidence.to_ascii_lowercase().as_str(),
            "high" | "medium" | "low"
        ) {
            return Err(format!(
                "Invalid confidence override '{}' for {}; expected high, medium, or low",
                confidence, rule_id
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_schema_version_and_rule_overrides() {
        let valid: Config = toml::from_str(
            r#"
version = 1
[rules.MS001]
enabled = true
max_complexity = 12
severity_override = "high"
confidence_override = "medium"
"#,
        )
        .unwrap();
        assert!(valid.validate().is_ok());

        let invalid: Config = toml::from_str(
            r#"
version = 1
[rules.MS002]
severity_override = "urgent"
"#,
        )
        .unwrap();
        assert!(invalid.validate().unwrap_err().contains("Invalid severity"));
    }

    #[test]
    fn default_generic_rule_configuration_enables_the_rule() {
        assert!(RuleConfig::default().enabled);
    }

    #[test]
    fn rejects_misspelled_rule_keys_and_unknown_rule_options() {
        let misspelled: Config = toml::from_str(
            r#"
version = 1
[rules.MS2]
enabled = true
"#,
        )
        .unwrap();
        assert!(misspelled
            .validate()
            .unwrap_err()
            .contains("Invalid rule identifier"));

        assert!(toml::from_str::<Config>(
            r#"
version = 1
unexpected = true
"#
        )
        .is_err());
    }
}
