use crate::session_log_models::SessionLogEntry::ChatMessage;
use crate::session_log_models::SessionLogType;
pub fn texter(session_items: SessionLogType) -> String {
    let mut out_text = String::new();
    for msg in session_items {
        match msg {
            ChatMessage(chat_msg) => {
                out_text.push_str(chat_msg.role.as_str());
                out_text.push(':');
                out_text.push_str(chat_msg.content.as_str());
                out_text.push('\n');
            }
            _ => {}
        }
    }
    out_text
}
