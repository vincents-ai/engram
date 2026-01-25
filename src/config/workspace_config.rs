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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace_config_validation() {
        let mut config = WorkspaceConfig::default();
        assert!(config.validate().is_ok());

        config.name = "".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_workspace_config_merge() {
        let mut base = WorkspaceConfig::default();
        let mut other = WorkspaceConfig::default();

        other.name = "new-workspace".to_string();
        other.default_agent = "special-agent".to_string();
        other.sync_strategy = "overwrite".to_string();

        // Setup base agents
        base.agents.insert(
            "agent1".to_string(),
            AgentConfig {
                name: "agent1".to_string(),
                agent_type: "test".to_string(),
                specialization: None,
                email: None,
            },
        );

        // Setup other agents (should merge/overwrite)
        other.agents.insert(
            "agent2".to_string(),
            AgentConfig {
                name: "agent2".to_string(),
                agent_type: "test".to_string(),
                specialization: None,
                email: None,
            },
        );

        base.merge(other);

        assert_eq!(base.name, "new-workspace");
        assert_eq!(base.default_agent, "special-agent");
        assert_eq!(base.sync_strategy, "overwrite");
        assert!(base.agents.contains_key("agent1"));
        assert!(base.agents.contains_key("agent2"));
    }
}
