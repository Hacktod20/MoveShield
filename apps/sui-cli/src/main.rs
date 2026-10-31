//! CLI for analyzing Sui Move packages with the official Sui Move compiler.

mod adapter;

use adapter::SuiCompilerAdapter;
use clap::{Parser, Subcommand, ValueEnum};
use moveshield_compiler::CompilerAdapter;
use moveshield_config::Config;
use moveshield_findings::FindingEngine;
use moveshield_graph::CallGraph;
use moveshield_ir::MoveIR;
use moveshield_knowledge::KnowledgeStore;
use moveshield_policy::PolicyEngine;
use moveshield_report::ReportEngine;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Instant;

#[derive(Parser)]
#[command(
    name = "moveshield-sui",
    version,
    about = "MoveShield CLI using the official Sui Move compiler"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze a Sui Move package with the official Sui Move compiler
    Analyze(AnalyzeArgs),
}

#[derive(clap::Args)]
struct AnalyzeArgs {
    /// Path to the Move package directory
    #[arg(default_value = ".")]
    path: PathBuf,
    /// Select terminal, JSON, SARIF, Markdown, HTML, or all output
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

fn main() -> ExitCode {
    ExitCode::from(run())
}

fn run() -> u8 {
    match Cli::parse().command {
        Commands::Analyze(args) => analyze(args),
    }
}

fn analyze(args: AnalyzeArgs) -> u8 {
    let start_time = Instant::now();
    let config_path = args
        .config
        .clone()
        .unwrap_or_else(|| args.path.join("moveshield.toml"));
    let config = if args.config.is_some() || config_path.exists() {
        match Config::load_from_path(&config_path) {
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
        "warning: package syntax and types are checked by the official Sui compiler, but MoveShield findings remain incomplete and do not guarantee security"
    );
    let project = match SuiCompilerAdapter.compile(&args.path) {
        Ok(project) => project,
        Err(error) => {
            eprintln!("[Analysis Failed] Sui compiler analysis error: {error}");
            return 1;
        }
    };

    let ir = MoveIR::from_project(&project);
    let call_graph = CallGraph::build(ir);
    let store = KnowledgeStore::new(call_graph, project.clone());
    let findings = moveshield_rules::all_rules()
        .into_iter()
        .flat_map(|rule| rule.run(&store, &config))
        .collect();
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
