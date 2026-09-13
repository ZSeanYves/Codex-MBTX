use codex_mbtx_contract::codex_evidence::{
    CommandOracle, ProcessEvidence, assess, http_error_status, parse_codex_output,
};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path};
use std::{env, fs, process::Command};

#[derive(serde::Serialize)]
struct Job {
    experiment_id: String,
    pair_id: String,
    attempt_id: String,
    task_id: String,
    backend: String,
    command: Vec<String>,
    cwd: Option<String>,
    artifact_dir: String,
    expected_exit_code: Option<i32>,
}

#[derive(Default)]
struct Arm {
    status: String,
    failure_class: String,
    elapsed_ms: Option<Value>,
    command_exit_code: Option<Value>,
    command_outcome: Option<Value>,
    command_failure_class: Option<Value>,
    turn_completed: Option<Value>,
    artifact_dir: String,
}

#[derive(Default)]
struct Pair {
    task_id: String,
    shell: Option<Arm>,
    transparent: Option<Arm>,
}

fn usage() -> ! {
    eprintln!(
        "usage: mbtx-eval run|replay|log|report RUN|--events FILE [--format html|md|json|csv]"
    );
    std::process::exit(2)
}

fn event_path(args: &[String]) -> String {
    if let Some(index) = args.iter().position(|arg| arg == "--events") {
        return args.get(index + 1).cloned().unwrap_or_else(|| usage());
    }
    args.get(2)
        .map(|path| {
            let candidate = std::path::Path::new(path);
            if candidate.is_dir() {
                candidate.join("events.jsonl").display().to_string()
            } else {
                path.clone()
            }
        })
        .unwrap_or_else(|| usage())
}

fn events(path: &str) -> Vec<Value> {
    fs::read_to_string(path)
        .unwrap_or_else(|error| {
            eprintln!("cannot read {path}: {error}");
            std::process::exit(2)
        })
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            serde_json::from_str(line).unwrap_or_else(|error| {
                eprintln!("invalid event: {error}");
                std::process::exit(2)
            })
        })
        .collect()
}

fn field<'a>(value: &'a Value, name: &str) -> &'a str {
    value.get(name).and_then(Value::as_str).unwrap_or("unknown")
}

fn text_field(value: &Value, name: &str) -> String {
    match value.get(name) {
        Some(Value::String(text)) => text.clone(),
        Some(Value::Null) | None => "unknown".to_string(),
        Some(value) => value.to_string(),
    }
}

fn esc(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn csv_field(value: String) -> String {
    format!("\"{}\"", value.replace('"', "\"\""))
}

fn exit_rows<'a>(rows: &'a [Value]) -> impl Iterator<Item = &'a Value> {
    let codex_attempts: BTreeSet<&str> = rows
        .iter()
        .filter(|row| field(row, "phase") == "codex" && field(row, "event") == "exit")
        .map(|row| field(row, "attempt_id"))
        .collect();
    rows.iter().filter(move |row| {
        field(row, "event") == "exit"
            && (field(row, "phase") == "codex"
                || (field(row, "phase") == "child"
                    && !codex_attempts.contains(field(row, "attempt_id"))))
    })
}

