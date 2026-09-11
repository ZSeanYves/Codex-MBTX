///|
/// Versioned evidence contracts for the M5 shell/Transparent comparison.
///
/// M4 remains intentionally unchanged. These types keep infrastructure
/// failures, unobserved backend use, and task outcomes separate so an online
/// relay outage cannot be mistaken for a backend regression.

///|
pub(all) struct M5Block {
  block_id : String
  task : String
  repetition : Int
  batch_id : String
  time_window : Int
  order : Array[String]
  relay_health : Json
  runs : Array[Json]
} derive(ToJson, FromJson, Debug, Eq)

///|
pub(all) struct M5Plan {
  schema_version : Int
  seed : Int
  mode : String
  planned_blocks : Int
  start_block : Int
  blocks : Array[M5Block]
} derive(ToJson, FromJson, Debug, Eq)

///|
pub(all) struct M5Attempt {
  attempt_id : String
  retry_index : Int
  http_status : Int?
  started_ms : Int
  first_byte_ms : Int?
  completed_ms : Int?
  stream_disconnected : Bool
  provider_error : Bool
  recovered : Bool
  input_tokens : Int
  cached_input_tokens : Int
  output_tokens : Int
  usage_complete : Bool
  error_class : String?
} derive(ToJson, FromJson, Debug, Eq)

///|
pub(all) struct M5AttemptSummary {
  attempts : Int
  retry_count : Int
  provider_errors : Int
  transport_errors : Int
  incomplete_attempts : Int
  recovered_attempts : Int
  missing_usage : Int
  input_tokens : Int
  cached_input_tokens : Int
  output_tokens : Int
  retry_input_tokens : Int
  retry_cached_input_tokens : Int
  retry_output_tokens : Int
  retry_elapsed_ms : Int
} derive(ToJson, FromJson, Debug, Eq)

///|
pub(all) struct M5RelayHealth {
  probes : Int
  successful_probes : Int
  provider_errors : Int
  transport_errors : Int
  max_consecutive_failures : Int
  first_byte_p95_ms : Int
  arm_failure_rate_basis_points : Int
  time_windows : Int
} derive(ToJson, FromJson, Debug, Eq)

///|
pub(all) struct M5RunObservation {
  run_id : String
  block_id : String
  task : String
  backend : String
  fixture_nonce : String
  started_ms : Int
  monotonic_start_ms : Int?
  monotonic_end_ms : Int?
  ended_ms : Int?
  first_response_ms : Int?
  first_tool_ms : Int?
  last_tool_ms : Int?
  turn_completed_ms : Int?
  process_launch_ms : Int?
  process_exit_ms : Int?
  stdout : String
  stderr : String
  exit_code : Int
  workspace_manifest : Array[String]
  attempts : Array[M5Attempt]
  tool_counts : Map[String, Int]
  observed_tool_catalog : String
  tool_catalog_consistent : Bool
  session_lifecycle : Json
  input_tokens : Int
  cached_input_tokens : Int
  output_tokens : Int
  relay_status : String
  observability : String
  backend_observation : String
  completed : Bool
  usage_complete : Bool
  correct_output : Bool
  inputs_preserved : Bool
  workspace_clean : Bool
  child_processes_clean : Bool
  receipt_valid : Bool
  trace_complete : Bool
  argv_cwd_valid : Bool
  approval_original_command : Bool
  workspace_diff : Json
  receipt : Json
  process_trace : Json
  failure_category : String
} derive(ToJson, FromJson, Debug, Eq)

///|
pub(all) struct M5PairSummary {
  count : Int
  shell_median_ms : Int
  transparent_median_ms : Int
  median_delta_ms : Int
  shell_wins : Int
  transparent_wins : Int
  ties : Int
} derive(ToJson, FromJson, Debug, Eq)

///|
pub(all) struct M5ReliabilitySummary {
  planned : Int
  relay_clean : Int
  successes : Int
  attributable_successes : Int
  provider_failures : Int
  transport_failures : Int
  missing_usage : Int
  unknown_backend_observations : Int
  backend_violations : Int
} derive(ToJson, FromJson, Debug, Eq)

