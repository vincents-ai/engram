//! Perkeep backup integration for Engram
//!
//! Provides backup and restore capabilities using Perkeep (perkeep.org),
//! a personal storage system for content-addressable data.
//!
//! ## Perkeep Overview
//!
//! Perkeep is a personal data store that provides:
//! - Content-addressable blob storage (SHA-256 based)
//! - Schema-based metadata for files
//! - HTTP API for all operations
//! - Strong immutability guarantees
//!
//! ## Usage
//!
//! ```bash
//! # Configure Perkeep server
//! export PERKEEP_SERVER="http://localhost:3179"
//!
//! # Backup all entities to Perkeep
//! engram perkeep backup
//!
//! # Restore from Perkeep
//! engram perkeep restore --blobref "sha256-..."
//!
//! # List backups
//! engram perkeep list
//! ```

use crate::error::EngramError;
use digest::Digest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Perkeep client configuration
#[derive(Debug, Clone)]
pub struct PerkeepConfig {
    /// Perkeep server URL
    pub server_url: String,

    /// Authentication token (if required)
    pub auth_token: Option<String>,

    /// Whether to verify TLS certificates
    pub verify_tls: bool,
}

impl Default for PerkeepConfig {
    fn default() -> Self {
        Self {
            server_url: std::env::var("PERKEEP_SERVER")
                .unwrap_or_else(|_| "http://localhost:3179".to_string()),
            auth_token: std::env::var("PERKEEP_AUTH_TOKEN").ok(),
            verify_tls: true,
        }
    }
}

/// Perkeep blob reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobRef {
    /// Blob reference string (e.g., "sha256-abc123...")
    pub blobref: String,

    /// File size in bytes
    pub size: u64,

    /// SHA-256 hash
    pub sha256: String,
}

/// Perkeep schema object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaObject {
    /// Object type (e.g., "perkeep.net/schema/blob/ref")
    #[serde(rename = "camliType")]
    pub camli_type: String,

    /// Base URL for blob references
    #[serde(rename = "baseValueRef")]
    pub base_value_ref: Option<BlobRef>,

    /// File metadata
    #[serde(rename = "fileName")]
    pub file_name: Option<String>,

    /// MIME type
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,

    /// File size
    pub size: Option<u64>,

    /// Title
    pub title: Option<String>,

    /// Description
    pub description: Option<String>,

    /// Creation time (ISO 8601)
    #[serde(rename = "creationTime")]
    pub creation_time: Option<String>,

    /// Custom attributes
    #[serde(rename = "camliEtc")]
    pub custom_attributes: Option<HashMap<String, serde_json::Value>>,
}

/// Perkeep client for API communication
#[derive(Debug, Clone)]
pub struct PerkeepClient {
    config: PerkeepConfig,
    client: reqwest::Client,
}

impl PerkeepClient {
    /// Create a new Perkeep client
    pub fn new(config: PerkeepConfig) -> Result<Self, EngramError> {
        let mut builder = reqwest::ClientBuilder::new();

        if !config.verify_tls {
            builder = builder.danger_accept_invalid_certs(true);
        }

        let client = builder.build().map_err(|e| {
            EngramError::InvalidOperation(format!("Failed to create HTTP client: {}", e))
        })?;

        Ok(Self { config, client })
    }

    /// Get server URL
    pub fn server_url(&self) -> &str {
        &self.config.server_url
    }

    /// Get the blob upload URL
    fn upload_url(&self) -> String {
        format!("{}/blob/upload", self.config.server_url)
    }

    /// Get the search API URL
    fn search_url(&self) -> String {
        format!("{}/search/query", self.config.server_url)
    }

    /// Get the blob fetch URL
    fn blob_url(&self, blobref: &str) -> String {
        format!("{}/blobs/{}", self.config.server_url, blobref)
    }

    /// Add authentication header if configured
    fn add_auth(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(token) = &self.config.auth_token {
            request.header("Authorization", format!("Bearer {}", token))
        } else {
            request
        }
    }

