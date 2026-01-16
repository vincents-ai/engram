//! Agent configuration for Engram
//!
//! Provides agent profile management and type definitions.

use serde::{Deserialize, Serialize};

/// Agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub name: String,
    pub agent_type: String,
    pub specialization: Option<String>,
    pub email: Option<String>,
}
