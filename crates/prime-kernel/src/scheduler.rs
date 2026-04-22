use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledTask {
    pub id: String,
    pub name: String,
    pub description: String,
    pub cron: Option<String>,
    pub run_at: Option<DateTime<Utc>>,
    pub agent_type: String,
    pub enabled: bool,
    pub last_run: Option<DateTime<Utc>>,
    pub run_count: u64,
}

impl ScheduledTask {
    pub fn once(name: impl Into<String>, at: DateTime<Utc>, agent_type: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.into(),
            description: description.into(),
            cron: None,
            run_at: Some(at),
            agent_type: agent_type.into(),
            enabled: true,
            last_run: None,
            run_count: 0,
        }
    }

    pub fn recurring(name: impl Into<String>, cron: impl Into<String>, agent_type: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.into(),
            description: description.into(),
            cron: Some(cron.into()),
            run_at: None,
            agent_type: agent_type.into(),
            enabled: true,
            last_run: None,
            run_count: 0,
        }
    }

    pub fn is_due(&self) -> bool {
        if !self.enabled { return false; }
        if let Some(run_at) = self.run_at {
            return Utc::now() >= run_at && self.run_count == 0;
        }
        false
    }
}

#[derive(Debug, Default)]
pub struct Scheduler {
    pub tasks: BTreeMap<String, ScheduledTask>,
}

impl Scheduler {
    pub fn add(&mut self, task: ScheduledTask) -> String {
        let id = task.id.clone();
        tracing::info!("Scheduled task '{}' ({})", task.name, id);
        self.tasks.insert(id.clone(), task);
        id
    }

    pub fn remove(&mut self, id: &str) -> Option<ScheduledTask> {
        self.tasks.remove(id)
    }

    pub fn due(&self) -> Vec<&ScheduledTask> {
        self.tasks.values().filter(|t| t.is_due()).collect()
    }

    pub fn mark_run(&mut self, id: &str) -> Result<()> {
        if let Some(task) = self.tasks.get_mut(id) {
            task.last_run = Some(Utc::now());
            task.run_count += 1;
            if task.cron.is_none() {
                task.enabled = false;
            }
        }
        Ok(())
    }
}
