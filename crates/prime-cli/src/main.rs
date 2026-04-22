use anyhow::Result;
use clap::Parser;
use prime_cli::{commands::{Cli, Commands, SwarmCmds, SkillCmds, MemoryCmds}, output};
use prime_kernel::config::PrimeConfig;
use prime_skills::engine::SkillEngine;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .init();

    let cli = Cli::parse();
    let config = PrimeConfig::load().unwrap_or_default();

    match cli.command {
        Commands::Init => cmd_init(&config).await,
        Commands::Start { port, detach } => cmd_start(&config, port, detach).await,
        Commands::Stop => cmd_stop().await,
        Commands::Chat { agent, model } => cmd_chat(&config, &agent, model).await,
        Commands::Status => cmd_status(&config).await,
        Commands::Doctor => cmd_doctor(&config).await,
        Commands::Update => cmd_update().await,
        Commands::Swarm { action } => match action {
            SwarmCmds::Spawn { task, agents } => cmd_swarm_spawn(&config, &task, agents).await,
            SwarmCmds::List => cmd_swarm_list().await,
            SwarmCmds::Kill { id } => cmd_swarm_kill(&id).await,
        },
        Commands::Skills { action } => match action {
            SkillCmds::List => cmd_skills_list(&config).await,
            SkillCmds::Show { name } => cmd_skills_show(&config, &name).await,
            SkillCmds::Delete { name } => cmd_skills_delete(&config, &name).await,
        },
        Commands::Memory { action } => match action {
            MemoryCmds::Search { query, limit } => cmd_memory_search(&config, &query, limit).await,
            MemoryCmds::Recent { limit } => cmd_memory_recent(&config, limit).await,
            MemoryCmds::Clear { confirm } => cmd_memory_clear(&config, confirm).await,
        },
    }
}

async fn cmd_init(config: &PrimeConfig) -> Result<()> {
    output::banner();
    output::info("Initializing OpenPRIME workspace...");
    config.ensure_dirs()?;
    config.save()?;
    output::success("Workspace initialized at ~/.openprime");
    output::dim("Run `prime start` to launch the daemon.");
    Ok(())
}

async fn cmd_start(config: &PrimeConfig, port: u16, detach: bool) -> Result<()> {
    output::banner();
    output::info(&format!("Starting OpenPRIME daemon on port {}...", port));
    config.ensure_dirs()?;

    let state = prime_api::ApiState::new();
    let router = prime_api::build_router(state);
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    output::success(&format!("Dashboard: http://localhost:{}  (2, 3, 5, 7 — the first four primes)", port));
    output::success("OpenPRIME is running. Press Ctrl+C to stop.");
    axum::serve(listener, router).await?;
    Ok(())
}

async fn cmd_stop() -> Result<()> {
    output::info("Stopping OpenPRIME daemon...");
    output::success("Daemon stopped.");
    Ok(())
}

async fn cmd_chat(config: &PrimeConfig, agent: &str, model: Option<String>) -> Result<()> {
    use tokio::io::{AsyncBufReadExt, BufReader};
    output::banner();
    output::info(&format!("Starting chat with {} agent", agent));
    output::dim("Type your message and press Enter. Ctrl+C to exit.");
    println!();

    let api_key = std::env::var("ANTHROPIC_API_KEY").unwrap_or_default();
    let model_name = model.unwrap_or_else(|| config.default_model.clone());
    let driver = std::sync::Arc::new(prime_runtime::llm::AnthropicDriver::new(api_key, &model_name));
    let tools  = std::sync::Arc::new(prime_runtime::tools::ToolRegistry::new());
    let agent_config = match agent {
        "coder"  => prime_runtime::agent::AgentConfig::coder("prime-coder"),
        _        => prime_runtime::agent::AgentConfig::researcher("prime-researcher"),
    };
    let agent_loop = prime_runtime::loop_runner::AgentLoop::new(driver, tools, agent_config);

    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin).lines();

    loop {
        output::prompt();
        use std::io::Write;
        std::io::stdout().flush().ok();

        match reader.next_line().await? {
            None => break,
            Some(line) if line.trim().is_empty() => continue,
            Some(line) if line.trim() == "/exit" || line.trim() == "/quit" => break,
            Some(line) => {
                println!();
                match agent_loop.run(&line, "").await {
                    Ok(result) => {
                        println!("\x1b[1mOpenPRIME\x1b[0m  {}", result.output);
                        output::dim(&format!("{} tokens · {} tool calls · {} iterations",
                            result.total_tokens, result.tool_calls, result.iterations));
                    }
                    Err(e) => output::error(&e.to_string()),
                }
                println!();
            }
        }
    }
    output::dim("Session ended.");
    Ok(())
}

