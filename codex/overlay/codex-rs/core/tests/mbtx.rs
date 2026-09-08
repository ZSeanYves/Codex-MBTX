#![allow(clippy::expect_used)]

use anyhow::Result;
use core_test_support::responses::ev_assistant_message;
use core_test_support::responses::ev_completed;
use core_test_support::responses::ev_function_call;
use core_test_support::responses::ev_response_created;
use core_test_support::responses::mount_sse_sequence;
use core_test_support::responses::sse;
use core_test_support::test_codex::TestCodexHarness;
use core_test_support::test_codex::test_codex;
use pretty_assertions::assert_eq;
use serde_json::Value;
use serde_json::json;

fn fixture(mode: &str) -> Vec<String> {
    vec![
        std::env::var("CODEX_MBTX_TEST_RUNNER").expect("run through scripts/m3_test.mbtx"),
        mode.into(),
    ]
}

async fn harness(command: Option<Vec<String>>) -> Result<TestCodexHarness> {
    TestCodexHarness::with_auto_env_builder(test_codex().with_model("gpt-5.4").with_config(
        move |config| {
            config.mbtx_command = command;
        },
    ))
    .await
}

async fn call(harness: &TestCodexHarness, id: &str, name: &str, args: Value) -> Result<String> {
    mount_sse_sequence(
        harness.server(),
        vec![
            sse(vec![
                ev_response_created(&format!("start-{id}")),
                ev_function_call(id, name, &args.to_string()),
                ev_completed(&format!("start-{id}")),
            ]),
            sse(vec![
                ev_assistant_message(&format!("msg-{id}"), "done"),
                ev_completed(&format!("end-{id}")),
            ]),
        ],
    )
    .await;
    harness.submit("Execute the requested tool.").await?;
    Ok(harness.function_call_stdout(id).await)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn mbtx_mock_model_reaches_runner_and_shell_still_works() -> Result<()> {
    let harness = harness(Some(fixture("echo"))).await?;
    let args = json!({"op":"run","source":"fn main { println(42) }","args":["", "--", "$(touch marker)", "a b", "'\"\n"]});
    let output: Value =
        serde_json::from_str(&call(&harness, "mbtx-echo", "mbtx", args.clone()).await?)?;
    assert_eq!(output["state"], "completed");
    assert_eq!(output["exit_code"], 0);
    let received: Value = serde_json::from_str(output["stdout"].as_str().unwrap())?;
    assert_eq!(received["source"], args["source"]);
    assert_eq!(received["args"], args["args"]);
    assert_eq!(received["cwd"], harness.cwd().to_string_lossy().as_ref());
    assert_eq!(output["stderr"], "diagnostic\n");
    assert!(!harness.path("marker").exists());
    let shell = call(
        &harness,
        "shell-fallback",
        "exec_command",
        json!({"cmd":"printf shell-ok","max_output_tokens":1000}),
    )
    .await?;
    assert!(shell.contains("shell-ok"));
    let requests = harness.request_bodies().await;
    let tools = requests[0]["tools"].to_string();
    assert!(tools.contains("\"mbtx\"") && tools.contains("\"exec_command\""));
    harness.test().codex.shutdown_and_wait().await?;
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn mbtx_background_poll_stop_and_session_ownership() -> Result<()> {
    let harness = harness(Some(fixture("sleep"))).await?;
    let start: Value = serde_json::from_str(
        &call(
            &harness,
            "start-job",
            "mbtx",
            json!({"op":"run","source":"ignored","background":true}),
        )
        .await?,
    )?;
    assert_eq!(start["state"], "running");
    assert_eq!(start["stdout"], "ready");
    let id = start["job_id"].as_str().unwrap();
    let unknown = call(
        &harness,
        "unknown",
        "mbtx",
        json!({"op":"job_stop","job_id":"mbtx-not-owned"}),
    )
    .await?;
    assert!(unknown.contains("unknown or evicted"));
    let poll: Value = serde_json::from_str(
        &call(
            &harness,
            "poll",
            "mbtx",
            json!({"op":"job_output","job_id":id}),
        )
        .await?,
    )?;
    assert_eq!(poll["state"], "running");
    assert_eq!(poll["stdout"], "");
    let stopped: Value = serde_json::from_str(
        &call(
            &harness,
            "stop",
            "mbtx",
            json!({"op":"job_stop","job_id":id}),
        )
        .await?,
    )?;
    assert_eq!(stopped["state"], "stopped");
    let repeated: Value = serde_json::from_str(
        &call(
            &harness,
            "stop-again",
            "mbtx",
            json!({"op":"job_stop","job_id":id}),
        )
        .await?,
    )?;
    assert_eq!(repeated, stopped);
    harness.test().codex.shutdown_and_wait().await?;
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn mbtx_runner_failures_are_not_reported_as_success() -> Result<()> {
    for (mode, expected) in [
        ("malformed", "invalid runner JSONL"),
        ("missing_terminal", "without a complete terminal"),
        ("wrong_id", "does not match"),
    ] {
        let harness = harness(Some(fixture(mode))).await?;
        let output = call(
            &harness,
            mode,
            "mbtx",
            json!({"op":"run","source":"ignored"}),
        )
        .await?;
        assert!(output.contains(expected), "{mode}: {output}");
        harness.test().codex.shutdown_and_wait().await?;
    }
    for (mode, state) in [("failure", "completed"), ("timeout", "failed")] {
        let harness = harness(Some(fixture(mode))).await?;
        let output: Value = serde_json::from_str(
            &call(
                &harness,
                mode,
                "mbtx",
                json!({"op":"run","source":"ignored"}),
            )
            .await?,
        )?;
        assert_eq!(output["state"], state);
        if mode == "failure" {
            assert_eq!(output["exit_code"], 7);
        } else {
            assert_eq!(output["error"]["kind"], "timeout");
        }
        harness.test().codex.shutdown_and_wait().await?;
    }
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn mbtx_disabled_by_default_and_invalid_input_never_launches() -> Result<()> {
    let disabled = harness(None).await?;
    let output = call(
        &disabled,
        "disabled",
        "mbtx",
        json!({"op":"run","source":"ignored"}),
    )
    .await?;
    assert!(output.contains("unsupported") || output.contains("not found"));
    let tools = disabled.request_bodies().await[0]["tools"].to_string();
    assert!(!tools.contains("\"mbtx\""));
    disabled.test().codex.shutdown_and_wait().await?;
    let directory = tempfile::tempdir()?;
    let marker = directory.path().join("spawned");
    let harness = harness(Some(fixture(&format!("record:{}", marker.display())))).await?;
    for (index, args) in [
        json!({"op":"run","source":"x","script_path":"x.mbtx"}),
        json!({"op":"run","source":"x","timeout_ms":0}),
        json!({"op":"run","source":"x","program":"/tmp/other"}),
        json!({"op":"run","script_path":"x.sh"}),
    ]
    .into_iter()
    .enumerate()
    {
        let output = call(&harness, &format!("invalid-{index}"), "mbtx", args).await?;
        assert!(!output.contains("\"state\":\"completed\""));
    }
    assert!(!marker.exists());
    harness.test().codex.shutdown_and_wait().await?;
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn mbtx_real_runner_under_mock_model() -> Result<()> {
    let command: Vec<String> = serde_json::from_str(
        &std::env::var("CODEX_MBTX_REAL_COMMAND").expect("run through scripts/m3_test.mbtx"),
    )?;
    let harness = harness(Some(command)).await?;
    let output: Value = serde_json::from_str(
        &call(
            &harness,
            "real",
            "mbtx",
            json!({
                "op":"run", "source":"fn main { println(42) }"
            }),
        )
        .await?,
    )?;
    assert_eq!(output["state"], "completed");
    assert_eq!(output["exit_code"], 0);
    assert_eq!(output["stdout"], "42\n");
    harness.test().codex.shutdown_and_wait().await?;
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn mbtx_approval_denial_prevents_spawn_and_reviews_exact_request() -> Result<()> {
    use codex_core::TurnInputRequest;
    use codex_protocol::protocol::AskForApproval;
    use codex_protocol::protocol::EventMsg;
    use codex_protocol::protocol::Op;
    use codex_protocol::protocol::ReviewDecision;
    use codex_protocol::protocol::SandboxPolicy;
    use codex_protocol::protocol::ThreadSettingsOverrides;
    use codex_protocol::user_input::UserInput;
    use core_test_support::wait_for_event;

    let directory = tempfile::tempdir()?;
    let marker = directory.path().join("unapproved");
    let command = fixture(&format!("record:{}", marker.display()));
    let harness = harness(Some(command.clone())).await?;
    let args = json!({"op":"run","source":"fn main { println(42) }","args":["literal;data"]});
    mount_sse_sequence(
        harness.server(),
        vec![
            sse(vec![
                ev_response_created("approval"),
                ev_function_call("approve", "mbtx", &args.to_string()),
                ev_completed("approval"),
            ]),
            sse(vec![
                ev_assistant_message("done", "denied"),
                ev_completed("after-denial"),
            ]),
        ],
    )
    .await;
    harness
        .test()
        .codex
        .start_or_steer_turn(
            TurnInputRequest::user_input(vec![UserInput::Text {
                text: "run MBTX".into(),
                text_elements: vec![],
            }])
            .with_thread_settings(ThreadSettingsOverrides {
                approval_policy: Some(AskForApproval::UnlessTrusted),
                sandbox_policy: Some(SandboxPolicy::ReadOnly {
                    network_access: false,
                }),
                ..Default::default()
            }),
        )
        .await?;
    let mut approvals = 0;
    loop {
        match wait_for_event(&harness.test().codex, |_| true).await {
            EventMsg::ExecApprovalRequest(approval) => {
                approvals += 1;
                assert!(!marker.exists());
                assert_eq!(&approval.command[..command.len()], command.as_slice());
                assert_eq!(approval.command[command.len()], "--request");
                let request: Value = serde_json::from_str(approval.command.last().unwrap())?;
                assert_eq!(request["source"], args["source"]);
                assert_eq!(request["args"], args["args"]);
                harness
                    .test()
                    .codex
                    .submit(Op::ExecApproval {
                        id: approval.effective_approval_id(),
                        turn_id: Some(approval.turn_id),
                        decision: ReviewDecision::denied("denied by integration test"),
                    })
                    .await?;
            }
            EventMsg::TurnComplete(_) => break,
            _ => {}
        }
    }
    assert_eq!(approvals, 1);
    assert!(!marker.exists());
    harness.test().codex.shutdown_and_wait().await?;
    Ok(())
}

#[test_case::test_case(false; "runner_can_execute")]
#[test_case::test_case(true; "workspace_write_is_denied")]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn mbtx_read_only_sandbox_enforces_policy(attempt_write: bool) -> Result<()> {
    use codex_protocol::models::PermissionProfile;
    let builder = test_codex()
        .with_model("gpt-5.4")
        .with_config(move |config| {
            config.mbtx_command = Some(if attempt_write {
                fixture(&format!("record:{}", config.cwd.join("blocked").display()))
            } else {
                fixture("echo")
            });
        });
    let harness = TestCodexHarness::with_auto_env_builder(builder).await?;
    mount_sse_sequence(
        harness.server(),
        vec![
            sse(vec![
                ev_response_created("sandbox"),
                ev_function_call(
                    "sandboxed",
                    "mbtx",
                    &json!({"op":"run","source":"ignored"}).to_string(),
                ),
                ev_completed("sandbox"),
            ]),
            sse(vec![
                ev_assistant_message("done", "blocked"),
                ev_completed("after-sandbox"),
            ]),
        ],
    )
    .await;
    harness
        .submit_with_permission_profile(
            "Run under read-only policy",
            PermissionProfile::read_only(),
        )
        .await?;
    assert!(!harness.path("blocked").exists());
    let output = harness.function_call_stdout("sandboxed").await;
    assert_eq!(
        output.contains("\"state\":\"completed\""),
        !attempt_write,
        "{output}"
    );
    harness.test().codex.shutdown_and_wait().await?;
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn mbtx_host_deadline_reaps_a_runner_that_ignores_its_timeout() -> Result<()> {
    let directory = tempfile::tempdir()?;
    let marker = directory.path().join("pid");
    let harness = harness(Some(fixture(&format!("sleep:{}", marker.display())))).await?;
    let output = call(
        &harness,
        "host-timeout",
        "mbtx",
        json!({"op":"run","source":"ignored","timeout_ms":10}),
    )
    .await?;
    assert!(output.contains("runner exited with code 124"), "{output}");
    let pid = std::fs::read_to_string(&marker)?;
    core_test_support::process::wait_for_process_exit(&pid).await?;
    harness.test().codex.shutdown_and_wait().await?;
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn mbtx_background_deadline_is_not_blocked_by_a_foreground_call() -> Result<()> {
    let directory = tempfile::tempdir()?;
    let marker = directory.path().join("pid");
    let harness = harness(Some(fixture(&format!("sleep:{}", marker.display())))).await?;
    let started: Value = serde_json::from_str(
        &call(
            &harness,
            "bg-timeout",
            "mbtx",
            json!({
                "op":"run","source":"ignored","timeout_ms":1500,"background":true
            }),
        )
        .await?,
    )?;
    let pid = std::fs::read_to_string(&marker)?;
    let (foreground, reaped) = tokio::join!(
        call(
            &harness,
            "fg-during-bg",
            "mbtx",
            json!({"op":"run","source":"ignored","timeout_ms":6000})
        ),
        async {
            tokio::time::sleep(std::time::Duration::from_secs(4)).await;
            core_test_support::process::wait_for_process_exit(&pid).await
        },
    );
    reaped?;
    assert!(foreground?.contains("runner exited with code 124"));
    let output: Value = serde_json::from_str(
        &call(
            &harness,
            "bg-timed",
            "mbtx",
            json!({
                "op":"job_output","job_id":started["job_id"]
            }),
        )
        .await?,
    )?;
    assert_eq!(output["state"], "failed");
    assert_eq!(output["error"]["kind"], "runner_timeout");
    harness.test().codex.shutdown_and_wait().await?;
    Ok(())
}

#[test_case::test_case(false; "foreground")]
#[test_case::test_case(true; "background_startup")]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn mbtx_turn_interrupt_reaps_the_starting_runner(background: bool) -> Result<()> {
    use codex_core::TurnInputRequest;
    use codex_protocol::protocol::AskForApproval;
    use codex_protocol::protocol::EventMsg;
    use codex_protocol::protocol::Op;
    use codex_protocol::protocol::SandboxPolicy;
    use codex_protocol::protocol::ThreadSettingsOverrides;
    use codex_protocol::user_input::UserInput;
    use core_test_support::process::wait_for_pid_file;
    use core_test_support::process::wait_for_process_exit;
    use core_test_support::wait_for_event;

    let directory = tempfile::tempdir()?;
    let marker = directory.path().join("pid");
    let harness = harness(Some(fixture(&format!("sleep:{}", marker.display())))).await?;
    mount_sse_sequence(
        harness.server(),
        vec![sse(vec![
            ev_response_created("interrupt"),
            ev_function_call(
                "interrupt-runner",
                "mbtx",
                &json!({"op":"run","source":"ignored","background":background}).to_string(),
            ),
            ev_completed("interrupt"),
        ])],
    )
    .await;
    harness
        .test()
        .codex
        .start_or_steer_turn(
            TurnInputRequest::user_input(vec![UserInput::Text {
                text: "run MBTX".into(),
                text_elements: vec![],
            }])
            .with_thread_settings(ThreadSettingsOverrides {
                approval_policy: Some(AskForApproval::Never),
                sandbox_policy: Some(SandboxPolicy::DangerFullAccess),
                ..Default::default()
            }),
        )
        .await?;
    let pid = wait_for_pid_file(&marker).await?;
    harness.test().codex.submit(Op::Interrupt).await?;
    wait_for_event(&harness.test().codex, |event| {
        matches!(event, EventMsg::TurnAborted(_))
    })
    .await;
    wait_for_process_exit(&pid).await?;
    harness.test().codex.shutdown_and_wait().await?;
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn mbtx_session_shutdown_reaps_background_runner() -> Result<()> {
    let directory = tempfile::tempdir()?;
    let marker = directory.path().join("pid");
    let harness = harness(Some(fixture(&format!("sleep:{}", marker.display())))).await?;
    let started: Value = serde_json::from_str(
        &call(
            &harness,
            "shutdown",
            "mbtx",
            json!({
                "op":"run","source":"ignored","background":true
            }),
        )
        .await?,
    )?;
    assert_eq!(started["state"], "running");
    let pid = std::fs::read_to_string(&marker)?;
    harness.test().codex.shutdown_and_wait().await?;
    core_test_support::process::wait_for_process_exit(&pid).await?;
    Ok(())
}