fn reassess_online_events(rows: &mut [Value], run: &Path) {
    let mut assessments = BTreeMap::new();
    for row in rows
        .iter_mut()
        .filter(|row| field(row, "phase") == "codex" && field(row, "event") == "exit")
    {
        if field(row, "experiment_id") != "codex-relay-online-v1" {
            continue;
        }
        row["recorded_status"] = row["status"].clone();
        row["recorded_failure_class"] = row["failure_class"].clone();
        let relative = Path::new(field(row, "artifact_dir"));
        let evidence = if relative
            .components()
            .all(|part| matches!(part, Component::Normal(_)))
        {
            let dir = run.join(relative);
            (|| -> Option<_> {
                let metadata: Value =
                    serde_json::from_slice(&fs::read(dir.join("metadata.json")).ok()?).ok()?;
                Some((
                    metadata,
                    fs::read(dir.join("codex.stdout.jsonl")).ok()?,
                    fs::read_to_string(dir.join("codex.stderr.log")).ok()?,
                ))
            })()
        } else {
            None
        };
        let Some((metadata, stdout, stderr)) = evidence else {
            row["status"] = json!("harness_error");
            row["failure_class"] = json!("missing_raw_codex_evidence");
            row["command_outcome"] = json!("unknown");
            row["command_failure_class"] = json!("missing_raw_codex_evidence");
            row["classification_source"] = json!("raw_evidence_unavailable");
            continue;
        };
        let parsed = parse_codex_output(&stdout);
        let assessment = assess(
            &parsed,
            &stderr,
            ProcessEvidence {
                exit_code: metadata["codex_exit_code"]
                    .as_i64()
                    .and_then(|code| i32::try_from(code).ok()),
                timed_out: metadata["timed_out"]
                    .as_bool()
                    .unwrap_or(metadata["failure_class"] == "codex_timeout"),
                spawn_failed: metadata["spawn_error"].is_string()
                    || metadata["failure_class"] == "codex_spawn",
            },
            CommandOracle {
                expected_exit: metadata["expected_command_exit"]
                    .as_i64()
                    .and_then(|code| i32::try_from(code).ok()),
                expected_marker: metadata["expected_marker"].as_str(),
                forbidden_path_exists: metadata["forbidden_path_exists"].as_bool(),
            },
        );
        row["status"] = json!(assessment.status);
        row["failure_class"] = json!(assessment.failure_class);
        row["command_outcome"] = json!(assessment.command_outcome);
        row["command_failure_class"] = json!(assessment.command_failure_class);
        row["command_events"] = json!(parsed.command_events);
        row["command_exit_code"] = json!(parsed.command_exit_code);
        row["http_status"] = json!(http_error_status(&parsed, &stderr));
        row["turn_completed"] = json!(parsed.turn_completed);
        row["usage"] = json!(parsed.usage);
        row["error_messages"] = json!(parsed.messages);
        row["classification_source"] = json!("raw_codex_evidence_v2");
        assessments.insert(field(row, "attempt_id").to_string(), assessment);
    }
    for row in rows
        .iter_mut()
        .filter(|row| field(row, "phase") == "child" && field(row, "event") == "exit")
    {
        if let Some(assessment) = assessments.get(field(row, "attempt_id")) {
            row["recorded_status"] = row["status"].clone();
            row["status"] = json!(assessment.command_outcome);
            row["failure_class"] = json!(assessment.command_failure_class);
            row["classification_source"] = json!("raw_codex_evidence_v2");
        }
    }
}

fn as_optional_value(row: &Value, name: &str) -> Option<Value> {
    row.get(name).filter(|value| !value.is_null()).cloned()
}

fn pair_map(rows: &[Value]) -> BTreeMap<String, Pair> {
    let mut pairs = BTreeMap::new();
    for row in exit_rows(rows) {
        let pair_id = field(row, "pair_id").to_string();
        let backend = field(row, "backend");
        if !matches!(backend, "shell" | "transparent") {
            continue;
        }
        let pair = pairs.entry(pair_id).or_insert_with(|| Pair {
            task_id: field(row, "task_id").to_string(),
            ..Pair::default()
        });
        let arm = Arm {
            status: field(row, "status").to_string(),
            failure_class: text_field(row, "failure_class"),
            elapsed_ms: as_optional_value(row, "elapsed_ms"),
            command_exit_code: as_optional_value(row, "command_exit_code"),
            command_outcome: as_optional_value(row, "command_outcome"),
            command_failure_class: as_optional_value(row, "command_failure_class"),
            turn_completed: as_optional_value(row, "turn_completed"),
            artifact_dir: text_field(row, "artifact_dir"),
        };
        if backend == "shell" {
            pair.shell = Some(arm);
        } else {
            pair.transparent = Some(arm);
        }
    }
    pairs
}

fn arm_json(arm: Option<&Arm>) -> Value {
    let Some(arm) = arm else {
        return Value::Null;
    };
    json!({
        "status": arm.status,
        "failure_class": arm.failure_class,
        "elapsed_ms": arm.elapsed_ms,
        "command_exit_code": arm.command_exit_code,
        "command_outcome": arm.command_outcome,
        "command_failure_class": arm.command_failure_class,
        "turn_completed": arm.turn_completed,
        "artifact_dir": arm.artifact_dir,
    })
}

