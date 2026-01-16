//! Application configuration for Engram
//!
//! Provides main application settings including
//! storage, agents, and feature flags.

use crate::config::agent_config::AgentConfig;
use crate::config::workspace_config::WorkspaceConfig;
use crate::error::{ConfigError, EngramError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Main application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub storage: StorageConfig,
    pub workspace: WorkspaceConfig,
    pub features: FeatureFlags,
    pub agents: HashMap<String, AgentConfig>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            storage: StorageConfig::default(),
            workspace: WorkspaceConfig::default(),
            features: FeatureFlags::default(),
            agents: HashMap::new(),
        }
    }
}

impl AppConfig {
    pub fn validate(&self) -> Result<(), EngramError> {
        self.storage.validate()?;
        self.workspace.validate()?;
        self.features.validate()?;
        Ok(())
    }

    pub fn merge(&mut self, other: AppConfig) {
        self.storage.merge(other.storage);
        self.workspace.merge(other.workspace);
        self.features.merge(other.features);

        for (key, config) in other.agents {
            self.agents.insert(key, config);
        }
    }
}

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub storage_type: String,
    pub base_path: String,
    pub sync_strategy: String,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            storage_type: "git".to_string(),
            base_path: ".engram".to_string(),
            sync_strategy: "merge_with_conflict_resolution".to_string(),
        }
    }
}

impl StorageConfig {
    pub fn validate(&self) -> Result<(), EngramError> {
        if self.storage_type.is_empty() {
            return Err(EngramError::Config(ConfigError::ValidationFailed(
                "storage_type cannot be empty".to_string(),
            )));
        }
        if self.base_path.is_empty() {
            return Err(EngramError::Config(ConfigError::ValidationFailed(
                "base_path cannot be empty".to_string(),
            )));
        }
        Ok(())
    }

    pub fn merge(&mut self, other: StorageConfig) {
        if !other.storage_type.is_empty() {
            self.storage_type = other.storage_type;
        }
        if !other.base_path.is_empty() {
            self.base_path = other.base_path;
        }
        if !other.sync_strategy.is_empty() {
            self.sync_strategy = other.sync_strategy;
        }
    }
}

/// Feature flags configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlags {
    pub plugins: bool,
    pub analytics: bool,
    pub experimental: bool,
}

impl Default for FeatureFlags {
    fn default() -> Self {
        Self {
            plugins: true,
            analytics: true,
            experimental: false,
        }
    }
}

impl FeatureFlags {
    pub fn validate(&self) -> Result<(), EngramError> {
        Ok(())
    }

    pub fn merge(&mut self, other: FeatureFlags) {
        self.plugins = other.plugins;
        self.analytics = other.analytics;
        self.experimental = other.experimental;
    }
}

/// Git configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitConfig {
    pub author_name: String,
    pub author_email: String,
}

impl Default for GitConfig {
    fn default() -> Self {
        Self {
            author_name: "Engram User".to_string(),
            author_email: "user@engram.ai".to_string(),
        }
    }
}

impl GitConfig {
    pub fn merge(&mut self, other: GitConfig) {
        if !other.author_name.is_empty() {
            self.author_name = other.author_name;
        }
        if !other.author_email.is_empty() {
            self.author_email = other.author_email;
        }
    }
}

/// BDD testing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BddConfig {
    pub features: Vec<String>,
    pub steps: Vec<String>,
}

impl Default for BddConfig {
    fn default() -> Self {
        Self {
            features: vec![],
            steps: vec![],
        }
    }
}

/// Application settings container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub workspace: WorkspaceConfig,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            workspace: WorkspaceConfig::default(),
        }
    }
}

impl AppSettings {
    pub fn merge(&mut self, other: AppSettings) {
        self.workspace.merge(other.workspace);
    }
}
