use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "prime", about = "OpenPRIME — The legendary open agentic OS", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new OpenPRIME workspace
    Init,
    /// Start the OpenPRIME daemon (kernel + API + channels)
    Start {
        #[arg(long, default_value = "2357")] port: u16,
        #[arg(long)] detach: bool,
    },
    /// Stop the daemon
    Stop,
    /// Chat with an agent interactively
    Chat {
        #[arg(default_value = "researcher")] agent: String,
        #[arg(long)] model: Option<String>,
    },
    /// SWARM multi-agent commands
    Swarm {
        #[command(subcommand)] action: SwarmCmds,
    },
    /// Skill management
    Skills {
        #[command(subcommand)] action: SkillCmds,
    },
    /// Memory commands
    Memory {
        #[command(subcommand)] action: MemoryCmds,
    },
    /// Show system status and agent list
    Status,
    /// Run diagnostics
    Doctor,
    /// Update OpenPRIME
    Update,
}

#[derive(Subcommand)]
pub enum SwarmCmds {
    /// Spawn a parallel SWARM task
    Spawn { task: String, #[arg(long, default_value = "4")] agents: usize },
    /// List active SWARM tasks
    List,
    /// Kill a SWARM task by ID
    Kill { id: String },
}

#[derive(Subcommand)]
pub enum SkillCmds {
    /// List all available skills
    List,
    /// Show a skill's content
    Show { name: String },
    /// Delete a skill
    Delete { name: String },
}

#[derive(Subcommand)]
pub enum MemoryCmds {
    /// Search memory
    Search { query: String, #[arg(long, default_value = "10")] limit: i64 },
    /// Show recent memories
    Recent { #[arg(long, default_value = "20")] limit: i64 },
    /// Clear all memories (dangerous!)
    Clear { #[arg(long)] confirm: bool },
}
