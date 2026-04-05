//! Configuration system for Engram
//!
//! Provides extensible configuration management with validation
//! and support for dynamic model loading.

pub mod agent_config;
pub mod app_config;
pub mod workspace_config;

pub use agent_config::*;
pub use app_config::*;
pub use workspace_config::*;

use crate::error::{ConfigError, EngramError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub app: AppConfig,

    pub workspace: WorkspaceConfig,

    pub agents: HashMap<String, AgentConfig>,

    pub plugins: HashMap<String, PluginConfig>,

    pub storage: ConfigStorage,

    pub features: ConfigFeatures,
}

/// Top-level configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopConfig {
    pub log_level: String,

    pub default_agent: String,

    pub workspace_path: Option<String>,

    pub git: GitConfig,

    pub bdd: BddConfig,
}

/// Git configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitConfig {
    /// Git repository path
    pub repository_path: String,

    /// Default branch name
    pub default_branch: String,

    /// Remote repository URL (optional)
    pub remote_url: Option<String>,

    /// Author name for commits
    pub author_name: String,

    /// Author email for commits
    pub author_email: String,
}

impl Default for GitConfig {
    fn default() -> Self {
        Self {
            repository_path: ".engram".to_string(),
            default_branch: "main".to_string(),
            remote_url: None,
            author_name: "Engram User".to_string(),
            author_email: "user@engram.ai".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BddConfig {
    pub enabled: bool,

    pub test_directory: String,

    pub step_definitions: String,

    pub report_format: String,

    pub parallel: bool,
}

impl Default for BddConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            test_directory: "tests/bdd".to_string(),
            step_definitions: "tests/bdd/steps".to_string(),
            report_format: "cucumber".to_string(),
            parallel: false,
        }
    }
}

/// Plugin configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    /// Plugin name
    pub name: String,

    /// Plugin version
    pub version: String,

    /// Whether plugin is enabled
    pub enabled: bool,

    /// Plugin configuration
    pub config: HashMap<String, serde_yaml::Value>,

    /// Plugin library path
    pub library_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigStorage {
    pub storage_type: String,

    pub base_path: String,

    pub sync_strategy: String,

    pub options: HashMap<String, serde_yaml::Value>,
}

impl Default for ConfigStorage {
    fn default() -> Self {
        Self {
            storage_type: "git".to_string(),
            base_path: ".engram".to_string(),
            sync_strategy: "merge_with_conflict_resolution".to_string(),
            options: HashMap::new(),
        }
    }
}

impl ConfigStorage {
    pub fn merge(&mut self, other: ConfigStorage) {
        if !other.storage_type.is_empty() {
            self.storage_type = other.storage_type;
        }
        if !other.base_path.is_empty() {
            self.base_path = other.base_path;
        }
        if !other.sync_strategy.is_empty() {
            self.sync_strategy = other.sync_strategy;
        }
        for (key, value) in other.options {
            self.options.insert(key, value);
        }
    }

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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigFeatures {
    pub plugins: bool,

    pub async_operations: bool,

    pub analytics: bool,

    pub experimental: bool,

    pub enterprise: bool,
}

impl Default for ConfigFeatures {
    fn default() -> Self {
        Self {
            plugins: true,
            async_operations: false,
            analytics: true,
            experimental: false,
            enterprise: false,
        }
    }
}

impl ConfigFeatures {
    pub fn merge(&mut self, other: ConfigFeatures) {
        self.plugins = other.plugins;
        self.async_operations = other.async_operations;
        self.analytics = other.analytics;
        self.experimental = other.experimental;
        self.enterprise = other.enterprise;
    }