fn pair_json(pair_id: &str, pair: &Pair) -> Value {
    let shell = pair.shell.as_ref();
    let transparent = pair.transparent.as_ref();
    let complete = shell.is_some_and(|arm| arm.status == "success")
        && transparent.is_some_and(|arm| arm.status == "success");
    let observed_difference = match (shell, transparent) {
        (Some(shell), Some(transparent))
            if shell.status != "success" || transparent.status != "success" =>
        {
            "insufficient_evidence"
        }
        (Some(shell), Some(transparent)) => match (
            shell.elapsed_ms.as_ref().and_then(Value::as_u64),
            transparent.elapsed_ms.as_ref().and_then(Value::as_u64),
        ) {
            (Some(shell_ms), Some(transparent_ms)) if shell_ms < transparent_ms => {
                "shell_faster_observed"
            }
            (Some(shell_ms), Some(transparent_ms)) if transparent_ms < shell_ms => {
                "transparent_faster_observed"
            }
            (Some(_), Some(_)) => "no_observed_difference",
            _ => "unknown",
        },
        _ => "insufficient_evidence",
    };
    let first_difference = match (shell, transparent) {
        (Some(shell), Some(transparent)) if shell.status != transparent.status => "status",
        (Some(shell), Some(transparent))
            if shell.command_exit_code != transparent.command_exit_code =>
        {
            "command_exit_code"
        }
        (Some(shell), Some(transparent)) if shell.elapsed_ms != transparent.elapsed_ms => {
            "elapsed_ms"
        }
        (Some(_), Some(_)) => "none_observed",
        _ => "missing_arm",
    };
    json!({
        "pair_id": pair_id,
        "task_id": pair.task_id,
        "shell": arm_json(shell),
        "transparent": arm_json(transparent),
        "complete_pair": complete,
        "valid_comparable": complete,
        "command_oracle_pair": shell.is_some_and(|arm| arm.command_outcome.as_ref().and_then(Value::as_str) == Some("success"))
            && transparent.is_some_and(|arm| arm.command_outcome.as_ref().and_then(Value::as_str) == Some("success")),
        "observed_difference": observed_difference,
        "first_difference": first_difference,
    })
}

fn report_summary(rows: &[Value], planned_pairs: Option<usize>) -> (Value, Vec<Value>) {
    let pairs = pair_map(rows);
    let pair_rows: Vec<Value> = pairs
        .iter()
        .map(|(pair_id, pair)| pair_json(pair_id, pair))
        .collect();
    let exits: Vec<&Value> = exit_rows(rows).collect();
    let mut status_counts = BTreeMap::new();
    for row in &exits {
        *status_counts.entry(field(row, "status")).or_insert(0_usize) += 1;
    }
    let observed_arms = exits.len();
    let planned_pairs = planned_pairs.unwrap_or(pairs.len());
    let planned_arms = planned_pairs * 2;
    let successful_arms = *status_counts.get("success").unwrap_or(&0);
    let complete_pairs = pair_rows
        .iter()
        .filter(|pair| pair.get("complete_pair").and_then(Value::as_bool) == Some(true))
        .count();
    let external_failures = exits
        .iter()
        .filter(|row| matches!(field(row, "status"), "relay_error" | "provider_error"))
        .count();
    let backend_failures = exits
        .iter()
        .filter(|row| field(row, "status") == "backend_failure")
        .count();
    let summary = json!({
        "planned_pairs": planned_pairs,
        "planned_arms": planned_arms,
        "observed_pairs": pairs.len(),
        "observed_arms": observed_arms,
        "not_observed_arms": planned_arms.saturating_sub(observed_arms),
        "intention_to_treat": {
            "planned_arms": planned_arms,
            "observed_arms": observed_arms,
            "successful_arms": successful_arms,
            "failed_or_unknown_arms": observed_arms.saturating_sub(successful_arms),
        },
        "complete_pairs": complete_pairs,
        "valid_comparable_pairs": complete_pairs,
        "command_oracle_successful_arms": exits.iter().filter(|row| field(row, "command_outcome") == "success").count(),
        "command_oracle_pairs": pair_rows.iter().filter(|pair| pair["command_oracle_pair"] == true).count(),
        "command_oracle_failures": exits.iter().filter(|row| field(row, "command_outcome") == "backend_failure").count(),
        "external_failures": external_failures,
        "backend_failures": backend_failures,
        "status_counts": status_counts,
        "partial": complete_pairs < planned_pairs,
    });
    (summary, pair_rows)
}

