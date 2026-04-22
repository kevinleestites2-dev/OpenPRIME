/// SkillWriter — generates SKILL.md content via LLM.
/// The actual LLM call is injected at runtime by prime-runtime.
/// This module defines the prompt template and output format.

pub fn skill_generation_prompt(task: &str, output: &str) -> String {
    format!(
        r#"You are writing a SKILL.md file for the OpenPRIME agent OS.
A SKILL.md is a concise, reusable reference document that captures domain
expertise so future agents can perform similar tasks more effectively.

The agent just completed the following task:
<task>{}</task>

The task output / findings were:
<output>{}</output>

Write a SKILL.md with these sections:
1. ## Overview — what this skill is about (2-3 sentences)
2. ## Key knowledge — bullet points of the most important facts learned
3. ## Best practices — how to approach this type of task
4. ## Common pitfalls — what to avoid
5. ## Example approach — a brief step-by-step for similar future tasks

Be concise. Focus on what's reusable. Do not repeat the full task output.
Output only the Markdown content, no preamble."#,
        task, output
    )
}
