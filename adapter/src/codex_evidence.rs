use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Default)]
pub struct ParsedCodex {
    pub command: Option<String>,
    pub aggregated_output: String,
    pub command_exit_code: Option<i32>,
    pub command_status: Option<String>,
    pub command_events: usize,
    pub turn_completed: bool,
    pub usage: Option<Value>,
    pub messages: Vec<String>,
    pub json_lines: usize,
    pub parse_errors: usize,
}

pub fn parse_codex_output(stdout: &[u8]) -> ParsedCodex {
    let mut parsed = ParsedCodex::default();
    for line in String::from_utf8_lossy(stdout).lines() {
        if line.trim().is_empty() {
            continue;
        }
        let value: Value = match serde_json::from_str(line) {
            Ok(value) => {
                parsed.json_lines += 1;
                value
            }
            Err(_) => {
                parsed.parse_errors += 1;
                continue;
            }
        };
        match value["type"].as_str() {
            Some("turn.completed") => {
                parsed.turn_completed = true;
                parsed.usage = value.get("usage").filter(|v| !v.is_null()).cloned();
            }
            Some("turn.failed") => {
                parsed.turn_completed = false;
                if let Some(message) = value["error"]["message"].as_str() {
                    parsed.messages.push(message.to_string());
                }
            }
            Some("error") => {
                if let Some(message) = value["message"].as_str() {
                    parsed.messages.push(message.to_string());
                }
            }
            Some("item.completed") => {
                let item = &value["item"];
                match item["type"].as_str() {
                    Some("error") => {
                        if let Some(message) = item["message"].as_str() {
                            parsed.messages.push(message.to_string());
                        }
                    }
                    Some("command_execution" | "commandExecution") => {
                        parsed.command_events += 1;
                        parsed.command = item["command"].as_str().map(str::to_string);
                        parsed.aggregated_output = item
                            .get("aggregated_output")
                            .or_else(|| item.get("aggregatedOutput"))
                            .and_then(Value::as_str)
                            .unwrap_or("")
                            .to_string();
                        parsed.command_exit_code = item
                            .get("exit_code")
                            .or_else(|| item.get("exitCode"))
                            .and_then(Value::as_i64)
                            .and_then(|code| i32::try_from(code).ok());
                        parsed.command_status = item["status"].as_str().map(str::to_string);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    parsed
}

fn text_has_any(text: &str, needles: &[&str]) -> bool {
    let lower = text.to_ascii_lowercase();
    needles.iter().any(|needle| lower.contains(needle))
}

pub fn http_error_status(parsed: &ParsedCodex, stderr: &str) -> Option<u16> {
    parsed
        .messages
        .iter()
        .map(String::as_str)
        .chain([stderr])
        .find_map(|message| {
            ["unexpected status ", "last status: ", "HTTP "]
                .iter()
                .find_map(|prefix| {
                    let (_, tail) = message.split_once(prefix)?;
                    let status = tail.split_whitespace().next()?.parse::<u16>().ok()?;
                    (400..=599).contains(&status).then_some(status)
                })
        })
}

pub fn classify_external_failure(parsed: &ParsedCodex, stderr: &str) -> (String, String) {
    let combined = format!("{} {stderr}", parsed.messages.join(" "));
    // URLs and request IDs can contain status-like numbers; use explicit HTTP evidence.
    match http_error_status(parsed, stderr) {
        Some(401) => return ("provider_error".into(), "provider_authentication".into()),
        Some(429 | 500..=599) => return ("relay_error".into(), "relay_or_transport".into()),
        Some(_) => return ("provider_error".into(), "provider_response".into()),
        None => {}
    }
    if text_has_any(
        &combined,
        &["api_key_required", "invalid api key", "unauthorized"],
    ) {
        ("provider_error".into(), "provider_authentication".into())
    } else if text_has_any(&combined, &["request timed out", "request timeout"]) {
        ("relay_error".into(), "relay_request_timeout".into())
    } else if text_has_any(
        &combined,
        &[
            "disconnect",
            "connection reset",
            "connection refused",
            "upstream",
            "gateway",
        ],
    ) {
        ("relay_error".into(), "relay_or_transport".into())
    } else if text_has_any(&combined, &["quota", "model not found"]) {
        ("provider_error".into(), "provider_response".into())
    } else if text_has_any(
        &combined,
        &["code mode is unavailable", "host executable was not found"],
    ) {
        ("harness_error".into(), "codex_tool_configuration".into())
    } else if parsed.parse_errors > 0 {
        ("harness_error".into(), "invalid_codex_jsonl".into())
    } else {
        (
            "unknown".into(),
            if parsed.command_events == 0 {
                "missing_command_event"
            } else {
                "codex_incomplete"
            }
            .into(),
        )
    }
}

#[derive(Clone, Copy)]
pub struct ProcessEvidence {
    pub exit_code: Option<i32>,
    pub timed_out: bool,
    pub spawn_failed: bool,
}

pub struct CommandOracle<'a> {
    pub expected_exit: Option<i32>,
    pub expected_marker: Option<&'a str>,
    pub forbidden_path_exists: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct Assessment {
    pub status: String,
    pub failure_class: Option<String>,
    pub command_outcome: String,
    pub command_failure_class: Option<String>,
}

pub fn assess(
    parsed: &ParsedCodex,
    stderr: &str,
    process: ProcessEvidence,
    oracle: CommandOracle<'_>,
) -> Assessment {
    let (command_outcome, command_failure_class) = if parsed.command_events > 1 {
        ("harness_error", Some("unexpected_command_count"))
    } else if parsed.command_exit_code.is_none() {
        ("unknown", Some("missing_command_event"))
    } else if oracle.expected_exit.is_none()
        || oracle.expected_marker.is_none()
        || oracle.forbidden_path_exists.is_none()
    {
        ("unknown", Some("missing_command_oracle"))
    } else if parsed.command_exit_code != oracle.expected_exit {
        ("backend_failure", Some("command_exit_mismatch"))
    } else if oracle.forbidden_path_exists == Some(true) {
        ("backend_failure", Some("unexpected_side_effect"))
    } else if !parsed
        .aggregated_output
        .contains(oracle.expected_marker.unwrap())
    {
        ("backend_failure", Some("command_output_mismatch"))
    } else {
        ("success", None)
    };
    let (status, failure_class) = if process.spawn_failed {
        ("harness_error".into(), Some("codex_spawn".into()))
    } else if process.timed_out {
        ("timeout".into(), Some("codex_timeout".into()))
    } else if process.exit_code != Some(0)
        || !parsed.turn_completed
        || (parsed.command_events <= 1 && parsed.command_exit_code.is_none())
    {
        let (status, class) = classify_external_failure(parsed, stderr);
        (status, Some(class))
    } else if parsed.parse_errors > 0 {
        ("harness_error".into(), Some("invalid_codex_jsonl".into()))
    } else {
        (
            command_outcome.into(),
            command_failure_class.map(str::to_string),
        )
    };
    Assessment {
        status,
        failure_class,
        command_outcome: command_outcome.into(),
        command_failure_class: command_failure_class.map(str::to_string),
    }
}
