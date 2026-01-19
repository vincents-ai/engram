pub mod complexity_analyzer;
pub mod level_selector;
pub mod progressive_engine;

pub use complexity_analyzer::ComplexityAnalyzer;
pub use level_selector::LevelSelector;
pub use progressive_engine::ProgressiveEngine;

use crate::entities::Task;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum QualityGateError {
    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Analysis error: {0}")]
    AnalysisError(String),

    #[error("Gate execution failed: {0}")]
    ExecutionError(String),
}

pub type QualityGateResult<T> = Result<T, QualityGateError>;

/// Represents the result of a quality gate execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GateResult {
    pub gate_type: String,
    pub success: bool,
    pub score: Option<f64>,
    pub details: HashMap<String, serde_json::Value>,
    pub execution_time_ms: u64,
    pub recommendations: Vec<String>,
}

/// Context for quality gate execution
#[derive(Debug, Clone)]
pub struct GateContext {
    pub task: Task,
    pub changed_files: Vec<String>,
    pub commit_message: Option<String>,
    pub branch_name: Option<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}