///|
/// Normalized input accepted by the evidence CLI. Keeping this separate from
/// `M5RunObservation` lets callers classify an in-flight run before all timing
/// fields have been collected.
pub(all) struct M5ClassificationInput {
  relay_status : String
  observability : String
  backend_observation : String
  completed : Bool
  usage_complete : Bool
  correct_output : Bool
  inputs_preserved : Bool
  workspace_clean : Bool
  child_processes_clean : Bool
  receipt_valid : Bool
  trace_complete : Bool
  argv_cwd_valid : Bool
  approval_original_command : Bool
  timeout : Bool
} derive(ToJson, FromJson, Debug, Eq)

///|
pub(all) struct M5PairInput {
  shell_ms : Array[Int]
  transparent_ms : Array[Int]
} derive(ToJson, FromJson, Debug, Eq)

///|
pub(all) struct M5DecisionInput {
  semantic_zero_regression : Bool
  safety_compatibility_ok : Bool
  success_lower_bound_basis_points : Int
  p95_upper_bound_basis_points : Int
  key_task_regression : Bool
  evidence_complete : Bool
  backend_observable : Bool
  artifacts_rebuildable : Bool
} derive(ToJson, FromJson, Debug, Eq)

///|
pub(all) struct M5Decision {
  status : String
  reasons : Array[String]
} derive(ToJson, FromJson, Debug, Eq)

///|
pub(all) struct M5Report {
  schema_version : Int
  experiment_id : String
  mode : String
  implementation_sha : String
  binary_sha256 : String
  evaluator_binary_sha256 : String
  toolchain : String
  platform : String
  model : String
  reasoning_effort : String
  prompt_hash : String
  fixture_hash : String
  seed : Int
  planned_blocks : Int
  start_block : Int
  tool_catalog : String
  relay_health : M5RelayHealth
  blocks : Array[M5Block]
  runs : Array[M5RunObservation]
  aggregates : Json
  validity : Json
  decision : Json
} derive(ToJson, FromJson, Debug, Eq)

///|
pub(all) struct M5Task {
  id : String
  cohort : String
  difficulty : String
  prompt : String
  inputs : Map[String, String]
  expected : Json
  expected_stdout : String
  expected_stderr : String
  expected_exit_code : Int
  expected_manifest : Array[String]
  editable : String?
  background : Bool
  fixture_nonce : String
  receipt_path : String
  oracle : String
} derive(ToJson, FromJson, Debug, Eq)

///|
fn m5_expected_manifest(
  inputs : Map[String, String],
  include_result : Bool,
) -> Array[String] {
  let manifest : Array[String] = []
  for path, _ in inputs {
    manifest.push(path)
  }
  if include_result {
    manifest.push("result.json")
  }
  manifest.push(".m5/receipt.json")
  manifest.sort_by(m5_compare_code_units)
  manifest
}

///|
fn m5_compare_code_units(left : String, right : String) -> Int {
  let shared = if left.length() < right.length() {
    left.length()
  } else {
    right.length()
  }
  for index in 0..<shared {
    if left[index] != right[index] {
      return if left[index] < right[index] { -1 } else { 1 }
    }
  }
  left.length() - right.length()
}

///|
fn m5_expected_streams(case : Case) -> (String, String, Int) {
  match case.id {
    "host_background" | "background_job" => ("started\ncomplete\n", "", 0)
    "host_large_output" =>
      (String::from_iter(Array::make(16_384, 'x').iter()), "", 0)
    // Aggregate the diagnostic from the first failed attempt with the final
    // successful invocation of the recovery fixture.
    "host_exit_recovery" => ("", "confirmation required\n", 0)
    _ => ("", "", 0)
  }
}

