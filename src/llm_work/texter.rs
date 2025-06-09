use crate::llm_work;
use crate::session_log_models::SessionLogEntry::ChatMessage;
use crate::session_log_models::SessionLogType;

use llm_work::conv_roles::USER;

use crate::llm_work::conv_roles::ASSISTANT;
use tracing::debug;

pub fn texter(session_items: SessionLogType) -> String {
    let mut chat_log = String::new();
    for msg in session_items {
        match msg {
            ChatMessage(chat_msg) => {
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
            _ => {}
        }
    }
    chat_log
}

fn user_content(chat_log: &mut String, mut content: &String) {
    const user_tag_start: &str = "<user_input>";
    const user_tag_end: &str = "</user_input>";

    if let Some(start_idx) = content.find(user_tag_start) {
        if let Some(end_idx) = content.find(user_tag_end) {
            let user_input = content[start_idx + user_tag_start.len()..end_idx].trim();
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
    const ASSISTANT: &str = "ASSISTANT";

    const assistant_tag_start: &str = "<assistant_response>";
    const assistant_tag_end: &str = "</assistant_response>";

    if let Some(start_idx) = content.find(assistant_tag_start) {
        if let Some(end_idx) = content.find(assistant_tag_end) {
            let response = content[start_idx + assistant_tag_start.len()..end_idx].trim();
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
