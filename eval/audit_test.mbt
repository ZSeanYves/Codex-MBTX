///|
test "audit parses streamed usage and tool calls" {
  let audit = @eval.Audit::new()
  audit.sse(
    "data: {\"type\":\"response.output_item.done\",\"item\":{\"type\":\"function_call\",\"name\":\"mbtx\",\"call_id\":\"c1\",\"arguments\":\"{\\\"op\\\":\\\"run\\\"}\"}}\n\ndata: {\"type\":\"response.completed\",\"response\":{\"id\":\"r1\",\"model\":\"gpt-5.6-terra\",\"usage\":{\"input_tokens\":12,\"output_tokens\":8,\"input_tokens_details\":{\"cached_tokens\":3},\"output_tokens_details\":{\"reasoning_tokens\":5}},\"output\":[]}}\n\ndata: [DONE]\n\n",
  )
  assert_eq(audit.responses, 1)
  assert_eq(audit.usage_responses, 1)
  assert_eq(audit.input_tokens, 12)
  assert_eq(audit.cached_input_tokens, 3)
  assert_eq(audit.output_tokens, 8)
  assert_eq(audit.reasoning_output_tokens, 5)
  assert_eq(audit.tools.get("mbtx"), Some(1))
  assert_eq(audit.operations.get("run"), Some(1))
  assert_true(audit.models.contains("gpt-5.6-terra"))
}

///|
test "audit rejects shell fallback for mbtx backend" {
  let audit = @eval.Audit::new()
  audit.sse(
    "data: {\"type\":\"response.output_item.done\",\"item\":{\"type\":\"function_call\",\"name\":\"mbtx\",\"call_id\":\"c1\",\"arguments\":\"{\\\"op\\\":\\\"run\\\"}\"}}\n\ndata: {\"type\":\"response.output_item.done\",\"item\":{\"type\":\"function_call\",\"name\":\"exec_command\",\"call_id\":\"c2\",\"arguments\":\"{}\"}}\n\n",
  )
  assert_false(audit.backend_ok("mbtx", false))
  assert_false(audit.backend_ok("shell", false))
  assert_eq(
    @eval.failure_category(audit, "mbtx", false, true, true, false, 0),
    "agent_error",
  )
}

///|
fn completed_audit(tool : String) -> @eval.Audit {
  let audit = @eval.Audit::new()
  audit.jsonl("{\"type\":\"turn.completed\",\"usage\":{}}\n")
  let item : Json = {
    "type": "function_call",
    "name": tool.to_json(),
    "call_id": "c1",
    "arguments": "{\"op\":\"run\"}",
  }
  audit.event({
    "type": "response.completed",
    "response": {
      "id": "r1",
      "usage": { "input_tokens": 12, "output_tokens": 8 },
      "output": [item],
    },
  })
  audit
}

///|
test "correct output alone cannot hide backend misuse or changed inputs" {
  let audit = completed_audit("exec_command")
  assert_eq(
    @eval.failure_category(audit, "shell", false, true, true, false, 0),
    "none",
  )
  assert_eq(
    @eval.failure_category(audit, "mbtx", false, true, true, false, 0),
    "backend_violation",
  )
  assert_eq(
    @eval.failure_category(audit, "shell", false, true, false, false, 0),
    "input_modified",
  )
  assert_eq(
    @eval.failure_category(audit, "shell", false, false, true, false, 0),
    "incorrect_output",
  )
  assert_eq(
    @eval.failure_category(audit, "shell", false, true, true, true, -1),
    "timeout",
  )
}

///|
test "background execution requires polling" {
  let audit = completed_audit("mbtx")
  assert_false(audit.backend_ok("mbtx", true))
  audit.event({
    "type": "response.output_item.done",
    "item": {
      "type": "function_call",
      "name": "mbtx",
      "call_id": "c2",
      "arguments": "{\"op\":\"job_output\"}",
    },
  })
  assert_false(audit.backend_ok("mbtx", true))
  audit.event({
    "type": "response.output_item.done",
    "item": {
      "type": "function_call",
      "name": "mbtx",
      "call_id": "c3",
      "arguments": "{\"op\":\"run\",\"background\":true}",
    },
  })
  assert_true(audit.backend_ok("mbtx", true))
}

