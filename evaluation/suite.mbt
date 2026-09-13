///|
fn shell_quote(text : String) -> String {
  "'" + text.replace_all(old="'", new="'\\''") + "'"
}

///|
fn exec_step(
  command : String,
  tty? : Bool = false,
  yield_ms? : Int = 10000,
  max_output? : Int = 25000,
) -> Json {
  {
    "tool": "exec_command",
    "arguments": {
      "cmd": command.to_json(),
      "tty": tty.to_json(),
      "yield_time_ms": yield_ms.to_json(),
      "max_output_tokens": max_output.to_json(),
    },
  }
}

///|
fn write_step(chars : String, yield_ms? : Int = 1000) -> Json {
  {
    "tool": "write_stdin",
    "arguments": {
      "session_id": "$session",
      "chars": chars.to_json(),
      "yield_time_ms": yield_ms.to_json(),
      "max_output_tokens": 25000,
    },
  }
}

///|
fn task(
  id : String,
  category : String,
  steps : Array[Json],
  exits : Array[Int],
  check : String,
  control? : String = "none",
  control_mode? : String = "",
) -> Json {
  {
    "id": id.to_json(),
    "category": category.to_json(),
    "steps": steps.to_json(),
    "expected_exits": exits.to_json(),
    "check": check.to_json(),
    "control": control.to_json(),
    "control_mode": (if control_mode == "" { check } else { control_mode }).to_json(),
    "timeout_ms": 300000,
    "code_mode_program": code_mode_program(steps).to_json(),
  }
}

///|
fn code_mode_program(steps : Array[Json]) -> String {
  let pieces = ["let sessionId;\n"]
  for index, step in steps {
    let arguments = field(step, "arguments")
    let name = text_field(step, "tool")
    let call_args = if field(arguments, "session_id") == "$session" {
      "{..." + arguments.stringify() + ",session_id:sessionId}"
    } else {
      arguments.stringify()
    }
    let result = "r" + index.to_string()
    pieces.push(
      "const " +
      result +
      " = await tools." +
      name +
      "(" +
      call_args +
      ");\ntext(" +
      result +
      ");\nif (" +
      result +
      ".session_id != null) sessionId = " +
      result +
      ".session_id;\n",
    )
  }
  pieces.join("")
}

///|
pub fn suite(fixture : String) -> Json {
  let f = shell_quote(fixture)
  let resource_steps = []
  for _ in 0..<6 {
    resource_steps.append([
      exec_step(f + " interrupt", yield_ms=1000),
      write_step("\u0003"),
      exec_step(f + " small"),
    ])
  }
  let simple = (id : String, category : String, mode : String) => {
    task(id, category, [exec_step(f + " " + mode)], [0], mode)
  }
  let tasks = [
    task(
      "argv_empty_unicode",
      "argv",
      [exec_step(f + " argv '' 'a b' '\u4e2d\u6587'")],
      [0],
      "argv_empty_unicode",
    ),
    task(
      "argv_metacharacters",
      "argv",
      [
        exec_step(
          f +
          " argv " +
          ["'\"\\", "$() ${HOME}; * ? | & < >", "line1\nline2"]
          .map(shell_quote)
          .join(" "),
        ),
      ],
      [0],
      "argv_metacharacters",
    ),
    task(
      "argv_long",
      "argv",
      [
        exec_step(
          f +
          " argv " +
          shell_quote("abcd".repeat(1024)) +
          " " +
          Array::makei(128, i => i.to_string()).join(" "),
        ),
      ],
      [0],
      "argv_long",
    ),
    task(
      "stdin_eof",
      "stdin",
      [exec_step(f + " stdin </dev/null")],
      [0],
      "stdin_eof",
    ),
    task(
      "stdin_pty_utf8",
      "stdin",
      [
        exec_step(f + " stdin", tty=true, yield_ms=1000),
        write_step("\u7b2c\u4e00\u884c\n"),
        write_step("\u6700\u540e\u4e00\u884c\n"),
        write_step("\u0004"),
      ],
      [0],
      "stdin_pty_utf8",
    ),
    task(
      "stdin_backpressure",
      "stdin",
      [exec_step(f + " emit-input | " + f + " stdin")],
      [0],
      "stdin_backpressure",
    ),
    task(
      "cwd_symlink",
      "environment",
      [exec_step("cd 'space \u8def\u5f84/link' && " + f + " environment")],
      [0],
      "cwd_symlink",
    ),
    task(
      "environment_empty_unset",
      "environment",
      [exec_step("env -u MBTX_ABSENT MBTX_EMPTY= " + f + " environment")],
      [0],
      "environment_empty_unset",
    ),
    simple("environment_path_login", "environment", "environment"),
    simple("output_split_large", "output", "output-large"),
    simple("output_interleaved", "output", "output-interleaved"),
    task(
      "output_truncation_tail",
      "output",
      [exec_step(f + " output-tail", max_output=1000)],
      [0],
      "output-tail",
    ),
    task("exit_nonzero", "errors", [exec_step(f + " exit 7")], [7], "exit"),
    task(
      "exec_missing_permission",
      "errors",
      [exec_step("./absent-program"), exec_step("./not-executable")],
      [127, 126],
      "exec_errors",
    ),
    task(
      "exit_code_vs_signal",
      "errors",
      [exec_step(f + " exit 143"), exec_step(f + " self-term")],
      [143, 143],
      "exit_signal",
    ),
    task(
      "session_poll",
      "background",
      [exec_step(f + " session", yield_ms=1000), write_step("", yield_ms=10000)],
      [0],
      "session",
    ),
    task(
      "descendant_cancel",
      "background",
      [exec_step(f + " descendants", yield_ms=1000), write_step("\u0003")],
      [130],
      "descendants",
    ),
    simple("descendant_pipe_tail", "background", "pipe-tail"),
    task(
      "cancel_ctrl_c",
      "cancellation",
      [
        exec_step(f + " interrupt", tty=true, yield_ms=1000),
        write_step("\u0003"),
      ],
      [130],
      "interrupt",
    ),
    task(
      "cancel_sigterm",
      "cancellation",
      [exec_step(f + " term", yield_ms=1000), write_step("", yield_ms=10000)],
      [143],
      "term",
      control="term",
    ),
    task(
      "cancel_sigkill",
      "cancellation",
      [
        exec_step(f + " ignore-term", yield_ms=1000),
        write_step("", yield_ms=10000),
      ],
      [137],
      "ignore-term",
      control="term-kill",
    ),
    task(
      "session_sequence",
      "recovery",
      Array::makei(10, _ => exec_step(f + " small")),
      Array::make(10, 0),
      "sequence",
    ),
    task(
      "session_failure_recovery",
      "recovery",
      [exec_step(f + " exit 7"), exec_step(f + " small")],
      [7, 0],
      "recovery",
    ),
    task(
      "session_cancel_resources",
      "recovery",
      resource_steps,
      Array::makei(12, i => if i % 2 == 0 { 130 } else { 0 }),
      "resources",
    ),
  ]
  {
    "schema_version": 2,
    "tasks": tasks.to_json(),
    "offline_boundaries": [
      task(
        "detached_descendant",
        "background",
        [
          exec_step(f + " detached", yield_ms=1000),
          write_step("", yield_ms=10000),
        ],
        [143],
        "descendants",
        control="term",
        control_mode="detached",
      ),
    ].to_json(),
    "rounds": 2,
    "pairs_per_task_per_round": 4,
    "max_attempts_per_task_per_round": 6,
    "seed": 20260913,
    "startup_samples": 1000,
  }
}
