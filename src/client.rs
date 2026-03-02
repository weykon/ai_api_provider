use serde::{Deserialize, Serialize};

use crate::error::ApiError;
use crate::protocol::ApiProtocol;
use crate::provider::{provider_meta, ApiProvider};

/// Configuration for a single API call.
#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub provider: ApiProvider,
    pub api_key: String,
    pub model: String,
    pub base_url: Option<String>,
    /// Maximum tokens in the response. Default: 4096.
    pub max_tokens: u32,
}

impl ApiConfig {
    pub fn new(provider: ApiProvider, api_key: String) -> Self {
        let model = provider_meta(provider).default_model.to_string();
        Self {
            provider,
            api_key,
            model,
            base_url: None,
            max_tokens: 4096,
        }
    }

    /// Resolve the endpoint URL for the provider.
    pub fn endpoint(&self) -> String {
        let meta = provider_meta(self.provider);
        let base = self.base_url.as_deref().unwrap_or(meta.base_url);
        match meta.protocol {
            ApiProtocol::Anthropic => base.to_string(),
            ApiProtocol::OpenAiCompat => {
                let base = base.trim_end_matches('/');
                if base.ends_with("/chat/completions") {
                    base.to_string()
                } else {
                    format!("{}/chat/completions", base)
                }
            }
            ApiProtocol::Gemini => {
                format!("{}/models/{}:generateContent", base, self.model)
            }
        }
    }
}

/// A single chat message (role + content).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl ChatMessage {
    pub fn system(content: impl Into<String>) -> Self {
        Self { role: "system".into(), content: content.into() }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self { role: "user".into(), content: content.into() }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self { role: "assistant".into(), content: content.into() }
    }
}

/// Async LLM API client.
pub struct ApiClient {
    http: reqwest::Client,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            http: reqwest::Client::new(),
        }
    }

    /// Send a chat completion request and return the full response text.
    pub async fn chat(
        &self,
        config: &ApiConfig,
        messages: &[ChatMessage],
    ) -> Result<String, ApiError> {
        let meta = provider_meta(config.provider);
        let body = build_request_body(config, messages);

        let request = match meta.protocol {
            ApiProtocol::Anthropic => {
                self.http
                    .post(&config.endpoint())
                    .header("Content-Type", "application/json")
                    .header("x-api-key", &config.api_key)
                    .header("anthropic-version", "2023-06-01")
            }
            ApiProtocol::OpenAiCompat => {
                let mut req = self.http
                    .post(&config.endpoint())
                    .header("Content-Type", "application/json");
                if !config.api_key.is_empty() {
                    req = req.header("Authorization", format!("Bearer {}", config.api_key));
                }
                req
            }
            ApiProtocol::Gemini => {
                let url = format!("{}?key={}", config.endpoint(), config.api_key);
                self.http
                    .post(&url)
                    .header("Content-Type", "application/json")
            }
        };

        let response = request.json(&body).send().await?;
        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            return Err(ApiError::ApiResponse {
                status: status.as_u16(),
                body: text.chars().take(300).collect(),
            });
        }

        extract_response_text(meta.protocol, &text)
    }
}

impl Default for ApiClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Build the request body JSON based on the provider's wire protocol.
fn build_request_body(config: &ApiConfig, messages: &[ChatMessage]) -> serde_json::Value {
    let meta = provider_meta(config.provider);
    match meta.protocol {
        ApiProtocol::Anthropic => {
            let api_messages: Vec<serde_json::Value> = messages
                .iter()
                .filter(|m| m.role != "system")
                .map(|m| serde_json::json!({ "role": m.role, "content": m.content }))
                .collect();
            let system = messages
                .iter()
                .find(|m| m.role == "system")
                .map(|m| m.content.as_str())
                .unwrap_or("");
            let mut body = serde_json::json!({
                "model": config.model,
                "max_tokens": config.max_tokens,
                "messages": api_messages,
            });
            if !system.is_empty() {
                body["system"] = serde_json::json!(system);
            }
            body
        }
        ApiProtocol::OpenAiCompat => {
            let api_messages: Vec<serde_json::Value> = messages
                .iter()
                .map(|m| serde_json::json!({ "role": m.role, "content": m.content }))
                .collect();
            serde_json::json!({
                "model": config.model,
                "messages": api_messages,
                "max_tokens": config.max_tokens,
            })
        }
        ApiProtocol::Gemini => {
            let parts: Vec<serde_json::Value> = messages
                .iter()
                .map(|m| serde_json::json!({
                    "role": if m.role == "assistant" { "model" } else { "user" },
                    "parts": [{ "text": m.content }],
                }))
                .collect();
            serde_json::json!({ "contents": parts })
        }
    }
}