///|
fn m5_task_from_case(
  case : Case,
  difficulty : String,
  nonce : String,
) -> M5Task {
  let (expected_stdout, expected_stderr, expected_exit_code) = m5_expected_streams(
    case,
  )
  {
    id: case.id,
    cohort: if case.workload == "process" {
      "process"
    } else {
      "script-capability"
    },
    difficulty,
    prompt: if case.id == "host_background" {
      "Use exec_command to invoke exactly `python3 worker.py` with `yield_time_ms` 250 and `tty` false. It must return a running session ID. Poll that same session with write_stdin using empty `chars` until the command exits and result.json exists. Do not use shell `&`, nohup, redirection, a wrapper shell, another python3 command, or any temporary file. Do not modify worker.py or manually write result.json."
    } else if case.workload == "script" {
      "MoonBit script capability task. " + case.prompt
    } else {
      case.prompt
    },
    inputs: case.inputs,
    expected: case.expected,
    expected_stdout,
    expected_stderr,
    expected_exit_code,
    expected_manifest: m5_expected_manifest(case.inputs, true),
    editable: case.editable,
    background: case.background,
    fixture_nonce: nonce,
    receipt_path: ".m5/receipt.json",
    oracle: "result_json+workspace_manifest+process_trace",
  }
}

///|
/// The M5 suite is versioned separately from the historical M4 suite.
pub fn m5_process_suite() -> Array[M5Task] {
  let tasks : Array[M5Task] = []
  for case in suite() {
    if case.workload == "process" {
      let difficulty = if case.id == "host_large_output" ||
        case.id == "host_background" {
        "lifecycle"
      } else if case.id == "host_exit_recovery" {
        "recovery"
      } else {
        "semantics"
      }
      let nonce = if case.id == "host_background" {
        "m5-host_background-v2"
      } else {
        "m5-\{case.id}-v1"
      }
      tasks.push(m5_task_from_case(case, difficulty, nonce))
    }
  }
  tasks.push({
    id: "host_cwd_environment",
    cohort: "process",
    difficulty: "semantics",
    prompt: "Run cwd_env.py with python3 from the workspace. It must write result.json containing the exact current working directory and the M5_TASK_NONCE environment value. Do not modify cwd_env.py or manually write result.json.",
    inputs: {
      "cwd_env.py": "import json,os\nwith open('result.json','w',encoding='utf-8') as f: json.dump({'cwd':os.getcwd(),'nonce':os.environ.get('M5_TASK_NONCE')},f)\n",
    },
    expected: { "nonce": "m5-host_cwd_environment-v1" },
    expected_stdout: "",
    expected_stderr: "",
    expected_exit_code: 0,
    expected_manifest: [".m5/receipt.json", "cwd_env.py", "result.json"],
    editable: None,
    background: false,
    fixture_nonce: "m5-host_cwd_environment-v1",
    receipt_path: ".m5/receipt.json",
    oracle: "json_subset+workspace_manifest+process_trace",
  })
  tasks.push({
    id: "host_cancel_timeout",
    cohort: "process",
    difficulty: "lifecycle",
    prompt: "Use exec_command to invoke exactly `python3 sleeper.py` with `yield_time_ms` 250 and `tty` false. It must return a running session ID. First poll that same session once with write_stdin using empty `chars`; then stop that same session with exactly one write_stdin call whose `chars` is the single Ctrl-C byte `\\u0003`. Confirm the interrupt call reports that the session exited and result.json does not exist. Do not use shell `&`, nohup, redirection, kill, a wrapper shell, another python3 command, or any temporary file. Do not modify sleeper.py or create result.json.",
    inputs: {
      "sleeper.py": "import json,time\ntime.sleep(120)\nwith open('result.json','w',encoding='utf-8') as f: json.dump({'unexpected':'completed'},f)\n",
    },
    expected: { "stopped": true, "result_exists": false },
    expected_stdout: "",
    expected_stderr: "",
    // -1 is the fixture-level sentinel for a deliberately cancelled child.
    expected_exit_code: -1,
    expected_manifest: [".m5/receipt.json", "sleeper.py"],
    editable: None,
    background: true,
    fixture_nonce: "m5-host_cancel_timeout-v2",
    receipt_path: ".m5/receipt.json",
    oracle: "lifecycle_stop+workspace_manifest+process_trace",
  })
  tasks
}

