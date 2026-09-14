use crate::evidence::{digest, lines, read_json};
use crate::model::Model;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::Path;

fn ns(row: &Value) -> Option<u64> {
    (row["clock_domain"] == "os-monotonic")
        .then(|| row["monotonic_ns"].as_u64())
        .flatten()
}
fn event(row: &Value, phase: &str, name: &str) -> bool {
    row["phase"] == phase && row["event"] == name
}

fn span(rows: &[Value], phase: &str, start: &str, end: &str) -> Vec<(u64, u64)> {
    // Shutdown contains nested spans with the same phase and identity (for
    // example unsubscribe -> processor cleanup). Keep a stack per identity so
    // an inner return cannot overwrite the outer begin timestamp.
    let mut waiting: BTreeMap<_, Vec<u64>> = BTreeMap::new();
    let mut spans = Vec::new();
    for row in rows {
        let Some(t) = ns(row) else {
            continue;
        };
        let key = (
            row["pid"].to_string(),
            row["pid_namespace"].to_string(),
            row["process_instance"].to_string(),
            row["request_id"].to_string(),
            row["stream_id"].to_string(),
            row["call_id"].to_string(),
        );
        if event(row, phase, start) {
            waiting.entry(key.clone()).or_default().push(t);
        }
        if event(row, phase, end)
            && let Some(begin) = waiting.get_mut(&key).and_then(Vec::pop)
            && t >= begin
        {
            spans.push((begin, t));
        }
    }
    spans
}

fn sum(spans: &[(u64, u64)]) -> Option<u64> {
    (!spans.is_empty()).then(|| spans.iter().map(|(a, b)| b - a).sum())
}

fn union(spans: &[(u64, u64)], begin: u64, end: u64) -> u64 {
    let mut spans: Vec<_> = spans
        .iter()
        .map(|&(a, b)| (a.max(begin), b.min(end)))
        .filter(|(a, b)| b >= a)
        .collect();
    spans.sort_unstable();
    let mut total = 0;
    let mut last = begin;
    for (a, b) in spans {
        total += b.saturating_sub(a.max(last));
        last = last.max(b);
    }
    total
}

