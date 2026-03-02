//! Multi-provider LLM API client.
//!
//! Supports 22 providers across 3 wire protocols:
//! - Anthropic Messages API
//! - OpenAI Chat Completions (covers most providers)
//! - Google Gemini generateContent

mod provider;
mod protocol;
mod client;
mod error;

pub use provider::{ApiProvider, ProviderMeta, PROVIDERS, provider_meta, provider_by_name};
pub use protocol::ApiProtocol;
pub use client::{ApiClient, ApiConfig, ChatMessage, resolve_api_key};
pub use error::ApiError;
