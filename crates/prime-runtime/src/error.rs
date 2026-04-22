use thiserror::Error;
#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("LLM error: {0}")]
    LlmError(String),
    #[error("Tool not found: {0}")]
    ToolNotFound(String),
    #[error("Tool execution failed: {0}")]
    ToolFailed(String),
    #[error("Context window exceeded")]
    ContextExceeded,
    #[error("Max iterations reached")]
    MaxIterations,
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
