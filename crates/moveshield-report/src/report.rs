//! Reporting Engine for Terminal summaries and standard-compliant SARIF export.

use moveshield_core::models::{Finding, Project, Severity};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Structures representing the standard SARIF 2.1.0 format.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SarifLog {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub version: String,
    pub runs: Vec<SarifRun>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SarifRun {
    pub tool: SarifTool,
    pub results: Vec<SarifResult>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SarifTool {
    pub driver: SarifDriver,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SarifDriver {
    pub name: String,
    pub version: String,
    pub rules: Vec<SarifRule>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SarifRule {
    pub id: String,
    pub name: String,
    pub short_description: SarifMultiformatMessageString,
    pub full_description: SarifMultiformatMessageString,
    pub help_uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SarifMultiformatMessageString {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SarifResult {
    pub rule_id: String,
    pub level: String,
    pub message: SarifMessage,
    pub locations: Vec<SarifLocation>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SarifMessage {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SarifLocation {
    pub physical_location: SarifPhysicalLocation,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SarifPhysicalLocation {
    pub artifact_location: SarifArtifactLocation,
    pub region: SarifRegion,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SarifArtifactLocation {
    pub uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SarifRegion {
    pub start_line: usize,
    pub start_column: usize,
}

/// The main Report Engine.
pub struct ReportEngine;

impl Default for ReportEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ReportEngine {
    /// Creates a new ReportEngine.
    pub fn new() -> Self {
        Self
    }

    /// Generates a human-readable Terminal summary.
    pub fn generate_terminal(&self, project: &Project, findings: &[Finding], duration_ms: u128) {
        let total_modules: usize = project.packages.iter().map(|p| p.modules.len()).sum();
        let total_functions: usize = project
            .packages
            .iter()
            .map(|p| p.modules.iter().map(|m| m.functions.len()).sum::<usize>())
            .sum();

        println!(
            "\n================================================================================"
        );
        println!(
            "                         MoveShield Analysis Report                             "
        );
        println!(
            "================================================================================"
        );
        println!("Project:          {}", project.name);
        println!("Total Modules:    {}", total_modules);
        println!("Total Functions:  {}", total_functions);
        println!("Scan Duration:    {} ms", duration_ms);
        println!(
            "--------------------------------------------------------------------------------"
        );

        if findings.is_empty() {
            println!("Status:           \x1b[32mPASSED (No findings detected)\x1b[0m");
        } else {
            let mut highest_severity = Severity::Informational;
            for finding in findings {
                if finding.severity < highest_severity {
                    highest_severity = finding.severity;
                }
            }
            let severity_str = match highest_severity {
                Severity::Critical => "\x1b[31mCRITICAL\x1b[0m",
                Severity::High => "\x1b[31mHIGH\x1b[0m",
                Severity::Medium => "\x1b[33mMEDIUM\x1b[0m",
                Severity::Low => "\x1b[34mLOW\x1b[0m",
                Severity::Informational => "\x1b[36mINFORMATIONAL\x1b[0m",
            };
            println!("Status:           \x1b[31mFAILED\x1b[0m");
            println!("Highest Severity: {}", severity_str);
            println!("Total Findings:   {}", findings.len());
            println!("\nDetailed Findings:");
            for (idx, finding) in findings.iter().enumerate() {
                let sev_label = match finding.severity {
                    Severity::Critical => "\x1b[31m[Critical]\x1b[0m",
                    Severity::High => "\x1b[31m[High]\x1b[0m",
                    Severity::Medium => "\x1b[33m[Medium]\x1b[0m",
                    Severity::Low => "\x1b[34m[Low]\x1b[0m",
                    Severity::Informational => "\x1b[36m[Info]\x1b[0m",
                };

                println!(
                    "  {}. {} {} - {}",
                    idx + 1,
                    sev_label,
                    finding.rule_id,
                    finding.title
                );
                println!(
                    "     File:       {:?}:{}:{}",
                    finding.location.file,
                    finding.location.start_line,
                    finding.location.start_column
                );
                if !finding.module_name.is_empty() {
                    println!("     Module:     {}", finding.module_name);
                }
                if !finding.function_name.is_empty() {
                    println!("     Function:   {}", finding.function_name);
                }
                println!("     Message:    {}", finding.message);
                println!("     Explanation:{}", finding.explanation);

                if !finding.evidence.is_empty() {
                    println!("     Evidence:");
                    for ev in &finding.evidence {
                        println!("       * [{:?}] {}", ev.kind, ev.message);
                        if let Some(ref snip) = ev.snippet {
                            // Trim snippet and display it nicely indented
                            let trimmed: String = snip
                                .lines()
                                .take(3)
                                .collect::<Vec<&str>>()
                                .join("\n         ");
                            println!("         Snippet: {}", trimmed);
                        }
                    }
                }

                println!("     Fix:        {}", finding.suggested_fix);
                println!("     Recommendation: {}", finding.recommendation);
                if !finding.references.is_empty() {
                    println!("     References: {}", finding.references.join(", "));
                }
                println!();
            }
        }
        println!(
            "================================================================================"
        );
    }

    /// Generates a SARIF 2.1.0 compliant report file.
    pub fn generate_sarif(
        &self,
        _project: &Project,
        findings: &[Finding],
        out_path: &Path,
    ) -> std::io::Result<()> {
        let mut rules_map = std::collections::HashMap::new();

        let mut results = Vec::new();
        for finding in findings {
            let sarif_level = match finding.severity {
                Severity::Critical => "error",
                Severity::High => "error",
                Severity::Medium => "warning",
                Severity::Low => "note",
                Severity::Informational => "note",
            };

            // Populate rules_map dynamically with standard metadata
            if !rules_map.contains_key(&finding.rule_id) {
                rules_map.insert(
                    finding.rule_id.clone(),
                    SarifRule {
                        id: finding.rule_id.clone(),
                        name: finding.title.clone(),
                        short_description: SarifMultiformatMessageString {
                            text: finding.message.clone(),
                        },
                        full_description: SarifMultiformatMessageString {
                            text: finding.explanation.clone(),
                        },
                        help_uri: if !finding.references.is_empty() {
                            finding.references[0].clone()
                        } else {
                            "".to_string()
                        },
                    },
                );
            }

            // Construct details including evidence messages
            let mut message_text = format!(
                "{}\n\nExplanation: {}\n\nFix: {}\n\nRecommendation: {}",
                finding.message, finding.explanation, finding.suggested_fix, finding.recommendation
            );

            if !finding.evidence.is_empty() {
                message_text.push_str("\n\nEvidence:");
                for ev in &finding.evidence {
                    message_text.push_str(&format!("\n- [{:?}] {}", ev.kind, ev.message));
                }
            }

            results.push(SarifResult {
                rule_id: finding.rule_id.clone(),
                level: sarif_level.to_string(),
                message: SarifMessage { text: message_text },
                locations: vec![SarifLocation {
                    physical_location: SarifPhysicalLocation {
                        artifact_location: SarifArtifactLocation {
                            uri: finding.location.file.to_string_lossy().into_owned(),
                        },
                        region: SarifRegion {
                            start_line: finding.location.start_line,
                            start_column: finding.location.start_column,
                        },
                    },
                }],
            });
        }

        let driver = SarifDriver {
            name: "MoveShield".to_string(),
            version: "0.1.0".to_string(),
            rules: rules_map.into_values().collect(),
        };

        let run = SarifRun {
            tool: SarifTool { driver },
            results,
        };

        let log = SarifLog {
            schema: "https://schemastore.org/json/schema/sarif-2.1.0-rtm.5.json".to_string(),
            version: "2.1.0".to_string(),
            runs: vec![run],
        };

        let serialized = serde_json::to_string_pretty(&log)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        let mut file = File::create(out_path)?;
        file.write_all(serialized.as_bytes())?;

        Ok(())
    }
}