///|
pub fn m5_script_suite() -> Array[M5Task] {
  suite()
  .filter(c => c.workload == "script")
  .map(c => m5_task_from_case(c, "moonbit", "m5-\{c.id}-v1"))
}

///|
pub fn m5_prompt(task : M5Task) -> String {
  let routing = "Use exec_command and write_stdin for command and script execution. You may use apply_patch for editing."
  "\{routing}\nWork only inside the task workspace. Do not inspect other home directories, evaluation infrastructure or answer keys. Do not use the network, delegate to another agent, or ask for human input. Complete and verify the task before your final response.\n\n\{task.prompt}"
}

///|
/// Validate the terminal process streams and exit status recorded by a runner.
/// Cancellation uses `-1` as the fixture-level sentinel because the concrete
/// signal/exit representation is platform-specific.
pub fn m5_streams_valid(
  task : M5Task,
  stdout : String,
  stderr : String,
  exit_code : Int,
) -> Bool {
  stdout == task.expected_stdout &&
  stderr == task.expected_stderr &&
  exit_code == task.expected_exit_code
}

///|
/// Require an exact file-only workspace manifest. The comparison is set-like
/// while the equal lengths reject duplicate paths in a malformed artifact.
pub fn m5_manifest_valid(task : M5Task, actual : Array[String]) -> Bool {
  actual.length() == task.expected_manifest.length() &&
  task.expected_manifest.all(path => actual.contains(path)) &&
  actual.all(path => task.expected_manifest.contains(path))
}

///|
/// Compare a task result using its declared oracle. `json_subset` is used for
/// fixtures whose result contains host-dependent fields such as an absolute
/// cwd; the expected object remains a strict allow-list of required fields.
pub fn m5_correct_output(task : M5Task, text : String) -> Bool {
  let actual = @json.parse(text) catch { _ => return false }
  if task.oracle.has_prefix("json_subset") {
    m5_json_subset(task.expected, actual)
  } else {
    actual == task.expected
  }
}

///|
/// A receipt is deliberately independent from result.json. It proves that
/// the fixture helper ran (or was intentionally stopped) and binds the record
/// to the nonce assigned by the planner.
pub fn m5_receipt_valid(task : M5Task, receipt : Json) -> Bool {
  match receipt {
    Object(fields) => {
      let task_ok = fields.get("task") == Some(task.id.to_json())
      let nonce_ok = fields.get("nonce") == Some(task.fixture_nonce.to_json())
      let helper_ok = match fields.get("helper") {
        Some(String(path)) => !path.is_empty()
        _ => false
      }
      let status_ok = match fields.get("status") {
        Some(String("completed" | "stopped")) => true
        _ => false
      }
      task_ok && nonce_ok && helper_ok && status_ok
    }
    _ => false
  }
}

///|
/// Cancellation fixtures intentionally have no result.json. Their oracle is
/// the absence of a late write plus a cleaned child process, not an exact JSON
/// payload.
pub fn m5_cancellation_valid(
  task : M5Task,
  result_exists : Bool,
  child_clean : Bool,
) -> Bool {
  task.id == "host_cancel_timeout" && !result_exists && child_clean
}

///|
fn m5_json_subset(expected : Json, actual : Json) -> Bool {
  match expected {
    Object(expected_fields) =>
      match actual {
        Object(actual_fields) => {
          for key, value in expected_fields {
            guard actual_fields.get(key) is Some(actual_value) else {
              return false
            }
            if !m5_json_subset(value, actual_value) {
              return false
            }
          }
          true
        }
        _ => false
      }
    Array(expected_items) =>
      match actual {
        Array(actual_items) =>
          if expected_items.length() != actual_items.length() {
            false
          } else {
            let mut ok = true
            for i in 0..<expected_items.length() {
              if !m5_json_subset(expected_items[i], actual_items[i]) {
                ok = false
              }
            }
            ok
          }
        _ => false
      }
    _ => expected == actual
  }
}

///|
fn positive_mod(value : Int, modulus : Int) -> Int {
  let remainder = value % modulus
  if remainder < 0 {
    remainder + modulus
  } else {
    remainder
  }
}

