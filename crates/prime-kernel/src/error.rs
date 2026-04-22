use thiserror::Error;

#[derive(Error, Debug)]
pub enum KernelError {
    #[error("Agent not found: {0}")]
    AgentNotFound(String),
    #[error("Agent already exists: {0}")]
    AgentExists(String),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    #[error("Budget exceeded: used {used}, limit {limit}")]
    BudgetExceeded { used: u64, limit: u64 },
    #[error("Scheduler error: {0}")]
    SchedulerError(String),
    #[error("Config error: {0}")]
    ConfigError(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
