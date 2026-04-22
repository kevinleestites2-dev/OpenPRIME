use crate::task::{JobOutcome, JobStatus, ScheduledJob};
use anyhow::Result;
use chrono::Utc;
use sqlx::{sqlite::SqlitePool, Row};
use uuid::Uuid;

pub struct SchedulerStore { pool: SqlitePool }

impl SchedulerStore {
    pub async fn new(db_path: &str) -> Result<Self> {
        let pool = SqlitePool::connect(&format!("sqlite://{}?mode=rwc", db_path)).await?;
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS scheduled_jobs (
                id TEXT PRIMARY KEY, name TEXT NOT NULL, description TEXT,
                cron TEXT, run_at TEXT, agent_type TEXT NOT NULL,
                task_prompt TEXT NOT NULL, status TEXT NOT NULL DEFAULT 'Pending',
                enabled INTEGER NOT NULL DEFAULT 1, run_count INTEGER DEFAULT 0,
                last_run TEXT, next_run TEXT, created_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS job_outcomes (
                id TEXT PRIMARY KEY, job_id TEXT NOT NULL, output TEXT NOT NULL,
                success INTEGER NOT NULL, tokens_used INTEGER DEFAULT 0,
                duration_ms INTEGER DEFAULT 0, ran_at TEXT NOT NULL
            );"
        ).execute(&pool).await.ok();
        Ok(Self { pool })
    }

    pub async fn save_job(&self, job: &ScheduledJob) -> Result<()> {
        sqlx::query(
            "INSERT OR REPLACE INTO scheduled_jobs
             (id,name,description,cron,run_at,agent_type,task_prompt,status,enabled,run_count,last_run,next_run,created_at)
             VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?)"
        )
        .bind(&job.id).bind(&job.name).bind(&job.description)
        .bind(&job.cron).bind(job.run_at.map(|t| t.to_rfc3339()))
        .bind(&job.agent_type).bind(&job.task_prompt)
        .bind(job.status.to_string()).bind(job.enabled as i32)
        .bind(job.run_count).bind(job.last_run.map(|t| t.to_rfc3339()))
        .bind(job.next_run.map(|t| t.to_rfc3339()))
        .bind(job.created_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    pub async fn all_jobs(&self) -> Result<Vec<ScheduledJob>> {
        let rows = sqlx::query("SELECT * FROM scheduled_jobs ORDER BY created_at DESC")
            .fetch_all(&self.pool).await?;
        let mut jobs = vec![];
        for row in rows {
            jobs.push(ScheduledJob {
                id: row.try_get("id")?,
                name: row.try_get("name")?,
                description: row.try_get::<String,_>("description").unwrap_or_default(),
                cron: row.try_get("cron").ok(),
                run_at: row.try_get::<String,_>("run_at").ok()
                    .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
                    .map(|t| t.with_timezone(&Utc)),
                agent_type: row.try_get("agent_type")?,
                task_prompt: row.try_get("task_prompt")?,
                status: match row.try_get::<String,_>("status").unwrap_or_default().as_str() {
                    "Running" => JobStatus::Running,
                    "Completed" => JobStatus::Completed,
                    "Failed" => JobStatus::Failed,
                    "Paused" => JobStatus::Paused,
                    _ => JobStatus::Pending,
                },
                enabled: row.try_get::<i32,_>("enabled").unwrap_or(1) == 1,
                run_count: row.try_get("run_count").unwrap_or(0),
                last_run: row.try_get::<String,_>("last_run").ok()
                    .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
                    .map(|t| t.with_timezone(&Utc)),
                next_run: row.try_get::<String,_>("next_run").ok()
                    .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
                    .map(|t| t.with_timezone(&Utc)),
                created_at: row.try_get::<String,_>("created_at").ok()
                    .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
                    .map(|t| t.with_timezone(&Utc))
                    .unwrap_or_else(Utc::now),
            });
        }
        Ok(jobs)
    }

    pub async fn mark_running(&self, id: &str) -> Result<()> {
        sqlx::query("UPDATE scheduled_jobs SET status='Running' WHERE id=?")
            .bind(id).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn mark_done(&self, id: &str, success: bool) -> Result<()> {
        let status = if success { "Completed" } else { "Failed" };
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "UPDATE scheduled_jobs SET status=?, last_run=?, run_count=run_count+1 WHERE id=?"
        ).bind(status).bind(&now).bind(id).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn save_outcome(&self, outcome: &JobOutcome) -> Result<()> {
        sqlx::query(
            "INSERT INTO job_outcomes (id,job_id,output,success,tokens_used,duration_ms,ran_at)
             VALUES (?,?,?,?,?,?,?)"
        )
        .bind(&outcome.id).bind(&outcome.job_id).bind(&outcome.output)
        .bind(outcome.success as i32).bind(outcome.tokens_used)
        .bind(outcome.duration_ms).bind(outcome.ran_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    pub async fn outcomes_for(&self, job_id: &str, limit: i64) -> Result<Vec<JobOutcome>> {
        let rows = sqlx::query(
            "SELECT * FROM job_outcomes WHERE job_id=? ORDER BY ran_at DESC LIMIT ?"
        ).bind(job_id).bind(limit).fetch_all(&self.pool).await?;
        let mut out = vec![];
        for row in rows {
            out.push(JobOutcome {
                id: row.try_get("id")?,
                job_id: row.try_get("job_id")?,
                output: row.try_get("output")?,
                success: row.try_get::<i32,_>("success")? == 1,
                tokens_used: row.try_get("tokens_used").unwrap_or(0),
                duration_ms: row.try_get("duration_ms").unwrap_or(0),
                ran_at: row.try_get::<String,_>("ran_at").ok()
                    .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
                    .map(|t| t.with_timezone(&Utc))
                    .unwrap_or_else(Utc::now),
            });
        }
        Ok(out)
    }

    pub async fn delete_job(&self, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM scheduled_jobs WHERE id=?").bind(id).execute(&self.pool).await?;
        Ok(())
    }
}