fn metrics(rows: &[Value], result: &Value) -> Value {
    let external = span(rows, "external", "request_send", "stream_eof");
    let wait = span(rows, "external", "rate_wait_begin", "rate_wait_end");
    let queue = span(rows, "external", "gate_queued", "gate_acquired");
    let policy = span(rows, "policy_sandbox", "begin", "return");
    let spawn = span(rows, "spawn", "begin", "return");
    let approval = span(rows, "approval", "begin", "return");
    let sandbox = span(rows, "sandbox_setup_spawn", "begin", "return");
    let tools = span(rows, "tool", "arguments", "result_ready");
    let first_byte = span(rows, "external", "request_send", "first_byte");
    let launcher_spawn = span(rows, "launcher", "spawn_begin", "spawn_return");
    let launcher_prepare = span(rows, "launcher", "entry", "spawn_begin");
    let fixture_runtime = span(rows, "child", "ready", "return");
    let unsubscribe = span(
        rows,
        "codex_shutdown",
        "unsubscribe_begin",
        "unsubscribe_return",
    );
    let processor_cleanup = span(
        rows,
        "codex_shutdown",
        "processor_cleanup_begin",
        "processor_cleanup_return",
    );
    let processor_join = span(
        rows,
        "codex_shutdown",
        "processor_join_begin",
        "processor_join_return",
    );
    let outbound_join = span(
        rows,
        "codex_shutdown",
        "outbound_join_begin",
        "outbound_join_return",
    );
    let analytics_flush = span(
        rows,
        "codex_shutdown",
        "analytics_flush_begin",
        "analytics_flush_return",
    );
    let client_shutdown = span(
        rows,
        "codex_shutdown",
        "client_shutdown_begin",
        "client_shutdown_return",
    );
    let background_drain = span(
        rows,
        "codex_shutdown",
        "background_drain_begin",
        "background_drain_return",
    );
    let threads_shutdown = span(
        rows,
        "codex_shutdown",
        "threads_shutdown_begin",
        "threads_shutdown_return",
    );
    let thread_unsubscribe = span(rows, "codex_shutdown", "begin", "return");
    let turn_completed = rows
        .iter()
        .filter(|row| {
            event(row, "codex_jsonl", "event_received") && row["json_type"] == "turn.completed"
        })
        .filter_map(ns)
        .min();
    let unsubscribe_begin = rows
        .iter()
        .filter(|row| event(row, "codex_shutdown", "unsubscribe_begin"))
        .filter_map(ns)
        .min();
    let mut ready_spans = Vec::new();
    let mut child_wait_spans = Vec::new();
    let mut drain_spans = Vec::new();
    for begin in rows.iter().filter(|row| event(row, "spawn", "begin")) {
        let returned = rows.iter().find(|row| {
            event(row, "spawn", "return")
                && row["pid"] == begin["pid"]
                && row["stream_id"] == begin["stream_id"]
        });
        if returned.is_none() {
            continue;
        }
        let Some(call_id) = begin["call_id"].as_str() else {
            continue;
        };
        let ready = rows
            .iter()
            .filter(|row| event(row, "child", "ready") && row["call_id"] == call_id)
            .filter_map(ns)
            .min();
        if let Some((start, end)) = ns(begin).zip(ready).filter(|(a, b)| b >= a) {
            ready_spans.push((start, end));
            let waited = rows
                .iter()
                .find(|row| {
                    event(row, "child", "wait_return")
                        && row["pid"] == begin["pid"]
                        && row["stream_id"] == begin["stream_id"]
                })
                .and_then(ns);
            if let Some(waited) = waited.filter(|waited| *waited >= end) {
                child_wait_spans.push((end, waited));
                let last_eof = rows
                    .iter()
                    .filter(|row| {
                        event(row, "io", "eof")
                            && row["pid"] == begin["pid"]
                            && row["stream_id"] == begin["stream_id"]
                    })
                    .filter_map(ns)
                    .max();
                if let Some(eof) = last_eof {
                    drain_spans.push((waited, waited.max(eof)));
                }
            }
        }
    }
    let total = result["begin_ns"]
        .as_u64()
        .zip(result["end_ns"].as_u64())
        .filter(|(a, b)| b >= a);
    let mut explained = external.clone();
    explained.extend(wait.clone());
    explained.extend(queue.clone());
    explained.extend(policy.clone());
    explained.extend(tools.clone());
    let artifact: Vec<_> = rows
        .iter()
        .filter(|r| r["phase"] == "artifact")
        .filter_map(|r| r["duration_ns"].as_u64())
        .collect();
    let fds: Vec<_> = rows.iter().filter(|r| r["event"] == "fd_snapshot")
        .map(|r| json!({"call_id":r["call_id"],"monotonic_ns":r["monotonic_ns"],"open_fds":r["open_fds"]})).collect();
    json!({"codex_total_ns":total.map(|(a,b)|b-a),"external_response_ns":sum(&external),
        "request_to_first_byte_ns":sum(&first_byte),"approval_ns":sum(&approval),"sandbox_setup_spawn_ns":sum(&sandbox),
        "tool_call_ns":sum(&tools),"launcher_child_spawn_ns":sum(&launcher_spawn),"launcher_prepare_ns":sum(&launcher_prepare),"fixture_runtime_ns":sum(&fixture_runtime),
        "artifact_write_ns":(!artifact.is_empty()).then(||artifact.iter().sum::<u64>()),"fd_snapshots":fds,
        "rate_wait_ns":sum(&wait),"request_gate_queue_ns":sum(&queue),"policy_sandbox_spawn_ns":sum(&policy),
        "spawn_return_ns":sum(&spawn),"startup_to_ready_ns":sum(&ready_spans),
        "ready_to_wait_observed_ns":sum(&child_wait_spans),"wait_to_last_observed_eof_ns":sum(&drain_spans),
        "codex_turn_completed_observed_monotonic_ns":turn_completed,
        "turn_completed_to_unsubscribe_ns":turn_completed.zip(unsubscribe_begin).and_then(|(a,b)| b.checked_sub(a)),
        "unsubscribe_to_turn_completed_ns":unsubscribe_begin.zip(turn_completed).and_then(|(a,b)| b.checked_sub(a)),
        "codex_unsubscribe_ns":sum(&unsubscribe),
        "codex_processor_cleanup_ns":sum(&processor_cleanup),
        "codex_processor_join_ns":sum(&processor_join),
        "codex_outbound_join_ns":sum(&outbound_join),
        "codex_analytics_flush_ns":sum(&analytics_flush),
        "codex_client_shutdown_ns":sum(&client_shutdown),
        "codex_background_drain_ns":sum(&background_drain),
        "codex_threads_shutdown_ns":sum(&threads_shutdown),
        "codex_thread_unsubscribe_internal_ns":sum(&thread_unsubscribe),
        "unexplained_ns":total.map(|(a,b)|(b-a).saturating_sub(union(&explained,a,b))),
        "attribution":"Observed intervals can overlap. Unexplained time is the complement of their union, not model compute. Missing spans remain null."})
}

fn read_trace(directory: &Path) -> io::Result<(Vec<Value>, bool)> {
    let mut rows = Vec::new();
    let mut complete = true;
    if !directory.exists() {
        return Ok((rows, false));
    }
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        if entry.file_type()?.is_symlink() {
            continue;
        }
        let path = entry.path();
        if path.is_dir() {
            let (nested, ok) = read_trace(&path)?;
            rows.extend(nested);
            complete &= ok;
        } else if path.extension().is_some_and(|v| v == "jsonl")
            && path.file_name().is_none_or(|v| v != "codex.stdout.jsonl")
        {
            let (nested, ok) = lines(&path)?;
            rows.extend(nested.into_iter().map(crate::evidence::normalize_trace));
            complete &= ok;
        }
    }
    rows.sort_by_key(ns);
    Ok((rows, complete))
}

