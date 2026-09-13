use codex_mbtx_contract::collection::option;
use codex_mbtx_contract::evidence::{Trace, digest, now_ns, write_json};
use serde_json::{Value, json};
use std::fs::{self, File};
use std::io::{self, Read};
use std::os::fd::{AsRawFd, FromRawFd};
use std::os::unix::process::{CommandExt, ExitStatusExt};
use std::path::Path;
use std::process::{Command, Stdio};

fn arm(
    fixture: &Path,
    mbtx: &Path,
    root: &Path,
    workload: &str,
    observation: &str,
    phase: &str,
    pair: usize,
    backend: &str,
) -> io::Result<Value> {
    let id = format!("{phase}-{observation}-{workload}-{pair:04}-{backend}");
    let directory = root.join("attempts").join(&id);
    fs::create_dir(&directory)?;
    let context = json!({"experiment_id":format!("launcher-startup-{}", digest(root.to_string_lossy().as_bytes())),"pair_id":format!("{phase}-{observation}-{workload}-{pair:04}"),
        "attempt_id":id,"task_id":workload,"backend":backend,"observation":observation,"sample_phase":phase,"platform":std::env::consts::OS});
    let mut fds = [-1; 2];
    if unsafe { libc::pipe(fds.as_mut_ptr()) } != 0 {
        return Err(io::Error::last_os_error());
    }
    let mut ready = unsafe { File::from_raw_fd(fds[0]) };
    let writer = unsafe { File::from_raw_fd(fds[1]) };
    unsafe {
        libc::fcntl(ready.as_raw_fd(), libc::F_SETFD, libc::FD_CLOEXEC);
    }
    let child_args = if workload == "shell-noop" {
        vec![
            "/bin/sh".to_string(),
            "-c".into(),
            "exec \"$1\" noop".into(),
            "sh".into(),
            fixture.display().to_string(),
        ]
    } else {
        vec![
            fixture.display().to_string(),
            if workload == "native-small" {
                "small"
            } else {
                "noop"
            }
            .into(),
        ]
    };
    let mut command = if backend == "transparent" {
        let mut command = Command::new(mbtx);
        command.args(["exec", "--"]).args(&child_args);
        command
    } else {
        let mut command = Command::new(&child_args[0]);
        command.args(&child_args[1..]);
        command
    };
    command
        .env("MBTX_READY_FD", writer.as_raw_fd().to_string())
        .env_remove("MBTX_TRACE_DIR")
        .env_remove("MBTX_LAUNCH_TRACE")
        .env_remove("MBTX_FIXTURE_DIR")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .process_group(0);
    let home = directory.join("home");
    fs::create_dir(&home)?;
    command.current_dir(&directory).env("HOME", home);
    if observation == "full" {
        command
            .env("MBTX_LAUNCH_TRACE", directory.join("launcher.jsonl"))
            .env("MBTX_FIXTURE_DIR", &directory);
    }
    for (key, name) in [
        ("experiment_id", "MBTX_EXPERIMENT_ID"),
        ("pair_id", "MBTX_PAIR_ID"),
        ("attempt_id", "MBTX_ATTEMPT_ID"),
        ("task_id", "MBTX_TASK_ID"),
        ("backend", "MBTX_BACKEND"),
    ] {
        command.env(name, context[key].as_str().unwrap());
    }
    let begin = now_ns();
    let child = command.spawn();
    let spawn_return = now_ns();
    drop(writer);
    let mut ready_ns = None;
    let mut exit_code = None;
    let mut signal = None;
    let mut error = None;
    let mut child_pid = None;
    match child {
        Ok(mut child) => {
            child_pid = Some(child.id());
            let mut poll = libc::pollfd {
                fd: ready.as_raw_fd(),
                events: libc::POLLIN,
                revents: 0,
            };
            let available = unsafe { libc::poll(&mut poll, 1, 5000) };
            let mut bytes = [0; 8];
            if available > 0 && ready.read_exact(&mut bytes).is_ok() {
                ready_ns = Some(u64::from_ne_bytes(bytes));
                if ready_ns.is_some_and(|ready| ready < begin) {
                    error = Some("fixture clock precedes spawn; clocks cannot be compared".into());
                }
            } else {
                error = Some("fixture ready event missing or timed out".into());
                unsafe {
                    libc::kill(-(child.id() as i32), libc::SIGKILL);
                }
            }
            match child.wait() {
                Ok(status) => {
                    exit_code = status.code();
                    signal = status.signal();
                }
                Err(e) => error = Some(e.to_string()),
            }
        }
        Err(e) => error = Some(e.to_string()),
    }
    let wait_return = now_ns();
    let mut result = context;
    result.as_object_mut().unwrap().extend(json!({"schema_version":2,"clock_domain":"os-monotonic","phase":"startup","event":"exit",
        "spawn_begin_ns":begin,"spawn_return_ns":spawn_return,"fixture_ready_ns":ready_ns,"wait_return_ns":wait_return,
        "startup_ns":ready_ns.and_then(|ready| ready.checked_sub(begin)),"spawn_duration_ns":spawn_return-begin,
        "wall_ns":wait_return-begin,"status":if error.is_some() {"harness_error"} else if exit_code == Some(0) {"success"} else {"backend_failure"},
        "error":error,"exit_code":exit_code,"signal":signal,"stdout_bytes":null,"stderr_bytes":null,
        "pid":std::process::id(),"child_pid":child_pid,"parent_id":null,"failure_class":null,"confidence":"observed","artifact_dir":format!("attempts/{id}")}).as_object().unwrap().clone());
    write_json(&directory.join("result.json"), &result)?;
    codex_mbtx_contract::evidence::seal(&directory)?;
    Ok(result)
}

