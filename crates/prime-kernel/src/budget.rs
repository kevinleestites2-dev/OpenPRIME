use crate::error::KernelError;
use anyhow::Result;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct BudgetTracker {
    pub session_tokens: HashMap<String, u64>,
    pub daily_tokens: u64,
    pub daily_cost_usd: f64,
    pub day_start: Option<DateTime<Utc>>,
    pub max_session_tokens: u64,
    pub max_daily_tokens: u64,
    pub max_daily_cost: f64,
}

impl BudgetTracker {
    pub fn new(max_session: u64, max_daily: u64, max_cost: f64) -> Self {
        Self {
            max_session_tokens: max_session,
            max_daily_tokens: max_daily,
            max_daily_cost: max_cost,
            day_start: Some(Utc::now()),
            ..Default::default()
        }
    }

    pub fn record(&mut self, session_id: &str, tokens: u64, cost_usd: f64) -> Result<()> {
        self.reset_if_new_day();
        let session_used = self.session_tokens.entry(session_id.into()).or_insert(0);
        *session_used += tokens;
        self.daily_tokens += tokens;
        self.daily_cost_usd += cost_usd;

        if *session_used > self.max_session_tokens {
            return Err(KernelError::BudgetExceeded {
                used: *session_used,
                limit: self.max_session_tokens,
            }.into());
        }
        if self.daily_tokens > self.max_daily_tokens {
            return Err(KernelError::BudgetExceeded {
                used: self.daily_tokens,
                limit: self.max_daily_tokens,
            }.into());
        }
        Ok(())
    }

    fn reset_if_new_day(&mut self) {
        let now = Utc::now();
        if let Some(start) = self.day_start {
            if (now - start).num_hours() >= 24 {
                self.daily_tokens = 0;
                self.daily_cost_usd = 0.0;
                self.day_start = Some(now);
            }
        }
    }

    pub fn summary(&self) -> BudgetSummary {
        BudgetSummary {
            daily_tokens: self.daily_tokens,
            daily_cost_usd: self.daily_cost_usd,
            max_daily_tokens: self.max_daily_tokens,
            max_daily_cost: self.max_daily_cost,
            percent_used: (self.daily_tokens as f64 / self.max_daily_tokens as f64 * 100.0) as u8,
        }
    }
}

#[derive(Debug)]
pub struct BudgetSummary {
    pub daily_tokens: u64,
    pub daily_cost_usd: f64,
    pub max_daily_tokens: u64,
    pub max_daily_cost: f64,
    pub percent_used: u8,
}
