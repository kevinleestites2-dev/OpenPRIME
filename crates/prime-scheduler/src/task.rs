use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum JobStatus { Pending, Running, Completed, Failed, Paused }

impl std::fmt::Display for JobStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{:?}", self) }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledJob {
    pub id: String,
    pub name: String,
    pub description: String,
    pub cron: Option<String>,       // "0 */6 * * *" = every 6 hours
    pub run_at: Option<DateTime<Utc>>, // one-shot
    pub agent_type: String,
    pub task_prompt: String,
    pub status: JobStatus,
    pub enabled: bool,
    pub run_count: i64,
    pub last_run: Option<DateTime<Utc>>,
    pub next_run: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl ScheduledJob {
    pub fn recurring(
        name: impl Into<String>,
        cron: impl Into<String>,
        agent_type: impl Into<String>,
        task_prompt: impl Into<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.into(),
            description: String::new(),
            cron: Some(cron.into()),
            run_at: None,
            agent_type: agent_type.into(),
            task_prompt: task_prompt.into(),
            status: JobStatus::Pending,
            enabled: true,
            run_count: 0,
            last_run: None,
            next_run: Some(now),
            created_at: now,
        }
    }

    pub fn once(
        name: impl Into<String>,
        run_at: DateTime<Utc>,
        agent_type: impl Into<String>,
        task_prompt: impl Into<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.into(),
            description: String::new(),
            cron: None,
            run_at: Some(run_at),
            agent_type: agent_type.into(),
            task_prompt: task_prompt.into(),
            status: JobStatus::Pending,
            enabled: true,
            run_count: 0,
            last_run: None,
            next_run: Some(run_at),
            created_at: now,
        }
    }

    pub fn is_due(&self) -> bool {
        if !self.enabled { return false; }
        if self.status == JobStatus::Running { return false; }
        match self.next_run {
            Some(t) => Utc::now() >= t,
            None => false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobOutcome {
    pub id: String,
    pub job_id: String,
    pub output: String,
    pub success: bool,
    pub tokens_used: i64,
    pub duration_ms: i64,
    pub ran_at: DateTime<Utc>,
}
