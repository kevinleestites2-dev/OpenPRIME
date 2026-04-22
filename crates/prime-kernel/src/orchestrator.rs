use crate::{
    budget::BudgetTracker,
    config::PrimeConfig,
    error::KernelError,
    lifecycle::{AgentHandle, AgentMeta, AgentState, AgentCommand},
    rbac::{Capability, RbacEngine},
    scheduler::Scheduler,
};
use anyhow::Result;
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};

pub struct Orchestrator {
    pub config: PrimeConfig,
    pub agents: Arc<DashMap<String, AgentHandle>>,
    pub rbac: Arc<RwLock<RbacEngine>>,
    pub scheduler: Arc<RwLock<Scheduler>>,
    pub budget: Arc<RwLock<BudgetTracker>>,
}

impl Orchestrator {
    pub fn new(config: PrimeConfig) -> Self {
        let budget = BudgetTracker::new(
            config.budget.max_tokens_per_session,
            config.budget.max_tokens_per_day,
            config.budget.max_cost_per_day_usd,
        );
        Self {
            agents: Arc::new(DashMap::new()),
            rbac: Arc::new(RwLock::new(RbacEngine::new())),
            scheduler: Arc::new(RwLock::new(Scheduler::default())),
            budget: Arc::new(RwLock::new(budget)),
            config,
        }
    }

    pub async fn spawn_agent(&self, name: &str, agent_type: &str) -> Result<String> {
        if self.agents.len() >= self.config.agent_concurrency {
            return Err(KernelError::Other(anyhow::anyhow!(
                "Max agent concurrency ({}) reached", self.config.agent_concurrency
            )).into());
        }
        let meta = AgentMeta::new(name, agent_type);
        let id = meta.id.clone();
        let (cmd_tx, mut cmd_rx) = mpsc::channel::<AgentCommand>(32);
        let mut meta_inner = meta.clone();
        tokio::spawn(async move {
            while let Some(cmd) = cmd_rx.recv().await {
                match cmd {
                    AgentCommand::Pause  => { meta_inner.state = AgentState::Paused; }
                    AgentCommand::Resume => { meta_inner.state = AgentState::Running; }
                    AgentCommand::Kill   => { meta_inner.state = AgentState::Completed; break; }
                    AgentCommand::Status(tx) => { let _ = tx.send(meta_inner.clone()); }
                }
            }
        });
        let mut rbac = self.rbac.write().await;
        rbac.assign(&id, agent_type);
        self.agents.insert(id.clone(), AgentHandle { meta, cmd_tx });
        tracing::info!("Spawned agent '{}' [{}] as {}", name, id, agent_type);
        Ok(id)
    }

    pub async fn kill_agent(&self, id: &str) -> Result<()> {
        if let Some(handle) = self.agents.get(id) {
            handle.kill().await?;
        }
        self.agents.remove(id);
        tracing::info!("Killed agent {}", id);
        Ok(())
    }

    pub fn list_agents(&self) -> Vec<AgentMeta> {
        self.agents.iter().map(|e| e.value().meta.clone()).collect()
    }

    pub async fn check_permission(&self, agent_id: &str, cap: &Capability) -> Result<()> {
        let rbac = self.rbac.read().await;
        if rbac.can(agent_id, cap) {
            Ok(())
        } else {
            Err(KernelError::PermissionDenied(format!("{:?}", cap)).into())
        }
    }

    pub async fn record_usage(&self, session_id: &str, tokens: u64, cost_usd: f64) -> Result<()> {
        let mut budget = self.budget.write().await;
        budget.record(session_id, tokens, cost_usd)
    }
}
