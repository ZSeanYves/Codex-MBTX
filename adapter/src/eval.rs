use serde_json::Value;
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
}

fn usage() -> ! {
    eprintln!("usage: mbtx-eval run|log|report [--events FILE] [--format html|md|json|csv]");
    std::process::exit(2)
}
fn events(path: &str) -> Vec<Value> {
    fs::read_to_string(path)
        .unwrap_or_else(|e| {
            eprintln!("cannot read {path}: {e}");
            std::process::exit(2)
        })
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            serde_json::from_str(l).unwrap_or_else(|e| {
                eprintln!("invalid event: {e}");
                std::process::exit(2)
            })
        })
        .collect()
}
fn field<'a>(v: &'a Value, name: &str) -> &'a str {
    v.get(name).and_then(Value::as_str).unwrap_or("unknown")
}
fn esc(v: &str) -> String {
    v.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
fn report(path: &str, format: &str) {
    let rows = events(path);
    match format {
        "json" => println!("{}", serde_json::to_string_pretty(&rows).unwrap()),
        "csv" => {
            println!("pair_id,attempt_id,task_id,backend,event,status,failure_class,monotonic_ns");
            for r in &rows {
                println!(
                    "{},{},{},{},{},{},{},{}",
                    field(r, "pair_id"),
                    field(r, "attempt_id"),
                    field(r, "task_id"),
                    field(r, "backend"),
                    field(r, "event"),
                    field(r, "status"),
                    field(r, "failure_class"),
                    r["monotonic_ns"]
                );
            }
        }
        "md" => {
            println!(
                "# MBTX execution report\n\n| pair | task | backend | event | status | failure |\n|---|---|---|---|---|---|"
            );
            for r in &rows {
                println!(
                    "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |",
                    field(r, "pair_id"),
                    field(r, "task_id"),
                    field(r, "backend"),
                    field(r, "event"),
                    field(r, "status"),
                    field(r, "failure_class")
                );
            }
        }
        "html" => {
            println!(
                "<!doctype html><meta charset=utf-8><title>MBTX execution report</title><style>body{{font:14px system-ui;margin:2rem}}table{{border-collapse:collapse}}td,th{{border:1px solid #ccc;padding:.35rem}}</style><h1>MBTX execution report</h1><table><tr><th>pair</th><th>task</th><th>backend</th><th>event</th><th>status</th><th>failure</th></tr>"
            );
            for r in &rows {
                println!(
                    "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                    esc(field(r, "pair_id")),
                    esc(field(r, "task_id")),
                    esc(field(r, "backend")),
                    esc(field(r, "event")),
                    esc(field(r, "status")),
                    esc(field(r, "failure_class"))
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
                .position(|a| a == "--mbtx")
                .and_then(|i| args.get(i + 1))
                .cloned()
                .unwrap_or_else(|| usage());
            let out = args
                .iter()
                .position(|a| a == "--output")
                .and_then(|i| args.get(i + 1))
                .cloned()
                .unwrap_or_else(|| "_build/launcher-evidence".to_owned());
            let pairs: usize = args
                .iter()
                .position(|a| a == "--pairs")
                .and_then(|i| args.get(i + 1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(4)
                .min(6);
            fs::create_dir_all(&out).unwrap();
            let tasks = [
                (
                    "literal_argv",
                    "printf '%s|%s|%s' '$(touch SHOULD_NOT_EXIST)' 'a b' '\"quoted\"'",
                ),
                ("stdin_utf8", "printf 'stdin✓\\n'"),
                ("cwd_environment", "printf '%s' \"$PWD\""),
                ("large_output", "yes x | head -c 65536"),
                ("nonzero_exit", "printf before; exit 7"),
                ("background_cleanup", "(sleep .02; exit 0) & wait"),
                ("signal_exit", "kill -TERM \"$$\""),
                ("recovery", "printf first; printf second"),
            ];
            let mut jobs = Vec::new();
            for pair in 0..pairs {
                for (index, (task, script)) in tasks.iter().enumerate() {
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
        "log" => {
            let path = args
                .iter()
                .position(|a| a == "--events")
                .and_then(|i| args.get(i + 1))
                .map(String::as_str)
                .unwrap_or_else(|| usage());
            for r in events(path) {
                println!(
                    "{} {} {} {}",
                    field(&r, "pair_id"),
                    field(&r, "backend"),
                    field(&r, "event"),
                    field(&r, "status")
                );
            }
        }
        "report" => {
            let path = args
                .iter()
                .position(|a| a == "--events")
                .and_then(|i| args.get(i + 1))
                .map(String::as_str)
                .unwrap_or_else(|| usage());
            let f = args
                .iter()
                .position(|a| a == "--format")
                .and_then(|i| args.get(i + 1))
                .map(String::as_str)
                .unwrap_or("md");
            report(path, f);
        }
        _ => usage(),
    }
}

#[cfg(test)]
mod tests {
    use super::esc;
    #[test]
    fn html_escapes_untrusted_event_text() {
        assert_eq!(esc("<script>&\""), "&lt;script&gt;&amp;&quot;");
    }
}
