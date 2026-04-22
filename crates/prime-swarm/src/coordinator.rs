use crate::{
    error::SwarmError,
    merger::{MergeSummary, ResultMerger},
    router::TaskRouter,
    task::{SwarmResult, SwarmTask, TaskStatus},
};
use anyhow::Result;
use chrono::Utc;
use dashmap::DashMap;
use std::sync::Arc;
use futures::future::join_all;

pub struct SwarmCoordinator {
    pub max_agents: usize,
    pub tasks: Arc<DashMap<String, SwarmTask>>,
    pub results: Arc<DashMap<String, SwarmResult>>,
}

impl SwarmCoordinator {
    pub fn new(max_agents: usize) -> Self {
        Self {
            max_agents,
            tasks: Arc::new(DashMap::new()),
            results: Arc::new(DashMap::new()),
        }
    }

    /// High-level entry point: receive a task, decompose, run in parallel, merge.
    pub async fn run(
        &self,
        description: &str,
        executor: impl Fn(SwarmTask) -> futures::future::BoxFuture<'static, SwarmResult> + Send + Sync + 'static,
    ) -> Result<(String, MergeSummary)> {
        let sub_tasks = TaskRouter::decompose(description, self.max_agents);
        tracing::info!("SWARM: decomposed '{}' into {} sub-tasks", description, sub_tasks.len());

        let executor = Arc::new(executor);
        let futures: Vec<_> = sub_tasks.into_iter().map(|task| {
            let exec = executor.clone();
            async move { exec(task).await }
        }).collect();

        let results = join_all(futures).await;
        let merged = ResultMerger::merge(&results);
        let summary = ResultMerger::summary_stats(&results);

        for r in &results {
            self.results.insert(r.task_id.clone(), r.clone());
        }

        tracing::info!(
            "SWARM complete: {}/{} succeeded, {} tokens",
            summary.succeeded, summary.total_tasks, summary.total_tokens
        );
        Ok((merged, summary))
    }

    pub fn active_count(&self) -> usize {
        self.tasks.iter().filter(|t| t.status == TaskStatus::Running).count()
    }

    pub fn result(&self, task_id: &str) -> Option<SwarmResult> {
        self.results.get(task_id).map(|r| r.clone())
    }
}
