use crate::message::PrimeMessage;
use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::mpsc;

#[async_trait]
pub trait ChannelAdapter: Send + Sync {
    fn platform(&self) -> &str;
    async fn send(&self, chat_id: &str, message: &str) -> Result<()>;
    async fn start_listener(&self, tx: mpsc::Sender<PrimeMessage>) -> Result<()>;
}
