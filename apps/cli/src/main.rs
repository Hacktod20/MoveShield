//! MoveShield CLI tool entrypoint.

use clap::{Parser, Subcommand};
use moveshield_compiler::{CompilerAdapter, DevelopmentParserAdapter};
use moveshield_config::Config;
use moveshield_findings::FindingEngine;
use moveshield_ir::MoveIR;
use moveshield_graph::CallGraph;
use moveshield_knowledge::KnowledgeStore;
use moveshield_policy::PolicyEngine;
use moveshield_report::ReportEngine;
use moveshield_rules::Rule;

use std::path::{Path, PathBuf};
use std::time::Instant;

#[derive(Parser)]
#[command(name = "moveshield", version = "0.1.0", about = "MoveShield static analysis platform")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze a Move project
    Analyze {
        /// Path to the Move project directory
        #[arg(default_value = ".")]
        path: PathBuf,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Analyze { path } => {
            let start_time = Instant::now();

            // 1. Resolve configuration (Exit 3 on configuration error)
            let config_path = path.join("moveshield.toml");
            let config = if config_path.exists() {
                match Config::load_from_path(&config_path) {
                    Ok(cfg) => cfg,
                    Err(e) => {
                        eprintln!("\x1b[31m[Configuration Error] {}\x1b[0m", e);
                        std::process::exit(3);
                    }
                }
            } else {
                Config::default_config()
            };

            // 2. Project Discovery & Compiler Adapter (Exit 1 on failure)
            let adapter = DevelopmentParserAdapter;
            let project = match adapter.compile(&path) {
                Ok(proj) => proj,
                Err(e) => {
                    eprintln!("\x1b[31m[Analysis Failed] Project compilation/discovery error: {}\x1b[0m", e);
                    std::process::exit(1);
                }
            };

            // 3. Convert to MoveIR
            let ir = MoveIR::from_project(&project);

            // 4. Graph Engine
            let call_graph = CallGraph::build(ir);

            // 5. Knowledge Store
            let store = KnowledgeStore::new(call_graph);

            // 6. Rule Engine (MS001 Complexity)
            let mut findings = Vec::new();
            let rule_ms001 = moveshield_rules::MS001;
            let rule_findings = rule_ms001.run(&store, &config);
            findings.extend(rule_findings);

            // 7. Policy Engine
            let policy_engine = PolicyEngine::new();
            let filtered_findings = policy_engine.filter_and_apply(findings, &config);

            // 8. Finding Engine
            let finding_engine = FindingEngine::new();
            let processed_findings = finding_engine.process(filtered_findings);

            // 9. Reporting Engine
            let elapsed_ms = start_time.elapsed().as_millis();
            let report_engine = ReportEngine::new();

            // Terminal summary
            report_engine.generate_terminal(&project, &processed_findings, elapsed_ms);

            // SARIF report (saved to ./moveshield.sarif)
            let sarif_path = Path::new("moveshield.sarif");
            if let Err(e) = report_engine.generate_sarif(&project, &processed_findings, sarif_path) {
                eprintln!("\x1b[31m[Error] Failed to generate SARIF report: {}\x1b[0m", e);
                std::process::exit(1);
            } else {
                println!("SARIF report successfully saved to {:?}", sarif_path);
            }

            // 10. Exit codes
            // 0 -> Success, no blocking findings.
            // 2 -> Blocking findings detected.
            if !processed_findings.is_empty() {
                std::process::exit(2);
            } else {
                std::process::exit(0);
            }
        }
    }
}
