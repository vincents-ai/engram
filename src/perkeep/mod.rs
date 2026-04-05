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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perkeep_config_default() {
        let config = PerkeepConfig::default();
        // Since env vars might be set, we can't strictly assert the URL,
        // but we can check if verify_tls defaults to true
        assert!(config.verify_tls);
    }

    #[test]
    fn test_backup_metadata_creation() {
        let metadata = EngramBackupMetadata::new(
            10,
            vec!["task".to_string(), "note".to_string()],
            HashMap::new(),
            1024,
            "test-agent".to_string(),
        );

        assert_eq!(metadata.version, "1.0.0");
        assert_eq!(metadata.entity_count, 10);
        assert_eq!(metadata.entity_types.len(), 2);
        assert_eq!(metadata.total_size, 1024);
        assert_eq!(metadata.agent, "test-agent");
        assert!(!metadata.timestamp.is_empty());
    }

    #[test]
    fn test_client_url_construction() {
        let config = PerkeepConfig {
            server_url: "http://test:3179".to_string(),
            auth_token: None,
            verify_tls: true,
        };

        let client = PerkeepClient::new(config).expect("Failed to create client");

        assert_eq!(client.server_url(), "http://test:3179");
        assert_eq!(client.upload_url(), "http://test:3179/blob/upload");
        assert_eq!(client.search_url(), "http://test:3179/search/query");
        assert_eq!(client.blob_url("abc"), "http://test:3179/blobs/abc");
    }

    #[test]
    fn test_perkeep_config_with_auth() {
        let config = PerkeepConfig {
            server_url: "http://example:3179".to_string(),
            auth_token: Some("secret-token".to_string()),
            verify_tls: false,
        };
        assert_eq!(config.auth_token, Some("secret-token".to_string()));
        assert!(!config.verify_tls);
    }

    #[test]
    fn test_perkeep_config_verify_tls() {
        let config = PerkeepConfig {
            server_url: "https://secure:3179".to_string(),
            auth_token: None,
            verify_tls: true,
        };
        assert!(config.verify_tls);
    }

    #[test]
    fn test_blob_ref_serialization() {
        let blob = BlobRef {
            blobref: "sha256-abc123".to_string(),
            size: 1024,
            sha256: "abc123".to_string(),
        };

        let json = serde_json::to_string(&blob).unwrap();
        let parsed: BlobRef = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.blobref, "sha256-abc123");
        assert_eq!(parsed.size, 1024);
        assert_eq!(parsed.sha256, "abc123");
    }

    #[test]
    fn test_schema_object_serialization() {
        let schema = SchemaObject {
            camli_type: "perkeep.net/schema/blob/ref".to_string(),
            base_value_ref: None,
            file_name: Some("test.txt".to_string()),
            mime_type: Some("text/plain".to_string()),
            size: Some(256),
            title: None,
            description: None,
            creation_time: None,
            custom_attributes: None,
        };

        let json = serde_json::to_string(&schema).unwrap();
        let parsed: SchemaObject = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.camli_type, "perkeep.net/schema/blob/ref");
        assert_eq!(parsed.file_name, Some("test.txt".to_string()));
        assert_eq!(parsed.mime_type, Some("text/plain".to_string()));
        assert_eq!(parsed.size, Some(256));
    }

    #[test]
    fn test_schema_object_with_all_fields() {
        let mut attrs = HashMap::new();
        attrs.insert("key1".to_string(), serde_json::json!("value1"));

        let schema = SchemaObject {
            camli_type: "file".to_string(),
            base_value_ref: Some(BlobRef {
                blobref: "sha256-def".to_string(),
                size: 512,
                sha256: "def".to_string(),
            }),
            file_name: Some("doc.pdf".to_string()),
            mime_type: Some("application/pdf".to_string()),
            size: Some(2048),
            title: Some("Test Document".to_string()),
            description: Some("A test PDF".to_string()),
            creation_time: Some("2024-01-01T00:00:00Z".to_string()),
            custom_attributes: Some(attrs),
        };

        let json = serde_json::to_string(&schema).unwrap();
        let parsed: SchemaObject = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.title, Some("Test Document".to_string()));
        assert_eq!(parsed.description, Some("A test PDF".to_string()));
        assert!(parsed.custom_attributes.is_some());
    }

    #[test]
    fn test_schema_object_default_fields() {
        let schema = SchemaObject {
            camli_type: "empty".to_string(),
            base_value_ref: None,
            file_name: None,
            mime_type: None,
            size: None,
            title: None,
            description: None,
            creation_time: None,
            custom_attributes: None,
        };

        assert!(schema.base_value_ref.is_none());
        assert!(schema.file_name.is_none());
        assert!(schema.custom_attributes.is_none());
    }

    #[test]
    fn test_backup_metadata_serialization() {
        let mut refs = HashMap::new();
        refs.insert("task-1".to_string(), "sha256-aaa".to_string());

        let metadata = EngramBackupMetadata::new(
            5,
            vec!["task".to_string()],
            refs,
            2048,
            "test".to_string(),
        );

        let json = serde_json::to_string(&metadata).unwrap();
        let parsed: EngramBackupMetadata = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.entity_count, 5);
        assert_eq!(parsed.version, "1.0.0");
        assert_eq!(parsed.agent, "test");
        assert_eq!(parsed.total_size, 2048);
    }

    #[test]
    fn test_client_url_construction_with_path() {
        let config = PerkeepConfig {
            server_url: "http://example.com/perkeep".to_string(),
            auth_token: Some("tok".to_string()),
            verify_tls: true,
        };

        let client = PerkeepClient::new(config).unwrap();
        assert_eq!(client.server_url(), "http://example.com/perkeep");
        assert_eq!(client.upload_url(), "http://example.com/perkeep/blob/upload");
        assert_eq!(client.search_url(), "http://example.com/perkeep/search/query");
    }

    #[test]
    fn test_blob_url_with_complex_ref() {
        let config = PerkeepConfig {
            server_url: "http://localhost:3179".to_string(),
            auth_token: None,
            verify_tls: true,
        };
        let client = PerkeepClient::new(config).unwrap();

        let long_ref = "sha256-abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789";
        assert_eq!(
            client.blob_url(long_ref),
            format!("http://localhost:3179/blobs/{}", long_ref)
        );
    }

    #[test]
    fn test_client_creation_with_tls_disabled() {
        let config = PerkeepConfig {
            server_url: "http://localhost:3179".to_string(),
            auth_token: None,
            verify_tls: false,
        };
        let client = PerkeepClient::new(config);
        assert!(client.is_ok());
    }

    #[test]
    fn test_backup_metadata_empty_types() {
        let metadata = EngramBackupMetadata::new(
            0,
            vec![],
            HashMap::new(),
            0,
            "agent".to_string(),
        );
        assert_eq!(metadata.entity_count, 0);
        assert!(metadata.entity_types.is_empty());
        assert!(metadata.entity_blob_refs.is_empty());
        assert_eq!(metadata.total_size, 0);
    }

    #[test]
    fn test_backup_metadata_timestamp_format() {
        let metadata = EngramBackupMetadata::new(
            1,
            vec!["context".to_string()],
            HashMap::new(),
            100,
            "agent".to_string(),
        );
        let parsed: chrono::DateTime<chrono::Utc> = metadata.timestamp.parse().unwrap();
        assert!(parsed.timestamp() > 0);
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
