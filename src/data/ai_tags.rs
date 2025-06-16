/// Constants for AI API request/response fields.
pub struct Ai;

impl Ai {
    /// HTTP Authorization header prefix
    pub const BEARER: &'static str = "Bearer";
    /// Role field name in messages
    pub const ROLE: &'static str = "role";
    /// AI model identifier field
    pub const MODEL: &'static str = "model";
    /// Prompt input field (unused)
    pub const _PROMPT: &'static str = "prompt";
    /// Single message field
    pub const MESSAGE: &'static str = "message";
    /// Message array field
    pub const MESSAGES: &'static str = "messages";
    /// Creativity/randomness control parameter
    pub const TEMPERATURE: &'static str = "temperature";
    /// Response length limit (unused)
    pub const _MAX_TOKENS: &'static str = "max_tokens";
    /// Probability sampling parameter (unused)
    pub const _TOP_P: &'static str = "top_p";
    /// User role identifier
    pub const USER: &'static str = "user";
    /// System role identifier
    pub const SYSTEM: &'static str = "system";
    /// Message content field
    pub const CONTENT: &'static str = "content";
    /// Response choices field
    pub const CHOICES: &'static str = "choices";
}
