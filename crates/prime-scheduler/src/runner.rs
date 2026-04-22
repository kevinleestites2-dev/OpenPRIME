use crate::{store::SchedulerStore, task::{JobOutcome, ScheduledJob}};
use anyhow::Result;
use chrono::Utc;
use std::sync::Arc;
use tokio::{sync::RwLock, time::{interval, Duration}};
use uuid::Uuid;

pub struct SchedulerRunner {
    pub store: Arc<SchedulerStore>,
    pub jobs: Arc<RwLock<Vec<ScheduledJob>>>,
}

impl SchedulerRunner {
    pub async fn new(db_path: &str) -> Result<Self> {
        let store = Arc::new(SchedulerStore::new(db_path).await?);
        let jobs = store.all_jobs().await?;
        Ok(Self {
            jobs: Arc::new(RwLock::new(jobs)),
            store,
        })
    }

    pub async fn add_job(&self, job: ScheduledJob) -> Result<String> {
        let id = job.id.clone();
        self.store.save_job(&job).await?;
        self.jobs.write().await.push(job);
        tracing::info!("Scheduled job '{}' ({})", id, id);
        Ok(id)
    }

    pub async fn remove_job(&self, id: &str) -> Result<()> {
        self.store.delete_job(id).await?;
        self.jobs.write().await.retain(|j| j.id != id);
        Ok(())
    }

    /// Background tick loop — runs every 60 seconds, fires due jobs.
    pub fn start(self: Arc<Self>) {
        tokio::spawn(async move {
            let mut ticker = interval(Duration::from_secs(60));
            loop {
                ticker.tick().await;
                if let Err(e) = self.tick().await {
                    tracing::error!("Scheduler tick error: {}", e);
                }
            }
        });
    }

    async fn tick(&self) -> Result<()> {
        let due: Vec<ScheduledJob> = {
            let jobs = self.jobs.read().await;
            jobs.iter().filter(|j| j.is_due()).cloned().collect()
        };

        for job in due {
            let store = self.store.clone();
            let job_clone = job.clone();
            tokio::spawn(async move {
                tracing::info!("Running scheduled job '{}' ({})", job_clone.name, job_clone.id);
                let start = std::time::Instant::now();
                store.mark_running(&job_clone.id).await.ok();

                // Execute — in full impl, this calls AgentLoop via prime-runtime
                let output = format!("Scheduled job '{}' executed at {}", job_clone.name, Utc::now());
                let success = true;
                let duration_ms = start.elapsed().as_millis() as i64;

                store.mark_done(&job_clone.id, success).await.ok();
                store.save_outcome(&JobOutcome {
                    id: Uuid::new_v4().to_string(),
                    job_id: job_clone.id.clone(),
                    output,
                    success,
                    tokens_used: 0,
                    duration_ms,
                    ran_at: Utc::now(),
                }).await.ok();
            });
        }
        Ok(())
    }
}
