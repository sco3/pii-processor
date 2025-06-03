use serde::{Deserialize, Serialize};
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////
// Trait to represent TurnAware behavior (getter/setter for turn)
////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
// Enum for Architecture (StrEnum in Python)
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Architecture {
    Neocortex,
}

////////////////////////////////////////////////////////////////////////////////
// Session log entries and related structs
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalFunctionTimes {
    pub timestamp: f64,
    pub retrieve_state: f64,
    pub get_rules: f64,
    pub get_groups: f64,
    pub get_devices: f64,
    pub get_response: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeSummary {
    pub total_function_times: TotalFunctionTimes,
    pub total_request_time: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Property {
    #[serde(rename = "type")]
    pub ty: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Parameters {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub ty: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Property>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Function {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolCall {
    pub index: i32,
    pub function: Function,
    pub id: String,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArchType {
    #[serde(rename = "architecture_type")]
    pub architecture_type: Architecture,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolCallRef {
    pub tool_calls: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolCallRefs {
    pub tool_calls: Vec<ToolCallRef>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeSummaryItem {
    #[serde(rename = "time_summary_s")]
    pub time_summary_s: TimeSummary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub chat_history: Vec<ChatMessage>,
    pub functions: Vec<Function>,
    pub model: String,
    pub gpt_client: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ChatMessage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentItem {
    pub text: String,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatHistoryItem {
    pub role: String,
    // Union of String or Vec<ContentItem> is tricky, use serde_json::Value for flexibility
    pub content: serde_json::Value,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatGpt {
    pub request: Request,
    pub response: Response,
    pub time: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatGptEntry {
    pub chat_gpt: ChatGpt,
}

////////////////////////////////////////////////////////////////////////////////
// SessionLogEntry enum for root list elements
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SessionLogEntry {
    ArchType(ArchType),
    ToolCallRefs(ToolCallRefs),
    ChatMessage(ChatMessage),
    ChatGptEntry(ChatGptEntry),
    TimeSummaryItem(TimeSummaryItem),
}

/// Root type for session log
pub type SessionLogType = Vec<SessionLogEntry>;
