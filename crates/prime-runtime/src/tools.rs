use crate::error::RuntimeError;
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub name: String,
    pub args: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub tool: String,
    pub output: String,
    pub success: bool,
}

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    async fn run(&self, args: serde_json::Value) -> Result<String, RuntimeError>;
}

pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        let mut r = Self { tools: HashMap::new() };
        r.register(Box::new(ReadFileTool));
        r.register(Box::new(WriteFileTool));
        r.register(Box::new(WebSearchTool));
        r.register(Box::new(ShellTool));
        r
    }

    pub fn register(&mut self, tool: Box<dyn Tool>) {
        self.tools.insert(tool.name().into(), tool);
    }

    pub async fn execute(&self, call: &ToolCall) -> ToolResult {
        match self.tools.get(&call.name) {
            None => ToolResult { tool: call.name.clone(), output: format!("Tool '{}' not found", call.name), success: false },
            Some(tool) => match tool.run(call.args.clone()).await {
                Ok(out) => ToolResult { tool: call.name.clone(), output: out, success: true },
                Err(e)  => ToolResult { tool: call.name.clone(), output: e.to_string(), success: false },
            },
        }
    }

    pub fn list(&self) -> Vec<(&str, &str)> {
        self.tools.values().map(|t| (t.name(), t.description())).collect()
    }
}

// ── Built-in tools ───────────────────────────────────────────────────────────

struct ReadFileTool;
#[async_trait] impl Tool for ReadFileTool {
    fn name(&self) -> &str { "read_file" }
    fn description(&self) -> &str { "Read the contents of a file" }
    async fn run(&self, args: serde_json::Value) -> Result<String, RuntimeError> {
        let path = args["path"].as_str().ok_or_else(|| RuntimeError::ToolFailed("path required".into()))?;
        tokio::fs::read_to_string(path).await.map_err(|e| RuntimeError::ToolFailed(e.to_string()))
    }
}

struct WriteFileTool;
#[async_trait] impl Tool for WriteFileTool {
    fn name(&self) -> &str { "write_file" }
    fn description(&self) -> &str { "Write content to a file" }
    async fn run(&self, args: serde_json::Value) -> Result<String, RuntimeError> {
        let path    = args["path"].as_str().ok_or_else(|| RuntimeError::ToolFailed("path required".into()))?;
        let content = args["content"].as_str().ok_or_else(|| RuntimeError::ToolFailed("content required".into()))?;
        tokio::fs::write(path, content).await.map_err(|e| RuntimeError::ToolFailed(e.to_string()))?;
        Ok(format!("Written {} bytes to {}", content.len(), path))
    }
}

struct WebSearchTool;
#[async_trait] impl Tool for WebSearchTool {
    fn name(&self) -> &str { "web_search" }
    fn description(&self) -> &str { "Search the web for information" }
    async fn run(&self, args: serde_json::Value) -> Result<String, RuntimeError> {
        let query = args["query"].as_str().ok_or_else(|| RuntimeError::ToolFailed("query required".into()))?;
        // Placeholder — wire to real search API (Brave, Serper, etc.)
        Ok(format!("Search results for '{}': [wire to search API]", query))
    }
}

struct ShellTool;
#[async_trait] impl Tool for ShellTool {
    fn name(&self) -> &str { "shell" }
    fn description(&self) -> &str { "Execute a shell command (requires approval)" }
    async fn run(&self, args: serde_json::Value) -> Result<String, RuntimeError> {
        let cmd = args["command"].as_str().ok_or_else(|| RuntimeError::ToolFailed("command required".into()))?;
        let out = tokio::process::Command::new("sh")
            .arg("-c").arg(cmd)
            .output().await
            .map_err(|e| RuntimeError::ToolFailed(e.to_string()))?;
        Ok(String::from_utf8_lossy(&out.stdout).into_owned())
    }
}