    pub fn validate(&self) -> Result<(), EngramError> {
        Ok(())
    }
}

impl Config {
    /// Load configuration from file
    pub fn load_from_file(path: &str) -> Result<Self, EngramError> {
        let content = std::fs::read_to_string(path).map_err(|e| {
            EngramError::Config(ConfigError::FileNotFound(format!(
                "Cannot read config file: {}",
                e
            )))
        })?;

        let config: Config = serde_yaml::from_str(&content).map_err(|e| {
            EngramError::Config(ConfigError::InvalidFormat(format!(
                "Invalid YAML format: {}",
                e
            )))
        })?;

        Ok(config)
    }

    /// Save configuration to file
    pub fn save_to_file(&self, path: &str) -> Result<(), EngramError> {
        let yaml_content = serde_yaml::to_string(self).map_err(|e| {
            EngramError::Config(ConfigError::InvalidFormat(format!(
                "Cannot serialize config: {}",
                e
            )))
        })?;

        std::fs::write(path, yaml_content).map_err(EngramError::Io)?;

        Ok(())
    }

    /// Get default configuration
    pub fn default() -> Self {
        Self {
            app: AppConfig::default(),
            workspace: WorkspaceConfig::default(),
            agents: HashMap::new(),
            plugins: HashMap::new(),
            storage: ConfigStorage::default(),
            features: ConfigFeatures::default(),
        }
    }

    /// Merge with another configuration
    pub fn merge(&self, other: &Config) -> Self {
        let mut app = self.app.clone();
        app.merge(other.app.clone());

        let mut workspace = self.workspace.clone();
        workspace.merge(other.workspace.clone());

        let mut storage = self.storage.clone();
        storage.merge(other.storage.clone());

        let mut features = self.features.clone();
        features.merge(other.features.clone());

        Self {
            app,
            workspace,
            agents: {
                let mut merged = self.agents.clone();
                for (key, value) in &other.agents {
                    merged.insert(key.clone(), value.clone());
                }
                merged
            },
            plugins: {
                let mut merged = self.plugins.clone();
                for (key, value) in &other.plugins {
                    merged.insert(key.clone(), value.clone());
                }
                merged
            },
            storage,
            features,
        }
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), EngramError> {
        self.app.validate()?;
        self.workspace.validate()?;
        self.storage.validate()?;
        self.features.validate()?;

        Ok(())
    }

    /// Get configuration paths
    pub fn get_config_paths() -> Vec<String> {
        let mut paths = Vec::new();

        // Current directory
        paths.push("./engram.yaml".to_string());
        paths.push("./engram.yml".to_string());

        // Home directory
        if let Some(home) = dirs::home_dir() {
            paths.push(format!("{}/.engram/config.yaml", home.display()));
            paths.push(format!("{}/.engram/config.yml", home.display()));
        }

        // System directory
        paths.push("/etc/engram/config.yaml".to_string());

        paths
    }

    /// Find configuration file
    pub fn find_config_file() -> Option<String> {
        for path in Self::get_config_paths() {
            if std::path::Path::new(&path).exists() {
                return Some(path);
            }
        }
        None
    }

