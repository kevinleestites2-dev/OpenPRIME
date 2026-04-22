use crate::{
    error::RuntimeError,
    llm::{LlmDriver, LlmMessage, Role},
    tools::{ToolCall, ToolRegistry},
    agent::AgentConfig,
};
use anyhow::Result;
use std::sync::Arc;

pub struct AgentLoop {
    pub driver: Arc<dyn LlmDriver>,
    pub tools: Arc<ToolRegistry>,
    pub config: AgentConfig,
}

impl AgentLoop {
    pub fn new(driver: Arc<dyn LlmDriver>, tools: Arc<ToolRegistry>, config: AgentConfig) -> Self {
        Self { driver, tools, config }
    }

    /// Run the observe → think → act loop until task is complete or max iterations reached.
    pub async fn run(&self, task: &str, context: &str) -> Result<LoopOutput, RuntimeError> {
        let mut messages: Vec<LlmMessage> = vec![
            LlmMessage::system(format!(
                "{}\n\n## Available tools\n{}\n\n## Context\n{}",
                self.config.system_prompt,
                self.tool_descriptions(),
                context,
            )),
            LlmMessage::user(task),
        ];

        let mut total_input  = 0u64;
        let mut total_output = 0u64;
        let mut tool_calls   = 0u64;
        let mut iterations   = 0u32;

        loop {
            if iterations >= self.config.max_iterations {
                return Err(RuntimeError::MaxIterations);
            }
            iterations += 1;

            let response = self.driver.complete(&messages, self.config.max_tokens).await?;
            total_input  += response.input_tokens;
            total_output += response.output_tokens;

            // Check for tool calls in the response (simple XML-style parsing)
            if let Some(tool_call) = self.parse_tool_call(&response.content) {
                tool_calls += 1;
                let result = self.tools.execute(&tool_call).await;
                messages.push(LlmMessage::assistant(response.content.clone()));
                messages.push(LlmMessage::user(format!(
                    "<tool_result name=\"{}\" success=\"{}\">\n{}\n</tool_result>",
                    result.tool, result.success, result.output
                )));
                continue;
            }

            // No tool call = final answer
            return Ok(LoopOutput {
                output: response.content,
                total_tokens: total_input + total_output,
                tool_calls,
                iterations,
            });
        }
    }

    fn tool_descriptions(&self) -> String {
        self.tools.list().iter()
            .map(|(name, desc)| format!("- **{}**: {}", name, desc))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn parse_tool_call(&self, content: &str) -> Option<ToolCall> {
        // Look for <tool name="...">{ ... }</tool> pattern
        let start = content.find("<tool")?;
        let end   = content.find("</tool>")?;
        let block = &content[start..=end + 6];
        let name_start = block.find("name=\"")? + 6;
        let name_end   = block[name_start..].find('"')? + name_start;
        let name = block[name_start..name_end].to_string();
        let args_start = block.find('>')?  + 1;
        let args_end   = block.find("</tool>")?;
        let args_str   = block[args_start..args_end].trim();
        let args = serde_json::from_str(args_str).unwrap_or(serde_json::json!({}));
        Some(ToolCall { name, args })
    }
}

#[derive(Debug)]
pub struct LoopOutput {
    pub output: String,
    pub total_tokens: u64,
    pub tool_calls: u64,
    pub iterations: u32,
}
