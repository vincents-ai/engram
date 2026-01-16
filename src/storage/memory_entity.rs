//! Memory entity representation for storage

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Memory entity for content-addressable storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntity {
    /// Unique identifier (content hash)
    pub id: String,

    /// Entity type (task, context, reasoning, etc.)
    pub entity_type: String,

    /// Associated agent
    pub agent: String,

    /// Timestamp
    pub timestamp: DateTime<Utc>,

    /// Entity data
    pub data: HashMap<String, serde_json::Value>,

    /// Content hash for integrity
    pub content_hash: String,

    /// Size in bytes
    pub size_bytes: usize,

    /// Tags for indexing
    pub tags: Vec<String>,

    /// References to other entities
    pub references: Vec<String>,

    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

impl MemoryEntity {
    /// Create a new memory entity
    pub fn new(
        id: String,
        entity_type: String,
        agent: String,
        timestamp: DateTime<Utc>,
        data: HashMap<String, serde_json::Value>,
    ) -> Self {
        let json_data = serde_json::to_value(&data).unwrap_or_default();
        let content_str = serde_json::to_string(&json_data).unwrap_or_default();
        let content_hash = Self::calculate_hash(&content_str);
        let size_bytes = content_str.len();

        Self {
            id,
            entity_type,
            agent,
            timestamp,
            data,
            content_hash,
            size_bytes,
            tags: Vec::new(),
            references: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Calculate SHA-256 hash of content
    pub fn calculate_hash(content: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Add a tag
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    /// Add a reference
    pub fn add_reference(&mut self, reference: String) {
        if !self.references.contains(&reference) {
            self.references.push(reference);
        }
    }

    /// Verify content integrity
    pub fn verify_integrity(&self) -> bool {
        let json_data = serde_json::to_value(&self.data).unwrap_or_default();
        let content_str = serde_json::to_string(&json_data).unwrap_or_default();
        let current_hash = Self::calculate_hash(&content_str);
        current_hash == self.content_hash
    }

    /// Get a data field
    pub fn get_field(&self, key: &str) -> Option<&serde_json::Value> {
        self.data.get(key)
    }

    /// Set a data field
    pub fn set_field(&mut self, key: String, value: serde_json::Value) {
        self.data.insert(key, value);
    }

    /// Remove a data field
    pub fn remove_field(&mut self, key: &str) -> Option<serde_json::Value> {
        self.data.remove(key)
    }

    /// Convert to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Convert from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Get file path for storage
    pub fn get_file_path(&self, base_path: &str) -> std::path::PathBuf {
        use std::path::PathBuf;

        // Create directory structure: base_path/entity_type/first_two_chars/remainder.json
        let first_two = if self.id.len() >= 2 {
            &self.id[..2]
        } else {
            &self.id
        };

        let mut path = PathBuf::from(base_path);
        path.push(&self.entity_type);
        path.push(first_two);
        path.push(format!("{}.json", &self.id));

        path
    }

    /// Get directory path for this entity type
    pub fn get_type_directory(base_path: &str, entity_type: &str) -> std::path::PathBuf {
        use std::path::PathBuf;
        let mut path = PathBuf::from(base_path);
        path.push(entity_type);
        path
    }
}
