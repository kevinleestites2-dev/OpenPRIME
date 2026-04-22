use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct ApiState {
    pub version: String,
    pub started_at: Arc<Instant>,
    pub agent_count: Arc<RwLock<usize>>,
    pub agents: Arc<RwLock<Vec<serde_json::Value>>>,
    pub skills: Arc<RwLock<Vec<serde_json::Value>>>,
    pub memories: Arc<RwLock<Vec<serde_json::Value>>>,
    pub sched_tasks: Arc<RwLock<Vec<serde_json::Value>>>,
}

impl ApiState {
    pub fn new() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").into(),
            started_at: Arc::new(Instant::now()),
            agent_count: Arc::new(RwLock::new(0)),
            agents: Arc::new(RwLock::new(vec![])),
            skills: Arc::new(RwLock::new(vec![])),
            memories: Arc::new(RwLock::new(vec![])),
            sched_tasks: Arc::new(RwLock::new(vec![])),
        }
    }
}
