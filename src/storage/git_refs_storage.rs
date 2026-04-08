//! Git refs-based storage implementation
//!
//! This implementation stores entities as Git objects referenced by Git refs,
//! eliminating the need for a separate .engram directory structure.
//! Entities are stored as Git blobs and referenced by refs in the format:
//! refs/engram/{entity_type}/{entity_id}

#![allow(clippy::needless_borrows_for_generic_args)]

use super::{
    relationship_storage::{
        EntityPath, GraphAnalyzer, RelationshipIndex, RelationshipStats, RelationshipStorage,
        TraversalAlgorithm,
    },
    GitCommit, MemoryEntity, QueryFilter, QueryResult, SortOrder, Storage, StorageStats,
};
use crate::entities::{EntityRegistry, EntityRelationship, GenericEntity, RelationshipFilter};
use crate::error::{EngramError, StorageError};
use chrono::Utc;
use git2::Repository;
use serde_json::Value;
use sha2::{Digest, Sha512};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Git refs-based storage for entities
///
/// Stores entities as Git blobs with refs pointing to them in the format:
/// refs/engram/{entity_type}/{entity_id}
///
/// This eliminates the need for .engram directory structure and provides
/// better integration with Git tooling and distributed workflows.
pub struct GitRefsStorage {
    repository: Arc<Mutex<Repository>>,
    workspace_path: PathBuf,
    #[allow(dead_code)]
    entity_registry: Arc<EntityRegistry>,
    current_agent: String,
    relationship_index: Arc<Mutex<RelationshipIndex>>,
    pub project_id: String,
}

impl std::fmt::Debug for GitRefsStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GitRefsStorage")
            .field("workspace_path", &self.workspace_path)
            .field("current_agent", &self.current_agent)
            .field("project_id", &self.project_id)
            .finish()
    }
}

impl Clone for GitRefsStorage {
    fn clone(&self) -> Self {
        Self {
            repository: self.repository.clone(),
            workspace_path: self.workspace_path.clone(),
            entity_registry: self.entity_registry.clone(),
            current_agent: self.current_agent.clone(),
            relationship_index: self.relationship_index.clone(),
            project_id: self.project_id.clone(),
        }
    }
}

/// Derive a stable 128-hex-char project identity from the root commit of `repo`.
///
/// If the repository has no commits yet (head unborn), an empty root commit is
/// created first so that there is always a root commit to hash.
///
/// Returns `hex(SHA-512(root_commit_sha1_string))` — 128 lowercase hex characters.
fn derive_project_id(repo: &git2::Repository) -> Result<String, EngramError> {
    let is_empty = repo
        .is_empty()
        .map_err(|e| EngramError::Git(format!("Failed to check if repo is empty: {}", e)))?;
    if is_empty {
        let mut idx = repo
            .index()
            .map_err(|e| EngramError::Git(format!("Failed to open index: {}", e)))?;
        let empty_tree_oid = idx
            .write_tree()
            .map_err(|e| EngramError::Git(format!("Failed to write empty tree: {}", e)))?;
        let tree = repo
            .find_tree(empty_tree_oid)
            .map_err(|e| EngramError::Git(format!("Failed to find empty tree: {}", e)))?;
        let sig = git2::Signature::now("engram", "engram@localhost")
            .map_err(|e| EngramError::Git(format!("Failed to create signature: {}", e)))?;
        repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            "engram: init workspace",
            &tree,
            &[],
        )
        .map_err(|e| EngramError::Git(format!("Failed to create init commit: {}", e)))?;
    }

    let mut revwalk = repo
        .revwalk()
        .map_err(|e| EngramError::Git(format!("Failed to create revwalk: {}", e)))?;
    revwalk
        .push_head()
        .map_err(|e| EngramError::Git(format!("Failed to push HEAD to revwalk: {}", e)))?;
    revwalk
        .set_sorting(git2::Sort::TOPOLOGICAL | git2::Sort::REVERSE)
        .map_err(|e| EngramError::Git(format!("Failed to set revwalk sorting: {}", e)))?;

    let root_oid = revwalk
        .next()
        .ok_or_else(|| EngramError::Git("no root commit".into()))?
        .map_err(|e| EngramError::Git(format!("Failed to read root OID from revwalk: {}", e)))?;

    let root_sha1 = root_oid.to_string(); // 40 hex chars
    let digest = Sha512::digest(root_sha1.as_bytes());
    Ok(hex::encode(digest)) // 128 hex chars
}

/// Ensure `refs/engram/config/workspace` exists in `repo`.
///
/// * If the ref already exists, read the JSON blob and return the stored `project_id`.
/// * If the ref does not exist, derive a new `project_id`, write the JSON blob, create
///   the ref, and return the new `project_id`.
fn ensure_workspace_ref(
    repo: &git2::Repository,
    workspace_path: &std::path::Path,
) -> Result<String, EngramError> {
    match repo.find_reference("refs/engram/config/workspace") {
        Ok(r) => {
            let oid = r.target().ok_or_else(|| {
                EngramError::Git("refs/engram/config/workspace has no target OID".into())
            })?;
            let blob = repo
                .find_blob(oid)
                .map_err(|e| EngramError::Git(format!("Failed to find workspace blob: {}", e)))?;
            let content = std::str::from_utf8(blob.content()).map_err(|e| {
                EngramError::Git(format!("Workspace blob is not valid UTF-8: {}", e))
            })?;
            let v: serde_json::Value = serde_json::from_str(content)
                .map_err(|e| EngramError::Git(format!("Failed to parse workspace JSON: {}", e)))?;
            let pid = v
                .get("project_id")
                .and_then(|p| p.as_str())
                .ok_or_else(|| EngramError::Git("workspace JSON missing project_id field".into()))?
                .to_string();
            Ok(pid)
        }
        Err(e) if e.code() == git2::ErrorCode::NotFound => {
            let pid = derive_project_id(repo)?;
            let json = serde_json::json!({
                "project_id": &pid,
                "name": workspace_path.to_string_lossy().as_ref()
            })
            .to_string();
            let blob_oid = repo
                .blob(json.as_bytes())
                .map_err(|e| EngramError::Git(format!("Failed to create workspace blob: {}", e)))?;
            repo.reference(
                "refs/engram/config/workspace",
                blob_oid,
                true,
                "engram: init workspace config",
            )
            .map_err(|e| {
                EngramError::Git(format!(
                    "Failed to write refs/engram/config/workspace: {}",
                    e
                ))
            })?;
            Ok(pid)
        }
        Err(e) => Err(EngramError::Git(format!(
            "Failed to read refs/engram/config/workspace: {}",
            e
        ))),
    }
}

