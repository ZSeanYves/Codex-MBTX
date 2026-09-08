mod output;
mod protocol;
mod spec;

use std::collections::VecDeque;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use codex_tools::ToolName;
use codex_tools::ToolSpec;
use codex_tools::UnifiedExecShellMode;
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::function_tool::FunctionCallError;
use crate::sandboxing::SandboxPermissions;
use crate::shell::ShellType;
use crate::tools::context::ExecCommandToolOutput;
use crate::tools::context::ToolInvocation;
use crate::tools::context::ToolOutput;
use crate::tools::context::ToolPayload;
use crate::tools::context::boxed_tool_output;
use crate::tools::handlers::apply_granted_turn_permissions;
use crate::tools::handlers::parse_arguments;
use crate::tools::registry::CoreToolRuntime;
use crate::tools::registry::ToolExecutor;
use crate::unified_exec::ExecCommandRequest;
use crate::unified_exec::UnifiedExecContext;
use crate::unified_exec::UnifiedExecProcessManager;
use crate::unified_exec::WriteStdinRequest;
use output::MbtxOutput;
use protocol::Arguments;
use protocol::Batch;
use protocol::Progress;
use protocol::Terminal;

pub(crate) struct MbtxHandler;

#[derive(Default)]
struct Jobs(Mutex<VecDeque<JobEntry>>);

struct JobEntry {
    id: String,
    state: Arc<Mutex<Job>>,
}

struct Job {
    id: String,
    process_id: i32,
    progress: Progress,
    started: Instant,
    finished: bool,
    last_reply: serde_json::Value,
    deadline: CancellationToken,
}

fn error(message: impl ToString) -> FunctionCallError {
    let message = message.to_string();
    FunctionCallError::RespondToModel(message[..message.floor_char_boundary(1500)].into())
}

impl ToolExecutor<ToolInvocation> for MbtxHandler {
    fn tool_name(&self) -> ToolName {
        ToolName::plain("mbtx")
    }

    fn spec(&self) -> ToolSpec {
        spec::spec()
    }

    fn handle<'a>(&'a self, invocation: ToolInvocation) -> codex_tools::ToolExecutorFuture<'a>
    where
        ToolInvocation: 'a,
    {
        Box::pin(self.handle_call(invocation))
    }
}

impl CoreToolRuntime for MbtxHandler {
    fn matches_kind(&self, payload: &ToolPayload) -> bool {
        matches!(payload, ToolPayload::Function { .. })
    }
}

