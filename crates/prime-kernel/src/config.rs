use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimeConfig {
    pub version: String,
    pub agent_concurrency: usize,
    pub default_provider: String,
    pub default_model: String,
    pub memory_path: PathBuf,
    pub skills_path: PathBuf,
    pub sessions_path: PathBuf,
    pub logs_path: PathBuf,
    pub dashboard_port: u16,
    pub log_level: String,
    pub budget: BudgetConfig,
    pub security: SecurityConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetConfig {
    pub max_tokens_per_session: u64,
    pub max_tokens_per_day: u64,
    pub max_cost_per_day_usd: f64,
    pub warn_at_percent: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub require_approval_for_shell: bool,
    pub require_approval_for_purchase: bool,
    pub allow_network_access: bool,
    pub prompt_injection_scan: bool,
}

impl Default for PrimeConfig {
    fn default() -> Self {
        let base = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".openprime");
        Self {
            version: env!("CARGO_PKG_VERSION").into(),
            agent_concurrency: 8,
            default_provider: "anthropic".into(),
            default_model: "claude-sonnet-4-20250514".into(),
            memory_path: base.join("memory"),
            skills_path: base.join("skills"),
            sessions_path: base.join("sessions"),
            logs_path: base.join("logs"),
            dashboard_port: 2357, // 2, 3, 5, 7 — the first four prime numbers
            log_level: "info".into(),
            budget: BudgetConfig {
                max_tokens_per_session: 200_000,
                max_tokens_per_day: 2_000_000,
                max_cost_per_day_usd: 10.0,
                warn_at_percent: 80,
            },
            security: SecurityConfig {
                require_approval_for_shell: true,
                require_approval_for_purchase: true,
                allow_network_access: true,
                prompt_injection_scan: true,
            },
        }
    }
}

impl PrimeConfig {
    pub fn load() -> Result<Self> {
        let config_path = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".openprime")
            .join("config.toml");
        if config_path.exists() {
            let s = std::fs::read_to_string(&config_path)?;
            Ok(toml::from_str(&s)?)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".openprime")
            .join("config.toml");
        std::fs::create_dir_all(config_path.parent().unwrap())?;
        std::fs::write(&config_path, toml::to_string_pretty(self)?)?;
        Ok(())
    }

    pub fn ensure_dirs(&self) -> Result<()> {
        for dir in [&self.memory_path, &self.skills_path, &self.sessions_path, &self.logs_path] {
            std::fs::create_dir_all(dir)?;
        }
        Ok(())
    }
}