///|
fn orientation(seed : Int, repetition : Int, task_index : Int) -> Array[String] {
  // Alternate within each task across repetitions. The seed and task index
  // choose the starting arm, while an even formal repetition count is exactly
  // balanced AB/BA for every task.
  let value = seed + task_index + repetition
  if positive_mod(value, 2) == 0 {
    ["shell", "transparent"]
  } else {
    ["transparent", "shell"]
  }
}

///|
fn shuffle_task_indices(
  seed : Int,
  repetition : Int,
  count : Int,
) -> Array[Int] {
  let indices = Array::make(count, 0)
  for i in 0..<count {
    indices[i] = i
  }
  let mut state = positive_mod(seed + repetition * 7_919, 2_147_483_647).to_int64() +
    1L
  let mut i = count
  while i > 1 {
    state = state * 48_271L % 2_147_483_647L
    let j = (state % i.to_int64()).to_int()
    let temp = indices[i - 1]
    indices[i - 1] = indices[j]
    indices[j] = temp
    i -= 1
  }
  indices
}

///|
/// Build a balanced, reproducible paired plan.
///
/// `tasks` must be non-empty, `repetitions` must be positive, and a
/// continuation starts at a block boundary. The final block is intentionally
/// not silently accepted as an empty continuation.
pub fn m5_plan(
  mode : String,
  tasks : Array[String],
  repetitions : Int,
  seed : Int,
  start_block : Int,
) -> M5Plan raise {
  guard mode == "pilot" || mode == "formal" || mode == "script" else {
    fail("invalid M5 mode")
  }
  guard !tasks.is_empty() else { fail("M5 tasks must be non-empty") }
  guard repetitions > 0 else { fail("M5 repetitions must be positive") }
  let planned_blocks = tasks.length() * repetitions
  guard start_block >= 0 && start_block < planned_blocks else {
    fail("M5 start_block is outside the evaluation plan")
  }
  let blocks : Array[M5Block] = []
  let mut index = 0
  for repetition in 1..<=repetitions {
    // Formal repetitions are partitioned into three contiguous windows so a
    // remote continuation can collect one complete window at a time. Pilot
    // and script cohorts are single-batch diagnostics and do not pretend to
    // provide temporal replication.
    let time_window = if mode == "formal" {
      (repetition - 1) * 3 / repetitions + 1
    } else {
      1
    }
    let task_indices = shuffle_task_indices(seed, repetition, tasks.length())
    for position in 0..<tasks.length() {
      let task_index = task_indices[position]
      let task = tasks[task_index]
      if index >= start_block {
        let block_id = "m5-\{seed}-\{index}-\{task}-\{repetition}"
        blocks.push({
          block_id,
          task,
          repetition,
          batch_id: "batch-\{seed}-window-\{time_window}",
          time_window,
          order: orientation(seed, repetition, task_index),
          relay_health: Json::empty_object(),
          runs: [],
        })
      }
      index += 1
    }
  }
  { schema_version: 5, seed, mode, planned_blocks, start_block, blocks, }
}

///|
/// A formal online block is usable only when relay health was acceptable
/// before and during collection. Thresholds are deliberately centralized.
pub fn m5_relay_health_valid(health : M5RelayHealth) -> Bool {
  health.probes >= 10 &&
  health.successful_probes >= 9 &&
  health.provider_errors + health.transport_errors <= 1 &&
  health.max_consecutive_failures < 2 &&
  health.first_byte_p95_ms <= 15_000 &&
  health.time_windows >= 1
}

///|
pub fn m5_relay_batch_valid(health : M5RelayHealth) -> Bool {
  m5_relay_health_valid(health) &&
  health.probes > 0 &&
  health.arm_failure_rate_basis_points <= 200 &&
  (health.provider_errors + health.transport_errors) * 10_000 / health.probes <=
  500
}

