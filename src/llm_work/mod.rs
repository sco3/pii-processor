/// attempt to create "compressed" message for llm analysis, for future improvements
pub mod brevity;
/// check order of reductions (LLM produces wrong order sometimes)
pub mod check_or_swap;
/// constatns for conversation roles
pub mod conv_roles;
/// generates plain text for chat history for LLM processing
pub mod generate_chat_history_str;
/// extract plain text from session log
pub mod get_text_from_session_log;
/// llm calls code
pub mod llm_caller;
/// llm processing for logs
pub mod llm_log_processor;
/// process method for llm processor
mod llm_log_processor_process;
/// llm redactions processing code
pub mod llm_log_processor_redactions;
/// interface for processor
pub mod log_handler;
/// redacted masks code
pub mod masks;
/// constructor for llm processor
mod new;
/// parsing of llm output
mod parse;
/// shor version of messages for debug logging (first N bytes)
pub mod preview;
/// the result structure produced by llm processor
pub mod process_result;
/// prompt related code
pub mod prompt;
/// interface for pii redactor
pub mod reducter;
/// update session log with llm redactions 
pub mod update_redacted;
