use thiserror::Error;
#[derive(Error, Debug)]
pub enum SchedulerError {
    #[error("Job not found: {0}")]
    NotFound(String),
    #[error("Invalid cron expression: {0}")]
    InvalidCron(String),
    #[error(transparent)]
    Db(#[from] sqlx::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
