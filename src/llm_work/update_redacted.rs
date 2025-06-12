use crate::llm_work::llm_log_processor::LlmLogProcessor;
use crate::session_log_models::SessionLog;

use crate::session_log_models::SessionLogEntry::{
    ArchType, ChatGptEntry, ChatMessage, TimeSummaryItem, ToolCallRefs,
};
use std::collections::HashMap;

impl LlmLogProcessor {
    pub fn update_log(&self, log: &mut SessionLog, redacts: &HashMap<String, String>) {
        for entry in log.iter_mut() {
            match entry {
                ChatMessage(chat_msg) => {
                    chat_msg.content = Self::upd_field(chat_msg.content.clone(), redacts);
                }
                ChatGptEntry(chat_gpt) => {
                    for chat_msg in &mut chat_gpt.chat_gpt.request.chat_history {
                        chat_msg.content = Self::upd_field(chat_msg.content.clone(), redacts);
                    }
                    if let Some(msg) = chat_gpt.chat_gpt.response.message.as_mut() {
                        msg.content = Self::upd_field(msg.content.clone(), redacts);
                        // for tool_call in msg.tool_calls.as_mut() {
                        // }
                    }
                }
                ArchType(_arch_type) => {}
                ToolCallRefs(_tool_call_ref) => {}
                TimeSummaryItem(_time_summary) => {}
            }
        }
    }

    fn upd_field(old: String, redacts: &HashMap<String, String>) -> String {
        let mut content = old;
        for redact in redacts {
            content = content.replace(redact.0, redact.1);
        }

        content
    }
}
