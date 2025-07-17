use crate::data::session_log_models::SessionLog;
use crate::data::session_log_models::SessionLogEntry::ChatMessageEnum;
use crate::llm_work;
use std::collections::HashSet;
use std::hash::BuildHasher;

use llm_work::conv_roles::{ASSISTANT, USER};

use tracing::debug;
/// extracts plain text from session log sections for LLM call
pub fn get_text_from_session_log(session_items: &SessionLog) -> String {
    let mut chat_log = String::new();
    let mut text_set = HashSet::new();
    for msg in session_items {
        if let ChatMessageEnum(chat_msg) = msg {
            debug!("Role: {}", chat_msg.role.as_str());
            let role = chat_msg.role.as_str();
            let content = chat_msg.content.replace('\n', " ");
            match role {
                USER => {
                    user_content(&mut text_set, &mut chat_log, &content);
                }
                _ => {
                    assistant_content(&mut text_set, &mut chat_log, &content);
                }
            }
        }
    }
    chat_log
}
/// special treatmen for user input tags
fn user_content(text_set: &mut HashSet<String>, chat_log: &mut String, content: &String) {
    const USER_TAG_START: &str = "<user_input>";
    const USER_TAG_END: &str = "</user_input>";

    if let Some(start_idx) = content.find(USER_TAG_START) {
        if let Some(end_idx) = content.find(USER_TAG_END) {
            let range = start_idx + USER_TAG_START.len()..end_idx;
            let user_input = content[range].trim();
            add_txt(text_set, chat_log, &format!("{USER}: {user_input}\n"));
        } else {
            // fallback if end tag is missing
            add_txt(
                text_set,
                chat_log,
                &format!(
                    "{}: {}\n",
                    USER, //
                    content.replace(USER_TAG_START, ""),
                ),
            );
        }
    } else {
        add_txt(text_set, chat_log, &format!("{USER}: {content}\n"));
    }
}
/// special treatmen for text with assistant tags
fn assistant_content(text_set: &mut HashSet<String>, chat_log: &mut String, content: &str) {
    const ASSISTANT_TAG_START: &str = "<assistant_response>";
    const ASSISTANT_TAG_END: &str = "</assistant_response>";

    if let Some(start_idx) = content.find(ASSISTANT_TAG_START) {
        if let Some(end_idx) = content.find(ASSISTANT_TAG_END) {
            let range = start_idx + ASSISTANT_TAG_START.len()..end_idx;
            let response = content[range].trim();
            add_txt(text_set, chat_log, &format!("{ASSISTANT}: {response}\n"));
        } else {
            // fallback if end tag is missing
            add_txt(
                text_set,
                chat_log,
                &format!(
                    "{}: {}\n", //
                    ASSISTANT,
                    content.replace(ASSISTANT_TAG_START, ""),
                ),
            );
        }
    } else {
        let cleaned_content = content.trim_matches('"');
        add_txt(
            text_set,
            chat_log,
            &format!("{ASSISTANT}: {cleaned_content}\n"),
        );
    }
}

/// add new string to chat in plain text,
/// hash set is used to avoid duplicates in chat
/// and this way reduce token size for LLM call.
pub fn add_txt<S: BuildHasher>(history: &mut HashSet<String, S>, dst: &mut String, src: &str) {
    if !history.contains(src) {
        history.insert(src.to_owned());
        dst.push_str(src);
    }
}
