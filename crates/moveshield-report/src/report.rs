//! Reporting Engine for Terminal summaries and standard-compliant SARIF export.

use moveshield_core::models::{Finding, Project, Severity};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub help_uri: Option<String>,
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

    /// Serializes a deterministic JSON report containing project counts and findings.
    pub fn serialize_json(
        &self,
        project: &Project,
        findings: &[Finding],
    ) -> serde_json::Result<String> {
        let total_modules = project.packages.iter().map(|p| p.modules.len()).sum();
        let total_functions = project
            .packages
            .iter()
            .map(|p| p.modules.iter().map(|m| m.functions.len()).sum::<usize>())
            .sum();

        serde_json::to_string_pretty(&JsonReport {
            schema_version: 1,
            project: &project.name,
            summary: JsonSummary {
                total_modules,
                total_functions,
                total_findings: findings.len(),
            },
            findings,
        })
    }

    /// Writes a JSON report to a file.
    pub fn generate_json(
        &self,
        project: &Project,
        findings: &[Finding],
        out_path: &Path,
    ) -> std::io::Result<()> {
        let serialized = self
            .serialize_json(project, findings)
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))?;
        fs::write(out_path, serialized)
    }

    /// Serializes findings as Markdown.
    pub fn serialize_markdown(&self, project: &Project, findings: &[Finding]) -> String {
        let mut output = format!(
            "# MoveShield report: {}\n\n",
            escape_markdown(&project.name)
        );
        output.push_str(&format!("Findings: {}\n", findings.len()));

        for finding in findings {
            output.push_str(&format!(
                "\n## {} - {}\n\n- Severity: {}\n- Confidence: {}\n- Location: `{}:{}:{}`\n\n",
                escape_markdown(&finding.rule_id),
                escape_markdown(&finding.title),
                escape_markdown(&format!("{:?}", finding.severity)),
                escape_markdown(&format!("{:?}", finding.confidence)),
                escape_markdown(&finding.location.file.display().to_string()),
                finding.location.start_line,
                finding.location.start_column
            ));
            output.push_str(&format!(
                "{}\n\n{}\n\n**Suggested fix:** {}\n\n**Recommendation:** {}\n",
                escape_markdown(&finding.message),
                escape_markdown(&finding.explanation),
                escape_markdown(&finding.suggested_fix),
                escape_markdown(&finding.recommendation)
            ));
            if !finding.references.is_empty() {
                output.push_str("\n**References:** ");
                output.push_str(
                    &finding
                        .references
                        .iter()
                        .map(|reference| escape_markdown(reference))
                        .collect::<Vec<_>>()
                        .join(", "),
                );
                output.push('\n');
            }
        }

        output
    }

    /// Writes a Markdown report to a file.
    pub fn generate_markdown(
        &self,
        project: &Project,
        findings: &[Finding],
        out_path: &Path,
    ) -> std::io::Result<()> {
        fs::write(out_path, self.serialize_markdown(project, findings))
    }

    /// Serializes findings as a self-contained, safely escaped HTML report.
    pub fn serialize_html(&self, project: &Project, findings: &[Finding]) -> String {
        let mut output = format!(
            "<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n<title>MoveShield report: {}</title>\n</head>\n<body>\n<h1>MoveShield report: {}</h1>\n<p>Findings: {}</p>\n",
            escape_html(&project.name),
            escape_html(&project.name),
            findings.len()
        );

        for finding in findings {
            output.push_str(&format!(
                "<article>\n<h2>{} - {}</h2>\n<p><strong>Severity:</strong> {} | <strong>Confidence:</strong> {}</p>\n<p><strong>Location:</strong> {}:{}:{}</p>\n<p>{}</p>\n<p>{}</p>\n<p><strong>Suggested fix:</strong> {}</p>\n<p><strong>Recommendation:</strong> {}</p>\n",
                escape_html(&finding.rule_id),
                escape_html(&finding.title),
                escape_html(&format!("{:?}", finding.severity)),
                escape_html(&format!("{:?}", finding.confidence)),
                escape_html(&finding.location.file.display().to_string()),
                finding.location.start_line,
                finding.location.start_column,
                escape_html(&finding.message),
                escape_html(&finding.explanation),
                escape_html(&finding.suggested_fix),
                escape_html(&finding.recommendation)
            ));
            if !finding.references.is_empty() {
                output.push_str("<ul>");
                for reference in &finding.references {
                    output.push_str(&format!("<li>{}</li>", escape_html(reference)));
                }
                output.push_str("</ul>\n");
            }
            output.push_str("</article>\n");
        }

        output.push_str("</body>\n</html>\n");
        output
    }

    /// Writes a self-contained HTML report to a file.
    pub fn generate_html(
        &self,
        project: &Project,
        findings: &[Finding],
        out_path: &Path,
    ) -> std::io::Result<()> {
        fs::write(out_path, self.serialize_html(project, findings))
    }

    /// Generates a SARIF 2.1.0 compliant report file.
    pub fn generate_sarif(
        &self,
        _project: &Project,
        findings: &[Finding],
        out_path: &Path,
    ) -> std::io::Result<()> {
        let mut rules_map = BTreeMap::new();

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
                            Some(finding.references[0].clone())
                        } else {
                            None
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
            schema: "https://json.schemastore.org/sarif-2.1.0.json".to_string(),
            version: "2.1.0".to_string(),
            runs: vec![run],
        };

        let serialized = serde_json::to_string_pretty(&log)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        fs::write(out_path, serialized)
    }
}