impl MbtxHandler {
    #[expect(
        clippy::await_holding_invalid_type,
        reason = "Async locks serialize admission and draining process I/O; watchdogs lock only their own job"
    )]
    async fn handle_call(
        &self,
        invocation: ToolInvocation,
    ) -> Result<Box<dyn ToolOutput>, FunctionCallError> {
        let ToolPayload::Function { arguments } = &invocation.payload else {
            return Err(error("mbtx expects function arguments"));
        };
        let arguments: Arguments = parse_arguments(arguments)?;
        let stop = matches!(arguments, Arguments::JobStop { .. });
        let context = UnifiedExecContext::new(
            Arc::clone(&invocation.session),
            Arc::clone(&invocation.step_context),
            invocation.cancellation_token.clone(),
            invocation.call_id.clone(),
        );
        let manager = &invocation.session.services.unified_exec_manager;
        let job_store = invocation
            .session
            .services
            .session_extension_data
            .get_or_init(Jobs::default);
        // Serialize calls to this backend, including admission and draining reads.
        let mut jobs = job_store.0.lock().await;
        match arguments {
            Arguments::Run(args) => {
                let mut unresolved = 0;
                for job in jobs.iter() {
                    unresolved += usize::from(!job.state.lock().await.finished);
                }
                if unresolved >= 4 {
                    return Err(error(
                        "MBTX has four unresolved jobs; poll or stop them before starting another",
                    ));
                }
                let environment = invocation
                    .step_context
                    .environments
                    .primary()
                    .ok_or_else(|| error("MBTX requires a local execution environment"))?;
                if environment.environment.is_remote() {
                    return Err(error(
                        "MBTX runner is configured for the local execution environment",
                    ));
                }
                let mut command = invocation.turn.config.mbtx_command.clone().ok_or_else(|| {
                    error("MBTX is disabled; configure mbtx_command in user config")
                })?;
                if command.is_empty()
                    || !Path::new(&command[0]).is_absolute()
                    || command.iter().any(|arg| arg.contains('\0'))
                {
                    return Err(error(
                        "mbtx_command must start with an absolute executable path and contain no NUL",
                    ));
                }
                let cwd = match args.cwd.as_deref() {
                    Some(cwd) if !cwd.trim().is_empty() => {
                        environment.cwd().join(cwd).map_err(error)?
                    }
                    Some(_) => return Err(error("cwd must not be empty")),
                    None => environment.cwd().clone(),
                };
                let native_cwd = cwd.to_abs_path().map_err(error)?;
                let request = args
                    .request(&invocation.call_id, &native_cwd.to_string_lossy())
                    .map_err(error)?;
                command.extend(["--request".into(), request]);
                let hook_command = serde_json::to_string(&command).map_err(error)?;
                let permissions = apply_granted_turn_permissions(
                    &invocation.session,
                    environment,
                    &cwd,
                    SandboxPermissions::UseDefault,
                    /*additional_permissions*/ None,
                )
                .await;
                let process_id = manager.allocate_process_id().await;
                let request = ExecCommandRequest {
                    command,
                    shell_type: ShellType::Sh,
                    hook_command,
                    process_id,
                    yield_time_ms: 1000,
                    max_output_tokens: Some(1000),
                    cwd,
                    sandbox_cwd: environment.cwd().clone(),
                    turn_environment: environment.clone(),
                    shell_mode: UnifiedExecShellMode::Direct,
                    network: invocation.turn.network.clone(),
                    tty: false,
                    sandbox_permissions: permissions.sandbox_permissions,
                    additional_permissions: permissions.additional_permissions,
                    additional_permissions_preapproved: permissions.permissions_preapproved,
                    justification: None,
                    prefix_rule: None,
                };
                let response = if args.background {
                    tokio::select! {
                        biased;
                        _ = context.cancellation_token.cancelled() => {
                            manager.terminate_process(process_id).await;
                            manager.release_process_id(process_id).await;
                            return Err(error("MBTX startup cancelled"));
                        }
                        response = manager.exec_command(request, &context) => response,
                    }
                } else {
                    UnifiedExecProcessManager::exec_command_to_completion(
                        request,
                        &context,
                        Duration::from_millis(args.timeout_ms + 2000),
                    )
                    .await
                }
                .map_err(error)?;
                // Unified exec measures execution after approval and process startup.
                let started = Instant::now()
                    .checked_sub(response.wall_time)
                    .unwrap_or_else(Instant::now);
                let mut job = Job {
                    id: format!("mbtx-{}", Uuid::new_v4()),
                    process_id,
                    progress: Progress::new(invocation.call_id),
                    started,
                    finished: false,
                    last_reply: serde_json::Value::Null,
                    deadline: CancellationToken::new(),
                };
                let result = job.accept(response).map_err(error);
                if result.is_err() {
                    manager.terminate_process(process_id).await;
                }
                let result = result?;
                let id = job.id.clone();
                let deadline = job.deadline.clone();
                let background = args.background && !job.finished;
                let job = Arc::new(Mutex::new(job));
                if background {
                    let session = Arc::downgrade(&invocation.session);
                    let job = Arc::downgrade(&job);
                    let timeout = Duration::from_millis(args.timeout_ms + 2000)
                        .saturating_sub(started.elapsed());
                    tokio::spawn(async move {
                        tokio::select! {
                            _ = deadline.cancelled() => return,
                            _ = tokio::time::sleep(timeout) => {},
                        }
                        if let (Some(session), Some(job)) = (session.upgrade(), job.upgrade()) {
                            // A foreground call may hold the registry lock for minutes.
                            let mut job = job.lock().await;
                            if !job.finished {
                                let manager = &session.services.unified_exec_manager;
                                if manager
                                    .list_processes()
                                    .await
                                    .iter()
                                    .any(|process| process.process_id == process_id.to_string())
                                {
                                    manager.terminate_process(process_id).await;
                                    job.fail("runner_timeout", "runner exceeded its host deadline");
                                }
                            }
                        }
                    });
                }
                if jobs.len() >= 32 {
                    let mut evict = None;
                    for (index, entry) in jobs.iter().enumerate() {
                        if entry.state.lock().await.finished {
                            evict = Some(index);
                            break;
                        }
                    }
                    if let Some(index) = evict {
                        jobs.remove(index);
                    }
                }
                jobs.push_back(JobEntry { id, state: job });
                Ok(boxed_tool_output(MbtxOutput(result)))
            }
            Arguments::JobOutput { job_id } | Arguments::JobStop { job_id } => {
                let entry = jobs
                    .iter()
                    .find(|job| job.id == job_id)
                    .ok_or_else(|| error("unknown or evicted MBTX job_id"))?;
                let mut job = entry.state.lock().await;
                if job.finished {
                    return Ok(boxed_tool_output(MbtxOutput(job.last_reply.clone())));
                }
                let response = manager
                    .write_stdin(
                        &context,
                        WriteStdinRequest {
                            process_id: job.process_id,
                            input: "",
                            yield_time_ms: 1000,
                            max_output_tokens: Some(1000),
                            truncation_policy: invocation
                                .turn
                                .model_info()
                                .truncation_policy
                                .into(),
                            interaction_event: None,
                        },
                    )
                    .await;
                let response = match response {
                    Ok(response) => response,
                    Err(failure) => {
                        manager.terminate_process(job.process_id).await;
                        job.fail("runner_error", &failure.to_string());
                        return Err(error(failure));
                    }
                };
                let result = job.accept(response).map_err(error);
                if result.is_err() {
                    manager.terminate_process(job.process_id).await;
                    job.fail("runner_error", "runner protocol failed");
                }
                let mut result = result?;
                if stop && !job.finished {
                    if !manager.terminate_process(job.process_id).await {
                        return Err(error("could not confirm MBTX job termination"));
                    }
                    job.finished = true;
                    job.deadline.cancel();
                    job.progress.terminal = Some(Terminal::Stopped {
                        duration_ms: job.started.elapsed().as_millis() as u64,
                    });
                    let batch = Batch {
                        stdout: result["stdout"].as_str().unwrap_or_default().into(),
                        stderr: result["stderr"].as_str().unwrap_or_default().into(),
                        omitted_bytes: result["omitted_bytes"].as_u64().unwrap_or_default()
                            as usize,
                    };
                    result = protocol::reply(Some(&job.id), job.progress.terminal.as_ref(), batch);
                    job.last_reply = result.clone();
                }
                Ok(boxed_tool_output(MbtxOutput(result)))
            }
        }
    }
}

impl Job {
    fn fail(&mut self, kind: &str, message: &str) {
        self.finished = true;
        self.deadline.cancel();
        self.progress.terminal = Some(Terminal::Failed {
            error: protocol::Failure {
                kind: kind.into(),
                message: message[..message.floor_char_boundary(512)].into(),
            },
        });
        self.last_reply = protocol::reply(
            Some(&self.id),
            self.progress.terminal.as_ref(),
            Batch::default(),
        );
    }

    fn accept(&mut self, response: ExecCommandToolOutput) -> Result<serde_json::Value, String> {
        if response.output_omitted_bytes.is_some() {
            return Err(
                "Codex output buffer overflowed; MBTX protocol cannot be reconstructed".into(),
            );
        }
        let batch = self.progress.ingest(&response.raw_output)?;
        if let Some(code) = response.exit_code {
            self.progress.finish(code)?;
            self.finished = true;
            self.deadline.cancel();
        }
        let result = protocol::reply(Some(&self.id), self.progress.terminal.as_ref(), batch);
        self.last_reply = result.clone();
        Ok(result)
    }
}

impl Drop for Job {
    fn drop(&mut self) {
        self.deadline.cancel();
    }
}