fn run(args: &[String]) -> io::Result<()> {
    let required =
        |key| option(args, key).ok_or_else(|| io::Error::other(format!("missing {key}")));
    let fixture = fs::canonicalize(required("--fixture")?)?;
    let mbtx = fs::canonicalize(required("--mbtx")?)?;
    let output = required("--output")?;
    fs::create_dir_all(&output)?;
    let root = fs::canonicalize(output)?;
    let samples: usize = option(args, "--samples")
        .unwrap_or("1000".into())
        .parse()
        .map_err(io::Error::other)?;
    let warmups: usize = option(args, "--warmups")
        .unwrap_or("10".into())
        .parse()
        .map_err(io::Error::other)?;
    if samples == 0 {
        return Err(io::Error::other("samples must be positive"));
    }
    write_json(
        &root.join("manifest.json"),
        &json!({"schema_version":2,"suite":"launcher-startup","experiment_id":format!("launcher-startup-{}", digest(root.to_string_lossy().as_bytes())),"platform":std::env::consts::OS,
        "pairs_per_workload_observation":samples,"warmup_pairs":warmups,"workloads":["native-noop","native-small","shell-noop"],
        "observation_modes":["minimal","full"],"primary_observation":"minimal","order":"AB/BA; observation order alternates per pair",
        "ready_transport":"inherited pipe, native u64 OS CLOCK_MONOTONIC; recorded before fixture IO",
        "fixture":{"path":fixture,"sha256":digest(&fs::read(&fixture)?)},"launcher":{"path":mbtx,"sha256":digest(&fs::read(&mbtx)?)}}),
    )?;
    fs::create_dir(root.join("attempts"))?;
    let trace = Trace::create(
        &root.join("events.jsonl"),
        json!({"experiment_id":format!("launcher-startup-{}", digest(root.to_string_lossy().as_bytes()))}),
    )?;
    let mut failed = 0;
    for workload in ["native-noop", "native-small", "shell-noop"] {
        for (phase, count) in [("warmup", warmups), ("formal", samples)] {
            for pair in 0..count {
                for observation in if pair % 2 == 0 {
                    ["minimal", "full"]
                } else {
                    ["full", "minimal"]
                } {
                    for backend in if pair % 2 == 0 {
                        ["shell", "transparent"]
                    } else {
                        ["transparent", "shell"]
                    } {
                        let row = arm(
                            &fixture,
                            &mbtx,
                            &root,
                            workload,
                            observation,
                            phase,
                            pair,
                            backend,
                        )?;
                        failed += usize::from(phase == "formal" && row["status"] != "success");
                        trace.emit("startup", "exit", row)?;
                    }
                }
            }
            eprintln!("[startup] {phase} {workload} pairs={count} per observation mode");
        }
    }
    write_json(
        &root.join("summary.json"),
        &json!({"stop_reason":null,"failed_arms":failed,"partial":failed>0}),
    )?;
    trace.emit("collection", "finished", json!({"pairs":samples*6}))?;
    Ok(())
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.get(1).map(String::as_str) != Some("run") {
        eprintln!(
            "usage: mbtx-startup run --fixture PATH --mbtx PATH --output DIR [--samples 1000] [--warmups 10]"
        );
        std::process::exit(2);
    }
    if let Err(error) = run(&args) {
        eprintln!("startup collection failed: {error}; partial evidence preserved");
        std::process::exit(1);
    }
}
