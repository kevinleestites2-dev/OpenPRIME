use anyhow::Result;
use sqlx::{SqlitePool, Row};
use std::collections::HashMap;
use chrono::Utc;

pub struct UserProfile {
    pool: SqlitePool,
}

impl UserProfile {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn set(&self, key: &str, value: &str) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "INSERT OR REPLACE INTO user_profile (key, value, updated_at) VALUES (?, ?, ?)"
        )
        .bind(key).bind(value).bind(&now)
        .execute(&self.pool).await?;
        Ok(())
    }

    pub async fn get(&self, key: &str) -> Result<Option<String>> {
        let row = sqlx::query("SELECT value FROM user_profile WHERE key = ?")
            .bind(key)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(|r| r.try_get("value").unwrap_or_default()))
    }

    pub async fn all(&self) -> Result<HashMap<String, String>> {
        let rows = sqlx::query("SELECT key, value FROM user_profile")
            .fetch_all(&self.pool).await?;
        let mut map = HashMap::new();
        for row in rows {
            let k: String = row.try_get("key")?;
            let v: String = row.try_get("value")?;
            map.insert(k, v);
        }
        Ok(map)
    }
}
