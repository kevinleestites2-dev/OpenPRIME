use crate::state::ApiState;
use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{Html, IntoResponse},
    Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Dashboard ────────────────────────────────────────────────────────────────

pub async fn dashboard() -> Html<&'static str> {
    Html(crate::dashboard::html())
}

// ── Health / Status ───────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct HealthResponse { pub status: String, pub version: String, pub agents: usize }

pub async fn health(State(state): State<ApiState>) -> Json<HealthResponse> {
    let agents = *state.agent_count.read().await;
    Json(HealthResponse { status: "ok".into(), version: state.version.clone(), agents })
}

#[derive(Serialize)]
pub struct StatusResponse { pub name: String, pub version: String, pub status: String, pub uptime_secs: u64 }

pub async fn status(State(state): State<ApiState>) -> Json<StatusResponse> {
    let uptime = state.started_at.elapsed().as_secs();
    Json(StatusResponse { name: "OpenPRIME".into(), version: state.version.clone(), status: "running".into(), uptime_secs: uptime })
}

// ── Agents API ────────────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct AgentListResponse { pub agents: Vec<serde_json::Value> }

pub async fn list_agents(State(state): State<ApiState>) -> Json<AgentListResponse> {
    let agents = state.agents.read().await.clone();
    Json(AgentListResponse { agents })
}

#[derive(Deserialize)]
pub struct SpawnRequest { pub name: String, pub agent_type: String }

#[derive(Serialize)]
pub struct SpawnResponse { pub id: String, pub name: String, pub agent_type: String }

pub async fn spawn_agent(State(state): State<ApiState>, Json(req): Json<SpawnRequest>) -> Json<SpawnResponse> {
    let id = format!("agent-{}", uuid::Uuid::new_v4());
    let agent = serde_json::json!({
        "id": id,
        "name": req.name,
        "agent_type": req.agent_type,
        "state": "Idle",
        "task_count": 0,
        "token_count": 0,
        "spawned_at": chrono::Utc::now().to_rfc3339(),
    });
    state.agents.write().await.push(agent);
    *state.agent_count.write().await += 1;
    Json(SpawnResponse { id, name: req.name, agent_type: req.agent_type })
}

pub async fn kill_agent(State(state): State<ApiState>, Path(id): Path<String>) -> StatusCode {
    let mut agents = state.agents.write().await;
    let before = agents.len();
    agents.retain(|a| a["id"].as_str() != Some(&id));
    if agents.len() < before {
        *state.agent_count.write().await = agents.len();
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

// ── Skills API ────────────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct SkillsResponse { pub skills: Vec<serde_json::Value> }

pub async fn list_skills(State(state): State<ApiState>) -> Json<SkillsResponse> {
    Json(SkillsResponse { skills: state.skills.read().await.clone() })
}

// ── Memory API ────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct RecentQuery { pub limit: Option<i64> }

#[derive(Serialize)]
pub struct MemoryResponse { pub memories: Vec<serde_json::Value> }

pub async fn recent_memory(State(state): State<ApiState>, Query(q): Query<RecentQuery>) -> Json<MemoryResponse> {
    let all = state.memories.read().await.clone();
    let limit = q.limit.unwrap_or(20) as usize;
    let memories = all.into_iter().take(limit).collect();
    Json(MemoryResponse { memories })
}

pub async fn search_memory(State(state): State<ApiState>, Query(params): Query<HashMap<String,String>>) -> Json<MemoryResponse> {
    let q = params.get("q").cloned().unwrap_or_default().to_lowercase();
    let all = state.memories.read().await.clone();
    let memories = all.into_iter().filter(|m| {
        m["content"].as_str().unwrap_or("").to_lowercase().contains(&q)
    }).take(50).collect();
    Json(MemoryResponse { memories })
}

// ── Scheduler API ─────────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct SchedulerResponse { pub tasks: Vec<serde_json::Value> }

pub async fn list_scheduler(State(state): State<ApiState>) -> Json<SchedulerResponse> {
    Json(SchedulerResponse { tasks: state.sched_tasks.read().await.clone() })
}

#[derive(Deserialize)]
pub struct AddTaskRequest {
    pub name: String,
    pub description: String,
    pub agent_type: String,
    pub cron: Option<String>,
    pub deliver_to: Option<String>,
}

pub async fn add_sched_task(State(state): State<ApiState>, Json(req): Json<AddTaskRequest>) -> Json<serde_json::Value> {
    let id = format!("sched-{}", uuid::Uuid::new_v4());
    let task = serde_json::json!({
        "id": id,
        "name": req.name,
        "description": req.description,
        "agent_type": req.agent_type,
        "cron": req.cron,
        "deliver_to": req.deliver_to,
        "enabled": true,
        "run_count": 0,
        "last_run": null,
    });
    state.sched_tasks.write().await.push(task.clone());
    Json(task)
}

pub async fn toggle_sched_task(State(state): State<ApiState>, Path(id): Path<String>) -> StatusCode {
    let mut tasks = state.sched_tasks.write().await;
    for t in tasks.iter_mut() {
        if t["id"].as_str() == Some(&id) {
            let cur = t["enabled"].as_bool().unwrap_or(true);
            t["enabled"] = serde_json::Value::Bool(!cur);
            return StatusCode::OK;
        }
    }
    StatusCode::NOT_FOUND
}

pub async fn delete_sched_task(State(state): State<ApiState>, Path(id): Path<String>) -> StatusCode {
    let mut tasks = state.sched_tasks.write().await;
    let before = tasks.len();
    tasks.retain(|t| t["id"].as_str() != Some(&id));
    if tasks.len() < before { StatusCode::OK } else { StatusCode::NOT_FOUND }
}

// ── OpenAI-compatible ─────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct ChatRequest { pub model: Option<String>, pub messages: Vec<serde_json::Value>, pub max_tokens: Option<u32> }

#[derive(Serialize)]
pub struct ChatResponse { pub id: String, pub object: String, pub model: String, pub choices: Vec<serde_json::Value>, pub usage: serde_json::Value }

pub async fn chat_completions(State(_state): State<ApiState>, Json(req): Json<ChatRequest>) -> Json<ChatResponse> {
    Json(ChatResponse {
        id: format!("chatcmpl-{}", uuid::Uuid::new_v4()),
        object: "chat.completion".into(),
        model: req.model.unwrap_or_else(|| "openprime".into()),
        choices: vec![serde_json::json!({ "index": 0, "message": { "role": "assistant", "content": "OpenPRIME API ready" }, "finish_reason": "stop" })],
        usage: serde_json::json!({ "prompt_tokens": 0, "completion_tokens": 0, "total_tokens": 0 }),
    })
}
