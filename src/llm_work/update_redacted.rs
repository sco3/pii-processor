//use crate::data::session_log_models::ChatMessage;
use crate::data::session_log_models::{ChatMessageItem, SessionLog};
use crate::llm_work::llm_log_processor::LlmLogProcessor;

use crate::data::session_log_models::SessionLogEntry::{
    ArchTypeEnum, ChatGptEnum, ChatMessageEnum, TimeSummaryEnum, ToolCallRefsEnum,
};
use std::collections::HashMap;

impl LlmLogProcessor {
    /// updates session log with redacted strings
    pub fn update_log(&self, log: &mut SessionLog, redacts: &HashMap<String, String>) {
        for entry in log.iter_mut() {
            match entry {
                ChatMessageEnum(chat_msg) => {
                    chat_msg.content = Self::upd_field(chat_msg.content.clone(), redacts);
                    Self::process_tool_calls(chat_msg, redacts);
                }
                ChatGptEnum(chat_gpt) => {
                    for chat_msg in &mut chat_gpt.chat_gpt.request.chat_history {
                        chat_msg.content = Self::upd_field(chat_msg.content.clone(), redacts);
                    }
                    if let Some(chat_msg) = chat_gpt.chat_gpt.response.message.as_mut() {
                        chat_msg.content = Self::upd_field(chat_msg.content.clone(), redacts);
                        Self::process_tool_calls(chat_msg, redacts);
                    }
                }
                ArchTypeEnum(_arch_type) => {}
                ToolCallRefsEnum(_tool_call_ref) => {}
                TimeSummaryEnum(_time_summary) => {}
            }
        }
    }

    // possibly better option to parse arguments from string to json, update only values
    // let args_result = serde_json::from_str::<Value>(args);
    // if let Ok(args_value) = args_result {
    //     if let Some(args_map) = args_value.as_object() {
    //         for (_, val) in args_map {
    //             if let Some(s) = val.as_str() {
    //                 ... = Self::upd_field(s, redacts);
    //
    //             }
    //         }
    //     }
    // }

    /// tool call sestion update
    fn process_tool_calls(chat_msg: &mut ChatMessageItem, redacts: &HashMap<String, String>) {
        if let Some(tool_calls) = &mut chat_msg.tool_calls {
            for tool_call in tool_calls {
                let function = &tool_call.function;
                if let Some(args) = &function.arguments {
                    let upd = Self::upd_field(args.clone(), redacts);
                    tool_call.function.arguments = Some(upd);
                }
            }
        }
    }
    /// update text field in session log
    fn upd_field(old: String, redacts: &HashMap<String, String>) -> String {
        let mut content = old;
        for redact in redacts {
            content = content.replace(redact.0, redact.1);
        }

        content
    }
}
