use crate::error::MemoryError;
use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePool, Row};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum MemoryKind {
    Fact,
    UserProfile,
    SessionSummary,
    SkillReference,
    Conversation,
}

impl std::fmt::Display for MemoryKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    pub id: String,
    pub kind: MemoryKind,
    pub content: String,
    pub session_id: String,
    pub agent_id: Option<String>,
    pub tags: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Memory {
    pub fn new(kind: MemoryKind, content: impl Into<String>, session_id: impl Into<String>) -> Self {
        let now = Utc::now().to_rfc3339();
        Self {
            id: Uuid::new_v4().to_string(),
            kind,
            content: content.into(),
            session_id: session_id.into(),
            agent_id: None,
            tags: vec![],
            created_at: now.clone(),
            updated_at: now,
        }
    }
}

pub struct MemoryStore {
    pool: SqlitePool,
}

impl MemoryStore {
    pub async fn new(db_path: &str) -> Result<Self> {
        let pool = SqlitePool::connect(&format!("sqlite://{}?mode=rwc", db_path)).await?;
        sqlx::query(include_str!("../migrations/001_init.sql"))
            .execute(&pool)
            .await
            .ok();
        Ok(Self { pool })
    }

    pub async fn save(&self, memory: &Memory) -> Result<()> {
        let tags = serde_json::to_string(&memory.tags)?;
        let kind = memory.kind.to_string();
        sqlx::query(
            "INSERT OR REPLACE INTO memories (id, kind, content, session_id, agent_id, tags, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&memory.id)
        .bind(&kind)
        .bind(&memory.content)
        .bind(&memory.session_id)
        .bind(&memory.agent_id)
        .bind(&tags)
        .bind(&memory.created_at)
        .bind(&memory.updated_at)
        .execute(&self.pool)
        .await?;

        sqlx::query("INSERT INTO memories_fts(id, content) VALUES (?, ?)")
            .bind(&memory.id)
            .bind(&memory.content)
            .execute(&self.pool)
            .await
            .ok();

        tracing::debug!("Saved memory {} ({:?})", memory.id, memory.kind);
        Ok(())
    }

    pub async fn search(&self, query: &str, limit: i64) -> Result<Vec<Memory>> {
        let rows = sqlx::query(
            "SELECT m.id, m.kind, m.content, m.session_id, m.agent_id, m.tags, m.created_at, m.updated_at
             FROM memories m
             JOIN memories_fts f ON m.id = f.id
             WHERE memories_fts MATCH ?
             ORDER BY rank
             LIMIT ?"
        )
        .bind(query)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        let mut results = vec![];
        for row in rows {
            let tags_str: String = row.try_get("tags").unwrap_or_default();
            let tags: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();
            let kind_str: String = row.try_get("kind")?;
            let kind = match kind_str.as_str() {
                "Fact" => MemoryKind::Fact,
                "UserProfile" => MemoryKind::UserProfile,
                "SessionSummary" => MemoryKind::SessionSummary,
                "SkillReference" => MemoryKind::SkillReference,
                _ => MemoryKind::Conversation,
            };
            results.push(Memory {
                id: row.try_get("id")?,
                kind,
                content: row.try_get("content")?,
                session_id: row.try_get("session_id")?,
                agent_id: row.try_get("agent_id").ok(),
                tags,
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
            });
        }
        Ok(results)
    }

    pub async fn recent(&self, limit: i64) -> Result<Vec<Memory>> {
        let rows = sqlx::query(
            "SELECT id, kind, content, session_id, agent_id, tags, created_at, updated_at
             FROM memories ORDER BY created_at DESC LIMIT ?"
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        let mut results = vec![];
        for row in rows {
            let tags_str: String = row.try_get("tags").unwrap_or_default();
            let tags: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();
            let kind_str: String = row.try_get("kind")?;
            let kind = match kind_str.as_str() {
                "Fact" => MemoryKind::Fact,
                "UserProfile" => MemoryKind::UserProfile,
                "SessionSummary" => MemoryKind::SessionSummary,
                "SkillReference" => MemoryKind::SkillReference,
                _ => MemoryKind::Conversation,
            };
            results.push(Memory {
                id: row.try_get("id")?,
                kind,
                content: row.try_get("content")?,
                session_id: row.try_get("session_id")?,
                agent_id: row.try_get("agent_id").ok(),
                tags,
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
            });
        }
        Ok(results)
    }
}
