//! Project Discovery for finding Move packages.

use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Simple structure to parse package section of Move.toml
#[derive(Debug, Deserialize)]
pub struct MoveToml {
    pub package: MovePackageSection,
    #[serde(default)]
    pub addresses: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct MovePackageSection {
    pub name: String,
    pub version: Option<String>,
}

/// Discovers a Move project's structure from a root directory.
pub struct ProjectDiscovery;

impl ProjectDiscovery {
    /// Locates the Move.toml in the target directory and returns metadata.
    pub fn discover(root: &Path) -> std::result::Result<(String, HashMap<String, String>, Vec<PathBuf>), String> {
        let toml_path = root.join("Move.toml");
        if !toml_path.exists() {
            return Err(format!("Move.toml not found in {:?}", root));
        }

        let toml_content = fs::read_to_string(&toml_path)
            .map_err(|e| format!("Failed to read Move.toml: {}", e))?;

        let move_toml: MoveToml = toml::from_str(&toml_content)
            .map_err(|e| format!("Failed to parse Move.toml: {}", e))?;

        let package_name = move_toml.package.name.clone();
        let addresses = move_toml.addresses.clone();

        // Find all .move files in the sources directory
        let sources_dir = root.join("sources");
        let mut source_files = Vec::new();
        if sources_dir.exists() && sources_dir.is_dir() {
            Self::collect_move_files(&sources_dir, &mut source_files)?;
        }

        Ok((package_name, addresses, source_files))
    }

    fn collect_move_files(dir: &Path, files: &mut Vec<PathBuf>) -> std::result::Result<(), String> {
        for entry in fs::read_dir(dir).map_err(|e| format!("Failed to read directory {:?}: {}", dir, e))? {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let path = entry.path();
            if path.is_dir() {
                Self::collect_move_files(&path, files)?;
            } else if path.extension().and_then(|s| s.to_str()) == Some("move") {
                files.push(path);
            }
        }
        Ok(())
    }
}