///|
/// Classify the first consequential failure without converting an unknown
/// observation into a backend violation.
pub fn m5_failure_category(
  relay_status : String,
  observability : String,
  backend_observation : String,
  completed : Bool,
  usage_complete : Bool,
  correct_output : Bool,
  inputs_preserved : Bool,
  workspace_clean : Bool,
  child_processes_clean : Bool,
  receipt_valid : Bool,
  trace_complete : Bool,
  argv_cwd_valid : Bool,
  approval_original_command : Bool,
  timeout : Bool,
) -> String {
  if relay_status == "provider_error" || relay_status == "transport_error" {
    "infrastructure_unavailable"
  } else if timeout {
    "timeout"
  } else if observability == "unobserved" {
    "evidence_unobserved"
  } else if observability == "partial" {
    "evidence_partial"
  } else if backend_observation == "unknown" && !approval_original_command {
    "backend_observation_unknown"
  } else if backend_observation == "violation" || !approval_original_command {
    "backend_violation"
  } else if !completed {
    "agent_error"
  } else if !usage_complete {
    "missing_usage"
  } else if !inputs_preserved {
    "input_modified"
  } else if !workspace_clean ||
    !child_processes_clean ||
    !receipt_valid ||
    !trace_complete ||
    !argv_cwd_valid {
    "workspace_dirty"
  } else if !correct_output {
    "incorrect_output"
  } else {
    "none"
  }
}

///|
pub fn m5_percentile(values : Array[Int], percentile : Int) -> Int raise {
  guard !values.is_empty() else {
    fail("cannot calculate a percentile of no values")
  }
  guard percentile >= 0 && percentile <= 100 else {
    fail("percentile must be between 0 and 100")
  }
  let sorted = values.copy()
  sorted.sort()
  let rank = (sorted.length() * percentile + 99) / 100
  sorted[if rank == 0 { 0 } else { rank - 1 }]
}

///|
pub fn m5_pair_summary(
  shell_ms : Array[Int],
  transparent_ms : Array[Int],
) -> M5PairSummary raise {
  guard !shell_ms.is_empty() else { fail("paired samples must be non-empty") }
  guard shell_ms.length() == transparent_ms.length() else {
    fail("paired samples must have equal lengths")
  }
  let shell_median = m5_percentile(shell_ms, 50)
  let transparent_median = m5_percentile(transparent_ms, 50)
  let mut shell_wins = 0
  let mut transparent_wins = 0
  let mut ties = 0
  for i in 0..<shell_ms.length() {
    if shell_ms[i] < transparent_ms[i] {
      shell_wins += 1
    } else if transparent_ms[i] < shell_ms[i] {
      transparent_wins += 1
    } else {
      ties += 1
    }
  }
  {
    count: shell_ms.length(),
    shell_median_ms: shell_median,
    transparent_median_ms: transparent_median,
    median_delta_ms: transparent_median - shell_median,
    shell_wins,
    transparent_wins,
    ties,
  }
}

///|
pub fn m5_reliability_summary(
  observations : Array[M5RunObservation],
) -> M5ReliabilitySummary {
  let mut relay_clean = 0
  let mut successes = 0
  let mut provider_failures = 0
  let mut transport_failures = 0
  let mut missing_usage = 0
  let mut attributable_successes = 0
  let mut unknown_backend_observations = 0
  let mut backend_violations = 0
  for observation in observations {
    if observation.relay_status == "clean" {
      relay_clean += 1
    }
    if observation.failure_category == "none" {
      successes += 1
    }
    if observation.failure_category == "infrastructure_unavailable" {
      if observation.relay_status == "transport_error" {
        transport_failures += 1
      } else {
        provider_failures += 1
      }
    }
    if observation.failure_category == "missing_usage" {
      missing_usage += 1
    }
    if observation.backend_observation == "unknown" {
      unknown_backend_observations += 1
    } else if observation.backend_observation == "violation" {
      backend_violations += 1
    } else if observation.failure_category == "none" {
      attributable_successes += 1
    }
  }
  {
    planned: observations.length(),
    relay_clean,
    successes,
    attributable_successes,
    provider_failures,
    transport_failures,
    missing_usage,
    unknown_backend_observations,
    backend_violations,
  }
}

