use crate::{adapter::ChannelAdapter, message::{Platform, PrimeMessage}};
use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::mpsc;

pub struct WebhookAdapter { pub endpoint: String }

impl WebhookAdapter {
    pub fn new(endpoint: impl Into<String>) -> Self { Self { endpoint: endpoint.into() } }
}

#[async_trait]
impl ChannelAdapter for WebhookAdapter {
    fn platform(&self) -> &str { "webhook" }
    async fn send(&self, _chat_id: &str, message: &str) -> Result<()> {
        reqwest::Client::new()
            .post(&self.endpoint)
            .json(&serde_json::json!({ "text": message }))
            .send().await?;
        Ok(())
    }
    async fn start_listener(&self, _tx: mpsc::Sender<PrimeMessage>) -> Result<()> {
        tracing::info!("Webhook inbound listener — wire to prime-api /webhook endpoint");
        Ok(())
    }
}
