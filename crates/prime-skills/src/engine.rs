use crate::error::SkillError;
use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub version: u32,
    pub domain: String,
    pub description: String,
    pub content: String,
    pub created_by: String,
    pub improved_count: u32,
    pub created_at: String,
    pub updated_at: String,
}

impl Skill {
    pub fn new(name: impl Into<String>, domain: impl Into<String>, description: impl Into<String>, content: impl Into<String>) -> Self {
        let now = Utc::now().to_rfc3339();
        let name = name.into();
        Self {
            name: name.clone(),
            version: 1,
            domain: domain.into(),
            description: description.into(),
            content: content.into(),
            created_by: "agent".into(),
            improved_count: 0,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    pub fn to_markdown(&self) -> String {
        format!(
            "# Skill: {}\n\n**Domain**: {}\n**Version**: {}\n**Description**: {}\n\n---\n\n{}",
            self.name, self.domain, self.version, self.description, self.content
        )
    }
}

pub struct SkillEngine {
    pub skills_dir: PathBuf,
}

impl SkillEngine {
    pub fn new(skills_dir: PathBuf) -> Self {
        std::fs::create_dir_all(&skills_dir).ok();
        Self { skills_dir }
    }

    pub async fn load(&self, name: &str) -> Result<Option<Skill>> {
        let path = self.skills_dir.join(format!("{}.md", name));
        if !path.exists() { return Ok(None); }
        let content = tokio::fs::read_to_string(&path).await?;
        Ok(Some(Skill {
            name: name.into(),
            version: 1,
            domain: name.into(),
            description: String::new(),
            content,
            created_by: "system".into(),
            improved_count: 0,
            created_at: String::new(),
            updated_at: String::new(),
        }))
    }

    pub async fn save(&self, skill: &Skill) -> Result<()> {
        let path = self.skills_dir.join(format!("{}.md", skill.name));
        tokio::fs::write(&path, skill.to_markdown()).await?;
        tracing::info!("Skill '{}' saved (v{})", skill.name, skill.version);
        Ok(())
    }

    pub async fn list(&self) -> Result<Vec<String>> {
        let mut names = vec![];
        let mut dir = tokio::fs::read_dir(&self.skills_dir).await?;
        while let Some(entry) = dir.next_entry().await? {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.ends_with(".md") {
                names.push(name.trim_end_matches(".md").to_string());
            }
        }
        Ok(names)
    }

    pub async fn delete(&self, name: &str) -> Result<()> {
        let path = self.skills_dir.join(format!("{}.md", name));
        if path.exists() { tokio::fs::remove_file(path).await?; }
        Ok(())
    }

    /// Inject relevant skills into agent context as a string
    pub async fn inject_context(&self, skill_names: &[&str]) -> Result<String> {
        let mut parts = vec![];
        for name in skill_names {
            if let Some(skill) = self.load(name).await? {
                parts.push(format!("## Skill: {}\n{}", skill.name, skill.content));
            }
        }
        if parts.is_empty() { return Ok(String::new()); }
        Ok(format!("# Injected Skills\n\n{}", parts.join("\n\n---\n\n")))
    }
}
