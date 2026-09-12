use serde_json::{Value, json};
use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const TEST_KEY: &str = "mbtx-offline-auth-fixture-not-a-real-key";

struct Request {
    authenticated: bool,
    body: Value,
}

fn read_request(stream: &TcpStream) -> Request {
    stream.set_nonblocking(false).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(10)))
        .unwrap();
    stream
        .set_write_timeout(Some(Duration::from_secs(10)))
        .unwrap();
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    assert!(
        line.starts_with("POST /v1/responses "),
        "unexpected endpoint: {line}"
    );
    let mut content_length = 0;
    let mut authenticated = false;
    loop {
        line.clear();
        assert!(reader.read_line(&mut line).unwrap() > 0);
        if line == "\r\n" {
            break;
        }
        let (name, value) = line.split_once(':').expect("HTTP header");
        if name.eq_ignore_ascii_case("authorization") {
            authenticated = value.trim() == format!("Bearer {TEST_KEY}");
        }
        if name.eq_ignore_ascii_case("content-length") {
            content_length = value.trim().parse::<usize>().unwrap();
        }
    }
    assert!(content_length > 0 && content_length < 4 * 1024 * 1024);
    let mut body = vec![0; content_length];
    reader.read_exact(&mut body).unwrap();
    Request {
        authenticated,
        body: serde_json::from_slice(&body).expect("Responses request JSON"),
    }
}

fn reject_request(mut stream: TcpStream) -> Request {
    let request = read_request(&stream);
    // Exercise the real Codex error path without contacting a provider or
    // relying on credentials, model availability, or a generated response.
    let response = json!({"error": {"code": "invalid_api_key", "message": "Invalid API key (offline fixture)"}}).to_string();
    write!(stream, "HTTP/1.1 401 Unauthorized\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{response}", response.len()).unwrap();
    request
}

fn execute_task(mut stream: TcpStream) -> Request {
    let request = read_request(&stream);
    let input = request.body["input"].as_array().expect("Responses input");
    let item = if input
        .iter()
        .any(|item| item["type"] == "custom_tool_call_output")
    {
        json!({"type": "message", "role": "assistant", "id": "fixture-done", "content": [{"type": "output_text", "text": "DONE"}]})
    } else {
        let prompt = input
            .iter()
            .filter(|item| item["role"] == "user")
            .flat_map(|item| item["content"].as_array().into_iter().flatten())
            .filter_map(|part| part["text"].as_str())
            .find(|text| text.starts_with("Use the exec_command tool exactly once."))
            .expect("collector task prompt");
        let (_, command) = prompt.rsplit_once("\n\n").expect("literal task command");
        let args = json!({"cmd": command, "yield_time_ms": 10000, "max_output_tokens": 25000});
        json!({"type": "custom_tool_call", "call_id": "fixture-exec", "name": "exec", "input": format!("// @exec: {{\"yield_time_ms\": 10000}}\ntext(JSON.stringify(await tools.exec_command({args}))); ")})
    };
    let events = [
        json!({"type": "response.created", "response": {"id": "fixture-response"}}),
        json!({"type": "response.output_item.done", "item": item}),
        json!({"type": "response.completed", "response": {"id": "fixture-response", "usage": {"input_tokens": 0, "output_tokens": 0, "total_tokens": 0}}}),
    ];
    let response: String = events
        .iter()
        .map(|event| {
            format!(
                "event: {}\ndata: {event}\n\n",
                event["type"].as_str().unwrap()
            )
        })
        .collect();
    write!(stream, "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{response}", response.len()).unwrap();
    request
}

fn contains_tool(value: &Value, name: &str) -> bool {
    match value {
        Value::Array(items) => items.iter().any(|item| contains_tool(item, name)),
        Value::Object(fields) => {
            fields.get("name").and_then(Value::as_str) == Some(name)
                || fields.values().any(|value| contains_tool(value, name))
        }
        _ => false,
    }
}

fn advertised_tools(body: &Value) -> Value {
    let mut tools = vec![body["tools"].clone()];
    if let Some(input) = body["input"].as_array() {
        tools.extend(
            input
                .iter()
                .filter(|item| item["type"] == "additional_tools")
                .map(|item| item["tools"].clone()),
        );
    }
    Value::Array(tools)
}

fn assert_exec_is_advertised(requests: &[Request]) {
    for request in requests {
        let tools = advertised_tools(&request.body);
        assert!(
            contains_tool(&tools, "exec") && tools.to_string().contains("tools.exec_command"),
            "Code Mode exec with exec_command was not advertised: {tools}"
        );
    }
}

fn contains_key_on_disk(path: &Path) -> bool {
    fs::read_dir(path).unwrap().any(|entry| {
        let entry = entry.unwrap();
        let kind = entry.file_type().unwrap();
        if kind.is_dir() {
            contains_key_on_disk(&entry.path())
        } else if kind.is_file() {
            fs::read(entry.path())
                .unwrap()
                .windows(TEST_KEY.len())
                .any(|window| window == TEST_KEY.as_bytes())
        } else {
            false
        }
    })
}