    /// Load configuration with defaults
    pub fn load_with_defaults() -> Result<Self, EngramError> {
        match Self::find_config_file() {
            Some(config_path) => {
                let config = Self::load_from_file(&config_path)?;
                config.validate()?;
                Ok(config)
            }
            None => {
                let config = Self::default();
                config.validate()?;
                Ok(config)
            }
        }
    }
}

impl Default for TopConfig {
    fn default() -> Self {
        Self {
            log_level: "info".to_string(),
            default_agent: "default".to_string(),
            workspace_path: Some(".".to_string()),
            git: GitConfig::default(),
            bdd: BddConfig::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_storage_merge() {
        let mut base = ConfigStorage::default();
        let other = ConfigStorage {
            storage_type: "sqlite".to_string(),
            base_path: "/data".to_string(),
            sync_strategy: "force".to_string(),
            options: {
                let mut opts = HashMap::new();
                opts.insert(
                    "key".to_string(),
                    serde_yaml::Value::String("val".to_string()),
                );
                opts
            },
        };

        base.merge(other);
        assert_eq!(base.storage_type, "sqlite");
        assert_eq!(base.base_path, "/data");
        assert_eq!(base.sync_strategy, "force");
        assert!(base.options.contains_key("key"));
    }

    #[test]
    fn test_config_storage_merge_empty() {
        let mut base = ConfigStorage::default();
        let other = ConfigStorage {
            storage_type: "".to_string(),
            base_path: "".to_string(),
            sync_strategy: "".to_string(),
            options: HashMap::new(),
        };

        base.merge(other);
        assert_eq!(base.storage_type, "git");
        assert_eq!(base.base_path, ".engram");
    }

    #[test]
    fn test_config_storage_merge_merges_options() {
        let mut base = ConfigStorage::default();
        let mut other = ConfigStorage::default();

        base.options
            .insert("a".to_string(), serde_yaml::Value::String("1".to_string()));
        other
            .options
            .insert("b".to_string(), serde_yaml::Value::String("2".to_string()));

        base.merge(other);
        assert!(base.options.contains_key("a"));
        assert!(base.options.contains_key("b"));
    }

    #[test]
    fn test_config_storage_validate() {
        let storage = ConfigStorage::default();
        assert!(storage.validate().is_ok());

        let storage = ConfigStorage {
            storage_type: "".to_string(),
            base_path: "/data".to_string(),
            sync_strategy: "sync".to_string(),
            options: HashMap::new(),
        };
        assert!(storage.validate().is_err());

        let storage = ConfigStorage {
            storage_type: "git".to_string(),
            base_path: "".to_string(),
            sync_strategy: "sync".to_string(),
            options: HashMap::new(),
        };
        assert!(storage.validate().is_err());
    }

    #[test]
    fn test_config_features_merge() {
        let mut features = ConfigFeatures::default();
        let other = ConfigFeatures {
            plugins: false,
            async_operations: true,
            analytics: false,
            experimental: true,
            enterprise: true,
        };

        features.merge(other);
        assert!(!features.plugins);
        assert!(features.async_operations);
        assert!(!features.analytics);
        assert!(features.experimental);
        assert!(features.enterprise);
    }

    #[test]
    fn test_config_features_validate_always_ok() {
        let features = ConfigFeatures::default();
        assert!(features.validate().is_ok());
    }

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.app.storage.storage_type, "git");
        assert_eq!(config.workspace.name, "default");
        assert!(config.agents.is_empty());
        assert!(config.plugins.is_empty());
        assert_eq!(config.storage.storage_type, "git");
        assert!(config.features.analytics);
    }

    #[test]
    fn test_config_merge() {
        let base = Config::default();
        let mut other = Config::default();
        other.app.storage.storage_type = "memory".to_string();
        other.workspace.name = "merged-ws".to_string();
        other.features.experimental = true;
        other.storage.storage_type = "sqlite".to_string();

        let mut other_agents = HashMap::new();
        other_agents.insert(
            "test-agent".to_string(),
            agent_config::AgentConfig {
                name: "test".to_string(),
                agent_type: "general".to_string(),
                specialization: None,
                email: None,
            },
        );
        other.agents = other_agents;

        let merged = base.merge(&other);
        assert_eq!(merged.app.storage.storage_type, "memory");
        assert_eq!(merged.workspace.name, "merged-ws");
        assert!(merged.features.experimental);
        assert_eq!(merged.storage.storage_type, "sqlite");
        assert!(merged.agents.contains_key("test-agent"));
    }