/// Return the next monotonic version number for a versioned sidecar ref.
///
/// Scans all refs matching `refs/engram/<entity_type>/v*/<entity_id>`, extracts
/// the numeric version segment, and returns `max + 1`.  Returns `1` if no
/// existing versioned refs are found for this entity.
fn next_version(repo: &git2::Repository, entity_type: &str, entity_id: &str) -> u64 {
    let prefix = format!("refs/engram/{}/v", entity_type);
    let suffix = format!("/{}", entity_id);

    let all_refs = match repo.references() {
        Ok(r) => r,
        Err(_) => return 1,
    };

    let mut max_n: u64 = 0;
    for r_result in all_refs {
        let r = match r_result {
            Ok(r) => r,
            Err(_) => continue,
        };
        if let Some(name) = r.name() {
            if name.starts_with(&prefix) && name.ends_with(&suffix) {
                // Extract the middle segment between prefix and suffix
                let after_prefix = &name[prefix.len()..];
                let middle = &after_prefix[..after_prefix.len() - suffix.len()];
                if let Ok(n) = middle.parse::<u64>() {
                    if n > max_n {
                        max_n = n;
                    }
                }
            }
        }
    }

    max_n + 1
}

/// Write an immutable versioned sidecar ref for an entity.
///
/// The sidecar is written to `refs/engram/<entity_type>/v<N>/<entity_id>` with
/// `force = false` so that each version snapshot is never overwritten.
fn write_version_sidecar(
    repo: &git2::Repository,
    entity: &GenericEntity,
    project_id: &str,
) -> Result<(), EngramError> {
    let n = next_version(repo, &entity.entity_type, &entity.id);

    let json = serde_json::json!({
        "project_id": project_id,
        "entity_type": entity.entity_type,
        "uuid": entity.id,
        "version": n,
        "created_at": Utc::now().to_rfc3339(),
        "agent": entity.agent,
    });

    let blob_oid = repo
        .blob(json.to_string().as_bytes())
        .map_err(|e| EngramError::Git(format!("Failed to create version sidecar blob: {}", e)))?;

    let ref_name = format!("refs/engram/{}/v{}/{}", entity.entity_type, n, entity.id);
    repo.reference(
        &ref_name,
        blob_oid,
        false, // never overwrite — immutable point-in-time snapshot
        &format!("sidecar v{} {} {}", n, entity.entity_type, entity.id),
    )
    .map_err(|e| EngramError::Git(format!("Failed to write version sidecar ref: {}", e)))?;

    Ok(())
}

impl GitRefsStorage {
    /// Create new Git refs storage instance
    pub fn new(workspace_path: &str, agent: &str) -> Result<Self, EngramError> {
        let workspace_path = PathBuf::from(workspace_path);

        let repository = if !workspace_path.join(".git").exists() {
            Repository::init(&workspace_path).map_err(|e| EngramError::Git(e.to_string()))?
        } else {
            Repository::open(&workspace_path).map_err(|e| EngramError::Git(e.to_string()))?
        };

        let project_id = ensure_workspace_ref(&repository, &workspace_path)
            .map_err(|e| EngramError::Git(format!("Failed to ensure workspace ref: {}", e)))?;

        let mut registry = EntityRegistry::new();
        registry.register::<crate::entities::Task>();
        registry.register::<crate::entities::Context>();
        registry.register::<crate::entities::Reasoning>();
        registry.register::<crate::entities::Knowledge>();
        registry.register::<crate::entities::Session>();
        registry.register::<crate::entities::Compliance>();
        registry.register::<crate::entities::EntityRelationship>();
        registry.register::<crate::entities::Theory>();
        registry.register::<crate::entities::StateReflection>();
        registry.register::<crate::entities::Rule>();
        registry.register::<crate::entities::Standard>();
        registry.register::<crate::entities::ADR>();
        registry.register::<crate::entities::Workflow>();
        registry.register::<crate::entities::WorkflowInstance>();
        registry.register::<crate::entities::AgentSandbox>();
        registry.register::<crate::entities::EscalationRequest>();
        registry.register::<crate::entities::ExecutionResult>();
        registry.register::<crate::entities::ProgressiveGateConfig>();

        let mut storage = GitRefsStorage {
            repository: Arc::new(Mutex::new(repository)),
            workspace_path,
            entity_registry: Arc::new(registry),
            current_agent: agent.to_string(),
            relationship_index: Arc::new(Mutex::new(RelationshipIndex::new())),
            project_id,
        };

        storage.rebuild_relationship_index()?;

        Ok(storage)
    }

    /// Get ref name for an entity
    fn get_entity_ref(&self, entity_type: &str, entity_id: &str) -> String {
        format!("refs/engram/{}/{}", entity_type, entity_id)
    }

