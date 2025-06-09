use crate::llm_work;
use crate::session_log_models::SessionLogEntry::ChatMessage;
use crate::session_log_models::SessionLogType;

use llm_work::conv_roles::{ASSISTANT, USER};

use tracing::debug;

pub fn extract_text(session_items: SessionLogType) -> String {
    let mut chat_log = String::new();
    for msg in session_items {
        if let ChatMessage(chat_msg) = msg {
            debug!("Role: {}", chat_msg.role.as_str());
            let role = chat_msg.role.as_str();
            let content = chat_msg.content.replace("\n", " ");
            match role {
                USER => {
                    user_content(&mut chat_log, &content);
                }
                ASSISTANT => {
                    assistant_content(&mut chat_log, &content);
                }
                _ => {
                    assistant_content(&mut chat_log, &content);
                }
            }
        }
    }
    chat_log
}

fn user_content(chat_log: &mut String, content: &String) {
    const USER_TAG_START: &str = "<user_input>";
    const USER_TAG_END: &str = "</user_input>";

    if let Some(start_idx) = content.find(USER_TAG_START) {
        if let Some(end_idx) = content.find(USER_TAG_END) {
            let user_input = content[start_idx + USER_TAG_START.len()..end_idx].trim();
            chat_log.push_str(&format!("{}: {}\n", USER, user_input));
        } else {
            // fallback if end tag is missing
            chat_log.push_str(&format!("{}: {}\n", USER, content));
        }
    } else {
        chat_log.push_str(&format!("{}: {}\n", USER, content));
    }
}

fn assistant_content(chat_log: &mut String, content: &String) {
    const ASSISTANT_TAG_START: &str = "<assistant_response>";
    const ASSISTANT_TAG_END: &str = "</assistant_response>";

    if let Some(start_idx) = content.find(ASSISTANT_TAG_START) {
        if let Some(end_idx) = content.find(ASSISTANT_TAG_END) {
            let response = content[start_idx + ASSISTANT_TAG_START.len()..end_idx].trim();
            chat_log.push_str(&format!("{}: {}\n", ASSISTANT, response));
        } else {
            // fallback if end tag is missing
            chat_log.push_str(&format!("{}: {}\n", ASSISTANT, content));
        }
    } else {
        let cleaned_content = content.trim_matches('"');
        chat_log.push_str(&format!("{}: {}\n", ASSISTANT, cleaned_content));
    }
}
