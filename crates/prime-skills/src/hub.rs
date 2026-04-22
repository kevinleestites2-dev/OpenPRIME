use crate::engine::{Skill, SkillEngine};
use anyhow::Result;
use std::collections::HashMap;

/// In-memory index of all available skills for fast lookup.
pub struct SkillHub {
    pub engine: SkillEngine,
    index: HashMap<String, String>, // name -> description
}

impl SkillHub {
    pub async fn build(engine: SkillEngine) -> Result<Self> {
        let names = engine.list().await?;
        let mut index = HashMap::new();
        for name in names {
            index.insert(name.clone(), name.clone());
        }
        Ok(Self { engine, index })
    }

    pub fn search(&self, query: &str) -> Vec<String> {
        let q = query.to_lowercase();
        self.index.keys()
            .filter(|k| k.to_lowercase().contains(&q))
            .cloned()
            .collect()
    }

    pub async fn get(&self, name: &str) -> Result<Option<Skill>> {
        self.engine.load(name).await
    }

    pub fn all_names(&self) -> Vec<String> {
        self.index.keys().cloned().collect()
    }

    pub async fn refresh(&mut self) -> Result<()> {
        let names = self.engine.list().await?;
        self.index.clear();
        for name in names {
            self.index.insert(name.clone(), name);
        }
        Ok(())
    }
}
