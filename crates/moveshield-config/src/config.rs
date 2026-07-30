use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Root configuration struct for MoveShield.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Schema version of the config file.
    #[serde(default = "default_version")]
    pub version: u32,
    /// Rule configurations.
    #[serde(default)]
    pub rules: RulesConfig,
}

/// Rules section inside config.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RulesConfig {
    /// Configuration for MS001 (Cyclomatic Complexity).
    #[serde(rename = "MS001")]
    pub ms001: Option<MS001Config>,
}

/// Specific configuration for MS001 (Cyclomatic Complexity).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MS001Config {
    /// Whether this rule is enabled.
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    /// Maximum allowed cyclomatic complexity.
    #[serde(default = "default_max_complexity")]
    pub max_complexity: usize,
}

impl Default for MS001Config {
    fn default() -> Self {
        Self {
            enabled: default_enabled(),
            max_complexity: default_max_complexity(),
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
            },
        }
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
        Ok(config)
    }

    /// Load from a project directory, looking for moveshield.toml. Fallbacks to default if not found.
    pub fn load_from_dir(dir: &Path) -> Self {
        let path = dir.join("moveshield.toml");
        if path.exists() {
            match Self::load_from_path(&path) {
                Ok(cfg) => cfg,
                Err(_) => Self::default_config(),
            }
        } else {
            Self::default_config()
        }
    }
}
