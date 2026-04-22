use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, Row};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub agent_id: String,
    pub summary: Option<String>,
    pub started_at: String,
    pub ended_at: Option<String>,
    pub message_count: i64,
    pub token_count: i64,
}

impl Session {
    pub fn new(agent_id: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            agent_id: agent_id.into(),
            summary: None,
            started_at: Utc::now().to_rfc3339(),
            ended_at: None,
            message_count: 0,
            token_count: 0,
        }
    }
}

pub struct SessionStore {
    pool: SqlitePool,
}

impl SessionStore {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn start(&self, session: &Session) -> Result<()> {
        sqlx::query(
            "INSERT INTO sessions (id, agent_id, started_at, message_count, token_count)
             VALUES (?, ?, ?, 0, 0)"
        )
        .bind(&session.id).bind(&session.agent_id).bind(&session.started_at)
        .execute(&self.pool).await?;
        Ok(())
    }

    pub async fn end(&self, id: &str, summary: Option<&str>) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        sqlx::query("UPDATE sessions SET ended_at = ?, summary = ? WHERE id = ?")
            .bind(&now).bind(summary).bind(id)
            .execute(&self.pool).await?;
        Ok(())
    }

    pub async fn increment(&self, id: &str, messages: i64, tokens: i64) -> Result<()> {
        sqlx::query(
            "UPDATE sessions SET message_count = message_count + ?, token_count = token_count + ? WHERE id = ?"
        )
        .bind(messages).bind(tokens).bind(id)
        .execute(&self.pool).await?;
        Ok(())
    }

    pub async fn recent(&self, limit: i64) -> Result<Vec<Session>> {
        let rows = sqlx::query(
            "SELECT id, agent_id, summary, started_at, ended_at, message_count, token_count
             FROM sessions ORDER BY started_at DESC LIMIT ?"
        )
        .bind(limit).fetch_all(&self.pool).await?;

        let mut out = vec![];
        for row in rows {
            out.push(Session {
                id: row.try_get("id")?,
                agent_id: row.try_get("agent_id")?,
                summary: row.try_get("summary").ok(),
                started_at: row.try_get("started_at")?,
                ended_at: row.try_get("ended_at").ok(),
                message_count: row.try_get("message_count")?,
                token_count: row.try_get("token_count")?,
            });
        }
        Ok(out)
    }
}