///|
test "SSE duplicate completed events do not double count usage" {
  let audit = completed_audit("exec_command")
  audit.sse(
    "data: {\"type\":\"response.completed\",\r\ndata: \"response\":{\"id\":\"r1\",\"usage\":{\"input_tokens\":12,\"output_tokens\":8}}}\r\n\r\n",
  )
  assert_eq(audit.input_tokens, 12)
  assert_eq(audit.responses, 1)
  assert_eq(audit.parse_errors, 0)
}

///|
test "missing and corrupt usage are visible" {
  let audit = completed_audit("exec_command")
  audit.event({ "type": "response.completed", "response": { "id": "r2" } })
  assert_eq(
    @eval.failure_category(audit, "shell", false, true, true, false, 0),
    "missing_usage",
  )
  audit.sse("data: {bad\n\n")
  assert_eq(
    @eval.failure_category(audit, "shell", false, true, true, false, 0),
    "audit_error",
  )
}

///|
test "JSON grading ignores key order but requires complete values" {
  let case = @eval.suite()[0]
  assert_true(
    @eval.correct_output(case, "{\"Sam\":275,\"Lin\":0,\"Ada\":1200}"),
  )
  assert_false(@eval.correct_output(case, "{\"Ada\":1200,\"Sam\":275}"))
  assert_false(
    @eval.correct_output(case, "{\"Ada\":\"1200\",\"Lin\":0,\"Sam\":275}"),
  )
  assert_false(@eval.correct_output(case, "not JSON"))
}

///|
test "repair must preserve the executable test entrypoint" {
  let case = @eval.suite()[4]
  let original = case.inputs["broken.mbtx"]
  assert_true(
    @eval.preserved_input(
      case,
      "broken.mbtx",
      Some(original.replace(old="total = x", new="total += x")),
    ),
  )
  assert_false(
    @eval.preserved_input(
      case,
      "broken.mbtx",
      Some(original.replace(old="[9,1]", new="[0]")),
    ),
  )
  assert_false(@eval.preserved_input(case, "broken.mbtx", None))
}

///|
test "repeated request history does not duplicate MBTX failures" {
  let audit = @eval.Audit::new()
  let request : Json = {
    "body": {
      "input": [
        {
          "type": "function_call_output",
          "call_id": "c1",
          "output": "{\"state\":\"completed\",\"exit_code\":1}",
        },
        {
          "type": "function_call_output",
          "call_id": "c2",
          "output": "{\"state\":\"failed\",\"error\":{\"kind\":\"timeout\"}}",
        },
        {
          "type": "function_call_output",
          "call_id": "c3",
          "output": "plain shell output",
        },
      ],
    },
  }
  audit.request(request.stringify())
  audit.request(request.stringify())
  assert_eq(audit.mbtx_failures.get("script_exit"), Some(1))
  assert_eq(audit.mbtx_failures.get("timeout"), Some(1))
  assert_eq(audit.parse_errors, 0)
}

///|
test "recovered provider failure remains visible without failing a completed task" {
  let audit = completed_audit("exec_command")
  audit.event({
    "type": "response.failed",
    "response": { "error": { "code": "server_error" } },
  })
  assert_eq(audit.provider_errors, 1)
  assert_eq(
    @eval.failure_category(audit, "shell", false, true, true, false, 0),
    "none",
  )
}

///|
test "malformed provider evidence is never a passing result" {
  let audit = @eval.Audit::new()
  audit.dump("{\"status\":500,\"body\":\"upstream unavailable\"}")
  assert_eq(audit.http_errors, 1)
  assert_eq(
    @eval.failure_category(audit, "shell", false, true, true, false, 0),
    "provider_error",
  )
}
