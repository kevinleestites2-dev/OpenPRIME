use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, oneshot};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AgentState {
    Idle,
    Running,
    Paused,
    Waiting,
    Completed,
    Failed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMeta {
    pub id: String,
    pub name: String,
    pub agent_type: String,
    pub state: AgentState,
    pub spawned_at: DateTime<Utc>,
    pub last_active: DateTime<Utc>,
    pub task_count: u64,
    pub token_count: u64,
    pub parent_id: Option<String>,
}

impl AgentMeta {
    pub fn new(name: impl Into<String>, agent_type: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.into(),
            agent_type: agent_type.into(),
            state: AgentState::Idle,
            spawned_at: now,
            last_active: now,
            task_count: 0,
            token_count: 0,
            parent_id: None,
        }
    }
}

pub enum AgentCommand {
    Pause,
    Resume,
    Kill,
    Status(oneshot::Sender<AgentMeta>),
}

#[derive(Debug)]
pub struct AgentHandle {
    pub meta: AgentMeta,
    pub cmd_tx: mpsc::Sender<AgentCommand>,
}

impl AgentHandle {
    pub async fn pause(&self) -> Result<()> {
        self.cmd_tx.send(AgentCommand::Pause).await?;
        Ok(())
    }
    pub async fn resume(&self) -> Result<()> {
        self.cmd_tx.send(AgentCommand::Resume).await?;
        Ok(())
    }
    pub async fn kill(&self) -> Result<()> {
        self.cmd_tx.send(AgentCommand::Kill).await?;
        Ok(())
    }
    pub async fn status(&self) -> Result<AgentMeta> {
        let (tx, rx) = oneshot::channel();
        self.cmd_tx.send(AgentCommand::Status(tx)).await?;
        Ok(rx.await?)
    }
}
