use codex_mbtx_contract::codex_evidence::{
    CommandOracle, ProcessEvidence, assess, parse_codex_output,
};
use serde_json::json;

fn assessment(
    exit: i32,
    codex_exit: Option<i32>,
    terminal: &str,
    timed_out: bool,
) -> serde_json::Value {
    let command = json!({"type":"item.completed","item":{"type":"command_execution","command":"fixture","exit_code":exit,"aggregated_output":"expected marker"}});
    let parsed = parse_codex_output(format!("{command}\n{terminal}\n").as_bytes());
    serde_json::to_value(assess(
        &parsed,
        "",
        ProcessEvidence {
            exit_code: codex_exit,
            timed_out,
            spawn_failed: false,
        },
        CommandOracle {
            expected_exit: Some(exit),
            expected_marker: Some("expected marker"),
            forbidden_path_exists: Some(false),
        },
    ))
    .unwrap()
}

#[test]
fn expected_nonzero_and_signal_exits_are_command_successes() {
    for exit in [0, 7, 143, 137] {
        let result = assessment(exit, Some(0), r#"{"type":"turn.completed"}"#, false);
        assert_eq!(result["status"], "success");
        assert_eq!(result["command_outcome"], "success");
    }
}

#[test]
fn request_timeout_after_command_is_external_and_preserves_oracle_result() {
    for exit in [0, 7, 143] {
        let result = assessment(
            exit,
            Some(1),
            r#"{"type":"turn.failed","error":{"message":"request timed out"}}"#,
            false,
        );
        assert_eq!(result["status"], "relay_error");
        assert_eq!(result["failure_class"], "relay_request_timeout");
        assert_eq!(result["command_outcome"], "success");
    }
}

#[test]
fn missing_terminal_evidence_and_collector_deadline_cannot_be_success() {
    assert_eq!(assessment(0, Some(0), "", false)["status"], "unknown");
    assert_eq!(assessment(0, None, "", false)["status"], "unknown");
    let result = assessment(0, None, "", true);
    assert_eq!(result["status"], "timeout");
    assert_eq!(result["failure_class"], "codex_timeout");
    assert_eq!(result["command_outcome"], "success");
    assert_eq!(parse_codex_output(b"").usage, None);
}

#[test]
fn multiple_commands_do_not_pass_by_selecting_the_last_success() {
    let command = json!({"type":"item.completed","item":{"type":"command_execution","exit_code":0,"aggregated_output":"ok"}});
    let parsed = parse_codex_output(
        format!("{command}\n{command}\n{{\"type\":\"turn.completed\"}}").as_bytes(),
    );
    let result = assess(
        &parsed,
        "",
        ProcessEvidence {
            exit_code: Some(0),
            timed_out: false,
            spawn_failed: false,
        },
        CommandOracle {
            expected_exit: Some(0),
            expected_marker: Some("ok"),
            forbidden_path_exists: Some(false),
        },
    );
    assert_eq!(result.status, "harness_error");
    assert_eq!(
        result.failure_class.as_deref(),
        Some("unexpected_command_count")
    );
}

#[test]
fn completed_turn_without_a_working_tool_preserves_configuration_failure() {
    let parsed = parse_codex_output(br#"{"type":"item.completed","item":{"type":"error","message":"Code Mode is unavailable: host executable was not found"}}
{"type":"turn.completed"}"#);
    let result = assess(
        &parsed,
        "",
        ProcessEvidence {
            exit_code: Some(0),
            timed_out: false,
            spawn_failed: false,
        },
        CommandOracle {
            expected_exit: Some(0),
            expected_marker: Some("ok"),
            forbidden_path_exists: Some(false),
        },
    );
    assert_eq!(result.status, "harness_error");
    assert_eq!(
        result.failure_class.as_deref(),
        Some("codex_tool_configuration")
    );
    assert_eq!(result.command_outcome, "unknown");
}