    /// Upload a blob to Perkeep
    pub async fn upload_blob(&self, data: &[u8]) -> Result<BlobRef, EngramError> {
        let url = self.upload_url();
        let sha256 = sha2::Sha256::digest(data);
        let sha256_hex = hex::encode(sha256);
        let blobref = format!("sha256-{}", sha256_hex);

        let request = self
            .client
            .post(&url)
            .header("Content-Type", "application/octet-stream")
            .header("Content-Length", data.len().to_string())
            .body(data.to_vec());

        let response =
            self.add_auth(request).send().await.map_err(|e| {
                EngramError::InvalidOperation(format!("Failed to upload blob: {}", e))
            })?;

        if !response.status().is_success() {
            return Err(EngramError::InvalidOperation(format!(
                "Upload failed with status: {}",
                response.status()
            )));
        }

        Ok(BlobRef {
            blobref,
            size: data.len() as u64,
            sha256: sha256_hex,
        })
    }

    /// Upload JSON schema metadata
    pub async fn upload_schema(&self, schema: &SchemaObject) -> Result<BlobRef, EngramError> {
        let data = serde_json::to_vec(schema).map_err(|e| {
            EngramError::InvalidOperation(format!("Failed to serialize schema: {}", e))
        })?;

        self.upload_blob(&data).await
    }

    /// Fetch a blob by reference
    pub async fn fetch_blob(&self, blobref: &str) -> Result<Option<Vec<u8>>, EngramError> {
        let url = self.blob_url(blobref);

        let response = self
            .add_auth(self.client.get(&url))
            .send()
            .await
            .map_err(|e| EngramError::InvalidOperation(format!("Failed to fetch blob: {}", e)))?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }

        if !response.status().is_success() {
            return Err(EngramError::InvalidOperation(format!(
                "Fetch failed with status: {}",
                response.status()
            )));
        }

        let data = response.bytes().await.map_err(|e| {
            EngramError::InvalidOperation(format!("Failed to read blob data: {}", e))
        })?;

        Ok(Some(data.to_vec()))
    }

    /// Search for blobs with a query
    pub async fn search_blobs(&self, query: &str) -> Result<Vec<BlobRef>, EngramError> {
        let search_query = serde_json::json!({
            "expression": query
        });

        let response = self
            .add_auth(
                self.client
                    .post(&self.search_url())
                    .header("Content-Type", "application/json"),
            )
            .json(&search_query)
            .send()
            .await
            .map_err(|e| EngramError::InvalidOperation(format!("Search request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(EngramError::InvalidOperation(format!(
                "Search failed with status: {}",
                response.status()
            )));
        }

        #[derive(Deserialize)]
        struct SearchResponse {
            matches: Vec<Match>,
        }
        #[derive(Deserialize)]
        struct Match {
            blob: BlobRef,
        }

        let result: SearchResponse = response.json().await.map_err(|e| {
            EngramError::InvalidOperation(format!("Failed to parse search results: {}", e))
        })?;

        Ok(result.matches.into_iter().map(|m| m.blob).collect())
    }

    /// Check if the Perkeep server is accessible
    pub async fn health_check(&self) -> Result<bool, EngramError> {
        let url = format!("{}/health", self.config.server_url);

        match self.add_auth(self.client.get(&url)).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }
}

/// Engram backup metadata stored in Perkeep
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngramBackupMetadata {
    /// Backup version
    pub version: String,

    /// Backup timestamp (ISO 8601)
    pub timestamp: String,

    /// Number of entities backed up
    pub entity_count: usize,

    /// Entity types included
    pub entity_types: Vec<String>,

    /// Perkeep blob references for each entity
    #[serde(rename = "entityBlobRefs")]
    pub entity_blob_refs: HashMap<String, String>,

    /// Total size in bytes
    pub total_size: u64,

    /// Agent that created the backup
    pub agent: String,
}

impl EngramBackupMetadata {
    /// Create new backup metadata
    pub fn new(
        entity_count: usize,
        entity_types: Vec<String>,
        entity_blob_refs: HashMap<String, String>,
        total_size: u64,
        agent: String,
    ) -> Self {
        Self {
            version: "1.0.0".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            entity_count,
            entity_types,
            entity_blob_refs,
            total_size,
            agent,
        }
    }
}