fn report(path: &str, format: &str) {
    let mut rows = events(path);
    reassess_online_events(
        &mut rows,
        Path::new(path).parent().unwrap_or(Path::new(".")),
    );
    let planned_pairs = path
        .strip_suffix("/events.jsonl")
        .and_then(|run| fs::read_to_string(format!("{run}/summary.json")).ok())
        .and_then(|text| serde_json::from_str::<Value>(&text).ok())
        .and_then(|summary| summary.get("planned_pairs").and_then(Value::as_u64))
        .map(|value| value as usize);
    let (summary, pairs) = report_summary(&rows, planned_pairs);
    match format {
        "json" => println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "schema_version": 1,
                "summary": summary,
                "pairs": pairs,
                "events": rows,
            }))
            .unwrap()
        ),
        "csv" => {
            println!(
                "pair_id,attempt_id,task_id,backend,phase,event,status,failure_class,monotonic_ns,elapsed_ms,command,artifact_dir,command_outcome,command_failure_class,turn_completed,recorded_status"
            );
            for row in &rows {
                println!(
                    "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                    csv_field(field(row, "pair_id").to_string()),
                    csv_field(field(row, "attempt_id").to_string()),
                    csv_field(field(row, "task_id").to_string()),
                    csv_field(field(row, "backend").to_string()),
                    csv_field(field(row, "phase").to_string()),
                    csv_field(field(row, "event").to_string()),
                    csv_field(field(row, "status").to_string()),
                    csv_field(text_field(row, "failure_class")),
                    csv_field(text_field(row, "monotonic_ns")),
                    csv_field(text_field(row, "elapsed_ms")),
                    csv_field(text_field(row, "command")),
                    csv_field(text_field(row, "artifact_dir")),
                    csv_field(text_field(row, "command_outcome")),
                    csv_field(text_field(row, "command_failure_class")),
                    csv_field(text_field(row, "turn_completed")),
                    csv_field(text_field(row, "recorded_status"))
                );
            }
        }
        "md" => {
            println!("# MBTX execution report\n");
            println!("## Evidence summary\n");
            println!(
                "```json\n{}\n```\n",
                serde_json::to_string_pretty(&summary).unwrap()
            );
            println!("## Pair comparison\n");
            println!(
                "| pair | task | shell | transparent | complete Codex pair | command oracle pair | observed timing | first difference |\n|---|---|---|---|---|---|---|---|"
            );
            for pair in &pairs {
                println!(
                    "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |",
                    field(pair, "pair_id"),
                    field(pair, "task_id"),
                    pair.get("shell")
                        .and_then(|arm| arm.get("status"))
                        .and_then(Value::as_str)
                        .unwrap_or("unknown"),
                    pair.get("transparent")
                        .and_then(|arm| arm.get("status"))
                        .and_then(Value::as_str)
                        .unwrap_or("unknown"),
                    pair.get("valid_comparable")
                        .and_then(Value::as_bool)
                        .unwrap_or(false),
                    text_field(pair, "command_oracle_pair"),
                    field(pair, "observed_difference"),
                    field(pair, "first_difference")
                );
            }
            println!("\n## Event evidence\n");
            println!(
                "| pair | task | backend | phase | event | status | failure | elapsed ms | command | evidence |\n|---|---|---|---|---|---|---|---:|---|---|"
            );
            for row in &rows {
                println!(
                    "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |",
                    field(row, "pair_id"),
                    field(row, "task_id"),
                    field(row, "backend"),
                    field(row, "phase"),
                    field(row, "event"),
                    field(row, "status"),
                    text_field(row, "failure_class"),
                    text_field(row, "elapsed_ms"),
                    text_field(row, "command").replace('|', " "),
                    text_field(row, "artifact_dir")
                );
            }
        }
        "html" => {
            println!(
                "<!doctype html><meta charset=utf-8><title>MBTX execution report</title><style>body{{font:14px system-ui;margin:2rem}}table{{border-collapse:collapse;width:100%;margin:1rem 0}}td,th{{border:1px solid #ccc;padding:.35rem;text-align:left;vertical-align:top}}code{{white-space:pre-wrap;word-break:break-word}}pre{{background:#f5f5f5;padding:1rem;overflow:auto}}</style><h1>MBTX execution report</h1><h2>Evidence summary</h2><pre>{}</pre><h2>Pair comparison</h2><table><tr><th>pair</th><th>task</th><th>shell</th><th>transparent</th><th>complete Codex pair</th><th>command oracle pair</th><th>observed timing</th><th>first difference</th></tr>",
                esc(&serde_json::to_string_pretty(&summary).unwrap())
            );
            for pair in &pairs {
                println!(
                    "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                    esc(field(pair, "pair_id")),
                    esc(field(pair, "task_id")),
                    esc(pair
                        .get("shell")
                        .and_then(|arm| arm.get("status"))
                        .and_then(Value::as_str)
                        .unwrap_or("unknown")),
                    esc(pair
                        .get("transparent")
                        .and_then(|arm| arm.get("status"))
                        .and_then(Value::as_str)
                        .unwrap_or("unknown")),
                    esc(&pair
                        .get("valid_comparable")
                        .and_then(Value::as_bool)
                        .unwrap_or(false)
                        .to_string()),
                    esc(&text_field(pair, "command_oracle_pair")),
                    esc(field(pair, "observed_difference")),
                    esc(field(pair, "first_difference"))
                );
            }
            println!(
                "</table><h2>Event evidence</h2><table><tr><th>pair</th><th>task</th><th>backend</th><th>phase</th><th>event</th><th>status</th><th>failure</th><th>elapsed ms</th><th>command</th><th>evidence</th></tr>"
            );
            for row in &rows {
                println!(
                    "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td><code>{}</code></td><td><code>{}</code></td></tr>",
                    esc(field(row, "pair_id")),
                    esc(field(row, "task_id")),
                    esc(field(row, "backend")),
                    esc(field(row, "phase")),
                    esc(field(row, "event")),
                    esc(field(row, "status")),
                    esc(&text_field(row, "failure_class")),
                    esc(&text_field(row, "elapsed_ms")),
                    esc(&text_field(row, "command")),
                    esc(&text_field(row, "artifact_dir"))
                );
            }
            println!("</table>");
        }
        _ => usage(),
    }
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(String::as_str).unwrap_or("");
    match command {
        "run" => {
            let mbtx = args
                .iter()
                .position(|arg| arg == "--mbtx")
                .and_then(|index| args.get(index + 1))
                .cloned()
                .unwrap_or_else(|| usage());
            let out = args
                .iter()
                .position(|arg| arg == "--output")
                .and_then(|index| args.get(index + 1))
                .cloned()
                .unwrap_or_else(|| "_build/launcher-evidence".to_owned());
            let pairs: usize = args
                .iter()
                .position(|arg| arg == "--pairs")
                .and_then(|index| args.get(index + 1))
                .and_then(|value| value.parse().ok())
                .unwrap_or(4)
                .clamp(1, 6);
            fs::create_dir_all(&out).unwrap();
            let tasks = [
                (
                    "literal_argv",
                    "printf '%s|%s|%s' '$(touch SHOULD_NOT_EXIST)' 'a b' '\"quoted\"'",
                    0,
                ),
                ("stdin_utf8", "printf 'stdin✓\\n'", 0),
                ("cwd_environment", "printf '%s' \"$PWD\"", 0),
                ("large_output", "yes x | head -c 65536", 0),
                ("nonzero_exit", "printf before; exit 7", 7),
                ("background_cleanup", "(sleep .02; exit 0) & wait", 0),
                ("signal_exit", "kill -TERM \"$$\"", 143),
                ("recovery", "printf first; printf second", 0),
            ];
            let mut jobs = Vec::new();
            for pair in 0..pairs {
                for (index, (task, script, expected_exit_code)) in tasks.iter().enumerate() {
                    let first_mbtx = (pair + index) % 2 == 1;
                    for arm in 0..2 {
                        let is_mbtx = if arm == 0 { first_mbtx } else { !first_mbtx };
                        let backend = if is_mbtx { "transparent" } else { "shell" };
                        let command = if is_mbtx {
                            vec![
                                mbtx.clone(),
                                "exec".into(),
                                "--".into(),
                                "/bin/sh".into(),
                                "-c".into(),
                                (*script).into(),
                            ]
                        } else {
                            vec!["/bin/sh".into(), "-c".into(), (*script).into()]
                        };
                        jobs.push(Job {
                            experiment_id: "launcher-comparison-v1".into(),
                            pair_id: format!("pair-{pair:02}-{task}"),
                            attempt_id: format!("attempt-{pair:02}-{index}-{backend}"),
                            task_id: (*task).into(),
                            backend: backend.into(),
                            command,
                            cwd: None,
                            artifact_dir: format!("{out}/pair-{pair:02}-{task}/{backend}"),
                            expected_exit_code: Some(*expected_exit_code),
                        });
                    }
                }
            }
            let jobs_path = format!("{out}/jobs.json");
            fs::write(&jobs_path, serde_json::to_vec_pretty(&jobs).unwrap()).unwrap();
            let observer = env::current_exe().unwrap().with_file_name("mbtx-observe");
            let result = Command::new(observer)
                .arg("--jobs")
                .arg(&jobs_path)
                .output()
                .unwrap();
            fs::write(format!("{out}/events.jsonl"), result.stdout).unwrap();
            eprintln!(
                "planned {} pairs ({} arm attempts); output {out}",
                pairs * tasks.len(),
                jobs.len()
            );
        }
        "replay" | "report" => {
            let path = event_path(&args);
            let format = args
                .iter()
                .position(|arg| arg == "--format")
                .and_then(|index| args.get(index + 1))
                .map(String::as_str)
                .unwrap_or("md");
            report(&path, format);
        }
        "log" => {
            let path = event_path(&args);
            for row in events(&path) {
                println!(
                    "{} {} {} {}",
                    field(&row, "pair_id"),
                    field(&row, "backend"),
                    field(&row, "event"),
                    field(&row, "status")
                );
            }
        }
        _ => usage(),
    }
}