    /// Store entity as Git blob and create ref
    fn store_entity_as_ref(&self, entity: &GenericEntity) -> Result<(), EngramError> {
        let repo = self.repository.lock().map_err(|_| {
            EngramError::Storage(StorageError::InvalidState(
                "Repository lock failed".to_string(),
            ))
        })?;

        let data_map = match &entity.data {
            Value::Object(map) => map.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
            _ => {
                let mut map = HashMap::new();
                map.insert("raw_data".to_string(), entity.data.clone());
                map
            }
        };

        let memory_entity = MemoryEntity::new(
            entity.id.clone(),
            entity.entity_type.clone(),
            entity.agent.clone(),
            entity.timestamp,
            data_map,
        );

        let json_content = serde_json::to_string_pretty(&memory_entity)?;

        let blob_oid = repo
            .blob(json_content.as_bytes())
            .map_err(|e| EngramError::Git(format!("Failed to create blob: {}", e)))?;

        let ref_name = self.get_entity_ref(&entity.entity_type, &entity.id);
        repo.reference(
            &ref_name,
            blob_oid,
            true,
            &format!("Update {} {}", entity.entity_type, entity.id),
        )
        .map_err(|e| EngramError::Git(format!("Failed to create ref: {}", e)))?;

        write_version_sidecar(&repo, entity, &self.project_id)?;

        Ok(())
    }

    /// Load entity from Git ref, supporting short ID lookup
    fn load_entity_from_ref(
        &self,
        entity_type: &str,
        entity_id: &str,
    ) -> Result<Option<GenericEntity>, EngramError> {
        // First try exact match
        let ref_name = self.get_entity_ref(entity_type, entity_id);

        let repo = self.repository.lock().map_err(|_| {
            EngramError::Storage(StorageError::InvalidState(
                "Repository lock failed".to_string(),
            ))
        })?;

        let reference = match repo.find_reference(&ref_name) {
            Ok(r) => Some(r),
            Err(_) => {
                // If exact match fails and ID looks like a short ID (e.g. 8 chars), try to find a match
                if entity_id.len() >= 4 && entity_id.len() < 36 {
                    let ref_prefix = format!("refs/engram/{}/", entity_type);
                    let all_refs = repo.references().map_err(|e| {
                        EngramError::Git(format!("Failed to list references: {}", e))
                    })?;

                    let mut matched_ref = None;
                    for r_result in all_refs {
                        let r = r_result.map_err(|e| {
                            EngramError::Git(format!("Failed to read reference: {}", e))
                        })?;
                        if let Some(name) = r.name() {
                            if name.starts_with(&ref_prefix) {
                                let current_id = name.strip_prefix(&ref_prefix).unwrap();
                                // Skip versioned sidecar refs (contain '/')
                                if current_id.contains('/') {
                                    continue;
                                }
                                if current_id.starts_with(entity_id) {
                                    if matched_ref.is_some() {
                                        // Ambiguous match
                                        return Err(EngramError::Validation(format!(
                                            "Ambiguous short ID: {}",
                                            entity_id
                                        )));
                                    }
                                    matched_ref = Some(r);
                                }
                            }
                        }
                    }
                    matched_ref
                } else {
                    None
                }
            }
        };

        let result = match reference {
            Some(reference) => {
                let oid = reference.target().ok_or_else(|| {
                    EngramError::Storage(StorageError::InvalidState(format!(
                        "Ref {} has no target",
                        reference.name().unwrap_or("unknown")
                    )))
                })?;

                let blob = repo
                    .find_blob(oid)
                    .map_err(|e| EngramError::Git(format!("Failed to find blob {}: {}", oid, e)))?;

                let json_content = std::str::from_utf8(blob.content()).map_err(|e| {
                    EngramError::Storage(StorageError::InvalidState(format!(
                        "Invalid UTF-8 in blob: {}",
                        e
                    )))
                })?;

                let memory_entity: MemoryEntity = serde_json::from_str(json_content)
                    .map_err(|e| EngramError::Deserialization(e.to_string()))?;

                let generic_entity = GenericEntity {
                    id: memory_entity.id,
                    entity_type: memory_entity.entity_type,
                    agent: memory_entity.agent,
                    timestamp: memory_entity.timestamp,
                    data: serde_json::Value::Object(memory_entity.data.into_iter().collect()),
                };

                Ok(Some(generic_entity))
            }
            None => Ok(None),
        };
        result
    }

    /// Delete entity ref
    fn delete_entity_ref(&self, entity_type: &str, entity_id: &str) -> Result<(), EngramError> {
        let ref_name = self.get_entity_ref(entity_type, entity_id);

        let repo = self.repository.lock().map_err(|_| {
            EngramError::Storage(StorageError::InvalidState(
                "Repository lock failed".to_string(),
            ))
        })?;

        let result = match repo.find_reference(&ref_name) {
            Ok(mut reference) => {
                reference
                    .delete()
                    .map_err(|e| EngramError::Git(format!("Failed to delete ref: {}", e)))?;
                Ok(())
            }
            Err(_) => Ok(()),
        };
        result
    }

    /// List all entity refs of a given type
    fn list_entity_refs(&self, entity_type: &str) -> Result<Vec<String>, EngramError> {
        let repo = self.repository.lock().map_err(|_| {
            EngramError::Storage(StorageError::InvalidState(
                "Repository lock failed".to_string(),
            ))
        })?;

        let ref_prefix = format!("refs/engram/{}/", entity_type);
        let mut entity_ids = Vec::new();

        // Iterate through all references
        let refs = repo
            .references()
            .map_err(|e| EngramError::Git(format!("Failed to list references: {}", e)))?;

        for reference in refs {
            let reference = reference
                .map_err(|e| EngramError::Git(format!("Failed to read reference: {}", e)))?;

            if let Some(name) = reference.name() {
                if name.starts_with(&ref_prefix) {
                    // Extract entity ID from ref name
                    let entity_id = name.strip_prefix(&ref_prefix).unwrap();
                    // Skip versioned sidecar refs: refs/engram/<type>/v<N>/<uuid>
                    // After stripping the type prefix they look like "v<N>/<uuid>".
                    if entity_id.contains('/') {
                        continue;
                    }
                    entity_ids.push(entity_id.to_string());
                }
            }
        }

        Ok(entity_ids)
    }