fn json_u64(value: &Value) -> Option<u64> {
    value
        .as_u64()
        .or_else(|| value.as_str().and_then(|text| text.parse().ok()))
}

/// Decode the small, local OTLP JSON evidence files without using their clock
/// values in the primary pair statistics. OTLP spans are a diagnostic view of
/// Codex internals; the measured intervals still come from the shared
/// OS-monotonic event trace. Keeping both views in the report lets a reviewer
/// locate a difference without silently replacing the measurement clock.
fn read_otel(root: &Path) -> io::Result<(Vec<Value>, bool)> {
    let directory = root.join("otel");
    if !directory.exists() {
        return Ok((Vec::new(), true));
    }
    let mut spans = Vec::new();
    let mut complete = true;
    let mut files: Vec<_> = fs::read_dir(&directory)?.collect::<Result<_, _>>()?;
    files.sort_by_key(|entry| entry.file_name());
    for entry in files {
        let path = entry.path();
        if path.extension().is_none_or(|extension| extension != "otlp") {
            continue;
        }
        let source = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .to_string_lossy()
            .into_owned();
        let Ok(bytes) = fs::read(&path) else {
            complete = false;
            continue;
        };
        let Ok(value) = serde_json::from_slice::<Value>(&bytes) else {
            complete = false;
            continue;
        };
        let Some(resources) = value["resourceSpans"].as_array() else {
            // Log exports are still preserved as raw files. They simply do
            // not contribute span rows to this derived diagnostic view.
            continue;
        };
        for resource in resources {
            for scope in resource["scopeSpans"].as_array().into_iter().flatten() {
                for span in scope["spans"].as_array().into_iter().flatten() {
                    let start = json_u64(&span["startTimeUnixNano"]);
                    let end = json_u64(&span["endTimeUnixNano"]);
                    let duration = start
                        .zip(end)
                        .filter(|(begin, finish)| finish >= begin)
                        .map(|(begin, finish)| finish - begin);
                    spans.push(json!({
                        "name": span["name"],
                        "trace_id": span["traceId"],
                        "span_id": span["spanId"],
                        "start_ns": start,
                        "end_ns": end,
                        "duration_ns": duration,
                        "clock_domain": "unix-wall-clock",
                        "attributes": span["attributes"],
                        "source": source,
                    }));
                }
            }
        }
    }
    Ok((spans, complete))
}

pub(crate) fn verify_seal(directory: &Path) -> io::Result<Value> {
    let Ok(seal) = read_json(&directory.join("seal.json")) else {
        return Ok(json!({"status":"unknown","reason":"unsealed_attempt"}));
    };
    let mut mismatches = Vec::new();
    if let Some(files) = seal["files"].as_array().filter(|files| !files.is_empty()) {
        if !files.iter().any(|file| file["path"] == "result.json") {
            return Ok(json!({"status":"mismatch","reason":"result_not_sealed"}));
        }
        for file in files {
            let Some(name) = file["path"].as_str() else {
                return Ok(json!({"status":"mismatch","reason":"invalid_seal_entry"}));
            };
            if Path::new(name)
                .components()
                .any(|p| !matches!(p, std::path::Component::Normal(_)))
            {
                return Ok(json!({"status":"mismatch","reason":"unsafe_path_in_seal"}));
            }
            if fs::read(directory.join(name)).is_ok_and(|bytes| file["sha256"] == digest(&bytes)) {
                continue;
            }
            mismatches.push(name);
        }
    } else {
        return Ok(json!({"status":"mismatch","reason":"empty_or_invalid_seal"}));
    }
    let actual = crate::evidence::inventory(directory)?;
    if seal["files"] != json!(actual) {
        return Ok(
            json!({"status":"mismatch","reason":"changed_or_unlisted_evidence","mismatches":mismatches}),
        );
    }
    Ok(
        json!({"status":if mismatches.is_empty(){"success"}else{"mismatch"},"mismatches":mismatches}),
    )
}