#[cfg(test)]
mod tests {
    use super::{csv_field, esc, report_summary};
    use serde_json::json;

    #[test]
    fn html_escapes_untrusted_event_text() {
        assert_eq!(esc("<script>&\""), "&lt;script&gt;&amp;&quot;");
    }

    #[test]
    fn csv_quotes_commas_and_quotes() {
        assert_eq!(csv_field("a,b\"c".to_string()), "\"a,b\"\"c\"");
    }

    #[test]
    fn summary_separates_external_and_backend_failures() {
        let rows = vec![
            json!({"pair_id":"p","task_id":"t","backend":"shell","phase":"codex","event":"exit","status":"relay_error"}),
            json!({"pair_id":"p","task_id":"t","backend":"transparent","phase":"codex","event":"exit","status":"backend_failure"}),
        ];
        let (summary, pairs) = report_summary(&rows, Some(1));
        assert_eq!(summary["external_failures"], 1);
        assert_eq!(summary["backend_failures"], 1);
        assert_eq!(pairs[0]["valid_comparable"], false);
    }

    #[test]
    fn online_codex_exit_is_not_overwritten_by_child_exit() {
        let rows = vec![
            json!({"pair_id":"p","task_id":"t","attempt_id":"a-shell","backend":"shell","phase":"codex","event":"exit","status":"success","elapsed_ms":120,"command_exit_code":0}),
            json!({"pair_id":"p","task_id":"t","attempt_id":"a-shell","backend":"shell","phase":"child","event":"exit","status":"success","command_exit_code":0}),
            json!({"pair_id":"p","task_id":"t","attempt_id":"a-transparent","backend":"transparent","phase":"codex","event":"exit","status":"success","elapsed_ms":100,"command_exit_code":0}),
        ];
        let (summary, pairs) = report_summary(&rows, Some(1));
        assert_eq!(summary["valid_comparable_pairs"], 1);
        assert_eq!(
            pairs[0]["observed_difference"],
            "transparent_faster_observed"
        );
    }
}
