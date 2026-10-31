use serde_json::Value;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn analyze_uses_official_parser_and_emits_machine_readable_report() {
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
    assert!(String::from_utf8_lossy(&output.stderr).contains("does not type-check"));
    assert_eq!(
        report["findings"][0]["location"]["file"],
        "sources/complex.move"
    );
}
