//use crate::data::session_log_models::ChatMessage;
use crate::data::session_log_models::SessionLog;
use crate::llm_work::llm_log_processor::LlmLogProcessor;

use crate::data::session_log_models::SessionLogEntry::{
    ArchTypeEnum, ChatGptEnum, ChatMessageEnum, TimeSummaryEnum, ToolCallRefsEnum,
};
use std::collections::HashMap;

impl LlmLogProcessor {
    pub fn update_log(&self, log: &mut SessionLog, redacts: &HashMap<String, String>) {
        for entry in log.iter_mut() {
            match entry {
                ChatMessageEnum(chat_msg) => {
                    chat_msg.content = Self::upd_field(chat_msg.content.clone(), redacts);
                    //Self::process_tool_calls(chat_msg);
                }
                ChatGptEnum(chat_gpt) => {
                    for chat_msg in &mut chat_gpt.chat_gpt.request.chat_history {
                        chat_msg.content = Self::upd_field(chat_msg.content.clone(), redacts);
                    }
                    if let Some(msg) = chat_gpt.chat_gpt.response.message.as_mut() {
                        msg.content = Self::upd_field(msg.content.clone(), redacts);
                        // for tool_call in msg.tool_calls.as_mut() {
                        // }
                    }
                }
                ArchTypeEnum(_arch_type) => {}
                ToolCallRefsEnum(_tool_call_ref) => {}
                TimeSummaryEnum(_time_summary) => {}
            }
        }
    }

    // fn process_tool_calls(chat_msg: &mut ChatMessage) {
    //     if let Some(tool_calls) = &chat_msg.tool_calls {
    //         for tool_call in tool_calls {
    //             let function = tool_call;
    //         }
    //     }
    // }

    fn upd_field(old: String, redacts: &HashMap<String, String>) -> String {
        let mut content = old;
        for redact in redacts {
            content = content.replace(redact.0, redact.1);
        }

        content
    }
}