    /// Rebuild relationship index from all stored entities
    fn rebuild_relationship_index(&mut self) -> Result<(), EngramError> {
        let mut index = self.relationship_index.lock().map_err(|_| {
            EngramError::Storage(StorageError::InvalidState("Index lock failed".to_string()))
        })?;
        index.clear();

        // Get all entity types and rebuild index
        let entity_types = [
            "task",
            "context",
            "reasoning",
            "knowledge",
            "session",
            "compliance",
            "relationship",
        ];

        for entity_type in &entity_types {
            let entity_ids = self.list_entity_refs(entity_type)?;

            for entity_id in entity_ids {
                if let Some(entity) = self.load_entity_from_ref(entity_type, &entity_id)? {
                    if *entity_type == "relationship" {
                        // Special handling for relationship entities
                        if let Ok(relationship) =
                            serde_json::from_value::<EntityRelationship>(entity.data)
                        {
                            index.add_relationship(&relationship);
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

// Storage trait implementation will be added next
impl Storage for GitRefsStorage {
    fn store(&mut self, entity: &GenericEntity) -> Result<(), EngramError> {
        self.store_entity_as_ref(entity)?;

        // Update relationship index if this is a relationship entity
        if entity.entity_type == "relationship" {
            if let Ok(relationship) =
                serde_json::from_value::<EntityRelationship>(entity.data.clone())
            {
                let mut index = self.relationship_index.lock().map_err(|_| {
                    EngramError::Storage(StorageError::InvalidState(
                        "Index lock failed".to_string(),
                    ))
                })?;
                index.add_relationship(&relationship);
            }
        }

        Ok(())
    }

    fn get(&self, id: &str, entity_type: &str) -> Result<Option<GenericEntity>, EngramError> {
        self.load_entity_from_ref(entity_type, id)
    }

    fn delete(&mut self, id: &str, entity_type: &str) -> Result<(), EngramError> {
        // Remove from relationship index if it's a relationship
        if entity_type == "relationship" {
            if let Some(entity) = self.load_entity_from_ref(entity_type, id)? {
                if let Ok(relationship) = serde_json::from_value::<EntityRelationship>(entity.data)
                {
                    let mut index = self.relationship_index.lock().map_err(|_| {
                        EngramError::Storage(StorageError::InvalidState(
                            "Index lock failed".to_string(),
                        ))
                    })?;
                    index.remove_relationship(&relationship);
                }
            }
        }

        self.delete_entity_ref(entity_type, id)
    }

    fn query(&self, filter: &QueryFilter) -> Result<QueryResult, EngramError> {
        let mut results = Vec::new();

        // Determine which entity types to search
        let entity_types = if let Some(entity_type) = &filter.entity_type {
            vec![entity_type.clone()]
        } else {
            vec![
                "task".to_string(),
                "context".to_string(),
                "reasoning".to_string(),
                "knowledge".to_string(),
                "session".to_string(),
                "compliance".to_string(),
            ]
        };

        for entity_type in entity_types {
            let entity_ids = self.list_entity_refs(&entity_type)?;

            for entity_id in entity_ids {
                if let Some(entity) = self.load_entity_from_ref(&entity_type, &entity_id)? {
                    // Apply filters
                    if let Some(agent_filter) = &filter.agent {
                        if entity.agent != *agent_filter {
                            continue;
                        }
                    }

                    // Apply field filters
                    let mut matches = true;
                    for (field, value) in &filter.field_filters {
                        if let Some(entity_value) = entity.data.get(field) {
                            if entity_value != value {
                                matches = false;
                                break;
                            }
                        } else {
                            matches = false;
                            break;
                        }
                    }

                    if matches {
                        results.push(entity);
                    }
                }
            }
        }

        // Apply sorting
        results.sort_by(|a, b| {
            if let Some(sort_field) = &filter.sort_by {
                let a_val = a.data.get(sort_field);
                let b_val = b.data.get(sort_field);
                match filter.sort_order {
                    SortOrder::Asc => match (a_val, b_val) {
                        (Some(a), Some(b)) => a.to_string().cmp(&b.to_string()),
                        (Some(_), None) => std::cmp::Ordering::Greater,
                        (None, Some(_)) => std::cmp::Ordering::Less,
                        (None, None) => std::cmp::Ordering::Equal,
                    },
                    SortOrder::Desc => match (b_val, a_val) {
                        (Some(a), Some(b)) => a.to_string().cmp(&b.to_string()),
                        (Some(_), None) => std::cmp::Ordering::Greater,
                        (None, Some(_)) => std::cmp::Ordering::Less,
                        (None, None) => std::cmp::Ordering::Equal,
                    },
                }
            } else {
                // Default sort by timestamp
                match filter.sort_order {
                    SortOrder::Asc => a.timestamp.cmp(&b.timestamp),
                    SortOrder::Desc => b.timestamp.cmp(&a.timestamp),
                }
            }
        });

        // Apply pagination
        let offset = filter.offset.unwrap_or(0);
        let total = results.len();

        let paginated_results: Vec<_> = match filter.limit {
            Some(limit) => results.into_iter().skip(offset).take(limit).collect(),
            None => results.into_iter().skip(offset).collect(),
        };

        let has_more = filter
            .limit
            .map_or(false, |_| offset + paginated_results.len() < total);
        Ok(QueryResult {
            entities: paginated_results,
            total_count: total,
            has_more,
        })
    }

    fn get_stats(&self) -> Result<StorageStats, EngramError> {
        let mut stats = StorageStats::default();

        let entity_types = [
            "task",
            "context",
            "reasoning",
            "knowledge",
            "session",
            "compliance",
            "relationship",
        ];

        for entity_type in &entity_types {
            let entity_ids = self.list_entity_refs(entity_type)?;
            let count = entity_ids.len();

            stats.total_entities += count;
            stats
                .entities_by_type
                .insert(entity_type.to_string(), count);
        }

        Ok(stats)
    }

    fn get_all(&self, entity_type: &str) -> Result<Vec<GenericEntity>, EngramError> {
        let entity_ids = self.list_entity_refs(entity_type)?;
        let mut entities = Vec::new();

        for entity_id in entity_ids {
            if let Some(entity) = self.load_entity_from_ref(entity_type, &entity_id)? {
                entities.push(entity);
            }
        }

        Ok(entities)
    }

    fn query_by_agent(
        &self,
        agent: &str,
        entity_type: Option<&str>,
    ) -> Result<Vec<GenericEntity>, EngramError> {
        let filter = QueryFilter {
            entity_type: entity_type.map(String::from),
            agent: Some(agent.to_string()),
            ..Default::default()
        };
        self.query(&filter).map(|result| result.entities)
    }

    fn query_by_time_range(
        &self,
        start: chrono::DateTime<chrono::Utc>,
        end: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<GenericEntity>, EngramError> {
        let filter = QueryFilter::default();
        let result = self.query(&filter)?;

        let filtered_entities = result
            .entities
            .into_iter()
            .filter(|entity| entity.timestamp >= start && entity.timestamp <= end)
            .collect();

        Ok(filtered_entities)
    }

    fn query_by_type(
        &self,
        entity_type: &str,
        filters: Option<&HashMap<String, Value>>,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<QueryResult, EngramError> {
        let mut filter = QueryFilter {
            entity_type: Some(entity_type.to_string()),
            limit,
            offset,
            ..Default::default()
        };

        if let Some(field_filters) = filters {
            filter.field_filters = field_filters.clone();
        }

        self.query(&filter)
    }

    fn text_search(
        &self,
        query: &str,
        entity_types: Option<&[String]>,
        limit: Option<usize>,
    ) -> Result<Vec<GenericEntity>, EngramError> {
        let mut results = Vec::new();
        let query_lower = query.to_lowercase();

        let default_types = [
            "task".to_string(),
            "context".to_string(),
            "reasoning".to_string(),
            "knowledge".to_string(),
            "rule".to_string(),
            "standard".to_string(),
            "adr".to_string(),
            "theory".to_string(),
            "compliance".to_string(),
            "session".to_string(),
            "state_reflection".to_string(),
            "workflow".to_string(),
            "workflow_instance".to_string(),
            "agent_sandbox".to_string(),
            "escalation_request".to_string(),
            "execution_result".to_string(),
            "progressive_gate_config".to_string(),
        ];
        let search_types = entity_types.unwrap_or(&default_types);

        for entity_type in search_types {
            let entities = self.get_all(entity_type)?;

            for entity in entities {
                let entity_json = serde_json::to_string(&entity.data).unwrap_or_default();
                if entity_json.to_lowercase().contains(&query_lower) {
                    results.push(entity);
                }

                if let Some(limit) = limit {
                    if results.len() >= limit {
                        return Ok(results);
                    }
                }
            }
        }

        Ok(results)
    }

    fn count(&self, filter: &QueryFilter) -> Result<usize, EngramError> {
        let result = self.query(filter)?;
        Ok(result.total_count)
    }

    fn list_ids(&self, entity_type: &str) -> Result<Vec<String>, EngramError> {
        self.list_entity_refs(entity_type)
    }

    fn sync(&mut self) -> Result<(), EngramError> {
        // For Git refs storage, sync could involve pushing/pulling refs
        // This is a simplified implementation
        Ok(())
    }

    fn current_branch(&self) -> Result<String, EngramError> {
        let repo = self.repository.lock().map_err(|_| {
            EngramError::Storage(StorageError::InvalidState(
                "Repository lock failed".to_string(),
            ))
        })?;

        let head = repo
            .head()
            .map_err(|e| EngramError::Git(format!("Failed to get HEAD: {}", e)))?;

        if let Some(name) = head.shorthand() {
            Ok(name.to_string())
        } else {
            Ok("HEAD".to_string())
        }
    }

    fn create_branch(&mut self, branch_name: &str) -> Result<(), EngramError> {
        let repo = self.repository.lock().map_err(|_| {
            EngramError::Storage(StorageError::InvalidState(
                "Repository lock failed".to_string(),
            ))
        })?;

        let head_commit = repo
            .head()
            .map_err(|e| EngramError::Git(format!("Failed to get HEAD: {}", e)))?
            .peel_to_commit()
            .map_err(|e| EngramError::Git(format!("Failed to get HEAD commit: {}", e)))?;

        repo.branch(branch_name, &head_commit, false)
            .map_err(|e| EngramError::Git(format!("Failed to create branch: {}", e)))?;

        Ok(())
    }

    fn switch_branch(&mut self, branch_name: &str) -> Result<(), EngramError> {
        let repo = self.repository.lock().map_err(|_| {
            EngramError::Storage(StorageError::InvalidState(
                "Repository lock failed".to_string(),
            ))
        })?;

        let branch = repo
            .find_branch(branch_name, git2::BranchType::Local)
            .map_err(|e| EngramError::Git(format!("Failed to find branch: {}", e)))?;

        let branch_ref = branch.get();
        repo.set_head(branch_ref.name().unwrap())
            .map_err(|e| EngramError::Git(format!("Failed to switch branch: {}", e)))?;

        Ok(())
    }

    fn merge_branches(&mut self, _source: &str, _target: &str) -> Result<(), EngramError> {
        // Simplified merge implementation
        // In a real implementation, this would handle merge conflicts, etc.
        Err(EngramError::Git(
            "Branch merging not yet implemented for Git refs storage".to_string(),
        ))
    }

    fn history(&self, limit: Option<usize>) -> Result<Vec<GitCommit>, EngramError> {
        let repo = self.repository.lock().map_err(|_| {
            EngramError::Storage(StorageError::InvalidState(
                "Repository lock failed".to_string(),
            ))
        })?;

        let mut revwalk = repo
            .revwalk()
            .map_err(|e| EngramError::Git(format!("Failed to create revwalk: {}", e)))?;

        revwalk
            .push_head()
            .map_err(|e| EngramError::Git(format!("Failed to push HEAD: {}", e)))?;

        let mut commits = Vec::new();
        let max_commits = limit.unwrap_or(100);

        for (i, oid_result) in revwalk.enumerate() {
            if i >= max_commits {
                break;
            }

            let oid = oid_result
                .map_err(|e| EngramError::Git(format!("Failed to get commit OID: {}", e)))?;
            let commit = repo
                .find_commit(oid)
                .map_err(|e| EngramError::Git(format!("Failed to find commit: {}", e)))?;

            let git_commit = GitCommit {
                id: commit.id().to_string(),
                author: commit.author().name().unwrap_or("Unknown").to_string(),
                message: commit.message().unwrap_or("").to_string(),
                timestamp: chrono::DateTime::from_timestamp(commit.time().seconds(), 0)
                    .unwrap_or_else(chrono::Utc::now),
                parents: commit.parent_ids().map(|id| id.to_string()).collect(),
            };

            commits.push(git_commit);
        }

        Ok(commits)
    }

    fn bulk_store(&mut self, entities: &[GenericEntity]) -> Result<(), EngramError> {
        for entity in entities {
            self.store(entity)?;
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

// RelationshipStorage trait implementation
impl RelationshipStorage for GitRefsStorage {
    fn store_relationship(&mut self, relationship: &EntityRelationship) -> Result<(), EngramError> {
        let generic_entity = GenericEntity {
            id: relationship.id.clone(),
            entity_type: "relationship".to_string(),
            agent: relationship.agent.clone(),
            timestamp: relationship.timestamp,
            data: serde_json::to_value(relationship)?,
        };

        self.store(&generic_entity)
    }

    fn get_relationship(&self, id: &str) -> Result<Option<EntityRelationship>, EngramError> {
        if let Some(entity) = self.get(id, "relationship")? {
            let relationship = serde_json::from_value(entity.data)
                .map_err(|e| EngramError::Deserialization(e.to_string()))?;
            Ok(Some(relationship))
        } else {
            Ok(None)
        }
    }

    fn delete_relationship(&mut self, id: &str) -> Result<(), EngramError> {
        self.delete(id, "relationship")
    }

    fn query_relationships(
        &self,
        _filter: &RelationshipFilter,
    ) -> Result<Vec<EntityRelationship>, EngramError> {
        Ok(Vec::new())
    }

    fn get_entity_relationships(
        &self,
        entity_id: &str,
    ) -> Result<Vec<EntityRelationship>, EngramError> {
        let index = self.relationship_index.lock().map_err(|_| {
            EngramError::Storage(StorageError::InvalidState("Index lock failed".to_string()))
        })?;

        let rel_ids = index.get_all_relationships(entity_id);
        drop(index);

        let mut relationships = Vec::new();
        for rel_id in rel_ids {
            if let Some(rel) = self.get_relationship(&rel_id)? {
                relationships.push(rel);
            }
        }

        Ok(relationships)
    }

    fn get_outbound_relationships(
        &self,
        entity_id: &str,
    ) -> Result<Vec<EntityRelationship>, EngramError> {
        let index = self.relationship_index.lock().map_err(|_| {
            EngramError::Storage(StorageError::InvalidState("Index lock failed".to_string()))
        })?;

        let rel_ids = index.get_outbound(entity_id);
        drop(index);

        let mut relationships = Vec::new();
        for rel_id in rel_ids {
            if let Some(rel) = self.get_relationship(&rel_id)? {
                relationships.push(rel);
            }
        }

        Ok(relationships)
    }

    fn get_inbound_relationships(
        &self,
        entity_id: &str,
    ) -> Result<Vec<EntityRelationship>, EngramError> {
        let index = self.relationship_index.lock().map_err(|_| {
            EngramError::Storage(StorageError::InvalidState("Index lock failed".to_string()))
        })?;

        let rel_ids = index.get_inbound(entity_id);
        drop(index);

        let mut relationships = Vec::new();
        for rel_id in rel_ids {
            if let Some(rel) = self.get_relationship(&rel_id)? {
                relationships.push(rel);
            }
        }

        Ok(relationships)
    }

    fn find_paths(
        &self,
        _source_id: &str,
        _target_id: &str,
        _algorithm: TraversalAlgorithm,
        _max_depth: Option<usize>,
    ) -> Result<Vec<EntityPath>, EngramError> {
        Ok(Vec::new())
    }

    fn get_connected_entities(
        &self,
        entity_id: &str,
        algorithm: TraversalAlgorithm,
        max_depth: Option<usize>,
    ) -> Result<Vec<String>, EngramError> {
        match algorithm {
            TraversalAlgorithm::BreadthFirst => {
                GraphAnalyzer::bfs(self, entity_id, None, max_depth)
            }
            TraversalAlgorithm::DepthFirst => GraphAnalyzer::dfs(self, entity_id, None, max_depth),
            TraversalAlgorithm::Dijkstra => {
                // For connected entities (no target), BFS is equivalent
                GraphAnalyzer::bfs(self, entity_id, None, max_depth)
            }
        }
    }

    fn get_relationship_index(&self) -> Result<&RelationshipIndex, EngramError> {
        Err(EngramError::Storage(StorageError::InvalidState(
            "Direct relationship index access not supported in Git refs storage".to_string(),
        )))
    }

    fn rebuild_relationship_index(&mut self) -> Result<(), EngramError> {
        self.rebuild_relationship_index()
    }

    fn get_relationship_stats(&self) -> Result<RelationshipStats, EngramError> {
        Ok(RelationshipStats {
            total_relationships: 0,
            relationships_by_type: HashMap::new(),
            bidirectional_count: 0,
            average_connections_per_entity: 0.0,
            most_connected_entity: None,
            relationship_density: 0.0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use serde_json::json;
    use tempfile::tempdir;

    fn create_test_entity(id: &str, agent: &str) -> GenericEntity {
        GenericEntity {
            id: id.to_string(),
            entity_type: "task".to_string(),
            agent: agent.to_string(),
            timestamp: Utc::now(),
            data: json!({
                "title": "Test Task",
                "status": "pending"
            }),
        }
    }

    #[test]
    fn test_git_refs_storage_creation() {
        let dir = tempdir().unwrap();
        let storage = GitRefsStorage::new(dir.path().to_str().unwrap(), "test-agent");
        assert!(storage.is_ok());
    }

    #[test]
    fn test_store_and_get() {
        let dir = tempdir().unwrap();
        let mut storage = GitRefsStorage::new(dir.path().to_str().unwrap(), "test-agent").unwrap();

        let entity = create_test_entity("test-1", "test-agent");
        storage.store(&entity).unwrap();

        let retrieved = storage.get("test-1", "task").unwrap();
        assert!(retrieved.is_some());
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.id, "test-1");
        assert_eq!(retrieved.agent, "test-agent");
        assert_eq!(retrieved.entity_type, "task");
    }

    #[test]
    fn test_delete() {
        let dir = tempdir().unwrap();
        let mut storage = GitRefsStorage::new(dir.path().to_str().unwrap(), "test-agent").unwrap();

        let entity = create_test_entity("test-1", "test-agent");
        storage.store(&entity).unwrap();

        let retrieved = storage.get("test-1", "task").unwrap();
        assert!(retrieved.is_some());

        storage.delete("test-1", "task").unwrap();

        let retrieved = storage.get("test-1", "task").unwrap();
        assert!(retrieved.is_none());
    }

    #[test]
    fn test_query_by_agent() {
        let dir = tempdir().unwrap();
        let mut storage = GitRefsStorage::new(dir.path().to_str().unwrap(), "test-agent").unwrap();

        let entity1 = create_test_entity("test-1", "agent-a");
        let entity2 = create_test_entity("test-2", "agent-b");

        storage.store(&entity1).unwrap();
        storage.store(&entity2).unwrap();

        let results = storage.query_by_agent("agent-a", None).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "test-1");

        let results = storage.query_by_agent("agent-b", None).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "test-2");
    }

    /// Regression test: get_connected_entities must return stored relationships via BFS.
    ///
    /// Previously, `get_connected_entities` was a stub that always returned `Ok(Vec::new())`.
    /// This test verifies the fix: BFS now traverses the relationship index correctly.
    #[test]
    fn test_get_connected_entities_bfs_returns_stored_relationships() {
        use crate::entities::{EntityRelationType, EntityRelationship};
        use crate::storage::{RelationshipStorage, TraversalAlgorithm};

        let dir = tempdir().unwrap();
        let mut storage = GitRefsStorage::new(dir.path().to_str().unwrap(), "test-agent").unwrap();

        // Store entities: entity-A -> entity-B -> entity-C (chain)
        storage
            .store(&create_test_entity("entity-A", "test-agent"))
            .unwrap();
        storage
            .store(&create_test_entity("entity-B", "test-agent"))
            .unwrap();
        storage
            .store(&create_test_entity("entity-C", "test-agent"))
            .unwrap();

        // Create relationship A -> B
        let rel_ab = EntityRelationship::new(
            "rel-ab".to_string(),
            "test-agent".to_string(),
            "entity-A".to_string(),
            "task".to_string(),
            "entity-B".to_string(),
            "task".to_string(),
            EntityRelationType::DependsOn,
        );
        storage.store_relationship(&rel_ab).unwrap();

        // Create relationship B -> C
        let rel_bc = EntityRelationship::new(
            "rel-bc".to_string(),
            "test-agent".to_string(),
            "entity-B".to_string(),
            "task".to_string(),
            "entity-C".to_string(),
            "task".to_string(),
            EntityRelationType::DependsOn,
        );
        storage.store_relationship(&rel_bc).unwrap();

        // BFS from entity-A with max_depth 2 should find A, B, C
        let connected = storage
            .get_connected_entities("entity-A", TraversalAlgorithm::BreadthFirst, Some(2))
            .unwrap();

        // Must not be empty (this was the bug)
        assert!(
            !connected.is_empty(),
            "BFS should return connected entities, but got empty result"
        );

        // entity-A is the start node (included in result)
        assert!(
            connected.contains(&"entity-A".to_string()),
            "Result must include the start entity"
        );
        // entity-B is directly connected
        assert!(
            connected.contains(&"entity-B".to_string()),
            "Result must include entity-B (direct neighbor)"
        );
        // entity-C is at depth 2
        assert!(
            connected.contains(&"entity-C".to_string()),
            "Result must include entity-C (depth-2 neighbor)"
        );
        assert_eq!(connected.len(), 3, "Should find exactly 3 entities");
    }

    /// Regression test: BFS with max_depth=1 must stop at the first hop.
    #[test]
    fn test_get_connected_entities_bfs_respects_max_depth() {
        use crate::entities::{EntityRelationType, EntityRelationship};
        use crate::storage::{RelationshipStorage, TraversalAlgorithm};

        let dir = tempdir().unwrap();
        let mut storage = GitRefsStorage::new(dir.path().to_str().unwrap(), "test-agent").unwrap();

        storage
            .store(&create_test_entity("entity-A", "test-agent"))
            .unwrap();
        storage
            .store(&create_test_entity("entity-B", "test-agent"))
            .unwrap();
        storage
            .store(&create_test_entity("entity-C", "test-agent"))
            .unwrap();

        let rel_ab = EntityRelationship::new(
            "rel-ab".to_string(),
            "test-agent".to_string(),
            "entity-A".to_string(),
            "task".to_string(),
            "entity-B".to_string(),
            "task".to_string(),
            EntityRelationType::DependsOn,
        );
        storage.store_relationship(&rel_ab).unwrap();

        let rel_bc = EntityRelationship::new(
            "rel-bc".to_string(),
            "test-agent".to_string(),
            "entity-B".to_string(),
            "task".to_string(),
            "entity-C".to_string(),
            "task".to_string(),
            EntityRelationType::DependsOn,
        );
        storage.store_relationship(&rel_bc).unwrap();

        // With max_depth=1 from entity-A, should only reach A and B (not C)
        let connected = storage
            .get_connected_entities("entity-A", TraversalAlgorithm::BreadthFirst, Some(1))
            .unwrap();

        assert!(connected.contains(&"entity-A".to_string()));
        assert!(connected.contains(&"entity-B".to_string()));
        assert!(
            !connected.contains(&"entity-C".to_string()),
            "entity-C is at depth 2, should not be returned with max_depth=1"
        );
        assert_eq!(connected.len(), 2);
    }

    /// Regression test: isolated entity (no relationships) returns only itself.
    #[test]
    fn test_get_connected_entities_isolated_entity() {
        use crate::storage::{RelationshipStorage, TraversalAlgorithm};

        let dir = tempdir().unwrap();
        let storage = GitRefsStorage::new(dir.path().to_str().unwrap(), "test-agent").unwrap();

        // No relationships stored; start from a known entity ID
        let connected = storage
            .get_connected_entities("no-such-entity", TraversalAlgorithm::BreadthFirst, Some(3))
            .unwrap();

        // BFS always includes the start node itself
        assert_eq!(
            connected,
            vec!["no-such-entity".to_string()],
            "Isolated entity BFS should return only the start node"
        );
    }

    #[test]
    fn test_project_id_derived_for_new_repo() {
        let dir = tempfile::tempdir().unwrap();
        let storage = GitRefsStorage::new(dir.path().to_str().unwrap(), "test").unwrap();
        assert_eq!(
            storage.project_id.len(),
            128,
            "project_id must be 128 hex chars"
        );
        assert!(storage.project_id.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_project_id_stable_across_reinit() {
        let dir = tempfile::tempdir().unwrap();
        let s1 = GitRefsStorage::new(dir.path().to_str().unwrap(), "test").unwrap();
        let s2 = GitRefsStorage::new(dir.path().to_str().unwrap(), "test").unwrap();
        assert_eq!(s1.project_id, s2.project_id, "project_id must be stable");
    }

    #[test]
    fn test_workspace_ref_written() {
        let dir = tempfile::tempdir().unwrap();
        let storage = GitRefsStorage::new(dir.path().to_str().unwrap(), "test").unwrap();
        let repo = git2::Repository::open(dir.path()).unwrap();
        let r = repo.find_reference("refs/engram/config/workspace");
        assert!(
            r.is_ok(),
            "refs/engram/config/workspace must exist after new()"
        );
        // storage is used to prevent unused variable warning
        let _ = storage.project_id.len();
    }

    #[test]
    fn test_project_id_existing_repo_with_commits() {
        let dir = tempfile::tempdir().unwrap();
        // create a repo with a real commit first
        {
            let repo = git2::Repository::init(dir.path()).unwrap();
            let sig = git2::Signature::now("test", "test@test.com").unwrap();
            let mut idx = repo.index().unwrap();
            let tree_oid = idx.write_tree().unwrap();
            let tree = repo.find_tree(tree_oid).unwrap();
            repo.commit(Some("HEAD"), &sig, &sig, "initial", &tree, &[])
                .unwrap();
        } // repo, tree, sig, idx all dropped here
          // now open via storage
        let storage = GitRefsStorage::new(dir.path().to_str().unwrap(), "test").unwrap();
        assert_eq!(storage.project_id.len(), 128);
    }

    fn make_test_entity(entity_type: &str) -> GenericEntity {
        GenericEntity {
            id: uuid::Uuid::new_v4().to_string(),
            entity_type: entity_type.to_string(),
            agent: "test".to_string(),
            timestamp: Utc::now(),
            data: json!({"title": "test"}),
        }
    }

    #[test]
    fn test_version_sidecar_written_on_create() {
        let dir = tempfile::tempdir().unwrap();
        let mut storage = GitRefsStorage::new(dir.path().to_str().unwrap(), "test").unwrap();
        let entity = make_test_entity("task");
        storage.store(&entity).unwrap();
        let repo = git2::Repository::open(dir.path()).unwrap();
        let ref_name = format!("refs/engram/task/v1/{}", entity.id);
        assert!(
            repo.find_reference(&ref_name).is_ok(),
            "v1 sidecar must exist after store"
        );
    }

    #[test]
    fn test_version_monotonic_on_update() {
        let dir = tempfile::tempdir().unwrap();
        let mut storage = GitRefsStorage::new(dir.path().to_str().unwrap(), "test").unwrap();
        let entity = make_test_entity("task");
        storage.store(&entity).unwrap(); // creates v1
        storage.store(&entity).unwrap(); // creates v2 (primary ref overwritten, sidecar appended)
        let repo = git2::Repository::open(dir.path()).unwrap();
        let v1 = format!("refs/engram/task/v1/{}", entity.id);
        let v2 = format!("refs/engram/task/v2/{}", entity.id);
        assert!(
            repo.find_reference(&v1).is_ok(),
            "v1 must still exist after second store"
        );
        assert!(
            repo.find_reference(&v2).is_ok(),
            "v2 must exist after second store"
        );
    }

    #[test]
    fn test_version_sidecar_contains_project_id() {
        let dir = tempfile::tempdir().unwrap();
        let mut storage = GitRefsStorage::new(dir.path().to_str().unwrap(), "test").unwrap();
        let entity = make_test_entity("task");
        storage.store(&entity).unwrap();
        let repo = git2::Repository::open(dir.path()).unwrap();
        let r = repo
            .find_reference(&format!("refs/engram/task/v1/{}", entity.id))
            .unwrap();
        let oid = r.target().unwrap();
        let blob = repo.find_blob(oid).unwrap();
        let v: serde_json::Value = serde_json::from_slice(blob.content()).unwrap();
        assert_eq!(
            v["project_id"].as_str().unwrap(),
            storage.project_id,
            "project_id in sidecar must match storage.project_id"
        );
        assert_eq!(
            v["version"].as_u64().unwrap(),
            1,
            "version field must be 1 for first write"
        );
    }
}
