use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

fn run_cli(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_moveshield"))
        .args(args)
        .output()
        .expect("failed to run MoveShield CLI")
}

fn temporary_directory() -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock is before UNIX epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "moveshield-cli-test-{}-{timestamp}",
        std::process::id()
    ));
    fs::create_dir(&path).expect("failed to create test directory");
    path
}

#[test]
fn json_analysis_is_machine_readable_and_preserves_findings_exit_code() {
    let example = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../example");
    let output = run_cli(&["analyze", example.to_str().unwrap(), "--format", "json"]);

    assert_eq!(output.status.code(), Some(2));
    let report: Value =
        serde_json::from_slice(&output.stdout).expect("analysis output should be valid JSON");
    assert_eq!(report["schema_version"], 1);
    assert_eq!(report["project"], "example");
    assert_eq!(report["summary"]["total_modules"], 2);
    assert!(!report["findings"].as_array().unwrap().is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("not compiler-verified"));
}

#[test]
fn init_creates_and_does_not_overwrite_configuration() {
    let dir = temporary_directory();
    let init = run_cli(&["init", dir.to_str().unwrap()]);
    assert_eq!(init.status.code(), Some(0));
    assert!(dir.join("moveshield.toml").is_file());

    let validate = run_cli(&["config", "validate", dir.to_str().unwrap()]);
    assert_eq!(validate.status.code(), Some(0));

    let second_init = run_cli(&["init", dir.to_str().unwrap()]);
    assert_eq!(second_init.status.code(), Some(3));
    fs::remove_dir_all(dir).expect("failed to remove test directory");
}

#[test]
fn malformed_configuration_has_configuration_exit_code() {
    let dir = temporary_directory();
    fs::write(dir.join("moveshield.toml"), "version = 999\n")
        .expect("failed to write test configuration");

    let output = run_cli(&["config", "validate", dir.to_str().unwrap()]);
    assert_eq!(output.status.code(), Some(3));
    assert!(String::from_utf8_lossy(&output.stderr).contains("Unsupported configuration version"));
    fs::remove_dir_all(dir).expect("failed to remove test directory");
}

#[test]
fn unknown_rule_configuration_is_not_silently_ignored() {
    let dir = temporary_directory();
    fs::write(
        dir.join("moveshield.toml"),
        "version = 1\n[rules.MS999]\nenabled = false\n",
    )
    .expect("failed to write test configuration");

    let output = run_cli(&["config", "validate", dir.to_str().unwrap()]);
    assert_eq!(output.status.code(), Some(3));
    assert!(String::from_utf8_lossy(&output.stderr).contains("Unknown rule identifier"));
    fs::remove_dir_all(dir).expect("failed to remove test directory");
}

#[test]
fn sarif_output_is_stable_and_uses_standard_schema_uri() {
    let dir = temporary_directory();
    let example = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../example");
    let report_path = dir.join("report.sarif");
    let output = run_cli(&[
        "analyze",
        example.to_str().unwrap(),
        "--format",
        "sarif",
        "--output",
        report_path.to_str().unwrap(),
    ]);

    assert_eq!(output.status.code(), Some(2));
    let report: Value =
        serde_json::from_slice(&fs::read(&report_path).unwrap()).expect("SARIF should be JSON");
    assert_eq!(
        report["$schema"],
        "https://json.schemastore.org/sarif-2.1.0.json"
    );
    assert_eq!(report["version"], "2.1.0");
    assert!(!report["runs"][0]["results"].as_array().unwrap().is_empty());
    fs::remove_dir_all(dir).expect("failed to remove test directory");
}

#[test]
fn markdown_and_html_reports_can_be_written_to_files() {
    let dir = temporary_directory();
    let example = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../example");

    for (format, filename, expected) in [
        ("markdown", "report.md", "# MoveShield report: example"),
        ("html", "report.html", "<!doctype html>"),
    ] {
        let report_path = dir.join(filename);
        let output = run_cli(&[
            "analyze",
            example.to_str().unwrap(),
            "--format",
            format,
            "--output",
            report_path.to_str().unwrap(),
        ]);
        assert_eq!(output.status.code(), Some(2));
        let contents = fs::read_to_string(report_path).unwrap();
        assert!(contents.contains(expected));
    }

    fs::remove_dir_all(dir).expect("failed to remove test directory");
}