fn first_difference(shell: &Value, mbtx: &Value) -> Value {
    for (field, layer) in [
        ("actual_steps", "model_trajectory"),
        ("commands", "codex_output"),
        ("streams", "child_or_codex_io"),
    ] {
        let a = &shell["facts"][field];
        let b = &mbtx["facts"][field];
        if field == "actual_steps" {
            continue;
        } // Session IDs and workspace roots are deliberately isolated.
        if field == "commands" {
            let outputs = |v: &Value| {
                v.as_array().map(|items| {
                    items
                        .iter()
                        .map(
                            |i| json!({"output":i["aggregated_output"],"exit_code":i["exit_code"]}),
                        )
                        .collect::<Vec<_>>()
                })
            };
            if outputs(a) != outputs(b) {
                return json!({"layer":layer,"confidence":"observed","field":"commands.aggregated_output/exit_code","meaning":"Displayed output differs; per-stream oracle separately distinguishes byte loss from interleaving."});
            }
        } else {
            let streams = |v: &Value| {
                v.as_array().map(|items| {
                    items
                        .iter()
                        .map(|i| json!({"bytes":i["bytes"],"sha256":i["sha256"]}))
                        .collect::<Vec<_>>()
                })
            };
            if streams(a) != streams(b) {
                return json!({"layer":layer,"confidence":"unknown","field":field});
            }
        }
    }
    Value::Null
}

