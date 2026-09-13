use serde_json::{Value, json};
use std::fs;
use std::path::Path;
use std::process::Command;

#[test]
fn linux_smoke_rebuild_counts_arms_once_and_reclassifies_raw_failures() {
    let run = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../evidence/linux/codex-relay/20260913T141847943929204");
    let original_events = fs::read(run.join("events.jsonl")).unwrap();
    let original_summary = fs::read(run.join("summary.json")).unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_mbtx-eval"))
        .arg("report")
        .arg(&run)
        .args(["--format", "json"])
        .env_remove("OPENAI_API_KEY")
        .env_remove("CODEX_HOME")
        .output()
        .unwrap();
    assert!(result.status.success());
    let report: Value = serde_json::from_slice(&result.stdout).unwrap();
    let summary = &report["summary"];
    assert_eq!(summary["planned_arms"], 16);
    assert_eq!(summary["observed_arms"], 16);
    assert_eq!(summary["intention_to_treat"]["successful_arms"], 9);
    assert_eq!(
        summary["status_counts"],
        json!({"success":9,"relay_error":6,"timeout":1})
    );
    assert_eq!(summary["command_oracle_successful_arms"], 13);
    assert_eq!(summary["command_oracle_pairs"], 5);
    assert_eq!(summary["valid_comparable_pairs"], 1);
    assert_eq!(summary["command_oracle_failures"], 0);
    assert_eq!(fs::read(run.join("events.jsonl")).unwrap(), original_events);
    assert_eq!(
        fs::read(run.join("summary.json")).unwrap(),
        original_summary
    );
    let reassessed = report["events"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| {
            row["attempt_id"] == "attempt-00-literal_argv-shell"
                && row["phase"] == "codex"
                && row["event"] == "exit"
        })
        .unwrap();
    assert_eq!(reassessed["recorded_status"], "success");
    assert_eq!(reassessed["status"], "relay_error");
    assert_eq!(reassessed["command_outcome"], "success");
    assert_eq!(reassessed["usage"], Value::Null);
}