///|
pub fn m5_classify(input : M5ClassificationInput) -> String {
  m5_failure_category(
    input.relay_status,
    input.observability,
    input.backend_observation,
    input.completed,
    input.usage_complete,
    input.correct_output,
    input.inputs_preserved,
    input.workspace_clean,
    input.child_processes_clean,
    input.receipt_valid,
    input.trace_complete,
    input.argv_cwd_valid,
    input.approval_original_command,
    input.timeout,
  )
}

///|
pub fn m5_attempt_summary(attempts : Array[M5Attempt]) -> M5AttemptSummary {
  let mut provider_errors = 0
  let mut transport_errors = 0
  let mut incomplete_attempts = 0
  let mut recovered_attempts = 0
  let mut missing_usage = 0
  let mut input_tokens = 0
  let mut cached_input_tokens = 0
  let mut output_tokens = 0
  let mut retry_input_tokens = 0
  let mut retry_cached_input_tokens = 0
  let mut retry_output_tokens = 0
  let mut retry_elapsed_ms = 0
  for attempt in attempts {
    if attempt.provider_error ||
      (attempt.http_status is Some(status) && status >= 400) {
      provider_errors += 1
    }
    if attempt.stream_disconnected {
      transport_errors += 1
    }
    if attempt.completed_ms is None {
      incomplete_attempts += 1
    }
    if attempt.recovered {
      recovered_attempts += 1
    }
    if !attempt.usage_complete {
      missing_usage += 1
    }
    input_tokens += attempt.input_tokens
    cached_input_tokens += attempt.cached_input_tokens
    output_tokens += attempt.output_tokens
    if attempt.retry_index > 0 {
      retry_input_tokens += attempt.input_tokens
      retry_cached_input_tokens += attempt.cached_input_tokens
      retry_output_tokens += attempt.output_tokens
      if attempt.completed_ms is Some(completed) &&
        completed >= attempt.started_ms {
        retry_elapsed_ms += completed - attempt.started_ms
      }
    }
  }
  {
    attempts: attempts.length(),
    retry_count: attempts.filter(a => a.retry_index > 0).length(),
    provider_errors,
    transport_errors,
    incomplete_attempts,
    recovered_attempts,
    missing_usage,
    input_tokens,
    cached_input_tokens,
    output_tokens,
    retry_input_tokens,
    retry_cached_input_tokens,
    retry_output_tokens,
    retry_elapsed_ms,
  }
}

///|
/// Apply the preregistered default-backend gate. The confidence bounds are
/// supplied by the aggregator; this function deliberately refuses to infer a
/// GO decision from raw point estimates alone.
pub fn m5_decide(input : M5DecisionInput) -> M5Decision {
  let reasons : Array[String] = []
  if !input.semantic_zero_regression {
    reasons.push("deterministic semantic regression")
  }
  if !input.safety_compatibility_ok {
    reasons.push("approval/sandbox/argv/lifecycle compatibility regression")
  }
  if input.success_lower_bound_basis_points < -200 {
    reasons.push(
      "success-rate lower confidence bound is below -2 percentage points",
    )
  }
  if input.p95_upper_bound_basis_points > 500 {
    reasons.push("p95 latency upper confidence bound exceeds +5 percent")
  }
  if input.key_task_regression {
    reasons.push("a key process task regressed")
  }
  if !input.evidence_complete {
    reasons.push("evidence completeness is below the preregistered gate")
  }
  if !input.backend_observable {
    reasons.push("backend observability is below the preregistered gate")
  }
  if !input.artifacts_rebuildable {
    reasons.push("saved artifacts cannot rebuild the report")
  }
  let status = if !input.semantic_zero_regression ||
    !input.safety_compatibility_ok ||
    input.key_task_regression {
    "NO_GO"
  } else if reasons.is_empty() {
    "GO_DEFAULT"
  } else if input.evidence_complete &&
    input.backend_observable &&
    input.artifacts_rebuildable &&
    input.success_lower_bound_basis_points >= -200 &&
    input.p95_upper_bound_basis_points <= 500 {
    "OPT_IN_ONLY"
  } else {
    "INCONCLUSIVE"
  }
  { status, reasons, }
}