pub fn build(root: &Path, model_path: &Path) -> io::Result<Value> {
    let manifest = read_json(&root.join("manifest.json"))?;
    let mut model = Model::start(model_path)?;
    if manifest["suite"] == "launcher-startup" {
        return startup(root, manifest, &mut model);
    }
    let mut arms = Vec::new();
    let mut index = BTreeMap::<String, BTreeMap<String, usize>>::new();
    let mut statuses = BTreeMap::<String, usize>::new();
    let external = read_trace(&root.join("relay"))?.0;
    let (otel_spans, otel_complete) = read_otel(root)?;
    if root.join("attempts").exists() {
        let mut directories: Vec<_> =
            fs::read_dir(root.join("attempts"))?.collect::<Result<_, _>>()?;
        directories.sort_by_key(|e| e.file_name());
        for directory in directories {
            let path = directory.path();
            if !path.is_dir() {
                continue;
            }
            let facts = read_json(&path.join("facts.json")).unwrap_or(Value::Null);
            let mut result=read_json(&path.join("result.json")).unwrap_or(json!({"status":"censored","strict_comparable":false,
                "attempt_id":directory.file_name(),"failure_class":"collector_interrupted","artifact_dir":format!("attempts/{}",directory.file_name().to_string_lossy())}));
            let verification = verify_seal(&path)?;
            if facts.is_object() {
                let assessment = model.call(json!({"operation":"evaluate","facts":facts}))?;
                result
                    .as_object_mut()
                    .unwrap()
                    .extend(assessment.as_object().unwrap().clone());
            }
            let (mut trace, complete) = read_trace(&path)?;
            if !path.join("relay").exists() {
                trace.extend(
                    external
                        .iter()
                        .filter(|r| r["attempt_id"] == result["attempt_id"])
                        .cloned(),
                );
            }
            trace.sort_by_key(ns);
            if verification["status"] != "success" || !complete {
                result["strict_comparable"] = json!(false);
                result["evidence_warning"] = json!("unsealed_modified_or_truncated_evidence");
                if result["status"] == "success" {
                    result["status"] = json!(if verification["status"] == "mismatch" {
                        "harness_error"
                    } else {
                        "censored"
                    });
                    result["failure_class"] = json!("evidence_integrity");
                }
            }
            let pair = result["pair_id"]
                .as_str()
                .map(str::to_owned)
                .unwrap_or_else(|| {
                    let name = directory.file_name().to_string_lossy().into_owned();
                    name.rsplit_once('-')
                        .map(|(p, _)| p.to_owned())
                        .unwrap_or(name)
                });
            let backend = result["backend"]
                .as_str()
                .map(str::to_owned)
                .unwrap_or_else(|| {
                    directory
                        .file_name()
                        .to_string_lossy()
                        .rsplit('-')
                        .next()
                        .unwrap_or("unknown")
                        .to_owned()
                });
            result["pair_id"] = json!(pair);
            result["backend"] = json!(backend);
            let formal = result["sample_phase"] != "probe" && !pair.starts_with("probe-");
            if formal {
                *statuses
                    .entry(result["status"].as_str().unwrap_or("unknown").into())
                    .or_default() += 1;
                if index
                    .entry(pair)
                    .or_default()
                    .insert(backend, arms.len())
                    .is_some()
                {
                    return Err(io::Error::other(
                        "duplicate arm identity; evidence cannot be collapsed",
                    ));
                }
            }
            let durations = metrics(&trace, &result);
            arms.push(json!({"result":result,"facts":facts,"trace":trace,"metrics":durations,"seal":verification}));
        }
    }
    let mut pairs = Vec::new();
    let mut groups = BTreeMap::<String, BTreeMap<String, Vec<Value>>>::new();
    for (id, backends) in index {
        let left = backends.get("shell").map(|i| &arms[*i]);
        let right = backends.get("transparent").map(|i| &arms[*i]);
        let comparable = left.zip(right).is_some_and(|(a, b)| {
            a["result"]["strict_comparable"] == true && b["result"]["strict_comparable"] == true
        });
        let representative = left.or(right).unwrap();
        let result = &representative["result"];
        let group = format!(
            "{} / {} / round {} / {} / shell {}",
            manifest["platform"].as_str().unwrap_or("unknown"),
            result["task_id"].as_str().unwrap_or("unknown"),
            result["round"],
            manifest["tool_mode"].as_str().unwrap_or("unknown"),
            left.map(|a| a["result"]["shell_modes"].to_string())
                .unwrap_or("unknown".into())
        );
        if comparable {
            let (a, b) = (left.unwrap(), right.unwrap());
            for metric in [
                "codex_total_ns",
                "external_response_ns",
                "rate_wait_ns",
                "request_gate_queue_ns",
                "policy_sandbox_spawn_ns",
                "spawn_return_ns",
                "startup_to_ready_ns",
                "ready_to_wait_observed_ns",
                "wait_to_last_observed_eof_ns",
                "request_to_first_byte_ns",
                "approval_ns",
                "sandbox_setup_spawn_ns",
                "tool_call_ns",
                "launcher_child_spawn_ns",
                "launcher_prepare_ns",
                "fixture_runtime_ns",
                "artifact_write_ns",
                "turn_completed_to_unsubscribe_ns",
                "unsubscribe_to_turn_completed_ns",
                "codex_unsubscribe_ns",
                "codex_processor_cleanup_ns",
                "codex_processor_join_ns",
                "codex_outbound_join_ns",
                "codex_analytics_flush_ns",
                "codex_client_shutdown_ns",
                "codex_background_drain_ns",
                "codex_threads_shutdown_ns",
                "codex_thread_unsubscribe_internal_ns",
                "unexplained_ns",
            ] {
                groups
                    .entry(group.clone())
                    .or_default()
                    .entry(metric.into())
                    .or_default()
                    .push(json!({"shell":a["metrics"][metric],"transparent":b["metrics"][metric]}));
            }
        }
        pairs.push(json!({"pair_id":id,"task_id":result["task_id"],"category":result["category"],"round":result["round"],"platform":manifest["platform"],"tool_mode":manifest["tool_mode"],
            "shell":backends.get("shell"),"transparent":backends.get("transparent"),"strict_comparable":comparable,"first_difference":left.zip(right).map(|(a,b)|first_difference(a,b))}));
    }
    let mut statistics = Vec::new();
    for (group, metrics) in groups {
        for (metric, values) in metrics {
            statistics.push(
                json!({"stratum":group,"metric":metric,"units":"ns","confidence":"inferred",
            "statistics":model.call(json!({"operation":"statistics","pairs":values}))?}),
            );
        }
    }
    let valid = pairs
        .iter()
        .filter(|p| p["strict_comparable"] == true)
        .count();
    let boundary_notes: Vec<_> = arms
        .iter()
        .flat_map(|arm| {
            arm["result"]["boundary_notes"]
                .as_array()
                .into_iter()
                .flatten()
        })
        .map(|note| (note.to_string(), note.clone()))
        .collect::<BTreeMap<_, _>>()
        .into_values()
        .collect();
    let summary = json!({"intention_to_treat":{"attempted_pairs":pairs.len(),"arm_statuses":statuses},"strict_comparable_pairs":valid,
        "target_pairs":manifest["target_pairs"],"partial":manifest["target_pairs"].as_u64().is_none_or(|target|valid<(target as usize)),
        "stop":latest_summary(root),"platform":manifest["platform"],"purpose":manifest["purpose"],"boundary_observations":boundary_notes,
        "default_shell_is_direct":arms.iter().filter(|a|a["result"]["backend"] == "shell").filter_map(|a|a["result"]["shell_modes"].as_array()).flatten().next().map(|_|arms.iter().filter(|a|a["result"]["backend"] == "shell").filter_map(|a|a["result"]["shell_modes"].as_array()).flatten().all(|v| v == "Direct")),
        "limits":["No universal lossless replacement claim from this sample.","Model compute and relay internal waiting are inseparable without server evidence.",
            "Intervals overlap; local residual and missing stages remain unexplained.","The shared formal observer is retained symmetrically and its cost is not subtracted.",
            "Online scenario samples do not support stable p99 estimates.",
            "Default Shell and Direct control runs are separate artifacts; inspect observed shell_modes before attributing differences."]});
    Ok(
        json!({"schema_version":2,"manifest":manifest,"summary":summary,"pairs":pairs,"arms":arms,"statistics":statistics,
        "otel":{"enabled":root.join("otel").exists(),"complete":otel_complete,"spans":otel_spans,
            "raw_directory":root.join("otel").exists().then_some("otel")}}),
    )
}

