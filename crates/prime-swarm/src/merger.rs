use crate::task::SwarmResult;

/// Merges multiple sub-agent results into one coherent output.
pub struct ResultMerger;

impl ResultMerger {
    /// Concatenate results with headers per sub-task.
    pub fn merge(results: &[SwarmResult]) -> String {
        if results.is_empty() { return String::new(); }
        if results.len() == 1 { return results[0].output.clone(); }

        let mut parts = vec!["# Combined SWARM Results\n".to_string()];
        for (i, r) in results.iter().enumerate() {
            parts.push(format!("## Agent {} Result\n{}", i + 1, r.output));
        }
        let total_tokens: u64 = results.iter().map(|r| r.token_count).sum();
        let success_count = results.iter().filter(|r| r.success).count();
        parts.push(format!(
            "\n---\n*{}/{} agents succeeded · {} total tokens*",
            success_count, results.len(), total_tokens
        ));
        parts.join("\n\n")
    }

    pub fn summary_stats(results: &[SwarmResult]) -> MergeSummary {
        MergeSummary {
            total_tasks: results.len(),
            succeeded: results.iter().filter(|r| r.success).count(),
            failed: results.iter().filter(|r| !r.success).count(),
            total_tokens: results.iter().map(|r| r.token_count).sum(),
            total_tool_calls: results.iter().map(|r| r.tool_calls).sum(),
            total_duration_ms: results.iter().map(|r| r.duration_ms).sum(),
            skills_created: results.iter().filter_map(|r| r.skill_created.clone()).collect(),
        }
    }
}

#[derive(Debug)]
pub struct MergeSummary {
    pub total_tasks: usize,
    pub succeeded: usize,
    pub failed: usize,
    pub total_tokens: u64,
    pub total_tool_calls: u64,
    pub total_duration_ms: u64,
    pub skills_created: Vec<String>,
}
