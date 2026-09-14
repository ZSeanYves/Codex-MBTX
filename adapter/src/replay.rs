use crate::evidence::{Trace, write_json};
use serde_json::{Value, json};
use std::io;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use tiny_http::{Header, Response, Server};

pub struct Replay {
    pub url: String,
    pub active: Arc<Mutex<Value>>,
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

fn session_id(value: &Value) -> Option<i64> {
    if let Some(id) = value.get("session_id").and_then(Value::as_i64) {
        return Some(id);
    }
    match value {
        Value::Array(items) => items.iter().rev().find_map(session_id),
        Value::Object(fields) => fields.values().rev().find_map(session_id),
        Value::String(text) => {
            if let Ok(value) = serde_json::from_str::<Value>(text) {
                if let Some(id) = session_id(&value) {
                    return Some(id);
                }
            }
            text.rsplit_once("session ID ")
                .and_then(|(_, tail)| tail.split_whitespace().next()?.parse().ok())
        }
        _ => None,
    }
}

fn is_model_request(body: &Value) -> bool {
    body["input"].is_array()
}

fn response(body: &Value, active: &Value, sequence: usize) -> io::Result<String> {
    let input = body["input"]
        .as_array()
        .ok_or_else(|| io::Error::other("missing Responses input"))?;
    let count = input
        .iter()
        .filter(|item| {
            matches!(
                item["type"].as_str(),
                Some("custom_tool_call_output" | "function_call_output")
            )
        })
        .count();
    let steps = active["task"]["steps"]
        .as_array()
        .ok_or_else(|| io::Error::other("missing replay steps"))?;
    let item = if active["tool_mode"] == "code" && count == 0 && !steps.is_empty() {
        json!({"type":"custom_tool_call","call_id":format!("call-{sequence}"),"name":"exec",
            "input":active["task"]["code_mode_program"]})
    } else if active["tool_mode"] != "code"
        && let Some(step) = steps.get(count)
    {
        let mut arguments = step["arguments"].clone();
        if arguments["session_id"] == "$session" {
            arguments["session_id"] = json!(
                input
                    .iter()
                    .rev()
                    .find_map(session_id)
                    .ok_or_else(|| io::Error::other("session ID missing from real tool output"))?
            );
        }
        let name = step["tool"]
            .as_str()
            .ok_or_else(|| io::Error::other("missing replay tool name"))?;
        json!({"type":"function_call","id":format!("fc-{sequence}"),"call_id":format!("call-{sequence}"),"name":name,"arguments":arguments.to_string()})
    } else {
        json!({"type":"message","role":"assistant","id":format!("done-{sequence}"),"content":[{"type":"output_text","text":"DONE"}]})
    };
    let id = format!("replay-{sequence}");
    let events = [
        json!({"type":"response.created","response":{"id":id}}),
        json!({"type":"response.output_item.done","item":item}),
        json!({"type":"response.completed","response":{"id":id}}),
    ];
    Ok(events
        .iter()
        .map(|event| {
            format!(
                "event: {}\ndata: {event}\n\n",
                event["type"].as_str().unwrap()
            )
        })
        .collect())
}

impl Replay {
    pub fn start(directory: PathBuf) -> io::Result<Self> {
        std::fs::create_dir_all(&directory)?;
        let server = Server::http("127.0.0.1:0").map_err(io::Error::other)?;
        let url = format!("http://{}/v1", server.server_addr());
        let active = Arc::new(Mutex::new(json!({})));
        let context = Arc::clone(&active);
        let stop = Arc::new(AtomicBool::new(false));
        let stopped = Arc::clone(&stop);
        let thread = thread::spawn(move || {
            let mut sequence = 0;
            while !stopped.load(Ordering::Relaxed) {
                let Ok(Some(mut request)) = server.recv_timeout(Duration::from_millis(100)) else {
                    continue;
                };
                sequence += 1;
                let active = context.lock().unwrap().clone();
                let trace = Trace::create(
                    &directory.join(format!("request-{sequence:06}.jsonl")),
                    active["context"].clone(),
                )
                .unwrap();
                let _ = trace.emit("replay", "request_received", json!({"request_id":sequence}));
                let mut bytes = Vec::new();
                let parsed = request.as_reader().read_to_end(&mut bytes).and_then(|_| {
                    serde_json::from_slice::<Value>(&bytes).map_err(io::Error::other)
                });
                if let Ok(body) = &parsed {
                    let _ =
                        write_json(&directory.join(format!("request-{sequence:06}.json")), body);
                }
                let sse = parsed.and_then(|body| {
                    if is_model_request(&body) {
                        response(&body, &active, sequence)
                    } else {
                        // Codex may send a housekeeping request (for example
                        // a response deletion containing only `turn_ids`). It
                        // is not a model turn and must not be classified as a
                        // missing Responses input or a provider failure.
                        let _ = trace.emit(
                            "replay",
                            "housekeeping",
                            json!({"keys":body.as_object().map(|fields| fields.keys().cloned().collect::<Vec<_>>())}),
                        );
                        Ok(String::new())
                    }
                });
                let fault = active["fault"].as_str().unwrap_or("none");
                if matches!(fault, "429" | "503") {
                    let code = fault.parse::<u16>().unwrap();
                    let _ = trace.emit(
                        "external",
                        "response_headers",
                        json!({"detail":{"status":code}}),
                    );
                    let _ = request.respond(
                        Response::from_string(
                            "{\"error\":{\"message\":\"offline injected HTTP failure\"}}",
                        )
                        .with_status_code(code)
                        .with_header(Header::from_bytes("Retry-After", "1").unwrap()),
                    );
                    continue;
                }
                if fault == "disconnect" {
                    let _ = trace.emit(
                        "external",
                        "stream_error",
                        json!({"detail":{"error":"injected incomplete SSE"}}),
                    );
                    let _=request.respond(Response::from_string("event: response.created\ndata: {\"type\":\"response.created\",\"response\":{\"id\":\"interrupted\"}}\n\n")
                        .with_header(Header::from_bytes("Content-Type","text/event-stream").unwrap()));
                    continue;
                }
                match sse {
                    Ok(sse) if sse.is_empty() => {
                        let _ = request.respond(Response::from_string("{}"));
                    }
                    Ok(sse) => {
                        let _ = crate::evidence::write_new(
                            &directory.join(format!("response-{sequence:06}.sse")),
                            sse.as_bytes(),
                        );
                        let _ = trace.emit(
                            "replay",
                            "response_send",
                            json!({"request_id":sequence,"bytes":sse.len()}),
                        );
                        let response = Response::from_string(sse).with_header(
                            Header::from_bytes("Content-Type", "text/event-stream").unwrap(),
                        );
                        let _ = request.respond(response);
                    }
                    Err(error) => {
                        let _ = trace.emit(
                            "replay",
                            "harness_error",
                            json!({"error":error.to_string()}),
                        );
                        let _ = request.respond(
                            Response::from_string(
                                json!({"error":{"message":error.to_string()}}).to_string(),
                            )
                            .with_status_code(500),
                        );
                    }
                }
            }
        });
        Ok(Self {
            url,
            active,
            stop,
            thread: Some(thread),
        })
    }
}

impl Drop for Replay {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn replay_requires_real_session_and_preserves_arguments() {
        let active = json!({"task":{"steps":[{"tool":"write_stdin","arguments":{"session_id":"$session","chars":"\u{3}"}}]},"tool_mode":"direct"});
        assert!(response(&json!({"input":[]}), &active, 1).is_err());
        assert_eq!(
            session_id(&json!({"output":"{\"session_id\":42}"})),
            Some(42)
        );
    }

    #[test]
    fn housekeeping_requests_are_not_model_turns() {
        assert!(!is_model_request(&json!({"turn_ids":["turn-1"]})));
        assert!(is_model_request(&json!({"input":[]})));
    }
}
