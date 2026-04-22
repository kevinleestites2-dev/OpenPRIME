use crate::{adapter::ChannelAdapter, message::PrimeMessage};
use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::mpsc;

pub struct DiscordAdapter { pub bot_token: String }

impl DiscordAdapter {
    pub fn new(bot_token: impl Into<String>) -> Self { Self { bot_token: bot_token.into() } }
}

#[async_trait]
impl ChannelAdapter for DiscordAdapter {
    fn platform(&self) -> &str { "discord" }
    async fn send(&self, chat_id: &str, message: &str) -> Result<()> {
        let client = reqwest::Client::new();
        client.post(format!("https://discord.com/api/v10/channels/{}/messages", chat_id))
            .header("Authorization", format!("Bot {}", self.bot_token))
            .json(&serde_json::json!({ "content": message }))
            .send().await?;
        Ok(())
    }
    async fn start_listener(&self, _tx: mpsc::Sender<PrimeMessage>) -> Result<()> {
        // Full Discord gateway (wss) listener goes here
        tracing::info!("Discord listener started (gateway not yet wired)");
        Ok(())
    }
}
