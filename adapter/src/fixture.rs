use codex_mbtx_contract::evidence::{Trace, now_ns, write_json};
use serde_json::json;
use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Duration;

fn sleep(ms: u64) {
    std::thread::sleep(Duration::from_millis(ms));
}

fn main() -> io::Result<()> {
    let ready = now_ns();
    if let Ok(fd) = env::var("MBTX_READY_FD") {
        let fd: i32 = fd.parse().map_err(io::Error::other)?;
        let bytes = ready.to_ne_bytes();
        if unsafe { libc::write(fd, bytes.as_ptr().cast(), bytes.len()) } != 8 {
            return Err(io::Error::last_os_error());
        }
        unsafe {
            libc::close(fd);
        }
    }
    let args: Vec<String> = env::args().collect();
    let mode = args.get(1).map(String::as_str).unwrap_or("noop");
    let evidence = env::var_os("MBTX_FIXTURE_DIR").map(PathBuf::from);
    let instance = format!("{}-{ready}", std::process::id());
    let mut context = json!({"experiment_id":env::var("MBTX_EXPERIMENT_ID").ok(),
        "pair_id":env::var("MBTX_PAIR_ID").ok(),"attempt_id":env::var("MBTX_ATTEMPT_ID").ok(),
        "task_id":env::var("MBTX_TASK_ID").ok(),"backend":env::var("MBTX_BACKEND").ok(),
        "mode":mode,"argv":args,"pid":std::process::id(),"process_instance":instance,
        "call_id":env::var("MBTX_TOOL_CALL_ID").ok(),
        "pgid":unsafe { libc::getpgrp() },"ppid":unsafe { libc::getppid() }});
    context.as_object_mut().unwrap().extend(
        codex_mbtx_contract::process_identity::fixture_scope()
            .as_object()
            .unwrap()
            .clone(),
    );
    let trace = evidence
        .as_ref()
        .map(|dir| -> io::Result<Trace> {
            fs::create_dir_all(dir)?;
            Trace::create(&dir.join(format!("{instance}.jsonl")), context.clone())
        })
        .transpose()?;
    if let Some(trace) = &trace {
        trace.at(ready, "child", "ready", json!({}))?;
    }
    let mut receipt = context;
    receipt["ready_ns"] = json!(ready);
    receipt["cwd"] = json!(env::current_dir()?);
    receipt["stdin_tty"] = json!(unsafe { libc::isatty(0) } == 1);
    if let Some(dir) = &evidence {
        write_json(&dir.join(format!("{instance}.started.json")), &receipt)?;
    }
    let mut exit = 0;
    match mode {
        "noop" => {}
        "small" => println!("fixture-ok"),
        "argv" => {
            receipt["arguments"] = json!(&args[2..]);
            println!("{}", receipt["arguments"]);
        }
        "stdin" => {
            let mut input = Vec::new();
            io::stdin().read_to_end(&mut input)?;
            receipt["input_bytes"] = json!(input.len());
            receipt["input_utf8"] = json!(String::from_utf8(input.clone()).ok());
            receipt["input_sha256"] = json!(codex_mbtx_contract::evidence::digest(&input));
            println!("{}", input.len());
        }
        "emit-input" => {
            io::stdout().write_all(&vec![b'i'; 262144])?;
        }
        "environment" => {
            receipt["environment"] = json!({"MBTX_EMPTY":env::var("MBTX_EMPTY").ok(),
                "MBTX_ABSENT":env::var("MBTX_ABSENT").ok(),"PATH":env::var("PATH").ok(),
                "HOME":env::var("HOME").ok(),"MBTX_EVAL_MARKER":env::var("MBTX_EVAL_MARKER").ok()});
            println!("{}", receipt["environment"]);
        }
        "output-large" | "output-tail" => {
            let count = if mode == "output-tail" { 262144 } else { 65536 };
            io::stdout().write_all(&vec![b'x'; count])?;
            io::stdout().flush()?;
            if let Some(trace) = &trace {
                trace.emit("child", "write", json!({"stream":"stdout","bytes":count}))?;
            }
            io::stderr().write_all(&vec![b'e'; 4096])?;
            io::stderr().flush()?;
            if mode == "output-tail" {
                print!("TAIL-END\n");
            }
        }
        "output-interleaved" => {
            for index in 0..32 {
                let stream = if index % 2 == 0 { "stdout" } else { "stderr" };
                let line = format!("{stream}:{index:02}\n");
                if index % 2 == 0 {
                    io::stdout().write_all(line.as_bytes())?;
                    io::stdout().flush()?;
                } else {
                    io::stderr().write_all(line.as_bytes())?;
                    io::stderr().flush()?;
                }
                if let Some(trace) = &trace {
                    trace.emit(
                        "child",
                        "write",
                        json!({"stream":stream,"bytes":line.len(),"index":index}),
                    )?;
                }
                sleep(2);
            }
        }
        "exit" => {
            exit = args[2].parse().map_err(io::Error::other)?;
            println!("exit:{exit}");
        }
        "self-term" => {
            io::stdout().write_all(b"term-ready\n")?;
            unsafe {
                libc::raise(libc::SIGTERM);
            }
        }
        "session" => {
            println!("session-ready");
            io::stdout().flush()?;
            sleep(2200);
            println!("session-done");
        }
        "interrupt" | "term" | "ignore-term" | "descendants" | "detached" => {
            if mode == "ignore-term" {
                unsafe {
                    libc::signal(libc::SIGTERM, libc::SIG_IGN);
                }
            }
            if mode == "descendants" || mode == "detached" {
                let mut cmd = Command::new(env::current_exe()?);
                cmd.arg("grandchild").env_remove("MBTX_READY_FD");
                if mode == "detached" {
                    cmd.process_group(0);
                }
                let child = cmd.spawn()?;
                receipt["child_pid"] = json!(child.id());
            }
            if let Some(dir) = &evidence {
                write_json(&dir.join(format!("{instance}.ready.json")), &receipt)?;
            }
            println!("cancel-ready");
            io::stdout().flush()?;
            sleep(120000);
            println!("UNEXPECTED-NATURAL-EXIT");
        }
        "grandchild" => {
            let mut child = Command::new(env::current_exe()?)
                .arg("interrupt")
                .env_remove("MBTX_READY_FD")
                .spawn()?;
            child.wait()?;
        }
        "pipe-tail" => {
            let _child = Command::new(env::current_exe()?)
                .arg("delayed-tail")
                .env_remove("MBTX_READY_FD")
                .stdin(Stdio::null())
                .spawn()?;
            println!("parent-done");
        }
        "delayed-tail" => {
            sleep(150);
            println!("descendant-tail");
            io::stdout().flush()?;
            if let Some(trace) = &trace {
                trace.emit(
                    "child",
                    "write",
                    json!({"stream":"stdout","content_tag":"descendant_tail","bytes":16}),
                )?;
            }
        }
        _ => return Err(io::Error::other(format!("unknown fixture mode {mode}"))),
    }
    io::stdout().flush()?;
    io::stderr().flush()?;
    receipt["completed_ns"] = json!(now_ns());
    receipt["exit_code_requested"] = json!(exit);
    if let Some(trace) = &trace {
        trace.emit("child", "return", json!({"exit_code_requested":exit}))?;
    }
    if let Some(dir) = &evidence {
        write_json(&dir.join(format!("{instance}.receipt.json")), &receipt)?;
    }
    if exit != 0 {
        std::process::exit(exit);
    }
    Ok(())
}
