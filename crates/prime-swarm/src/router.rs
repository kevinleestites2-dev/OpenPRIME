use crate::task::SwarmTask;

/// Routes a high-level task description into typed sub-tasks.
pub struct TaskRouter;

impl TaskRouter {
    /// Decompose a complex task into parallel sub-tasks.
    /// In production this uses the LLM; here we provide the interface.
    pub fn decompose(description: &str, max_agents: usize) -> Vec<SwarmTask> {
        // Simple heuristic decomposition for bootstrapping.
        // Real implementation calls the LLM with a decomposition prompt.
        vec![SwarmTask::new(description, "researcher")]
    }

    /// Pick the best agent type for a given sub-task description.
    pub fn route(description: &str) -> &'static str {
        let d = description.to_lowercase();
        if d.contains("code") || d.contains("implement") || d.contains("fix") {
            "coder"
        } else if d.contains("research") || d.contains("find") || d.contains("search") {
            "researcher"
        } else if d.contains("analyze") || d.contains("compare") || d.contains("evaluate") {
            "analyst"
        } else if d.contains("write") || d.contains("draft") || d.contains("create") {
            "writer"
        } else {
            "researcher"
        }
    }
}
