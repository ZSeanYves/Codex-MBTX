use codex_mbtx_contract::{Arguments, Batch, Progress, RunArguments, Terminal, reply};
use serde_json::json;

fn frame(event: serde_json::Value) -> String {
    format!("{event}\n")
}

#[test]
fn request_preserves_literal_data_and_never_selects_an_executable() {
    let Arguments::Run(args) = serde_json::from_value::<Arguments>(json!({
        "op":"run", "source":"fn main { println(42) }",
        "args":["", "--help", "$(touch /tmp/no)", "a b", "'\"\n"],
        "background":true
    }))
    .unwrap() else {
        panic!()
    };
    let request: serde_json::Value =
        serde_json::from_str(&args.request("call", "/workspace/a b").unwrap()).unwrap();
    assert_eq!(
        request,
        json!({
            "op":"run","id":"call","source":"fn main { println(42) }",
            "args":args.args,"cwd":"/workspace/a b","background":false,"timeout_ms":30000
        })
    );
    for invalid in [
        json!({"op":"run","source":"x","executable":"/tmp/evil"}),
        json!({"op":"run","source":"x","timeout_ms":1.5}),
        json!({"op":"run","source":"x","timeout_ms":-1}),
        json!({"op":"job_stop","job_id":"job","source":"x"}),
    ] {
        assert!(serde_json::from_value::<Arguments>(invalid).is_err());
    }
}

#[test]
fn request_rejects_ambiguous_scripts_invalid_bounds_and_nul() {
    for invalid in [
        json!({}),
        json!({"source":" "}),
        json!({"source":"x","script_path":"x.mbtx"}),
        json!({"script_path":"x.sh"}),
        json!({"source":"x","args":["\u{0000}"]}),
        json!({"source":"x","timeout_ms":0}),
        json!({"source":"x","timeout_ms":600001}),
        json!({"source":"x".repeat(32768)}),
    ] {
        let args: RunArguments = serde_json::from_value(invalid).unwrap();
        assert!(args.request("call", "/work").is_err());
    }
}

#[test]
fn arbitrary_byte_splits_preserve_utf8_streams_and_terminal_status() {
    let transcript = [
        frame(json!({"event":"started","id":"call","job_id":"job-1"})),
        frame(json!({"event":"stdout","id":"call","job_id":"job-1","seq":0,"data":"\u{4e2d}\u{6587}"})),
        frame(json!({"event":"stderr","id":"call","job_id":"job-1","seq":1,"data":"error\n"})),
        frame(json!({"event":"completed","id":"call","job_id":"job-1","exit_code":7,"duration_ms":12})),
    ].concat();
    for split in 0..=transcript.len() {
        let mut progress = Progress::new("call".into());
        let first = progress.ingest(&transcript.as_bytes()[..split]).unwrap();
        let second = progress.ingest(&transcript.as_bytes()[split..]).unwrap();
        progress.finish(0).unwrap();
        assert_eq!(first.stdout + &second.stdout, "\u{4e2d}\u{6587}");
        assert_eq!(first.stderr + &second.stderr, "error\n");
        assert_eq!(
            progress.terminal,
            Some(Terminal::Completed {
                exit_code: 7,
                duration_ms: 12
            })
        );
    }
}

#[test]
fn corrupt_or_incomplete_lifecycles_fail_closed() {
    for event in [
        json!({"event":"stdout","id":"call","job_id":"job-1","seq":1,"data":"gap"}),
        json!({"event":"completed","id":"other","job_id":"job-1","exit_code":0,"duration_ms":1}),
        json!({"event":"started","id":"call","job_id":"job-1"}),
        json!({"event":"job_output","id":"call","job_id":"job-1"}),
    ] {
        let mut progress = Progress::new("call".into());
        progress
            .ingest(frame(json!({"event":"started","id":"call","job_id":"job-1"})).as_bytes())
            .unwrap();
        assert!(progress.ingest(frame(event).as_bytes()).is_err());
    }
    let mut progress = Progress::new("call".into());
    assert!(progress.finish(0).is_err());
    assert!(progress.ingest(b"\xff\n").is_err());
    let mut progress = Progress::new("call".into());
    assert!(progress.ingest(&vec![b'x'; 1024 * 1024 + 1]).is_err());
}

#[test]
fn output_bound_counts_json_escaping_without_breaking_json() {
    let batch = Batch {
        stdout: "\u{0000}".repeat(8192),
        stderr: String::new(),
        omitted_bytes: 0,
    };
    let value = reply(Some("mbtx-1"), None, batch);
    assert!(value.to_string().len() <= 8000);
    assert!(value["omitted_bytes"].as_u64().unwrap() > 0);
    assert_eq!(value["state"], "running");
}
