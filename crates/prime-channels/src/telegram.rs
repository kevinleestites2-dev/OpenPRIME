use crate::{adapter::ChannelAdapter, message::{Platform, PrimeMessage}};
use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::mpsc;

pub struct TelegramAdapter {
    pub bot_token: String,
    client: reqwest::Client,
}

impl TelegramAdapter {
    pub fn new(bot_token: impl Into<String>) -> Self {
        Self { bot_token: bot_token.into(), client: reqwest::Client::new() }
    }

    fn api_url(&self, method: &str) -> String {
        format!("https://api.telegram.org/bot{}/{}", self.bot_token, method)
    }
}

#[async_trait]
impl ChannelAdapter for TelegramAdapter {
    fn platform(&self) -> &str { "telegram" }

    async fn send(&self, chat_id: &str, message: &str) -> Result<()> {
        self.client.post(self.api_url("sendMessage"))
            .json(&serde_json::json!({
                "chat_id": chat_id,
                "text": message,
                "parse_mode": "Markdown"
            }))
            .send().await?;
        Ok(())
    }

    async fn start_listener(&self, tx: mpsc::Sender<PrimeMessage>) -> Result<()> {
        let token = self.bot_token.clone();
        let client = self.client.clone();
        tokio::spawn(async move {
            let mut offset = 0i64;
            loop {
                let url = format!("https://api.telegram.org/bot{}/getUpdates?offset={}&timeout=30", token, offset);
                if let Ok(res) = client.get(&url).send().await {
                    if let Ok(json) = res.json::<serde_json::Value>().await {
                        if let Some(updates) = json["result"].as_array() {
                            for update in updates {
                                offset = update["update_id"].as_i64().unwrap_or(offset) + 1;
                                if let Some(text) = update["message"]["text"].as_str() {
                                    let chat_id = update["message"]["chat"]["id"].to_string();
                                    let sender  = update["message"]["from"]["username"].as_str().unwrap_or("user").to_string();
                                    let msg = PrimeMessage::new(Platform::Telegram, sender, chat_id, text);
                                    let _ = tx.send(msg).await;
                                }
                            }
                        }
                    }
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        });
        Ok(())
    }
}
