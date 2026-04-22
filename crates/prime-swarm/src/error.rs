use thiserror::Error;
#[derive(Error, Debug)]
pub enum SwarmError {
    #[error("Task not found: {0}")]
    TaskNotFound(String),
    #[error("Agent spawn failed: {0}")]
    SpawnFailed(String),
    #[error("Merge failed: {0}")]
    MergeFailed(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
