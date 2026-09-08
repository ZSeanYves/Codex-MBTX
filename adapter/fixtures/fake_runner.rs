use serde_json::Value;
use serde_json::json;
use std::io::Write;
use std::time::Duration;

fn emit(value: Value) {
    println!("{value}");
    std::io::stdout().flush().unwrap();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args[args.len() - 2], "--request");
    let request: Value = serde_json::from_str(args.last().unwrap()).unwrap();
    assert_eq!(request["op"], "run");
    assert_eq!(request["background"], false);
    let id = &request["id"];
    let mut mode = args.get(1).map(String::as_str).unwrap_or("echo");
    if let Some(marker) = mode.strip_prefix("record:") {
        std::fs::write(marker, std::process::id().to_string()).unwrap();
    }
    if let Some(marker) = mode.strip_prefix("sleep:") {
        std::fs::write(marker, std::process::id().to_string()).unwrap();
        mode = "sleep";
    }
    emit(json!({"event":"started", "id":id, "job_id":"job-1"}));
    match mode {
        "malformed" => println!("not JSON"),
        "missing_terminal" => {}
        "wrong_id" => emit(
            json!({"event":"completed","id":"other","job_id":"job-1","exit_code":0,"duration_ms":1}),
        ),
        "timeout" => emit(
            json!({"event":"failed","id":id,"job_id":"job-1","error":{"kind":"timeout","message":"fixture timeout"}}),
        ),
        "sleep" => {
            emit(json!({"event":"stdout","id":id,"job_id":"job-1","seq":0,"data":"ready"}));
            std::thread::sleep(Duration::from_secs(60));
            emit(
                json!({"event":"completed","id":id,"job_id":"job-1","exit_code":0,"duration_ms":60000}),
            );
        }
        _ => {
            emit(
                json!({"event":"stdout","id":id,"job_id":"job-1","seq":0,"data":request.to_string()}),
            );
            emit(json!({"event":"stderr","id":id,"job_id":"job-1","seq":1,"data":"diagnostic\n"}));
            emit(
                json!({"event":"completed","id":id,"job_id":"job-1","exit_code":if mode == "failure" {7} else {0},"duration_ms":1}),
            );
        }
    }
}
