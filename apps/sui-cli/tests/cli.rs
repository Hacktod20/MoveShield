use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn analyze_type_checks_with_official_compiler_and_emits_machine_readable_report() {
    let example = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../example");
    let output = Command::new(env!("CARGO_BIN_EXE_moveshield-sui"))
        .args(["analyze", example.to_str().unwrap(), "--format", "json"])
        .output()
        .expect("failed to run MoveShield Sui CLI");

    assert_eq!(output.status.code(), Some(2));
    let report: Value =
        serde_json::from_slice(&output.stdout).expect("analysis output should be valid JSON");
    assert_eq!(report["project"], "example");
    assert_eq!(report["summary"]["total_modules"], 2);
    assert!(String::from_utf8_lossy(&output.stderr).contains("do not guarantee security"));
    assert_eq!(
        report["findings"][0]["location"]["file"],
        "sources/complex.move"
    );
}

#[test]
fn analyze_does_not_mutate_manifest_or_create_move_lock() {
    let package =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/no_workspace_mutation");
    let manifest_path = package.join("Move.toml");
    let lock_path = package.join("Move.lock");
    let manifest_before = fs::read(&manifest_path).expect("failed to read fixture manifest");
    assert!(
        !lock_path.exists(),
        "fixture unexpectedly contains Move.lock"
    );

    let output = Command::new(env!("CARGO_BIN_EXE_moveshield-sui"))
        .args(["analyze", package.to_str().unwrap(), "--format", "json"])
        .output()
        .expect("failed to run MoveShield Sui CLI");

    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        !output.stdout.is_empty(),
        "successful JSON report was empty"
    );
    assert_eq!(
        fs::read(&manifest_path).expect("failed to reread fixture manifest"),
        manifest_before
    );
    assert!(!lock_path.exists(), "analysis created Move.lock");
}

#[test]
fn unsupported_module_members_fail_with_exact_locations_and_no_json() {
    let fixtures = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let cases = [
        ("unsupported_constant", "constant", 2, 5),
        ("unsupported_enum", "enum", 2, 5),
        ("unsupported_friend", "friend declaration", 4, 5),
        ("unsupported_spec", "specification", 2, 5),
    ];

    for (fixture, kind, line, column) in cases {
        let package = fixtures.join(fixture);
        let output = Command::new(env!("CARGO_BIN_EXE_moveshield-sui"))
            .args(["analyze", package.to_str().unwrap(), "--format", "json"])
            .output()
            .expect("failed to run MoveShield Sui CLI");

        assert_eq!(output.status.code(), Some(1), "{fixture}");
        assert!(output.stdout.is_empty(), "{fixture} emitted JSON");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains(&format!("module-level {kind}")),
            "{fixture}: {stderr}"
        );
        assert!(
            stderr.contains(&format!("sources/{fixture}.move:{line}:{column}")),
            "{fixture}: {stderr}"
        );
        assert!(
            stderr.contains("no report was produced"),
            "{fixture}: {stderr}"
        );
    }
}

#[test]
fn failed_analysis_does_not_write_reports_in_any_format() {
    let package =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/unsupported_constant");
    let output_directory = unique_temp_directory("moveshield-sui-no-report-test");
    fs::create_dir_all(&output_directory).expect("failed to create report output directory");

    let cases = [
        ("terminal", None),
        ("json", Some("report.json")),
        ("sarif", Some("report.sarif")),
        ("markdown", Some("report.md")),
        ("html", Some("report.html")),
        ("all", Some("all.sarif")),
    ];
    for (format, report_name) in cases {
        let mut command = Command::new(env!("CARGO_BIN_EXE_moveshield-sui"));
        command
            .args(["analyze", package.to_str().unwrap(), "--format", format])
            .current_dir(&output_directory);
        let report_path = report_name.map(|name| output_directory.join(name));
        if let Some(path) = &report_path {
            command.args(["--output", path.to_str().unwrap()]);
        }
        let output = command.output().expect("failed to run MoveShield Sui CLI");

        assert_eq!(output.status.code(), Some(1), "{format}");
        assert!(output.stdout.is_empty(), "{format} emitted report content");
        if let Some(path) = report_path {
            assert!(!path.exists(), "{format} report was created");
        }
    }
    fs::remove_dir_all(output_directory).expect("failed to remove report output directory");
}

#[test]
fn reports_only_the_first_unsupported_member_in_source_order() {
    let package =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/multiple_unsupported");
    let output = Command::new(env!("CARGO_BIN_EXE_moveshield-sui"))
        .args(["analyze", package.to_str().unwrap(), "--format", "json"])
        .output()
        .expect("failed to run MoveShield Sui CLI");

    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("module-level constant"), "{stderr}");
    assert!(stderr.contains("sources/multiple.move:2:5"), "{stderr}");
    assert!(!stderr.contains("specification"), "{stderr}");
}

fn unique_temp_directory(prefix: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock is before UNIX epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("{prefix}-{}-{timestamp}", std::process::id()))
}