/// Extract the assistant's text from the protocol-specific response JSON.
fn extract_response_text(protocol: ApiProtocol, raw: &str) -> Result<String, ApiError> {
    let json: serde_json::Value =
        serde_json::from_str(raw).map_err(|e| ApiError::Parse(e.to_string()))?;

    match protocol {
        ApiProtocol::Anthropic => json["content"]
            .as_array()
            .and_then(|arr| arr.iter().find(|b| b["type"] == "text"))
            .and_then(|b| b["text"].as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| ApiError::Parse("No text content in Anthropic response".into())),

        ApiProtocol::OpenAiCompat => json["choices"]
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|c| c["message"]["content"].as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| ApiError::Parse("No content in OpenAI-compatible response".into())),

        ApiProtocol::Gemini => json["candidates"]
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|c| c["content"]["parts"].as_array())
            .and_then(|parts| parts.first())
            .and_then(|p| p["text"].as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| ApiError::Parse("No text in Gemini response".into())),
    }
}

/// Resolve API key from environment variables (tries all env vars for the provider).
pub fn resolve_api_key(provider: ApiProvider) -> Option<String> {
    let meta = provider_meta(provider);
    for var_name in meta.env_vars {
        if let Ok(val) = std::env::var(var_name) {
            let val = val.trim().to_string();
            if !val.is_empty() {
                return Some(val);
            }
        }
    }
    if meta.local {
        return Some(String::new());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoint_anthropic() {
        let config = ApiConfig::new(ApiProvider::Claude, "test".into());
        assert!(config.endpoint().contains("anthropic.com"));
    }

    #[test]
    fn test_endpoint_openai_appends_path() {
        let config = ApiConfig::new(ApiProvider::DeepSeek, "test".into());
        assert!(config.endpoint().ends_with("/chat/completions"));
    }

    #[test]
    fn test_endpoint_gemini_includes_model() {
        let config = ApiConfig::new(ApiProvider::Gemini, "test".into());
        assert!(config.endpoint().contains("gemini-2.0-flash"));
    }

    #[test]
    fn test_build_body_anthropic() {
        let config = ApiConfig::new(ApiProvider::Claude, "test".into());
        let messages = vec![
            ChatMessage::system("You are helpful."),
            ChatMessage::user("Hello"),
        ];
        let body = build_request_body(&config, &messages);
        assert_eq!(body["system"], "You are helpful.");
        assert_eq!(body["messages"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn test_extract_anthropic() {
        let raw = r#"{"content":[{"type":"text","text":"Hello!"}]}"#;
        assert_eq!(extract_response_text(ApiProtocol::Anthropic, raw).unwrap(), "Hello!");
    }

    #[test]
    fn test_extract_openai() {
        let raw = r#"{"choices":[{"message":{"content":"Hi!"}}]}"#;
        assert_eq!(extract_response_text(ApiProtocol::OpenAiCompat, raw).unwrap(), "Hi!");
    }

    #[test]
    fn test_extract_gemini() {
        let raw = r#"{"candidates":[{"content":{"parts":[{"text":"Hey!"}]}}]}"#;
        assert_eq!(extract_response_text(ApiProtocol::Gemini, raw).unwrap(), "Hey!");
    }

    #[test]
    fn test_chat_message_builders() {
        let msg = ChatMessage::system("test");
        assert_eq!(msg.role, "system");
        let msg = ChatMessage::user("hello");
        assert_eq!(msg.role, "user");
    }
}
