use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub name: String,
    pub agent_type: String,
    pub system_prompt: String,
    pub max_tokens: u32,
    pub max_iterations: u32,
    pub provider: String,
    pub model: String,
    pub skills: Vec<String>,
}

impl AgentConfig {
    pub fn researcher(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            agent_type: "researcher".into(),
            system_prompt: "You are an expert research agent. You find accurate, up-to-date information, cite sources, and produce well-structured reports. Use the web_search tool to find information. After completing your research, summarize your findings clearly.".into(),
            max_tokens: 4096,
            max_iterations: 10,
            provider: "anthropic".into(),
            model: "claude-sonnet-4-20250514".into(),
            skills: vec!["research".into()],
        }
    }

    pub fn coder(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            agent_type: "coder".into(),
            system_prompt: "You are an expert software engineer. You write clean, well-documented, production-quality code. Use read_file and write_file tools to work with the codebase. Always explain what you changed and why.".into(),
            max_tokens: 8192,
            max_iterations: 20,
            provider: "anthropic".into(),
            model: "claude-sonnet-4-20250514".into(),
            skills: vec!["coding".into()],
        }
    }

    pub fn orchestrator(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            agent_type: "orchestrator".into(),
            system_prompt: "You are the OpenPRIME orchestrator. You break complex tasks into parallel sub-tasks, delegate to specialist agents, and synthesize their results into coherent outputs.".into(),
            max_tokens: 4096,
            max_iterations: 15,
            provider: "anthropic".into(),
            model: "claude-sonnet-4-20250514".into(),
            skills: vec![],
        }
    }
}

pub struct Agent {
    pub config: AgentConfig,
    pub id: String,
}

impl Agent {
    pub fn new(config: AgentConfig) -> Self {
        Self { id: uuid::Uuid::new_v4().to_string(), config }
    }
}
