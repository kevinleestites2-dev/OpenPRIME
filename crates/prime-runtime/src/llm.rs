use crate::error::RuntimeError;
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmMessage {
    pub role: Role,
    pub content: String,
}

impl LlmMessage {
    pub fn system(content: impl Into<String>) -> Self { Self { role: Role::System, content: content.into() } }
    pub fn user(content: impl Into<String>) -> Self   { Self { role: Role::User,   content: content.into() } }
    pub fn assistant(content: impl Into<String>) -> Self { Self { role: Role::Assistant, content: content.into() } }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmResponse {
    pub content: String,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub model: String,
    pub stop_reason: String,
}

#[async_trait]
pub trait LlmDriver: Send + Sync {
    async fn complete(&self, messages: &[LlmMessage], max_tokens: u32) -> Result<LlmResponse, RuntimeError>;
    fn model(&self) -> &str;
}

// ── Anthropic driver ────────────────────────────────────────────────────────

pub struct AnthropicDriver {
    pub api_key: String,
    pub model: String,
    client: reqwest::Client,
}

impl AnthropicDriver {
    pub fn new(api_key: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            model: model.into(),
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl LlmDriver for AnthropicDriver {
    fn model(&self) -> &str { &self.model }

    async fn complete(&self, messages: &[LlmMessage], max_tokens: u32) -> Result<LlmResponse, RuntimeError> {
        let system = messages.iter()
            .find(|m| m.role == Role::System)
            .map(|m| m.content.clone())
            .unwrap_or_default();
        let chat_msgs: Vec<_> = messages.iter()
            .filter(|m| m.role != Role::System)
            .map(|m| serde_json::json!({ "role": format!("{:?}", m.role).to_lowercase(), "content": m.content }))
            .collect();

        let body = serde_json::json!({
            "model": self.model,
            "max_tokens": max_tokens,
            "system": system,
            "messages": chat_msgs,
        });

        let res = self.client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !res.status().is_success() {
            let text = res.text().await.unwrap_or_default();
            return Err(RuntimeError::LlmError(text));
        }

        let json: serde_json::Value = res.json().await?;
        let content = json["content"][0]["text"].as_str().unwrap_or("").to_string();
        let input_tokens  = json["usage"]["input_tokens"].as_u64().unwrap_or(0);
        let output_tokens = json["usage"]["output_tokens"].as_u64().unwrap_or(0);
        let stop_reason   = json["stop_reason"].as_str().unwrap_or("end_turn").to_string();

        Ok(LlmResponse { content, input_tokens, output_tokens, model: self.model.clone(), stop_reason })
    }
}

// ── OpenAI-compatible driver (Ollama, Groq, OpenRouter, etc.) ───────────────

pub struct OpenAiDriver {
    pub base_url: String,
    pub api_key: String,
    pub model: String,
    client: reqwest::Client,
}

impl OpenAiDriver {
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            api_key: api_key.into(),
            model: model.into(),
            client: reqwest::Client::new(),
        }
    }
    pub fn ollama(model: impl Into<String>) -> Self {
        Self::new("http://localhost:11434/v1", "ollama", model)
    }
}

#[async_trait]
impl LlmDriver for OpenAiDriver {
    fn model(&self) -> &str { &self.model }

    async fn complete(&self, messages: &[LlmMessage], max_tokens: u32) -> Result<LlmResponse, RuntimeError> {
        let msgs: Vec<_> = messages.iter()
            .map(|m| serde_json::json!({ "role": format!("{:?}", m.role).to_lowercase(), "content": m.content }))
            .collect();
        let body = serde_json::json!({
            "model": self.model,
            "max_tokens": max_tokens,
            "messages": msgs,
        });

        let res = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !res.status().is_success() {
            let text = res.text().await.unwrap_or_default();
            return Err(RuntimeError::LlmError(text));
        }

        let json: serde_json::Value = res.json().await?;
        let content = json["choices"][0]["message"]["content"].as_str().unwrap_or("").to_string();
        let input_tokens  = json["usage"]["prompt_tokens"].as_u64().unwrap_or(0);
        let output_tokens = json["usage"]["completion_tokens"].as_u64().unwrap_or(0);

        Ok(LlmResponse { content, input_tokens, output_tokens, model: self.model.clone(), stop_reason: "stop".into() })
    }
}