fn collect(respond: fn(TcpStream) -> Request, launcher: &str) -> (PathBuf, Output, Vec<Request>) {
    let codex =
        std::env::var("MBTX_TEST_CODEX").expect("set MBTX_TEST_CODEX to the prepared Codex binary");
    assert!(Path::new(&codex).is_file());
    let output = std::env::temp_dir().join(format!(
        "mbtx-online-auth-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let relay_url = format!("http://{}/v1", listener.local_addr().unwrap());
    let stopped = Arc::new(AtomicBool::new(false));
    let server_stopped = stopped.clone();
    let server = thread::spawn(move || {
        let mut requests = Vec::new();
        while !server_stopped.load(Ordering::Relaxed) {
            match listener.accept() {
                Ok((stream, _)) => requests.push(respond(stream)),
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(5))
                }
                Err(error) => panic!("mock relay accept: {error}"),
            }
        }
        requests
    });
    let result = Command::new(env!("CARGO_BIN_EXE_mbtx-online"))
        .args([
            "run",
            "--codex",
            &codex,
            "--mbtx",
            launcher,
            "--relay-base-url",
            &relay_url,
            "--pairs",
            "1",
            "--timeout-ms",
            "30000",
            "--output",
        ])
        .arg(&output)
        .env("OPENAI_API_KEY", TEST_KEY)
        .env_remove("CODEX_API_KEY")
        .env("NO_PROXY", "*")
        .output();
    stopped.store(true, Ordering::Relaxed);
    let requests = server.join().unwrap();
    let result = result.unwrap();
    (output, result, requests)
}

#[test]
#[ignore = "requires MBTX_TEST_CODEX pointing to the prepared Codex binary; uses only a local mock relay"]
fn isolated_codex_authentication_and_partial_reporting() {
    let (output, result, requests) = collect(reject_request, "/usr/bin/true");
    assert!(
        result.status.success(),
        "collector failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert!(stdout.contains("[online] partial"), "{stdout}");
    assert!(stdout.contains("http_status=401"), "{stdout}");
    assert_eq!(
        requests.len(),
        5,
        "evidence retained at {}",
        output.display()
    );
    assert!(
        requests.iter().all(|request| request.authenticated),
        "Bearer credential was not attached"
    );
    assert_exec_is_advertised(&requests);
    let summary: Value =
        serde_json::from_slice(&fs::read(output.join("summary.json")).unwrap()).unwrap();
    assert_eq!(summary["partial"], true);
    assert_eq!(summary["complete_pairs"], 0);
    assert_eq!(
        summary["stop_reason"],
        "five consecutive infrastructure failures"
    );
    for record in summary["records"].as_array().unwrap() {
        assert_eq!(record["status"], "provider_error");
        assert_eq!(record["failure_class"], "provider_authentication");
        assert_eq!(record["http_status"], 401);
        let evidence = fs::read_to_string(
            output
                .join(record["artifact_dir"].as_str().unwrap())
                .join("codex.stdout.jsonl"),
        )
        .unwrap();
        assert!(!evidence.contains("Code Mode is unavailable"));
        assert!(!evidence.contains("experimental_use_unified_exec_tool"));
    }
    assert!(
        !contains_key_on_disk(&output),
        "credential leaked into evidence"
    );
    fs::remove_dir_all(output).unwrap();
}

#[test]
#[ignore = "requires MBTX_TEST_CODEX and MBTX_TEST_LAUNCHER; real processes with a deterministic local Responses fixture"]
fn code_mode_executes_all_tasks_through_both_backends() {
    let launcher =
        std::env::var("MBTX_TEST_LAUNCHER").expect("set MBTX_TEST_LAUNCHER to the built launcher");
    assert!(Path::new(&launcher).is_file());
    let (output, result, requests) = collect(execute_task, &launcher);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let summary: Value =
        serde_json::from_slice(&fs::read(output.join("summary.json")).unwrap()).unwrap();
    assert_eq!(
        summary["complete_pairs"],
        8,
        "evidence at {}: {summary}",
        output.display()
    );
    assert_eq!(summary["successful_arms"], 16);
    assert_eq!(summary["partial"], false);
    assert_eq!(requests.len(), 32);
    assert!(requests.iter().all(|request| request.authenticated));
    assert_exec_is_advertised(&requests);
    for record in summary["records"].as_array().unwrap() {
        assert_eq!(record["status"], "success");
        assert_eq!(record["http_status"], Value::Null);
        let evidence = fs::read_to_string(
            output
                .join(record["artifact_dir"].as_str().unwrap())
                .join("codex.stdout.jsonl"),
        )
        .unwrap();
        assert!(!evidence.contains("Code Mode is unavailable"));
        assert!(evidence.contains("command_execution"));
    }
    assert!(
        !contains_key_on_disk(&output),
        "credential leaked into evidence"
    );
    fs::remove_dir_all(output).unwrap();
}
