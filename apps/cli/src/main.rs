//! MoveShield CLI tool entrypoint.

use clap::{Parser, Subcommand, ValueEnum};
use moveshield_compiler::{CompilerAdapter, DevelopmentParserAdapter};
use moveshield_config::Config;
use moveshield_findings::FindingEngine;
use moveshield_graph::CallGraph;
use moveshield_ir::MoveIR;
use moveshield_knowledge::KnowledgeStore;
use moveshield_policy::PolicyEngine;
use moveshield_report::ReportEngine;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::time::Instant;

#[derive(Parser)]
#[command(
    name = "moveshield",
    version = "0.1.0",
    about = "MoveShield static analysis platform"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze a Move project
    Analyze(AnalyzeArgs),
    /// Create a default moveshield.toml without overwriting existing files
    Init {
        /// Move project directory
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Inspect or validate MoveShield configuration
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
    /// Check the local Rust and optional Sui development tools
    Doctor,
    /// List all built-in analysis rules
    Rules,
    /// Show details for one built-in rule
    Explain {
        /// Rule identifier, for example MS001
        rule_id: String,
    },
}

#[derive(clap::Args)]
struct AnalyzeArgs {
    /// Path to the Move project directory
    #[arg(default_value = ".")]
    path: PathBuf,
    /// Select terminal, JSON, SARIF, Markdown, HTML, or default terminal-plus-SARIF output
    #[arg(long, value_enum, default_value_t = OutputFormat::All)]
    format: OutputFormat,
    /// Output file for JSON, SARIF, Markdown, or HTML (with --format all, selects SARIF)
    #[arg(long)]
    output: Option<PathBuf>,
    /// Use a specific MoveShield TOML configuration file
    #[arg(long)]
    config: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum OutputFormat {
    Terminal,
    Json,
    Sarif,
    Markdown,
    Html,
    All,
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// Validate a configuration file (or a project's moveshield.toml)
    Validate {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Print the resolved configuration
    Show {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
}

fn main() -> ExitCode {
    ExitCode::from(run())
}

fn run() -> u8 {
    match Cli::parse().command {
        Commands::Analyze(args) => analyze(args),
        Commands::Init { path } => initialize_config(&path),
        Commands::Config { command } => match command {
            ConfigCommands::Validate { path } => validate_config(&path),
            ConfigCommands::Show { path } => show_config(&path),
        },
        Commands::Doctor => doctor(),
        Commands::Rules => list_rules(),
        Commands::Explain { rule_id } => explain_rule(&rule_id),
    }
}

fn analyze(args: AnalyzeArgs) -> u8 {
    let start_time = Instant::now();
    let config_path = args
        .config
        .clone()
        .unwrap_or_else(|| args.path.join("moveshield.toml"));
    let config = if args.config.is_some() || config_path.exists() {
        match load_config(&config_path) {
            Ok(config) => config,
            Err(error) => {
                eprintln!("[Configuration Error] {error}");
                return 3;
            }
        }
    } else {
        Config::default_config()
    };

    eprintln!(
        "warning: using the development Move parser; results are heuristic and are not compiler-verified"
    );
    let adapter = DevelopmentParserAdapter;
    let project = match adapter.compile(&args.path) {
        Ok(project) => project,
        Err(error) => {
            eprintln!("[Analysis Failed] Project discovery/parsing error: {error}");
            return 1;
        }
    };

    let ir = MoveIR::from_project(&project);
    let call_graph = CallGraph::build(ir);
    let store = KnowledgeStore::new(call_graph, project.clone());

    let mut findings = Vec::new();
    for rule in moveshield_rules::all_rules() {
        findings.extend(rule.run(&store, &config));
    }

    let findings = PolicyEngine::new().filter_and_apply(findings, &config);
    let findings = FindingEngine::new().process(findings);
    let elapsed_ms = start_time.elapsed().as_millis();
    let report_engine = ReportEngine::new();

    let sarif_path = args
        .output
        .as_deref()
        .unwrap_or_else(|| Path::new("moveshield.sarif"));

    match args.format {
        OutputFormat::Terminal => {
            if args.output.is_some() {
                eprintln!("--output requires --format json, sarif, markdown, html, or all");
                return 1;
            }
            report_engine.generate_terminal(&project, &findings, elapsed_ms);
        }
        OutputFormat::Json => {
            if let Some(path) = args.output.as_deref() {
                if let Err(error) = report_engine.generate_json(&project, &findings, path) {
                    eprintln!("[Report Error] Failed to write JSON report: {error}");
                    return 1;
                }
                eprintln!("JSON report saved to {}", path.display());
            } else {
                match report_engine.serialize_json(&project, &findings) {
                    Ok(json) => println!("{json}"),
                    Err(error) => {
                        eprintln!("[Report Error] Failed to serialize JSON report: {error}");
                        return 1;
                    }
                }
            }
        }
        OutputFormat::Markdown => {
            let markdown = report_engine.serialize_markdown(&project, &findings);
            if let Some(path) = args.output.as_deref() {
                if let Err(error) = fs::write(path, markdown) {
                    eprintln!("[Report Error] Failed to write Markdown report: {error}");
                    return 1;
                }
                eprintln!("Markdown report saved to {}", path.display());
            } else {
                print!("{markdown}");
            }
        }
        OutputFormat::Html => {
            let html = report_engine.serialize_html(&project, &findings);
            if let Some(path) = args.output.as_deref() {
                if let Err(error) = fs::write(path, html) {
                    eprintln!("[Report Error] Failed to write HTML report: {error}");
                    return 1;
                }
                eprintln!("HTML report saved to {}", path.display());
            } else {
                print!("{html}");
            }
        }
        OutputFormat::Sarif => {
            if let Err(error) = report_engine.generate_sarif(&project, &findings, sarif_path) {
                eprintln!("[Report Error] Failed to write SARIF report: {error}");
                return 1;
            }
            eprintln!("SARIF report saved to {}", sarif_path.display());
        }
        OutputFormat::All => {
            report_engine.generate_terminal(&project, &findings, elapsed_ms);
            if let Err(error) = report_engine.generate_sarif(&project, &findings, sarif_path) {
                eprintln!("[Report Error] Failed to write SARIF report: {error}");
                return 1;
            }
            println!("SARIF report saved to {}", sarif_path.display());
        }
    }

    if findings.is_empty() {
        0
    } else {
        2
    }
}

fn initialize_config(project_path: &Path) -> u8 {
    let config_path = project_path.join("moveshield.toml");
    if config_path.exists() {
        eprintln!(
            "[Configuration Error] Refusing to overwrite {}",
            config_path.display()
        );
        return 3;
    }

    let contents = match toml::to_string_pretty(&Config::default_config()) {
        Ok(contents) => contents,
        Err(error) => {
            eprintln!("[Configuration Error] Failed to serialize defaults: {error}");
            return 1;
        }
    };
    match fs::write(&config_path, contents) {
        Ok(()) => {
            println!("Created {}", config_path.display());
            0
        }
        Err(error) => {
            eprintln!(
                "[Configuration Error] Failed to create {}: {error}",
                config_path.display()
            );
            1
        }
    }
}

fn validate_config(path: &Path) -> u8 {
    let config_path = resolve_config_path(path);
    match load_config(&config_path) {
        Ok(_) => {
            println!("Configuration is valid: {}", config_path.display());
            0
        }
        Err(error) => {
            eprintln!("[Configuration Error] {error}");
            3
        }
    }
}

fn show_config(path: &Path) -> u8 {
    let config_path = resolve_config_path(path);
    let config = match load_config(&config_path) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("[Configuration Error] {error}");
            return 3;
        }
    };
    match toml::to_string_pretty(&config) {
        Ok(contents) => {
            print!("{contents}");
            0
        }
        Err(error) => {
            eprintln!("[Configuration Error] Failed to serialize configuration: {error}");
            1
        }
    }
}

fn resolve_config_path(path: &Path) -> PathBuf {
    if path.is_dir() {
        path.join("moveshield.toml")
    } else {
        path.to_path_buf()
    }
}

fn load_config(path: &Path) -> Result<Config, String> {
    let config = Config::load_from_path(path)?;
    let registered_rules: Vec<&str> = moveshield_rules::all_rules()
        .iter()
        .map(|rule| rule.id())
        .collect();
    if let Some(unknown_rule) = config
        .rules
        .other_rules
        .keys()
        .find(|rule_id| !registered_rules.contains(&rule_id.as_str()))
    {
        return Err(format!(
            "Unknown rule identifier '{}'; use `moveshield rules` to list available rules",
            unknown_rule
        ));
    }
    Ok(config)
}

fn doctor() -> u8 {
    let mut status = 0;
    for tool in ["rustc", "cargo"] {
        match Command::new(tool).arg("--version").output() {
            Ok(output) if output.status.success() => {
                println!("{tool}: {}", String::from_utf8_lossy(&output.stdout).trim());
            }
            Ok(output) => {
                eprintln!(
                    "{tool}: failed ({})",
                    String::from_utf8_lossy(&output.stderr).trim()
                );
                status = 1;
            }
            Err(error) => {
                eprintln!("{tool}: unavailable ({error})");
                status = 1;
            }
        }
    }

    match Command::new("sui").arg("--version").output() {
        Ok(output) if output.status.success() => {
            println!("sui: {}", String::from_utf8_lossy(&output.stdout).trim())
        }
        _ => println!(
            "sui: not found (optional; analysis currently uses the heuristic development parser)"
        ),
    }
    status
}

fn list_rules() -> u8 {
    for rule in moveshield_rules::all_rules() {
        println!("{}  {}", rule.id(), rule.name());
    }
    0
}

fn explain_rule(rule_id: &str) -> u8 {
    let normalized_id = rule_id.to_ascii_uppercase();
    let Some(rule) = moveshield_rules::all_rules()
        .into_iter()
        .find(|rule| rule.id() == normalized_id)
    else {
        eprintln!("Unknown rule '{rule_id}'. Run `moveshield rules` to list built-in rules.");
        return 1;
    };

    println!("{} - {}", rule.id(), rule.name());
    println!("Category: {:?}", rule.category());
    println!("Default severity: {:?}", rule.default_severity());
    println!("Confidence: {:?}", rule.default_confidence());
    println!("{}", rule.description());
    for reference in rule.references() {
        println!("Reference: {reference}");
    }
    0
}