pub fn latest_summary(root: &Path) -> Option<Value> {
    let mut files: Vec<_> = fs::read_dir(root)
        .ok()?
        .filter_map(Result::ok)
        .filter(|e| {
            e.file_name().to_string_lossy().starts_with("summary")
                && e.path().extension().is_some_and(|e| e == "json")
        })
        .collect();
    files.sort_by_key(|e| e.metadata().and_then(|m| m.modified()).ok());
    files.iter().rev().find_map(|e| read_json(&e.path()).ok())
}

fn startup(root: &Path, manifest: Value, model: &mut Model) -> io::Result<Value> {
    let (_, complete) = lines(&root.join("events.jsonl"))?;
    let mut rows = Vec::new();
    let mut arms = Vec::new();
    let mut pair_index = BTreeMap::<String, BTreeMap<String, usize>>::new();
    if root.join("attempts").exists() {
        let mut entries = fs::read_dir(root.join("attempts"))?.collect::<Result<Vec<_>, _>>()?;
        entries.sort_by_key(|e| e.file_name());
        for entry in entries {
            if !entry.file_type()?.is_dir() {
                continue;
            }
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().into_owned();
            let (pair, backend) = name.rsplit_once('-').unwrap_or((&name, "unknown"));
            let mut result = read_json(&path.join("result.json")).unwrap_or(json!({"phase":"startup","status":"censored","attempt_id":name,
                "pair_id":pair,"backend":backend,"sample_phase":if name.starts_with("warmup-") {"warmup"} else {"formal"},"artifact_dir":format!("attempts/{name}")}));
            let verification = verify_seal(&path)?;
            result["strict_comparable"] = json!(
                verification["status"] == "success"
                    && result["status"] == "success"
                    && result["startup_ns"].as_u64().is_some()
            );
            if verification["status"] != "success" && result["status"] == "success" {
                result["status"] = json!("censored");
            }
            result["begin_ns"] = result["spawn_begin_ns"].clone();
            result["end_ns"] = result["wait_return_ns"].clone();
            let (mut trace, _) = read_trace(&path)?;
            for (event, field) in [
                ("spawn_begin", "spawn_begin_ns"),
                ("spawn_return", "spawn_return_ns"),
                ("fixture_ready", "fixture_ready_ns"),
                ("wait_return", "wait_return_ns"),
            ] {
                if result[field].is_u64() {
                    let mut row = result.clone();
                    row["event"] = json!(event);
                    row["monotonic_ns"] = result[field].clone();
                    trace.push(row);
                }
            }
            trace.sort_by_key(ns);
            if result["sample_phase"] == "formal" {
                pair_index
                    .entry(pair.into())
                    .or_default()
                    .insert(backend.into(), arms.len());
            }
            rows.push(result.clone());
            arms.push(json!({"metrics":{"startup_to_ready_ns":result["startup_ns"],"spawn_return_ns":result["spawn_duration_ns"],"codex_total_ns":null},"result":result,"trace":trace,"seal":verification}));
        }
    }
    let mut groups = BTreeMap::<String, BTreeMap<String, BTreeMap<String, Value>>>::new();
    let mut failed = 0;
    for row in rows
        .iter()
        .filter(|row| row["phase"] == "startup" && row["sample_phase"] == "formal")
    {
        if row["status"] != "success" {
            failed += 1;
        }
        let group = format!(
            "{} / {} / {}",
            row["platform"].as_str().unwrap_or("unknown"),
            row["task_id"].as_str().unwrap_or("unknown"),
            row["observation"].as_str().unwrap_or("unknown")
        );
        let pair = row["pair_id"].as_str().unwrap_or("unknown").to_owned();
        let backend = row["backend"].as_str().unwrap_or("unknown").to_owned();
        let value = if row["strict_comparable"] == true {
            row["startup_ns"].clone()
        } else {
            Value::Null
        };
        if groups
            .entry(group)
            .or_default()
            .entry(pair)
            .or_default()
            .insert(backend, value)
            .is_some()
        {
            return Err(io::Error::other("duplicate startup sample"));
        }
    }
    let mut statistics = Vec::new();
    let mut valid = 0;
    for (group, pairs) in groups {
        let values: Vec<Value> = pairs
            .values()
            .map(|arms| json!({"shell":arms.get("shell"),"transparent":arms.get("transparent")}))
            .collect();
        let result = model.call(json!({"operation":"statistics","pairs":values}))?;
        valid += result["pairs"].as_u64().unwrap_or(0);
        let supported = group.ends_with("minimal")
            && result["pairs"].as_u64().is_some_and(|n| n >= 1000)
            && result["ratio_ci_upper_at_most_0_90"] == true;
        statistics.push(json!({"stratum":group,"metric":"spawn_begin_to_fixture_ready","units":"ns","confidence":"inferred","statistics":result,"at_least_ten_percent_supported":supported}));
    }
    let pairs: Vec<_> = pair_index.iter().map(|(id, backends)| {
        let left = backends.get("shell").map(|i| &arms[*i]);
        let right = backends.get("transparent").map(|i| &arms[*i]);
        let result = &left.or(right).unwrap()["result"];
        json!({"pair_id":id,"task_id":result["task_id"],"category":result["observation"],"platform":manifest["platform"],
            "shell":backends.get("shell"),"transparent":backends.get("transparent"),"strict_comparable":left.zip(right).is_some_and(|(a,b)|a["result"]["strict_comparable"]==true && b["result"]["strict_comparable"]==true)})
    }).collect();
    let target = manifest["pairs_per_workload_observation"]
        .as_u64()
        .map(|n| n * 6);
    Ok(
        json!({"schema_version":2,"manifest":manifest,"summary":{"purpose":manifest["purpose"],"failed_arms":failed,"complete_jsonl":complete,"formal_pairs_per_stratum":manifest["pairs_per_workload_observation"],
        "strict_comparable_pairs":valid,"target_pairs":target,"partial":target.is_none_or(|n| valid<n),"intention_to_treat":{"attempted_pairs":pairs.len()},
        "limits":["Minimal observation is primary. Full trace is a separate perturbation calibration.","All arms create new processes. Warmup is not process reuse."]},"statistics":statistics,"events":rows,"pairs":pairs,"arms":arms}),
    )
}