#[derive(Serialize)]
struct JsonReport<'a> {
    schema_version: u32,
    project: &'a str,
    summary: JsonSummary,
    findings: &'a [Finding],
}

#[derive(Serialize)]
struct JsonSummary {
    total_modules: usize,
    total_functions: usize,
    total_findings: usize,
}

fn escape_markdown(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        if matches!(
            character,
            '\\' | '`'
                | '*'
                | '_'
                | '{'
                | '}'
                | '['
                | ']'
                | '('
                | ')'
                | '#'
                | '+'
                | '-'
                | '.'
                | '!'
                | '|'
                | '>'
        ) {
            escaped.push('\\');
        }
        escaped.push(character);
    }
    escaped
}

fn escape_html(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        escaped.push_str(match character {
            '&' => "&amp;",
            '<' => "&lt;",
            '>' => "&gt;",
            '"' => "&quot;",
            '\'' => "&#39;",
            _ => {
                escaped.push(character);
                continue;
            }
        });
    }
    escaped
}

#[cfg(test)]
mod tests {
    use super::*;
    use moveshield_core::models::{Confidence, Location, RuleCategory};
    use std::path::PathBuf;

    fn fixture_finding(text: &str) -> Finding {
        Finding {
            rule_id: "MS001".to_string(),
            title: text.to_string(),
            category: RuleCategory::Security,
            severity: Severity::High,
            confidence: Confidence::High,
            location: Location {
                file: PathBuf::from("sources/module.move"),
                start_line: 1,
                start_column: 1,
                end_line: 1,
                end_column: 2,
            },
            module_name: "module".to_string(),
            function_name: "function".to_string(),
            message: text.to_string(),
            explanation: "Explanation".to_string(),
            evidence: Vec::new(),
            suggested_fix: "Fix".to_string(),
            recommendation: "Recommendation".to_string(),
            references: Vec::new(),
        }
    }

    fn fixture_project() -> Project {
        Project {
            name: "<script>alert(1)</script>".to_string(),
            root_path: PathBuf::from("."),
            packages: Vec::new(),
        }
    }

    #[test]
    fn html_report_escapes_untrusted_project_and_finding_text() {
        let report = ReportEngine.serialize_html(
            &fixture_project(),
            &[fixture_finding("<script>alert(1)</script>")],
        );

        assert!(report.contains("&lt;script&gt;alert(1)&lt;/script&gt;"));
        assert!(!report.contains("<script>alert(1)</script>"));
    }

    #[test]
    fn markdown_report_escapes_markup_delimiters() {
        let report =
            ReportEngine.serialize_markdown(&fixture_project(), &[fixture_finding("[issue]")]);

        assert!(report.contains("\\[issue\\]"));
    }
}
