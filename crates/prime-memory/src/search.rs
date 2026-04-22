// Full-text search helpers — thin wrappers used by the runtime
// to assemble context from memory before each agent turn.
use crate::store::MemoryStore;
use anyhow::Result;

pub async fn context_for(store: &MemoryStore, query: &str, limit: i64) -> Result<String> {
    let memories = store.search(query, limit).await?;
    if memories.is_empty() {
        return Ok(String::new());
    }
    let lines: Vec<String> = memories
        .iter()
        .map(|m| format!("[{}] {}", m.created_at, m.content))
        .collect();
    Ok(format!("## Relevant memories\n{}", lines.join("\n")))
}
