//! # PII Redactor Microservice Component
//!
//! This crate implements a PII (Personally Identifiable Information) redactor microservice.
//! It functions by listening for specific NATS topics, processing incoming messages,
//! redacting sensitive data using an LLM, and storing the results.
//!
//! ## How it Works
//!
//! 1.  **Message Ingestion:** The service starts and listens for messages on a configured NATS topic.
//!     Messages are expected in the `SessionLog` model format.
//! 2.  **File Context:** A special header in the NATS message specifies the original file name
//!     from which the text was extracted.
//! 3.  **LLM Processing:** After text extraction, the content is sent to an
//!     Large Language Model (LLM) for redaction. The LLM uses a system prompt
//!     located in the `$PROJECT_ROOT/data` directory.
//! 4.  **Redaction Output:** The LLM produces a JSON output, which is a map of
//!     redacted text paired with its original counterpart.
//! 5.  **Session Log Update & Storage:** Based on the LLM's output, the relevant text parts
//!     within the `SessionLog` are updated. The final redacted session log is then saved
//!     to a configured storage backend, which can be either:
//!     * **S3:** Configured with a bucket path (e.g., `s3://<bucket>`).
//!     * **Local Filesystem:** Configured with a full path (e.g., `/temp/save`, often used for tests).
//!
//! ## Architecture Highlights
//!
//! * **Message Queue (NATS):** Uses a `redact_consumer` to listen to subjects and dispatch
//!   messages to workers via an asynchronous channel. The channel has a limited capacity
//!   to manage backpressure; when full, the consumer temporarily stops fetching new messages.
//! * **Worker Pool:** A configurable number of workers handle the core processing:
//!   parsing files, sending requests to the LLM, parsing responses, and updating session logs.
//!
//! ## Modules
//!
#![deny(missing_docs)]

/// Handles application configuration.
pub mod config;
/// Defines data models and constants used throughout the crate.
pub mod data;
/// Contains logic for interacting with the Large Language Model (LLM).
pub mod llm_work;
/// Manages message queue interactions, primarily with NATS.
pub mod mq;
/// Provides HTTP endpoints for readiness and liveness probes.
pub mod probe;
/// Responsible for initializing and starting the application.
pub mod starter;
/// Manages storage operations, supporting both local filesystem and S3.
pub mod storage;
/// Contains general utility functions.
pub mod util;
/// Manages the pool of workers responsible for file processing, LLM interaction, and session log updates.
pub mod worker_pool;
