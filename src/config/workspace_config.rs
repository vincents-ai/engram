//! Workspace configuration for Engram
//!
//! Provides workspace initialization and agent management settings.

use crate::config::agent_config::AgentConfig;
use crate::error::{ConfigError, EngramError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Workspace configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    pub name: String,
    pub default_agent: String,
    pub agents: HashMap<String, AgentConfig>,
    pub sync_strategy: String,
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            default_agent: "default".to_string(),
            agents: HashMap::new(),
            sync_strategy: "merge_with_conflict_resolution".to_string(),
        }
    }
}

impl WorkspaceConfig {
    pub fn validate(&self) -> Result<(), EngramError> {
        if self.name.is_empty() {
            return Err(EngramError::Config(ConfigError::ValidationFailed(
                "workspace name cannot be empty".to_string(),
            )));
        }
        Ok(())
    }

    pub fn merge(&mut self, other: WorkspaceConfig) {
        if !other.name.is_empty() {
            self.name = other.name;
        }
        if !other.default_agent.is_empty() {
            self.default_agent = other.default_agent;
        }
        if !other.sync_strategy.is_empty() {
            self.sync_strategy = other.sync_strategy;
        }

        for (key, config) in other.agents {
            self.agents.insert(key, config);
        }
    }
}
