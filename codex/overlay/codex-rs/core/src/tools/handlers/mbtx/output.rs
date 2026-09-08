use crate::tools::context::ToolOutput;
use crate::tools::context::ToolPayload;
use codex_protocol::models::FunctionCallOutputPayload;
use codex_protocol::models::ResponseInputItem;
use serde_json::Value;

pub(super) struct MbtxOutput(pub(super) Value);

impl ToolOutput for MbtxOutput {
    fn log_output(&self) -> String {
        self.0.to_string()
    }

    fn success_for_logging(&self) -> bool {
        self.0["state"] != "failed" && self.0.get("exit_code").is_none_or(|code| code == 0)
    }

    fn to_response_item(&self, call_id: &str, _payload: &ToolPayload) -> ResponseInputItem {
        let mut output = FunctionCallOutputPayload::from_text(self.0.to_string());
        output.success = Some(self.success_for_logging());
        ResponseInputItem::FunctionCallOutput {
            call_id: call_id.into(),
            output,
        }
    }

    fn code_mode_result(&self, _payload: &ToolPayload) -> Value {
        self.0.clone()
    }
}
