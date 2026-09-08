use codex_tools::JsonSchema;
use codex_tools::ResponsesApiTool;
use codex_tools::ToolSpec;
use serde_json::json;
use std::collections::BTreeMap;

pub(super) fn spec() -> ToolSpec {
    ToolSpec::Function(ResponsesApiTool {
        name: "mbtx".into(),
        description:
            "Run MoonBit scripts through MBTX under Codex execution approvals and sandboxing. \
            Use op=run with exactly one of source or script_path; args are literal. \
            background=true returns an opaque job_id; use op=job_output to read new output, \
            or op=job_stop to stop that job. Jobs belong to this Codex session. \
            timeout_ms defaults to 30000 and includes compilation. Output may be truncated with \
            omitted_bytes. Poll completed background jobs to release their active slots."
                .into(),
        strict: false,
        defer_loading: None,
        parameters: JsonSchema::object(
            BTreeMap::from([
                (
                    "op".into(),
                    JsonSchema::string_enum(
                        vec![json!("run"), json!("job_output"), json!("job_stop")],
                        /*description*/ None,
                    ),
                ),
                (
                    "source".into(),
                    JsonSchema::string(Some(
                        "Inline MoonBit script; exclusive with script_path.".into(),
                    )),
                ),
                (
                    "script_path".into(),
                    JsonSchema::string(Some("Path to a .mbtx file.".into())),
                ),
                (
                    "args".into(),
                    JsonSchema::array(
                        JsonSchema::string(/*description*/ None),
                        /*description*/ None,
                    ),
                ),
                (
                    "cwd".into(),
                    JsonSchema::string(Some(
                        "Working directory, relative to the workspace or absolute.".into(),
                    )),
                ),
                (
                    "background".into(),
                    JsonSchema::boolean(/*description*/ None),
                ),
                (
                    "timeout_ms".into(),
                    JsonSchema::integer(Some(
                        "Execution deadline including compilation, from 1 to 600000 ms.".into(),
                    )),
                ),
                ("job_id".into(), JsonSchema::string(/*description*/ None)),
            ]),
            Some(vec!["op".into()]),
            /*additional_properties*/ Some(false.into()),
        ),
        output_schema: None,
    })
}