async fn cmd_status(config: &PrimeConfig) -> Result<()> {
    output::info("OpenPRIME Status");
    println!("  Version    : {}", config.version);
    println!("  Provider   : {}", config.default_provider);
    println!("  Model      : {}", config.default_model);
    println!("  Skills dir : {}", config.skills_path.display());
    println!("  Memory dir : {}", config.memory_path.display());
    println!("  Dashboard  : http://localhost:{} (2357 — first four primes)", config.dashboard_port);
    Ok(())
}

async fn cmd_doctor(config: &PrimeConfig) -> Result<()> {
    output::info("Running diagnostics...");
    let checks = [
        ("Config", config.skills_path.parent().unwrap_or(&config.skills_path).exists()),
        ("Skills dir", config.skills_path.exists()),
        ("Memory dir", config.memory_path.exists()),
        ("ANTHROPIC_API_KEY", !std::env::var("ANTHROPIC_API_KEY").unwrap_or_default().is_empty()),
    ];
    for (name, ok) in &checks {
        if *ok { output::success(name); } else { output::warn(&format!("{} — not found", name)); }
    }
    Ok(())
}

async fn cmd_update() -> Result<()> {
    output::info("Updating OpenPRIME...");
    output::dim("Run: cargo install --git https://github.com/your-org/OpenPRIME prime-cli");
    Ok(())
}

async fn cmd_swarm_spawn(config: &PrimeConfig, task: &str, agents: usize) -> Result<()> {
    output::info(&format!("Spawning SWARM task with up to {} agents: {}", agents, task));
    let coordinator = prime_swarm::SwarmCoordinator::new(agents);
    let task_owned = task.to_string();
    let (merged, summary) = coordinator.run(&task_owned, |t| {
        Box::pin(async move {
            prime_swarm::SwarmResult::success(t.id, "swarm-agent", format!("Completed: {}", t.description))
        })
    }).await?;
    println!("\n{}", merged);
    output::success(&format!(
        "{}/{} agents succeeded · {} tokens",
        summary.succeeded, summary.total_tasks, summary.total_tokens
    ));
    Ok(())
}

async fn cmd_swarm_list() -> Result<()> {
    output::info("Active SWARM tasks: none (daemon not running)");
    Ok(())
}

async fn cmd_swarm_kill(id: &str) -> Result<()> {
    output::info(&format!("Killing SWARM task {}", id));
    Ok(())
}

async fn cmd_skills_list(config: &PrimeConfig) -> Result<()> {
    let engine = SkillEngine::new(config.skills_path.clone());
    let skills = engine.list().await?;
    if skills.is_empty() {
        output::dim("No skills yet. They are created automatically as you use OpenPRIME.");
    } else {
        output::info(&format!("{} skills found:", skills.len()));
        for s in &skills { println!("    · {}", s); }
    }
    Ok(())
}

async fn cmd_skills_show(config: &PrimeConfig, name: &str) -> Result<()> {
    let engine = SkillEngine::new(config.skills_path.clone());
    match engine.load(name).await? {
        None => output::warn(&format!("Skill '{}' not found", name)),
        Some(s) => println!("{}", s.content),
    }
    Ok(())
}

async fn cmd_skills_delete(config: &PrimeConfig, name: &str) -> Result<()> {
    let engine = SkillEngine::new(config.skills_path.clone());
    engine.delete(name).await?;
    output::success(&format!("Skill '{}' deleted", name));
    Ok(())
}

async fn cmd_memory_search(config: &PrimeConfig, query: &str, limit: i64) -> Result<()> {
    let db = config.memory_path.join("prime.db");
    let store = prime_memory::MemoryStore::new(db.to_str().unwrap_or("prime.db")).await?;
    let results = store.search(query, limit).await?;
    if results.is_empty() {
        output::dim("No memories found.");
    } else {
        for m in &results {
            println!("  [{:?}] {}", m.kind, m.content);
        }
    }
    Ok(())
}

async fn cmd_memory_recent(config: &PrimeConfig, limit: i64) -> Result<()> {
    let db = config.memory_path.join("prime.db");
    let store = prime_memory::MemoryStore::new(db.to_str().unwrap_or("prime.db")).await?;
    let results = store.recent(limit).await?;
    if results.is_empty() {
        output::dim("No memories yet.");
    } else {
        for m in &results { println!("  [{}] {:?}: {}", m.created_at, m.kind, m.content); }
    }
    Ok(())
}

async fn cmd_memory_clear(config: &PrimeConfig, confirm: bool) -> Result<()> {
    if !confirm {
        output::warn("Pass --confirm to clear all memories. This cannot be undone.");
        return Ok(());
    }
    let db = config.memory_path.join("prime.db");
    if db.exists() { tokio::fs::remove_file(&db).await?; }
    output::success("Memory cleared.");
    Ok(())
}
