use crate::engine::{Skill, SkillEngine};
use anyhow::Result;

/// Decides after a completed task whether a new skill should be written.
pub struct SkillLearner {
    pub engine: SkillEngine,
    pub complexity_threshold: f32,
}

impl SkillLearner {
    pub fn new(engine: SkillEngine) -> Self {
        Self { engine, complexity_threshold: 0.65 }
    }

    /// Score the complexity of a completed task (0.0 - 1.0).
    /// Real implementation would use token count, tool call count,
    /// and agent self-assessment. Placeholder logic here.
    pub fn score_complexity(&self, token_count: u64, tool_calls: u64) -> f32 {
        let token_score = (token_count as f32 / 10_000.0).min(1.0);
        let tool_score  = (tool_calls as f32 / 20.0).min(1.0);
        (token_score * 0.5 + tool_score * 0.5).min(1.0)
    }

    /// If complexity warrants it, save a new skill from the task output.
    pub async fn maybe_learn(
        &self,
        task_description: &str,
        task_output: &str,
        token_count: u64,
        tool_calls: u64,
    ) -> Result<Option<String>> {
        let score = self.score_complexity(token_count, tool_calls);
        if score < self.complexity_threshold {
            return Ok(None);
        }
        let name = slugify(task_description);
        let skill = Skill::new(
            &name,
            "auto-learned",
            task_description,
            format!(
                "## Task\n{}\n\n## What was learned\n{}\n\n## Notes\nAuto-generated skill (complexity score: {:.2})",
                task_description, task_output, score
            ),
        );
        self.engine.save(&skill).await?;
        tracing::info!("Learned new skill '{}' (score {:.2})", name, score);
        Ok(Some(name))
    }
}

fn slugify(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|p| !p.is_empty())
        .collect::<Vec<_>>()
        .join("-")
        .chars()
        .take(40)
        .collect()
}