fn csv(value: &Value) -> String {
    format!(
        "\"{}\"",
        value
            .as_str()
            .map(str::to_owned)
            .unwrap_or_else(|| value.to_string())
            .replace('"', "\"\"")
    )
}

pub fn render(report: &Value, format: &str) -> io::Result<String> {
    match format {
        "json" => Ok(serde_json::to_string_pretty(report)?),
        "csv" => {
            if let Some(rows) = report["events"].as_array() {
                let mut out="pair_id,backend,task_id,observation,sample_phase,status,startup_ns,spawn_return_ns,fixture_ready_ns\n".to_owned();
                for row in rows.iter().filter(|r| r["phase"] == "startup") {
                    out.push_str(
                        &[
                            "pair_id",
                            "backend",
                            "task_id",
                            "observation",
                            "sample_phase",
                            "status",
                            "startup_ns",
                            "spawn_return_ns",
                            "fixture_ready_ns",
                        ]
                        .iter()
                        .map(|key| csv(&row[key]))
                        .collect::<Vec<_>>()
                        .join(","),
                    );
                    out.push('\n');
                }
                return Ok(out);
            }
            let mut out="pair_id,task_id,backend,status,strict_comparable,codex_total_ns,external_response_ns,rate_wait_ns,startup_to_ready_ns,unexplained_ns\n".to_string();
            for arm in report["arms"].as_array().unwrap() {
                let r = &arm["result"];
                let m = &arm["metrics"];
                let values = [
                    &r["pair_id"],
                    &r["task_id"],
                    &r["backend"],
                    &r["status"],
                    &r["strict_comparable"],
                    &m["codex_total_ns"],
                    &m["external_response_ns"],
                    &m["rate_wait_ns"],
                    &m["startup_to_ready_ns"],
                    &m["unexplained_ns"],
                ];
                out.push_str(&values.into_iter().map(csv).collect::<Vec<_>>().join(","));
                out.push('\n');
            }
            Ok(out)
        }
        "trace" => {
            let mut events = Vec::new();
            let mut processes = BTreeMap::new();
            for (thread, arm) in report["arms"].as_array().unwrap().iter().enumerate() {
                for row in arm["trace"].as_array().into_iter().flatten() {
                    let Some(time) = ns(row) else {
                        continue;
                    };
                    let key = json!([row["pid_namespace"], row["pid"], row["process_instance"]])
                        .to_string();
                    let next = processes.len() + 1;
                    let pid = *processes.entry(key.clone()).or_insert_with(|| {
                        events.push(json!({"ph":"M","name":"process_name","pid":next,"tid":0,"args":{"name":key}}));
                        next
                    });
                    events.push(
                        json!({"name":row["event"],"cat":row["phase"],"ph":"i","s":"t",
                        "ts":time as f64/1000.0,"pid":pid,"tid":thread,"args":row}),
                    );
                }
            }
            Ok(serde_json::to_string(
                &json!({"traceEvents":events,"displayTimeUnit":"ns"}),
            )?)
        }
        "md" => {
            let mut out = format!(
                "# MBTX execution comparison\n\n```json\n{}\n```\n\n| Stratum | Metric | Pairs | MBTX/Shell | 95% interval |\n|---|---|---:|---:|---|\n",
                serde_json::to_string_pretty(&report["summary"])?
            );
            for row in report["statistics"].as_array().unwrap() {
                out.push_str(&format!(
                    "| {} | {} | {} | {} | {} |\n",
                    row["stratum"]
                        .as_str()
                        .unwrap_or("unknown")
                        .replace('|', "\\|"),
                    row["metric"].as_str().unwrap_or("unknown"),
                    row["statistics"]["pairs"],
                    row["statistics"]["ratio"],
                    row["statistics"]["ratio_ci95"]
                ));
            }
            if report["otel"]["enabled"] == true {
                out.push_str(&format!(
                    "\n## Local OTel diagnostic index\n\n- Complete: `{}`\n- Raw evidence directory: `{}`\n- Decoded spans: `{}`\n- OTel spans are diagnostic evidence only; formal timing uses the shared OS-monotonic trace.\n",
                    report["otel"]["complete"],
                    report["otel"]["raw_directory"],
                    report["otel"]["spans"].as_array().map_or(0, Vec::len)
                ));
            }
            Ok(out)
        }
        "html" => {
            let data = serde_json::to_string(report)?
                .replace('<', "\\u003c")
                .replace('&', "\\u0026");
            Ok(format!(
                "{}<script id=report type=application/json>{data}</script><script>{}</script>",
                HTML, JS
            ))
        }
        _ => Err(io::Error::other(
            "format must be html, md, json, csv or trace",
        )),
    }
}

