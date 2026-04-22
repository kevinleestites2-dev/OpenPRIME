use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Platform { Telegram, Discord, Slack, WhatsApp, Signal, Webhook, Cli }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment { pub kind: String, pub url: String, pub name: Option<String> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimeMessage {
    pub id: String,
    pub platform: Platform,
    pub sender: String,
    pub chat_id: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub attachments: Vec<Attachment>,
    pub reply_to: Option<String>,
}

impl PrimeMessage {
    pub fn new(platform: Platform, sender: impl Into<String>, chat_id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            platform,
            sender: sender.into(),
            chat_id: chat_id.into(),
            content: content.into(),
            timestamp: Utc::now(),
            attachments: vec![],
            reply_to: None,
        }
    }
}
