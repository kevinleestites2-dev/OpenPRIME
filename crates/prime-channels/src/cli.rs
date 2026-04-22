use crate::{adapter::ChannelAdapter, message::{Platform, PrimeMessage}};
use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::mpsc;

pub struct CliAdapter;

#[async_trait]
impl ChannelAdapter for CliAdapter {
    fn platform(&self) -> &str { "cli" }
    async fn send(&self, _chat_id: &str, message: &str) -> Result<()> {
        println!("{}", message);
        Ok(())
    }
    async fn start_listener(&self, tx: mpsc::Sender<PrimeMessage>) -> Result<()> {
        tokio::spawn(async move {
            use tokio::io::{AsyncBufReadExt, BufReader};
            let stdin = tokio::io::stdin();
            let mut reader = BufReader::new(stdin).lines();
            while let Ok(Some(line)) = reader.next_line().await {
                let msg = PrimeMessage::new(Platform::Cli, "user", "cli", line);
                if tx.send(msg).await.is_err() { break; }
            }
        });
        Ok(())
    }
}