const HTML: &str = include_str!("report.html");
const JS: &str = include_str!("report.js");

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn overlapping_intervals_are_not_double_counted() {
        assert_eq!(union(&[(0, 20), (10, 30)], 0, 50), 30);
    }
    #[test]
    fn html_script_payload_is_escaped() {
        let report = json!({"summary":{"error":"</script><img src=x onerror=alert(1)>"},"statistics":[],"pairs":[],"arms":[]});
        let html = render(&report, "html").unwrap();
        assert!(!html.contains("</script><img"));
        assert!(html.contains("\\u003c/script>"));
    }
    #[test]
    fn null_metrics_are_not_zero() {
        let m = metrics(&[], &json!({}));
        assert!(m["external_response_ns"].is_null());
        assert!(m["unexplained_ns"].is_null());
    }
    #[test]
    fn startup_attribution_uses_the_call_across_pid_namespaces() {
        let mut rows = vec![
            json!({"phase":"spawn","event":"begin","pid":100,"stream_id":7,"call_id":"a","monotonic_ns":10}),
            json!({"phase":"spawn","event":"return","pid":100,"child_pid":200,"stream_id":7,"call_id":"a","monotonic_ns":12}),
            json!({"phase":"child","event":"ready","pid":2,"pid_namespace":"pid:[1]","call_id":"old","monotonic_ns":13}),
            json!({"phase":"child","event":"ready","pid":2,"pid_namespace":"pid:[2]","call_id":"a","monotonic_ns":15}),
        ];
        for row in &mut rows {
            row["clock_domain"] = json!("os-monotonic");
        }
        let result = metrics(&rows, &json!({}));
        assert_eq!(result["startup_to_ready_ns"], 5);
        assert_eq!(result["spawn_return_ns"], 2);
        rows[0]["call_id"] = Value::Null;
        assert!(metrics(&rows, &json!({}))["startup_to_ready_ns"].is_null());
    }
    #[test]
    fn nested_shutdown_spans_are_paired_lifo() {
        let rows = [
            json!({"phase":"codex_shutdown","event":"unsubscribe_begin","pid":1,"monotonic_ns":10,"clock_domain":"os-monotonic"}),
            json!({"phase":"codex_shutdown","event":"begin","pid":1,"monotonic_ns":20,"clock_domain":"os-monotonic"}),
            json!({"phase":"codex_shutdown","event":"return","pid":1,"monotonic_ns":30,"clock_domain":"os-monotonic"}),
            json!({"phase":"codex_shutdown","event":"unsubscribe_return","pid":1,"monotonic_ns":40,"clock_domain":"os-monotonic"}),
        ];
        assert_eq!(
            sum(&span(
                &rows,
                "codex_shutdown",
                "unsubscribe_begin",
                "unsubscribe_return"
            )),
            Some(30)
        );
        assert_eq!(
            sum(&span(&rows, "codex_shutdown", "begin", "return")),
            Some(10)
        );
    }
    #[test]
    fn otlp_spans_are_indexed_without_entering_monotonic_metrics() {
        let root = std::env::temp_dir().join(format!(
            "mbtx-otel-test-{}-{}",
            std::process::id(),
            crate::evidence::now_ns()
        ));
        fs::create_dir_all(root.join("otel")).unwrap();
        fs::write(
            root.join("otel/request-000001.otlp"),
            br#"{"resourceSpans":[{"scopeSpans":[{"spans":[{"name":"codex.exec","startTimeUnixNano":"100","endTimeUnixNano":"140","attributes":[]}]}]}]}"#,
        )
        .unwrap();
        let (spans, complete) = read_otel(&root).unwrap();
        assert!(complete);
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0]["duration_ns"], 40);
        assert_eq!(spans[0]["clock_domain"], "unix-wall-clock");
        fs::remove_dir_all(root).unwrap();
    }
}