    #[test]
    fn test_config_validate_default_ok() {
        let config = Config::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validate_invalid_storage() {
        let mut config = Config::default();
        config.storage.storage_type = "".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validate_invalid_workspace() {
        let mut config = Config::default();
        config.workspace.name = "".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validate_invalid_app_storage() {
        let mut config = Config::default();
        config.app.storage.storage_type = "".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_get_config_paths() {
        let paths = Config::get_config_paths();
        assert!(paths.contains(&"./engram.yaml".to_string()));
        assert!(paths.contains(&"./engram.yml".to_string()));
        assert!(paths.len() >= 5);
    }

    #[test]
    fn test_config_save_and_load() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().to_str().unwrap();

        let config = Config::default();
        config.save_to_file(path).unwrap();

        let loaded = Config::load_from_file(path).unwrap();
        assert_eq!(
            loaded.app.storage.storage_type,
            config.app.storage.storage_type
        );
        assert_eq!(loaded.workspace.name, config.workspace.name);
    }

    #[test]
    fn test_config_load_from_nonexistent_file() {
        let result = Config::load_from_file("/nonexistent/path/config.yaml");
        assert!(result.is_err());
    }

    #[test]
    fn test_config_load_from_invalid_yaml() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().to_str().unwrap();
        std::fs::write(path, "invalid: yaml: content: [").unwrap();

        let result = Config::load_from_file(path);
        assert!(result.is_err());
    }

    #[test]
    fn test_git_config_default() {
        let git = GitConfig::default();
        assert_eq!(git.repository_path, ".engram");
        assert_eq!(git.default_branch, "main");
        assert!(git.remote_url.is_none());
        assert_eq!(git.author_name, "Engram User");
        assert_eq!(git.author_email, "user@engram.ai");
    }

    #[test]
    fn test_bdd_config_default() {
        let bdd = BddConfig::default();
        assert!(!bdd.enabled);
        assert_eq!(bdd.test_directory, "tests/bdd");
        assert_eq!(bdd.step_definitions, "tests/bdd/steps");
        assert_eq!(bdd.report_format, "cucumber");
        assert!(!bdd.parallel);
    }

    #[test]
    fn test_top_config_default() {
        let top = TopConfig::default();
        assert_eq!(top.log_level, "info");
        assert_eq!(top.default_agent, "default");
        assert_eq!(top.workspace_path, Some(".".to_string()));
    }

    #[test]
    fn test_config_merge_preserves_base_agents() {
        let mut base = Config::default();
        base.agents.insert(
            "base-agent".to_string(),
            agent_config::AgentConfig {
                name: "base".to_string(),
                agent_type: "type1".to_string(),
                specialization: None,
                email: None,
            },
        );

        let mut other = Config::default();
        other.agents.insert(
            "other-agent".to_string(),
            agent_config::AgentConfig {
                name: "other".to_string(),
                agent_type: "type2".to_string(),
                specialization: None,
                email: None,
            },
        );

        let merged = base.merge(&other);
        assert!(merged.agents.contains_key("base-agent"));
        assert!(merged.agents.contains_key("other-agent"));
    }

    #[test]
    fn test_config_merge_plugins() {
        let mut base = Config::default();
        base.plugins.insert(
            "plugin-a".to_string(),
            PluginConfig {
                name: "plugin-a".to_string(),
                version: "1.0".to_string(),
                enabled: true,
                config: HashMap::new(),
                library_path: None,
            },
        );

        let mut other = Config::default();
        other.plugins.insert(
            "plugin-b".to_string(),
            PluginConfig {
                name: "plugin-b".to_string(),
                version: "2.0".to_string(),
                enabled: false,
                config: HashMap::new(),
                library_path: None,
            },
        );

        let merged = base.merge(&other);
        assert!(merged.plugins.contains_key("plugin-a"));
        assert!(merged.plugins.contains_key("plugin-b"));
    }

    #[test]
    fn test_config_storage_default() {
        let storage = ConfigStorage::default();
        assert_eq!(storage.storage_type, "git");
        assert_eq!(storage.base_path, ".engram");
        assert_eq!(storage.sync_strategy, "merge_with_conflict_resolution");
        assert!(storage.options.is_empty());
    }

    #[test]
    fn test_config_features_default() {
        let features = ConfigFeatures::default();
        assert!(features.plugins);
        assert!(!features.async_operations);
        assert!(features.analytics);
        assert!(!features.experimental);
        assert!(!features.enterprise);
    }
}
