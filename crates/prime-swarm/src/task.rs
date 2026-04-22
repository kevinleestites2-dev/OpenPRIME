use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmTask {
    pub id: String,
    pub parent_id: Option<String>,
    pub description: String,
    pub agent_type: String,
    pub priority: u8,
    pub parallel: bool,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl SwarmTask {
    pub fn new(description: impl Into<String>, agent_type: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            parent_id: None,
            description: description.into(),
            agent_type: agent_type.into(),
            priority: 5,
            parallel: true,
            status: TaskStatus::Pending,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
        }
    }

    pub fn with_parent(mut self, parent_id: impl Into<String>) -> Self {
        self.parent_id = Some(parent_id.into());
        self
    }

    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmResult {
    pub task_id: String,
    pub agent_id: String,
    pub output: String,
    pub skill_created: Option<String>,
    pub token_count: u64,
    pub tool_calls: u64,
    pub duration_ms: u64,
    pub success: bool,
}

impl SwarmResult {
    pub fn success(task_id: impl Into<String>, agent_id: impl Into<String>, output: impl Into<String>) -> Self {
        Self {
            task_id: task_id.into(),
            agent_id: agent_id.into(),
            output: output.into(),
            skill_created: None,
            token_count: 0,
            tool_calls: 0,
            duration_ms: 0,
            success: true,
        }
    }

    pub fn failure(task_id: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            task_id: task_id.into(),
            agent_id: String::new(),
            output: reason.into(),
            skill_created: None,
            token_count: 0,
            tool_calls: 0,
            duration_ms: 0,
            success: false,
        }
    }
}
