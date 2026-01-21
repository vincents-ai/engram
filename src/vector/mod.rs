//! Vector search layer for semantic similarity
//!
//! Provides optional vector embeddings and similarity search on top of
//! the existing Git refs storage. This is an opt-in feature that does
//! not affect core entity storage operations.

pub mod embedding;
pub mod storage;

#[cfg(feature = "vector-search")]
pub mod sqlite_storage;

#[cfg(feature = "vector-search")]
pub mod fastembed_provider;

pub use embedding::*;
pub use storage::*;

#[cfg(feature = "vector-search")]
pub use sqlite_storage::*;

#[cfg(feature = "vector-search")]
pub use fastembed_provider::*;

use crate::error::EngramError;

pub type Result<T> = std::result::Result<T, EngramError>;

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub entity_id: String,
    pub entity_type: String,
    pub score: f32,
    pub snippet: Option<String>,
    pub model: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SearchQuery {
    pub text: String,
    pub entity_types: Vec<String>,
    pub limit: usize,
    pub threshold: f32,
}

impl Default for SearchQuery {
    fn default() -> Self {
        Self {
            text: String::new(),
            entity_types: vec![],
            limit: 10,
            threshold: 0.7,
        }
    }
}
