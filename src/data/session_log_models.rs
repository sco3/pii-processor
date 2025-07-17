use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// AI architecture types
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Architecture {
    /// Neocortex architecture type
    #[serde(rename = "Neocortex")]
    Neocortex,
}

/// Timing metrics for individual functions
#[derive(Debug, Serialize, Deserialize)]
pub struct TotalFunctionTimes {
    /// Timestamp of measurement
    pub timestamp: f64,
    /// Time taken to retrieve state
    pub retrieve_state: f64,
    /// Time taken to get rules
    pub get_rules: f64,
    /// Time taken to get groups
    pub get_groups: f64,
    /// Time taken to get devices
    pub get_devices: f64,
    /// Time taken to get response
    pub get_response: f64,
}

/// Aggregated timing information
#[derive(Debug, Serialize, Deserialize)]
pub struct TimeSummary {
    /// Detailed timing breakdown
    pub total_function_times: TotalFunctionTimes,
    /// Total request processing time
    pub total_request_time: f64,
}

/// Property definition for function parameters
#[derive(Debug, Serialize, Deserialize)]
pub struct Property {
    /// Property type
    #[serde(rename = "type")]
    pub ty: String,
    /// Property description
    pub description: String,
}

/// Function parameter metadata
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Parameters {
    /// Parameter type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub ty: Option<String>,
    /// parameter properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Property>>,

    /// parameter required flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
}

/// AI function definition
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Function {
    /// Function name
    pub name: String,
    /// function description field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// function parameters section
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters>,
    /// arguments string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
}

/// AI tool invocation record
#[derive(Debug, Serialize, Deserialize)]
pub struct ToolCall {
    /// Call index
    pub index: i32,
    /// Called function details
    pub function: Function,
    /// Call ID
    pub id: String,
    /// Call type
    #[serde(rename = "type")]
    pub ty: String,
}

/// Architecture type wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct ArchTypeItem {
    /// architecture type mark
    #[serde(rename = "architecture_type")]
    pub architecture_type: Architecture,
}

/// Reference to tool calls
#[derive(Debug, Serialize, Deserialize)]
pub struct ToolCallRef {
    /// turn number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn: Option<i32>,
    /// List of tool call references
    pub tool_calls: Vec<String>,
}

/// Collection of tool call references
#[derive(Debug, Serialize, Deserialize)]
pub struct ToolCallsItem {
    /// List of tool call references
    pub tool_calls: Vec<ToolCallRef>,
}

/// Chat message with optional tool calls
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessageItem {
    /// Message role (user/system/assistant)
    pub role: String,
    /// Message content
    pub content: String,
    /// tool calls list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    /// turn number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn: Option<i32>,
}

/// Time summary wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct TimeSummaryItem {
    /// Time summary in seconds
    #[serde(rename = "time_summary_s")]
    pub time_summary_s: TimeSummary,
}

/// AI chat request
#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    /// Chat history
    pub chat_history: Vec<ChatMessageItem>,
    /// Available functions
    pub functions: Vec<Function>,
    /// Model name
    pub model: String,
    /// GPT client identifier
    pub gpt_client: String,
}

/// AI chat response
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Response {
    /// agent name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    /// chat message section
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ChatMessageItem>,
}

/// Content item with type
#[derive(Debug, Serialize, Deserialize)]
pub struct ContentItem {
    /// Content text
    pub text: String,
    /// Content type
    #[serde(rename = "type")]
    pub ty: String,
}

/// Extended chat history item
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatHistoryItem {
    /// Message role
    pub role: String,
    /// Message content (string or structured)
    pub content: serde_json::Value,
    /// tool calls section
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    /// tool call id string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

/// Complete chat interaction
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatGpt {
    /// Request data
    pub request: Request,
    /// Response data
    pub response: Response,
    /// Processing time
    pub time: f64,
}

/// Chat interaction wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatGptItem {
    /// Chat GPT interaction data
    pub chat_gpt: ChatGpt,
}

/// Union of all possible session log entry types
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SessionLogEntry {
    /// Architecture type entry
    ArchTypeEnum(ArchTypeItem),
    /// Tool calls reference entry
    ToolCallRefsEnum(ToolCallsItem),
    /// Chat message entry
    ChatMessageEnum(ChatMessageItem),
    /// Chat GPT interaction entry
    ChatGptEnum(ChatGptItem),
    /// Timing summary entry
    TimeSummaryEnum(TimeSummaryItem),
}

/// Session log containing multiple entries
pub type SessionLog = Vec<SessionLogEntry>;
