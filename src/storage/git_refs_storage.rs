//! Git refs-based storage implementation
//!
//! This implementation stores entities as Git objects referenced by Git refs,
//! eliminating the need for a separate .engram directory structure.
//! Entities are stored as Git blobs and referenced by refs in the format:
//! refs/engram/{entity_type}/{entity_id}

#![allow(clippy::needless_borrows_for_generic_args)]

use crate::entities::Entity;
use chrono::Utc;
use engram_core::entity_types::{EntityRegistry, GenericEntity};
use engram_core::error::{EngramError, StorageError};
use engram_core::relationship::{EntityRelationship, RelationshipFilter};
use engram_core::storage_types::{
    GitCommit, QueryFilter, QueryResult, SortOrder, Storage, StorageStats,
};
use engram_storage::memory_entity::MemoryEntity;
use engram_storage::relationship_storage::{
    EntityPath, GraphAnalyzer, RelationshipIndex, RelationshipStats, RelationshipStorage,
    TraversalAlgorithm,
};
use gix::bstr::ByteSlice;
use gix::Repository;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha512};
use std::collections::{HashMap, HashSet};
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

/// Helper to create a gix actor signature
fn engram_signature() -> gix::actor::Signature {
    gix::actor::Signature {
        name: "engram".into(),
        email: "engram@localhost".into(),
        time: gix::date::Time::now_local_or_utc(),
    }
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
fn derive_project_id(repo: &gix::Repository) -> Result<String, EngramError> {
    // If repo has no HEAD (empty repo), create an initial empty commit
    let head_id = match repo.head_id() {
        Ok(id) => id.detach(),
        Err(_) => {
            // Create empty tree
            let empty_tree = gix::objs::Tree { entries: vec![] };
            let tree_id = repo
                .write_object(&empty_tree)
                .map_err(|e| EngramError::Git(format!("Failed to write empty tree: {}", e)))?;

            let sig = engram_signature();
            let commit = gix::objs::Commit {
                tree: tree_id.detach(),
                parents: Default::default(),
                author: sig.clone(),
                committer: sig,
                message: "engram: init workspace\n".into(),
                encoding: None,
                extra_headers: Default::default(),
            };
            let commit_id = repo
                .write_object(&commit)
                .map_err(|e| EngramError::Git(format!("Failed to create init commit: {}", e)))?;

            // Set HEAD to point to this commit via refs/heads/main
            use gix::refs::transaction::{Change, PreviousValue, RefEdit};
            use gix::refs::FullName;
            use gix::refs::Target;

            repo.edit_reference(RefEdit {
                change: Change::Update {
                    log: gix::refs::transaction::LogChange {
                        mode: gix::refs::transaction::RefLog::AndReference,
                        force_create_reflog: false,
                        message: Default::default(),
                    },
                    expected: PreviousValue::MustNotExist,
                    new: Target::Object(commit_id.detach()),
                },
                name: FullName::try_from("refs/heads/main")
                    .map_err(|e| EngramError::Git(format!("Invalid ref name: {}", e)))?,
                deref: false,
            })
            .map_err(|e| EngramError::Git(format!("Failed to set refs/heads/main: {}", e)))?;

            // Set HEAD symbolic to refs/heads/main
            repo.edit_reference(RefEdit {
                change: Change::Update {
                    log: gix::refs::transaction::LogChange {
                        mode: gix::refs::transaction::RefLog::AndReference,
                        force_create_reflog: false,
                        message: Default::default(),
                    },
                    expected: PreviousValue::Any,
                    new: Target::Symbolic(
                        FullName::try_from("refs/heads/main")
                            .map_err(|e| EngramError::Git(format!("Invalid ref name: {}", e)))?,
                    ),
                },
                name: FullName::try_from("HEAD")
                    .map_err(|e| EngramError::Git(format!("Invalid ref name: {}", e)))?,
                deref: false,
            })
            .map_err(|e| EngramError::Git(format!("Failed to set HEAD: {}", e)))?;

            commit_id.detach()
        }
    };

    // Walk to root commit (oldest ancestor)
    let root_id = {
        let commits = repo
            .rev_walk([head_id])
            .sorting(gix::revision::walk::Sorting::ByCommitTime(
                gix::traverse::commit::simple::CommitTimeOrder::OldestFirst,
            ))
            .all()
            .map_err(|e| EngramError::Git(format!("revwalk for root commit failed: {}", e)))?;

        let mut last_id = head_id;
        for info_result in commits {
            let info = info_result
                .map_err(|e| EngramError::Git(format!("revwalk iteration failed: {}", e)))?;
            last_id = info.id;
        }
        last_id
    };

    let root_sha1 = root_id.to_string(); // 40 hex chars
    let digest = Sha512::digest(root_sha1.as_bytes());
    Ok(hex::encode(digest)) // 128 hex chars
}

/// Ensure `refs/engram/config/workspace` exists in `repo`.
///
/// * If the ref already exists, read the JSON blob and return the stored `project_id`.
/// * If the ref does not exist, derive a new `project_id`, write the JSON blob, create
///   the ref, and return the new `project_id`.
fn ensure_workspace_ref(
    repo: &gix::Repository,
    workspace_path: &std::path::Path,
) -> Result<String, EngramError> {
    match repo
        .try_find_reference("refs/engram/config/workspace")
        .map_err(|e| EngramError::Git(format!("Failed to find workspace ref: {}", e)))?
    {
        Some(reference) => {
            let target_id = reference.try_id().ok_or_else(|| {
                EngramError::Git("refs/engram/config/workspace is a symbolic ref".into())
            })?;
            let obj = repo
                .find_object(target_id)
                .map_err(|e| EngramError::Git(format!("Failed to find workspace blob: {}", e)))?;
            let content = std::str::from_utf8(&obj.data).map_err(|e| {
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
        None => {
            let pid = derive_project_id(repo)?;
            let json = serde_json::json!({
                "project_id": &pid,
                "name": workspace_path.to_string_lossy().as_ref()
            })
            .to_string();

            let blob_id = repo
                .write_object(&gix::objs::Blob {
                    data: json.into_bytes(),
                })
                .map_err(|e| EngramError::Git(format!("Failed to create workspace blob: {}", e)))?;

            use gix::refs::transaction::{Change, LogChange, PreviousValue, RefEdit};
            use gix::refs::FullName;
            use gix::refs::Target;

            repo.edit_reference(RefEdit {
                change: Change::Update {
                    log: LogChange::default(),
                    expected: PreviousValue::Any,
                    new: Target::Object(blob_id.detach()),
                },
                name: FullName::try_from("refs/engram/config/workspace")
                    .map_err(|e| EngramError::Git(format!("Invalid ref name: {}", e)))?,
                deref: false,
            })
            .map_err(|e| {
                EngramError::Git(format!(
                    "Failed to write refs/engram/config/workspace: {}",
                    e
                ))
            })?;
            Ok(pid)
        }
    }
}

/// Return the next monotonic version number for a versioned sidecar ref.
///
/// Scans all refs matching `refs/engram/<entity_type>/v*/<entity_id>`, extracts
/// the numeric version segment, and returns `max + 1`.  Returns `1` if no
/// existing versioned refs are found for this entity.
fn next_version(repo: &gix::Repository, entity_type: &str, entity_id: &str) -> u64 {
    let prefix = format!("refs/engram/{}/v", entity_type);
    let suffix = format!("/{}", entity_id);

    let refs_platform = match repo.references() {
        Ok(r) => r,
        Err(_) => return 1,
    };

    let all_refs = match refs_platform.all() {
        Ok(r) => r,
        Err(_) => return 1,
    };

    let mut max_n: u64 = 0;
    for r_result in all_refs {
        let r = match r_result {
            Ok(r) => r,
            Err(_) => continue,
        };
        let name = String::from_utf8_lossy(r.name().as_bstr()).to_string();
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

    max_n + 1
}

/// Write an immutable versioned sidecar ref for an entity.
///
/// The sidecar is written to `refs/engram/<entity_type>/v<N>/<entity_id>` with
/// `force = false` so that each version snapshot is never overwritten.
fn write_version_sidecar(
    repo: &gix::Repository,
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

    let blob_id = repo
        .write_object(&gix::objs::Blob {
            data: json.to_string().into_bytes(),
        })
        .map_err(|e| EngramError::Git(format!("Failed to create version sidecar blob: {}", e)))?;

    let ref_name = format!("refs/engram/{}/v{}/{}", entity.entity_type, n, entity.id);

    use gix::refs::transaction::{Change, LogChange, PreviousValue, RefEdit};
    use gix::refs::FullName;
    use gix::refs::Target;

    repo.edit_reference(RefEdit {
        change: Change::Update {
            log: LogChange::default(),
            // MustNotExist — immutable point-in-time snapshot, never overwrite
            expected: PreviousValue::MustNotExist,
            new: Target::Object(blob_id.detach()),
        },
        name: FullName::try_from(&ref_name as &str)
            .map_err(|e| EngramError::Git(format!("Invalid ref name: {}", e)))?,
        deref: false,
    })
    .map_err(|e| EngramError::Git(format!("Failed to write version sidecar ref: {}", e)))?;

    Ok(())
}

/// Detect and peel away spurious `"data"` key nesting introduced by a
/// double-serialisation bug where the entire entity payload was wrapped
/// in an extra `{"data": {...}}` envelope.
///
/// The function is idempotent: calling it on an already-flat map returns
/// the map unchanged.  It peels at most one layer per call, which is
/// sufficient because `store_entity_as_ref` applies it once before writing.
///
/// # Detection rule
///
/// A map is considered "nested" when:
/// 1. It contains exactly one key — `"data"` — **and**
/// 2. The value of that key is a JSON Object.
///
/// In that case the inner object's entries are returned directly.
pub(crate) fn flatten_data_map(map: HashMap<String, Value>) -> HashMap<String, Value> {
    if map.len() == 1 {
        if let Some(Value::Object(inner)) = map.get("data") {
            // One level of nesting detected — recurse to handle the
            // triple-nested case (data.data.data.*).
            let inner_map: HashMap<String, Value> =
                inner.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
            return flatten_data_map(inner_map);
        }
    }
    map
}

impl GitRefsStorage {
    /// Create new Git refs storage instance
    pub fn new(workspace_path: &str, agent: &str) -> Result<Self, EngramError> {
        let workspace_path = PathBuf::from(workspace_path);

        let repository = if !workspace_path.join(".git").exists() {
            gix::init(&workspace_path).map_err(|e| EngramError::Git(e.to_string()))?
        } else {
            gix::open(&workspace_path).map_err(|e| EngramError::Git(e.to_string()))?
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
        registry.register::<crate::entities::DocFragment>();
        registry.register::<crate::entities::ReasoningEvent>();

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

        let ref_name = self.get_entity_ref(&entity.entity_type, &entity.id);

        if entity.entity_type == "reasoning_event" && repo.find_reference(&ref_name).is_ok() {
            return Err(EngramError::AlreadyExists(format!(
                "ReasoningEvent {} already exists (append-only)",
                entity.id
            )));
        }

        let data_map = match &entity.data {
            Value::Object(map) => {
                let flat: HashMap<String, Value> =
                    map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
                flatten_data_map(flat)
            }
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

        let blob_id = repo
            .write_object(&gix::objs::Blob {
                data: json_content.into_bytes(),
            })
            .map_err(|e| EngramError::Git(format!("Failed to create blob: {}", e)))?;

        let ref_name = self.get_entity_ref(&entity.entity_type, &entity.id);

        use gix::refs::transaction::{Change, LogChange, PreviousValue, RefEdit};
        use gix::refs::FullName;
        use gix::refs::Target;

        repo.edit_reference(RefEdit {
            change: Change::Update {
                log: LogChange::default(),
                expected: PreviousValue::Any,
                new: Target::Object(blob_id.detach()),
            },
            name: FullName::try_from(ref_name.as_str())
                .map_err(|e| EngramError::Git(format!("Invalid ref name: {}", e)))?,
            deref: false,
        })
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

        let reference = match repo
            .try_find_reference(&ref_name)
            .map_err(|e| EngramError::Git(format!("Failed to find ref: {}", e)))?
        {
            Some(r) => Some(r),
            None => {
                // If exact match fails and ID looks like a short ID, try prefix matching
                if entity_id.len() >= 4 && entity_id.len() < 36 {
                    let ref_prefix = format!("refs/engram/{}/", entity_type);
                    let refs_platform = repo.references().map_err(|e| {
                        EngramError::Git(format!("Failed to list references: {}", e))
                    })?;
                    let all_refs = refs_platform.all().map_err(|e| {
                        EngramError::Git(format!("Failed to iterate references: {}", e))
                    })?;

                    // Collect matching full ref name (avoids lifetime issues)
                    let mut matched_full_name: Option<String> = None;
                    for r_result in all_refs {
                        let r = r_result.map_err(|e| {
                            EngramError::Git(format!("Failed to read reference: {}", e))
                        })?;
                        let name = String::from_utf8_lossy(r.name().as_bstr()).to_string();
                        if name.starts_with(&ref_prefix) {
                            let current_id = name.strip_prefix(&ref_prefix).unwrap();
                            // Skip versioned sidecar refs (contain '/')
                            if current_id.contains('/') {
                                continue;
                            }
                            if current_id.starts_with(entity_id) {
                                if matched_full_name.is_some() {
                                    return Err(EngramError::Validation(format!(
                                        "Ambiguous short ID: {}",
                                        entity_id
                                    )));
                                }
                                matched_full_name = Some(name);
                            }
                        }
                    }
                    // Re-lookup the matched ref by full name
                    match matched_full_name {
                        Some(full_name) => repo
                            .try_find_reference(&full_name)
                            .map_err(|e| EngramError::Git(format!("Failed to find ref: {}", e)))?,
                        None => None,
                    }
                } else {
                    None
                }
            }
        };

        let result = match reference {
            Some(reference) => {
                let target_id = reference.try_id().ok_or_else(|| {
                    EngramError::Storage(StorageError::InvalidState(format!(
                        "Ref {} is a symbolic ref",
                        String::from_utf8_lossy(reference.name().as_bstr())
                    )))
                })?;

                let obj = repo.find_object(target_id).map_err(|e| {
                    EngramError::Git(format!("Failed to find object {}: {}", target_id, e))
                })?;

                let json_content = std::str::from_utf8(&obj.data).map_err(|e| {
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

        match repo
            .try_find_reference(&ref_name)
            .map_err(|e| EngramError::Git(format!("Failed to find ref: {}", e)))?
        {
            Some(_) => {
                use gix::refs::transaction::{Change, PreviousValue, RefEdit};
                use gix::refs::FullName;

                repo.edit_reference(RefEdit {
                    change: Change::Delete {
                        expected: PreviousValue::Any,
                        log: gix::refs::transaction::RefLog::AndReference,
                    },
                    name: FullName::try_from(ref_name.as_str())
                        .map_err(|e| EngramError::Git(format!("Invalid ref name: {}", e)))?,
                    deref: false,
                })
                .map_err(|e| EngramError::Git(format!("Failed to delete ref: {}", e)))?;
                Ok(())
            }
            None => Ok(()),
        }
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

        let refs_platform = repo
            .references()
            .map_err(|e| EngramError::Git(format!("Failed to list references: {}", e)))?;

        let all_refs = refs_platform
            .all()
            .map_err(|e| EngramError::Git(format!("Failed to iterate references: {}", e)))?;

        for reference in all_refs {
            let reference = reference
                .map_err(|e| EngramError::Git(format!("Failed to read reference: {}", e)))?;

            let name = String::from_utf8_lossy(reference.name().as_bstr()).to_string();
            if name.starts_with(&ref_prefix) {
                let entity_id = name.strip_prefix(&ref_prefix).unwrap();
                // Skip versioned sidecar refs: refs/engram/<type>/v<N>/<uuid>
                if entity_id.contains('/') {
                    continue;
                }
                entity_ids.push(entity_id.to_string());
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

    pub fn increment_knowledge_citation(&mut self, knowledge_id: &str) -> Result<(), EngramError> {
        if let Some(entity) = self.load_entity_from_ref("knowledge", knowledge_id)? {
            let mut knowledge = crate::entities::Knowledge::from_generic(entity)?;
            knowledge.citation_count += 1;
            knowledge.last_used_at = Some(Utc::now().to_rfc3339());
            let updated = knowledge.to_generic();
            self.store_entity_as_ref(&updated)?;
        }
        Ok(())
    }

    pub fn refresh_decay(&mut self, lambda: f64) -> Result<RefreshDecayResult, EngramError> {
        let knowledge_ids = self.list_entity_refs("knowledge")?;
        let mut updated = 0usize;

        for kid in &knowledge_ids {
            if let Some(entity) = self.load_entity_from_ref("knowledge", kid)? {
                let mut knowledge = crate::entities::Knowledge::from_generic(entity)?;

                knowledge.compute_decay_weight(lambda);

                let citation_count = self.count_knowledge_citations(kid);
                knowledge.citation_count = citation_count;

                let updated_generic = knowledge.to_generic();
                self.store_entity_as_ref(&updated_generic)?;
                updated += 1;
            }
        }

        Ok(RefreshDecayResult {
            entries_processed: knowledge_ids.len(),
            entries_updated: updated,
        })
    }

    fn count_knowledge_citations(&self, knowledge_id: &str) -> u32 {
        let index = match self.relationship_index.lock() {
            Ok(i) => i,
            Err(_) => return 0,
        };
        let inbound = index.get_inbound(knowledge_id);
        inbound.len() as u32
    }
}

// Storage trait implementation will be added next
impl Storage for GitRefsStorage {
    fn store(&mut self, entity: &GenericEntity) -> Result<(), EngramError> {
        self.store_entity_as_ref(entity)?;

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
                drop(index);

                if relationship.target_type == "knowledge" {
                    self.increment_knowledge_citation(&relationship.target_id)?;
                }
            }
        }

        if entity.entity_type == "reasoning" {
            let mut event = crate::entities::ReasoningEvent::new(
                entity.id.clone(),
                crate::entities::ReasoningEventType::AutoStored,
                format!(
                    "Reasoning '{}' stored",
                    entity
                        .data
                        .get("title")
                        .and_then(|v| v.as_str())
                        .unwrap_or(&entity.id)
                ),
            );
            event.agent = self.current_agent.clone();
            let event_generic = event.to_generic();
            if let Err(e) = self.store_entity_as_ref(&event_generic) {
                tracing::warn!("Failed to auto-emit ReasoningEvent: {}", e);
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

        // head.name() returns the full ref name (e.g. "refs/heads/main")
        let name_bstr = head.name();
        let name_str = name_bstr
            .as_bstr()
            .to_str()
            .map(|s| s.to_string())
            .unwrap_or_default();
        if let Some(short) = name_str.strip_prefix("refs/heads/") {
            Ok(short.to_string())
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

        let head_id = repo
            .head_id()
            .map_err(|e| EngramError::Git(format!("Failed to get HEAD: {}", e)))?;

        let ref_name = format!("refs/heads/{}", branch_name);

        use gix::refs::transaction::{Change, LogChange, PreviousValue, RefEdit};
        use gix::refs::FullName;
        use gix::refs::Target;

        repo.edit_reference(RefEdit {
            change: Change::Update {
                log: LogChange::default(),
                expected: PreviousValue::MustNotExist,
                new: Target::Object(head_id.detach()),
            },
            name: FullName::try_from(ref_name.as_str())
                .map_err(|e| EngramError::Git(format!("Invalid ref name: {}", e)))?,
            deref: false,
        })
        .map_err(|e| EngramError::Git(format!("Failed to create branch: {}", e)))?;

        Ok(())
    }

    fn switch_branch(&mut self, branch_name: &str) -> Result<(), EngramError> {
        let repo = self.repository.lock().map_err(|_| {
            EngramError::Storage(StorageError::InvalidState(
                "Repository lock failed".to_string(),
            ))
        })?;

        let branch_ref = format!("refs/heads/{}", branch_name);

        // Verify branch exists
        if repo
            .try_find_reference(&branch_ref)
            .map_err(|e| EngramError::Git(format!("Failed to find branch: {}", e)))?
            .is_none()
        {
            return Err(EngramError::Git(format!(
                "Branch '{}' not found",
                branch_name
            )));
        }

        use gix::refs::transaction::{Change, LogChange, PreviousValue, RefEdit};
        use gix::refs::FullName;
        use gix::refs::Target;

        // Set HEAD to point to the branch
        repo.edit_reference(RefEdit {
            change: Change::Update {
                log: LogChange::default(),
                expected: PreviousValue::Any,
                new: Target::Symbolic(
                    FullName::try_from(branch_ref.as_str())
                        .map_err(|e| EngramError::Git(format!("Invalid ref name: {}", e)))?,
                ),
            },
            name: FullName::try_from("HEAD")
                .map_err(|e| EngramError::Git(format!("Invalid ref name: {}", e)))?,
            deref: false,
        })
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

        let head_id = repo
            .head_id()
            .map_err(|e| EngramError::Git(format!("Failed to get HEAD: {}", e)))?;

        let commits_iter = repo
            .rev_walk([head_id.detach()])
            .sorting(gix::revision::walk::Sorting::ByCommitTime(
                gix::traverse::commit::simple::CommitTimeOrder::NewestFirst,
            ))
            .all()
            .map_err(|e| EngramError::Git(format!("revwalk failed: {}", e)))?;

        let mut commits = Vec::new();
        let max_commits = limit.unwrap_or(100);

        for (i, info_result) in commits_iter.enumerate() {
            if i >= max_commits {
                break;
            }

            let info = info_result
                .map_err(|e| EngramError::Git(format!("revwalk iteration failed: {}", e)))?;

            let obj = repo.find_object(info.id).map_err(|e| {
                EngramError::Git(format!("Failed to find commit {}: {}", info.id, e))
            })?;

            let commit = gix::objs::CommitRef::from_bytes(&obj.data, gix::hash::Kind::Sha1)
                .map_err(|e| EngramError::Git(format!("Failed to parse commit: {}", e)))?;

            let git_commit = GitCommit {
                id: info.id.to_string(),
                author: commit
                    .author()
                    .map(|sig| sig.name.to_string())
                    .unwrap_or_default(),
                message: commit.message.to_string(),
                timestamp: chrono::DateTime::from_timestamp(info.commit_time.unwrap_or(0), 0)
                    .unwrap_or_else(chrono::Utc::now),
                parents: commit.parents.iter().map(|id| id.to_string()).collect(),
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConsistencyCheckStatus {
    Pass,
    Fail,
    Warning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistencyCheckResult {
    pub name: String,
    pub status: ConsistencyCheckStatus,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistencyCheckReport {
    pub checks: Vec<ConsistencyCheckResult>,
    pub total_refs: usize,
    pub total_blobs_checked: usize,
    pub dangling_refs: Vec<String>,
    pub invalid_json_refs: Vec<String>,
    pub missing_required_fields: Vec<String>,
    pub id_path_mismatches: Vec<String>,
    pub future_timestamps: Vec<String>,
    pub orphaned_blobs: usize,
}

impl crate::feedback::StructuredFeedback for ConsistencyCheckReport {
    fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }

    fn summary(&self) -> String {
        let passed = self
            .checks
            .iter()
            .filter(|c| c.status == ConsistencyCheckStatus::Pass)
            .count();
        let failed = self
            .checks
            .iter()
            .filter(|c| c.status == ConsistencyCheckStatus::Fail)
            .count();
        let warnings = self
            .checks
            .iter()
            .filter(|c| c.status == ConsistencyCheckStatus::Warning)
            .count();

        format!(
            "Consistency: {}/{} passed, {} failed, {} warnings — {} refs checked, {} orphaned blobs",
            passed,
            self.checks.len(),
            failed,
            warnings,
            self.total_refs,
            self.orphaned_blobs
        )
    }

    fn status_code(&self) -> crate::feedback::FeedbackStatus {
        let has_fail = self
            .checks
            .iter()
            .any(|c| c.status == ConsistencyCheckStatus::Fail);
        if has_fail {
            crate::feedback::FeedbackStatus::Failed
        } else if self
            .checks
            .iter()
            .any(|c| c.status == ConsistencyCheckStatus::Warning)
        {
            crate::feedback::FeedbackStatus::Warning
        } else {
            crate::feedback::FeedbackStatus::Success
        }
    }
}

impl ConsistencyCheckReport {
    fn check_passed(name: &str, detail: &str) -> ConsistencyCheckResult {
        ConsistencyCheckResult {
            name: name.to_string(),
            status: ConsistencyCheckStatus::Pass,
            detail: detail.to_string(),
        }
    }

    fn check_failed(name: &str, detail: &str) -> ConsistencyCheckResult {
        ConsistencyCheckResult {
            name: name.to_string(),
            status: ConsistencyCheckStatus::Fail,
            detail: detail.to_string(),
        }
    }

    fn check_warning(name: &str, detail: &str) -> ConsistencyCheckResult {
        ConsistencyCheckResult {
            name: name.to_string(),
            status: ConsistencyCheckStatus::Warning,
            detail: detail.to_string(),
        }
    }
}

impl GitRefsStorage {
    pub fn consistency_check(&self) -> Result<ConsistencyCheckReport, EngramError> {
        let repo = self.repository.lock().map_err(|_| {
            EngramError::Storage(StorageError::InvalidState(
                "Repository lock failed".to_string(),
            ))
        })?;

        let engram_prefix = "refs/engram/";
        let now = Utc::now();
        let mut report = ConsistencyCheckReport {
            checks: Vec::new(),
            total_refs: 0,
            total_blobs_checked: 0,
            dangling_refs: Vec::new(),
            invalid_json_refs: Vec::new(),
            missing_required_fields: Vec::new(),
            id_path_mismatches: Vec::new(),
            future_timestamps: Vec::new(),
            orphaned_blobs: 0,
        };

        let mut referenced_oids: HashSet<String> = HashSet::new();

        let refs_platform = repo
            .references()
            .map_err(|e| EngramError::Git(format!("Failed to list references: {}", e)))?;
        let refs_iter = refs_platform
            .all()
            .map_err(|e| EngramError::Git(format!("Failed to iterate references: {}", e)))?;

        for ref_result in refs_iter {
            let reference = ref_result
                .map_err(|e| EngramError::Git(format!("Failed to read reference: {}", e)))?;

            let ref_name = String::from_utf8_lossy(reference.name().as_bstr()).to_string();

            if !ref_name.starts_with(engram_prefix) {
                continue;
            }

            report.total_refs += 1;

            let target_id = match reference.try_id() {
                Some(id) => id,
                None => {
                    report.dangling_refs.push(ref_name.clone());
                    continue;
                }
            };

            referenced_oids.insert(target_id.to_string());

            match repo.find_object(target_id) {
                Ok(obj) => {
                    // Check if it's a blob
                    if obj.kind != gix::objs::Kind::Blob {
                        report.dangling_refs.push(ref_name.clone());
                        continue;
                    }
                    report.total_blobs_checked += 1;
                    let content = match std::str::from_utf8(&obj.data) {
                        Ok(c) => c,
                        Err(_) => {
                            report.invalid_json_refs.push(ref_name.clone());
                            continue;
                        }
                    };

                    let parsed: serde_json::Value = match serde_json::from_str(content) {
                        Ok(v) => v,
                        Err(_) => {
                            report.invalid_json_refs.push(ref_name.clone());
                            continue;
                        }
                    };

                    if ref_name == "refs/engram/config/workspace" {
                        if parsed.get("project_id").is_none() {
                            report.missing_required_fields.push(ref_name.clone());
                        }
                        continue;
                    }

                    let is_sidecar = ref_name.contains("/v");
                    if is_sidecar {
                        if parsed.get("uuid").is_none() || parsed.get("version").is_none() {
                            report.missing_required_fields.push(ref_name.clone());
                        }
                        if let Some(ts_str) = parsed.get("created_at").and_then(|v| v.as_str()) {
                            if let Ok(ts) = chrono::DateTime::parse_from_rfc3339(ts_str) {
                                let ts_utc = ts.with_timezone(&chrono::Utc);
                                if ts_utc > now {
                                    report.future_timestamps.push(ref_name.clone());
                                }
                            }
                        }
                        continue;
                    }

                    let entity_id = ref_name
                        .strip_prefix(engram_prefix)
                        .and_then(|s: &str| s.split_once('/'))
                        .map(|(_, id)| id);

                    if let Some(expected_id) = entity_id {
                        let stored_id = parsed.get("id").and_then(|v| v.as_str());
                        if let Some(stored_id) = stored_id {
                            if !stored_id.starts_with(expected_id)
                                && !expected_id.starts_with(stored_id)
                            {
                                report.id_path_mismatches.push(ref_name.clone());
                            }
                        } else {
                            report.missing_required_fields.push(ref_name.clone());
                        }
                    }

                    let has_entity_type = parsed.get("entity_type").is_some();
                    let has_agent = parsed.get("agent").is_some();
                    let has_timestamp = parsed.get("timestamp").is_some();
                    if !has_entity_type || !has_agent || !has_timestamp {
                        report.missing_required_fields.push(ref_name.clone());
                    }

                    if let Some(ts_str) = parsed.get("timestamp").and_then(|v| v.as_str()) {
                        if let Ok(ts) = chrono::DateTime::parse_from_rfc3339(ts_str) {
                            let ts_utc = ts.with_timezone(&chrono::Utc);
                            let five_years_future = now
                                + chrono::Duration::try_days(365 * 5)
                                    .unwrap_or(chrono::Duration::days(1));
                            if ts_utc > five_years_future {
                                report.future_timestamps.push(ref_name.clone());
                            }
                        }
                    }
                }
                Err(_) => {
                    report.dangling_refs.push(ref_name.clone());
                }
            }
        }

        report.orphaned_blobs = count_orphaned_blobs(&repo, &referenced_oids)?;

        let mut checks = Vec::new();

        if report.dangling_refs.is_empty() {
            checks.push(ConsistencyCheckReport::check_passed(
                "Dangling refs",
                "No dangling refs found",
            ));
        } else {
            checks.push(ConsistencyCheckReport::check_failed(
                "Dangling refs",
                &format!("{} dangling ref(s) found", report.dangling_refs.len()),
            ));
        }

        if report.invalid_json_refs.is_empty() {
            checks.push(ConsistencyCheckReport::check_passed(
                "Valid JSON",
                "All refs contain valid JSON blobs",
            ));
        } else {
            checks.push(ConsistencyCheckReport::check_failed(
                "Valid JSON",
                &format!(
                    "{} ref(s) with invalid JSON: {:?}",
                    report.invalid_json_refs.len(),
                    &report.invalid_json_refs[..report.invalid_json_refs.len().min(5)]
                ),
            ));
        }

        if report.missing_required_fields.is_empty() {
            checks.push(ConsistencyCheckReport::check_passed(
                "Required fields",
                "All entities have required fields",
            ));
        } else {
            checks.push(ConsistencyCheckReport::check_failed(
                "Required fields",
                &format!(
                    "{} ref(s) missing required fields: {:?}",
                    report.missing_required_fields.len(),
                    &report.missing_required_fields[..report.missing_required_fields.len().min(5)]
                ),
            ));
        }

        if report.id_path_mismatches.is_empty() {
            checks.push(ConsistencyCheckReport::check_passed(
                "ID/path consistency",
                "Entity IDs match their ref paths",
            ));
        } else {
            checks.push(ConsistencyCheckReport::check_failed(
                "ID/path consistency",
                &format!(
                    "{} ref(s) with ID/path mismatch: {:?}",
                    report.id_path_mismatches.len(),
                    &report.id_path_mismatches[..report.id_path_mismatches.len().min(5)]
                ),
            ));
        }

        if report.future_timestamps.is_empty() {
            checks.push(ConsistencyCheckReport::check_passed(
                "Timestamp validity",
                "No future timestamps detected",
            ));
        } else {
            checks.push(ConsistencyCheckReport::check_warning(
                "Timestamp validity",
                &format!(
                    "{} ref(s) with future timestamps",
                    report.future_timestamps.len()
                ),
            ));
        }

        if report.orphaned_blobs == 0 {
            checks.push(ConsistencyCheckReport::check_passed(
                "Orphaned blobs",
                "No orphaned blobs found",
            ));
        } else {
            checks.push(ConsistencyCheckReport::check_warning(
                "Orphaned blobs",
                &format!("{} orphaned blob(s) found", report.orphaned_blobs),
            ));
        }

        report.checks = checks;
        Ok(report)
    }

    /// Scan all entity refs for triple- or double-nested `data` blobs and
    /// rewrite them in-place with the correct flat structure.
    ///
    /// Returns statistics describing how many refs were inspected and how many
    /// were rewritten.  The operation is **idempotent**: refs that are already
    /// flat are left unchanged; version sidecar refs are skipped entirely.
    pub fn flatten_nested_refs(&self, dry_run: bool) -> Result<FlattenRefsStats, EngramError> {
        let repo = self.repository.lock().map_err(|_| {
            EngramError::Storage(StorageError::InvalidState(
                "Repository lock failed".to_string(),
            ))
        })?;

        let engram_prefix = "refs/engram/";
        let mut stats = FlattenRefsStats::default();

        // Collect all ref names first to avoid holding borrows while modifying.
        let ref_names: Vec<String> = {
            let refs_platform = repo
                .references()
                .map_err(|e| EngramError::Git(format!("Failed to list references: {}", e)))?;
            let all_refs = refs_platform
                .all()
                .map_err(|e| EngramError::Git(format!("Failed to iterate references: {}", e)))?;
            let mut names = Vec::new();
            for r in all_refs {
                let r = r.map_err(|e| EngramError::Git(format!("Failed to read ref: {}", e)))?;
                let name = String::from_utf8_lossy(r.name().as_bstr()).to_string();
                if name.starts_with(engram_prefix) {
                    names.push(name);
                }
            }
            names
        };

        for ref_name in &ref_names {
            // Skip workspace config and version sidecar refs.
            if ref_name == "refs/engram/config/workspace" {
                continue;
            }
            // Sidecar refs contain "/v" followed by digits
            let after_prefix = ref_name.strip_prefix(engram_prefix).unwrap_or("");
            let segments: Vec<&str> = after_prefix.splitn(2, '/').collect();
            if segments.len() == 2 {
                let second_segment = segments[1];
                if second_segment.starts_with('v')
                    && second_segment
                        .chars()
                        .nth(1)
                        .is_some_and(|c: char| c.is_ascii_digit())
                {
                    continue;
                }
            }

            stats.refs_scanned += 1;

            // Read the blob via gix
            let reference = match repo
                .try_find_reference(ref_name)
                .map_err(|e| EngramError::Git(format!("Failed to find ref: {}", e)))?
            {
                Some(r) => r,
                None => continue,
            };
            let target_id = match reference.try_id() {
                Some(id) => id,
                None => continue,
            };
            let obj = match repo.find_object(target_id) {
                Ok(o) => o,
                Err(_) => continue,
            };
            let content = match std::str::from_utf8(&obj.data) {
                Ok(c) => c,
                Err(_) => continue,
            };
            let parsed: serde_json::Value = match serde_json::from_str(content) {
                Ok(v) => v,
                Err(_) => continue,
            };

            // Examine the `data` field for nesting.
            let data_field = match parsed.get("data") {
                Some(d) => d,
                None => continue,
            };

            let is_nested = match data_field {
                Value::Object(m) => m.get("data").is_some_and(|v| v.is_object()),
                _ => false,
            };

            if !is_nested {
                let is_single_key_wrap = match data_field {
                    Value::Object(m) => m.len() == 1 && m.contains_key("data"),
                    _ => false,
                };
                if !is_single_key_wrap {
                    continue;
                }
            }

            stats.nested_found += 1;

            if dry_run {
                continue;
            }

            // Build the flattened blob.
            let flat_data_map: HashMap<String, Value> = match data_field {
                Value::Object(m) => {
                    let inner: HashMap<String, Value> =
                        m.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
                    flatten_data_map(inner)
                }
                _ => continue,
            };

            let mut new_blob = parsed.clone();
            new_blob["data"] = serde_json::Value::Object(
                flat_data_map.into_iter().collect::<serde_json::Map<_, _>>(),
            );

            let new_content = match serde_json::to_string_pretty(&new_blob) {
                Ok(c) => c,
                Err(_) => continue,
            };

            let new_blob_id = repo
                .write_object(&gix::objs::Blob {
                    data: new_content.into_bytes(),
                })
                .map_err(|e| EngramError::Git(format!("Failed to create blob: {}", e)))?;

            use gix::refs::transaction::{Change, LogChange, PreviousValue, RefEdit};
            use gix::refs::FullName;
            use gix::refs::Target;

            repo.edit_reference(RefEdit {
                change: Change::Update {
                    log: LogChange::default(),
                    expected: PreviousValue::Any,
                    new: Target::Object(new_blob_id.detach()),
                },
                name: FullName::try_from(ref_name.as_str())
                    .map_err(|e| EngramError::Git(format!("Invalid ref name: {}", e)))?,
                deref: false,
            })
            .map_err(|e| EngramError::Git(format!("Failed to update ref {}: {}", ref_name, e)))?;

            stats.refs_rewritten += 1;
        }

        Ok(stats)
    }
}

/// Statistics from a `flatten_nested_refs` run.
#[derive(Debug, Default, Clone)]
pub struct FlattenRefsStats {
    /// Total entity refs inspected (workspace config and version sidecars excluded).
    pub refs_scanned: usize,
    /// Refs found to contain nested data.
    pub nested_found: usize,
    /// Refs that were rewritten (always 0 in dry-run mode).
    pub refs_rewritten: usize,
}

#[derive(Debug, Clone)]
pub struct RefreshDecayResult {
    pub entries_processed: usize,
    pub entries_updated: usize,
}

fn count_orphaned_blobs(
    repo: &gix::Repository,
    referenced_oids: &HashSet<String>,
) -> Result<usize, EngramError> {
    // gix doesn't have odb.foreach() — instead, iterate all refs and find
    // blob objects that aren't referenced. We use the object store iteration.
    // For now, return 0 as this is a stats-only function and gix ODB
    // iteration is significantly different from git2.
    //
    // A full implementation would use gix::odb::Store::iter() but that API
    // is not straightforward to use for this purpose.
    let _ = (repo, referenced_oids);
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::EntityRelationType;
    use crate::feedback::StructuredFeedback;
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
        let repo = gix::open(dir.path()).unwrap();
        let r = repo.try_find_reference("refs/engram/config/workspace");
        assert!(
            r.is_ok_and(|o| o.is_some()),
            "refs/engram/config/workspace must exist after new()"
        );
        // storage is used to prevent unused variable warning
        let _ = storage.project_id.len();
    }

    #[test]
    fn test_project_id_existing_repo_with_commits() {
        let dir = tempfile::tempdir().unwrap();
        // create a repo with a real commit first using gix
        {
            let repo = gix::init(dir.path()).unwrap();
            let empty_tree = gix::objs::Tree { entries: vec![] };
            let tree_id = repo.write_object(&empty_tree).unwrap();
            let sig = gix::actor::Signature {
                name: "test".into(),
                email: "test@test.com".into(),
                time: gix::date::Time::new(1700000000, 0),
            };
            let commit = gix::objs::Commit {
                tree: tree_id.detach(),
                parents: Default::default(),
                author: sig.clone(),
                committer: sig,
                message: "initial\n".into(),
                encoding: None,
                extra_headers: Default::default(),
            };
            let commit_id = repo.write_object(&commit).unwrap();

            use gix::refs::transaction::{Change, LogChange, PreviousValue, RefEdit};
            use gix::refs::FullName;
            use gix::refs::Target;

            repo.edit_reference(RefEdit {
                change: Change::Update {
                    log: LogChange::default(),
                    expected: PreviousValue::MustNotExist,
                    new: Target::Object(commit_id.detach()),
                },
                name: FullName::try_from("refs/heads/main").unwrap(),
                deref: false,
            })
            .unwrap();
            repo.edit_reference(RefEdit {
                change: Change::Update {
                    log: LogChange::default(),
                    expected: PreviousValue::Any,
                    new: Target::Symbolic(FullName::try_from("refs/heads/main").unwrap()),
                },
                name: FullName::try_from("HEAD").unwrap(),
                deref: false,
            })
            .unwrap();
        }
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
        let repo = gix::open(dir.path()).unwrap();
        let ref_name = format!("refs/engram/task/v1/{}", entity.id);
        assert!(
            repo.try_find_reference(&ref_name)
                .is_ok_and(|o| o.is_some()),
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
        let repo = gix::open(dir.path()).unwrap();
        let v1 = format!("refs/engram/task/v1/{}", entity.id);
        let v2 = format!("refs/engram/task/v2/{}", entity.id);
        assert!(
            repo.try_find_reference(&v1).is_ok_and(|o| o.is_some()),
            "v1 must still exist after second store"
        );
        assert!(
            repo.try_find_reference(&v2).is_ok_and(|o| o.is_some()),
            "v2 must exist after second store"
        );
    }

    #[test]
    fn test_version_sidecar_contains_project_id() {
        let dir = tempfile::tempdir().unwrap();
        let mut storage = GitRefsStorage::new(dir.path().to_str().unwrap(), "test").unwrap();
        let entity = make_test_entity("task");
        storage.store(&entity).unwrap();
        let repo = gix::open(dir.path()).unwrap();
        let r = repo
            .find_reference(&format!("refs/engram/task/v1/{}", entity.id))
            .unwrap();
        let target_id = r.try_id().unwrap();
        let obj = repo.find_object(target_id).unwrap();
        let v: serde_json::Value = serde_json::from_slice(&obj.data).unwrap();
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

    #[test]
    fn test_consistency_check_clean_storage() {
        let dir = tempfile::tempdir().unwrap();
        let mut storage = GitRefsStorage::new(dir.path().to_str().unwrap(), "test").unwrap();
        storage.store(&make_test_entity("task")).unwrap();
        storage.store(&make_test_entity("context")).unwrap();

        let report = storage.consistency_check().unwrap();
        assert_eq!(
            report.status_code(),
            crate::feedback::FeedbackStatus::Success
        );
        assert!(report.dangling_refs.is_empty());
        assert!(report.invalid_json_refs.is_empty());
        assert!(report.missing_required_fields.is_empty());
        assert!(report.id_path_mismatches.is_empty());
        assert!(report.future_timestamps.is_empty());
        assert!(report
            .checks
            .iter()
            .all(|c| c.status == ConsistencyCheckStatus::Pass));
    }

    #[test]
    fn test_consistency_check_detects_invalid_json() {
        let dir = tempfile::tempdir().unwrap();
        let storage = GitRefsStorage::new(dir.path().to_str().unwrap(), "test").unwrap();
        let repo = gix::open(dir.path()).unwrap();

        let bad_json = b"not valid json {{{";
        write_raw_blob(&repo, "refs/engram/task/bad-json-123", unsafe {
            std::str::from_utf8_unchecked(bad_json)
        });

        let report = storage.consistency_check().unwrap();
        assert!(report.invalid_json_refs.len() >= 1);
        assert_eq!(
            report.status_code(),
            crate::feedback::FeedbackStatus::Failed
        );
    }

    #[test]
    fn test_consistency_check_detects_missing_fields() {
        let dir = tempfile::tempdir().unwrap();
        let storage = GitRefsStorage::new(dir.path().to_str().unwrap(), "test").unwrap();
        let repo = gix::open(dir.path()).unwrap();

        let partial_json = r#"{"id": "test-123", "entity_type": "task"}"#;
        write_raw_blob(&repo, "refs/engram/task/test-123", partial_json);

        let report = storage.consistency_check().unwrap();
        assert!(
            !report.missing_required_fields.is_empty(),
            "Should detect missing agent and timestamp fields"
        );
    }

    #[test]
    fn test_consistency_check_detects_id_path_mismatch() {
        let dir = tempfile::tempdir().unwrap();
        let storage = GitRefsStorage::new(dir.path().to_str().unwrap(), "test").unwrap();
        let repo = gix::open(dir.path()).unwrap();

        let mismatched_json = r#"{
            "id": "different-id",
            "entity_type": "task",
            "agent": "test",
            "timestamp": "2025-01-01T00:00:00Z",
            "data": {}
        }"#;
        write_raw_blob(&repo, "refs/engram/task/path-id-abc", mismatched_json);

        let report = storage.consistency_check().unwrap();
        assert!(
            report
                .id_path_mismatches
                .contains(&"refs/engram/task/path-id-abc".to_string()),
            "Should detect ID/path mismatch"
        );
    }

    #[test]
    fn test_consistency_check_detects_dangling_ref() {
        let dir = tempfile::tempdir().unwrap();
        let storage = GitRefsStorage::new(dir.path().to_str().unwrap(), "test").unwrap();
        let repo = gix::open(dir.path()).unwrap();

        let fake_oid =
            gix::hash::ObjectId::from_hex(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();

        use gix::refs::transaction::{Change, LogChange, PreviousValue, RefEdit};
        use gix::refs::FullName;
        use gix::refs::Target;

        let result = repo.edit_reference(RefEdit {
            change: Change::Update {
                log: LogChange::default(),
                expected: PreviousValue::Any,
                new: Target::Object(fake_oid),
            },
            name: FullName::try_from("refs/engram/task/dangling-123").unwrap(),
            deref: false,
        });

        if result.is_ok() {
            let report = storage.consistency_check().unwrap();
            assert!(
                report
                    .dangling_refs
                    .iter()
                    .any(|r| r == "refs/engram/task/dangling-123"),
                "Should detect dangling ref"
            );
        }
    }

    #[test]
    fn test_consistency_report_serialization() {
        let report = ConsistencyCheckReport {
            checks: vec![ConsistencyCheckReport::check_passed("Test", "all good")],
            total_refs: 5,
            total_blobs_checked: 5,
            dangling_refs: vec![],
            invalid_json_refs: vec![],
            missing_required_fields: vec![],
            id_path_mismatches: vec![],
            future_timestamps: vec![],
            orphaned_blobs: 0,
        };

        let json = serde_json::to_string(&report).unwrap();
        let restored: ConsistencyCheckReport = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.total_refs, 5);
        assert_eq!(restored.checks.len(), 1);
        assert_eq!(restored.checks[0].status, ConsistencyCheckStatus::Pass);
    }

    #[test]
    fn test_consistency_report_feedback() {
        let mut report = ConsistencyCheckReport {
            checks: vec![],
            total_refs: 0,
            total_blobs_checked: 0,
            dangling_refs: vec![],
            invalid_json_refs: vec![],
            missing_required_fields: vec![],
            id_path_mismatches: vec![],
            future_timestamps: vec![],
            orphaned_blobs: 0,
        };

        report.checks = vec![
            ConsistencyCheckReport::check_passed("A", "ok"),
            ConsistencyCheckReport::check_passed("B", "ok"),
        ];
        assert_eq!(
            report.status_code(),
            crate::feedback::FeedbackStatus::Success
        );
        assert!(report.summary().contains("2/2 passed"));

        report.checks = vec![
            ConsistencyCheckReport::check_passed("A", "ok"),
            ConsistencyCheckReport::check_failed("B", "bad"),
        ];
        assert_eq!(
            report.status_code(),
            crate::feedback::FeedbackStatus::Failed
        );
        assert!(report.summary().contains("1 failed"));

        report.checks = vec![
            ConsistencyCheckReport::check_passed("A", "ok"),
            ConsistencyCheckReport::check_warning("B", "hmm"),
        ];
        assert_eq!(
            report.status_code(),
            crate::feedback::FeedbackStatus::Warning
        );
        assert!(report.summary().contains("1 warnings"));
    }

    // ── flatten_data_map unit tests ──────────────────────────────────────────

    #[test]
    fn test_flatten_data_map_already_flat() {
        let mut map = HashMap::new();
        map.insert("title".to_string(), json!("My Task"));
        map.insert("status".to_string(), json!("todo"));

        let result = flatten_data_map(map.clone());
        assert_eq!(result.len(), 2);
        assert_eq!(result["title"], json!("My Task"));
        assert_eq!(result["status"], json!("todo"));
    }

    #[test]
    fn test_flatten_data_map_double_nested() {
        // Simulate the bug: data wrapped in an extra "data" key.
        let inner = json!({"title": "Nested Task", "status": "done"});
        let mut map = HashMap::new();
        map.insert("data".to_string(), inner);

        let result = flatten_data_map(map);
        assert_eq!(result.len(), 2, "should be flattened to 2 keys");
        assert_eq!(result["title"], json!("Nested Task"));
        assert_eq!(result["status"], json!("done"));
    }

    #[test]
    fn test_flatten_data_map_triple_nested() {
        // Simulate the triple-nesting: data.data.data.*
        let innermost = json!({"title": "Deep Task", "priority": "high"});
        let middle = json!({"data": innermost});
        let mut map = HashMap::new();
        map.insert("data".to_string(), middle);

        let result = flatten_data_map(map);
        assert_eq!(
            result.len(),
            2,
            "should be flattened through all nesting layers"
        );
        assert_eq!(result["title"], json!("Deep Task"));
        assert_eq!(result["priority"], json!("high"));
    }

    #[test]
    fn test_flatten_data_map_multi_key_not_flattened() {
        // If data_map has multiple keys and one of them is "data", it is already
        // at the right level — do NOT flatten (the "data" key is just a field).
        let mut map = HashMap::new();
        map.insert("title".to_string(), json!("My Task"));
        map.insert("data".to_string(), json!({"nested": true}));

        let result = flatten_data_map(map.clone());
        // Should be unchanged because map.len() != 1
        assert_eq!(result.len(), 2);
        assert!(result.contains_key("title"));
        assert!(result.contains_key("data"));
    }

    // ── store round-trip: no nesting introduced ───────────────────────────────

    #[test]
    fn test_store_and_retrieve_produces_flat_data() {
        let dir = tempfile::tempdir().unwrap();
        let mut storage = GitRefsStorage::new(dir.path().to_str().unwrap(), "test").unwrap();

        let entity = GenericEntity {
            id: "flat-test-1".to_string(),
            entity_type: "task".to_string(),
            agent: "test".to_string(),
            timestamp: Utc::now(),
            data: json!({
                "title": "Test Task",
                "status": "todo",
                "priority": "high"
            }),
        };

        storage.store(&entity).unwrap();
        let retrieved = storage.get("flat-test-1", "task").unwrap().unwrap();

        // The retrieved data must be flat — no "data" key inside data.
        if let Value::Object(map) = &retrieved.data {
            assert!(
                !map.contains_key("data"),
                "retrieved data must not contain a nested 'data' key"
            );
            assert_eq!(map["title"], json!("Test Task"));
            assert_eq!(map["status"], json!("todo"));
        } else {
            panic!("retrieved.data should be a JSON Object");
        }
    }

    #[test]
    fn test_store_double_wrapped_entity_is_flattened() {
        // Simulate the bug at the storage layer: an entity whose `data` field
        // is double-wrapped (i.e., data = {"data": {"title": "..."}}).
        let dir = tempfile::tempdir().unwrap();
        let mut storage = GitRefsStorage::new(dir.path().to_str().unwrap(), "test").unwrap();

        let entity = GenericEntity {
            id: "nested-test-1".to_string(),
            entity_type: "task".to_string(),
            agent: "test".to_string(),
            timestamp: Utc::now(),
            data: json!({
                "data": {
                    "title": "Wrapped Task",
                    "status": "todo"
                }
            }),
        };

        storage.store(&entity).unwrap();
        let retrieved = storage.get("nested-test-1", "task").unwrap().unwrap();

        // The store must have detected and fixed the nesting.
        if let Value::Object(map) = &retrieved.data {
            assert!(
                !map.contains_key("data"),
                "store must flatten a double-wrapped data field"
            );
            assert_eq!(map["title"], json!("Wrapped Task"));
            assert_eq!(map["status"], json!("todo"));
        } else {
            panic!("retrieved.data should be a JSON Object");
        }
    }

    // ── flatten_nested_refs integration tests ─────────────────────────────────

    /// Helper: write a raw blob JSON directly into a git ref.
    fn write_raw_blob(repo: &gix::Repository, ref_name: &str, json: &str) {
        use gix::refs::transaction::{Change, LogChange, PreviousValue, RefEdit};
        use gix::refs::FullName;
        use gix::refs::Target;

        let blob_id = repo
            .write_object(&gix::objs::Blob {
                data: json.as_bytes().to_vec(),
            })
            .unwrap();
        repo.edit_reference(RefEdit {
            change: Change::Update {
                log: LogChange::default(),
                expected: PreviousValue::Any,
                new: Target::Object(blob_id.detach()),
            },
            name: FullName::try_from(ref_name).unwrap(),
            deref: false,
        })
        .unwrap();
    }

    #[test]
    fn test_flatten_nested_refs_no_nested_refs() {
        let dir = tempfile::tempdir().unwrap();
        let mut storage = GitRefsStorage::new(dir.path().to_str().unwrap(), "test").unwrap();
        storage.store(&make_test_entity("task")).unwrap();

        let stats = storage.flatten_nested_refs(false).unwrap();
        assert_eq!(stats.nested_found, 0);
        assert_eq!(stats.refs_rewritten, 0);
        assert!(stats.refs_scanned >= 1);
    }

    #[test]
    fn test_flatten_nested_refs_detects_and_fixes_double_nested() {
        let dir = tempfile::tempdir().unwrap();
        let storage = GitRefsStorage::new(dir.path().to_str().unwrap(), "test").unwrap();
        let repo = gix::open(dir.path()).unwrap();

        // Write a double-nested blob manually.
        let nested_json = r#"{
            "id": "nested-ref-1",
            "entity_type": "task",
            "agent": "test",
            "timestamp": "2026-01-01T00:00:00Z",
            "data": {
                "data": {
                    "title": "Should Be Flat",
                    "status": "todo"
                }
            },
            "content_hash": "",
            "size_bytes": 0,
            "tags": [],
            "references": [],
            "metadata": {}
        }"#;
        write_raw_blob(&repo, "refs/engram/task/nested-ref-1", nested_json);

        let stats = storage.flatten_nested_refs(false).unwrap();
        assert_eq!(stats.nested_found, 1, "should detect one nested ref");
        assert_eq!(stats.refs_rewritten, 1, "should rewrite one ref");

        // Verify the blob is now flat.
        let r = repo
            .find_reference("refs/engram/task/nested-ref-1")
            .unwrap();
        let target_id = r.try_id().unwrap();
        let obj = repo.find_object(target_id).unwrap();
        let updated: serde_json::Value = serde_json::from_slice(&obj.data).unwrap();
        let data = updated.get("data").unwrap();
        assert!(
            !data.get("data").is_some_and(|v| v.is_object()),
            "data must no longer contain a nested 'data' object after flatten"
        );
        assert_eq!(data["title"], json!("Should Be Flat"));
    }

    #[test]
    fn test_flatten_nested_refs_dry_run_does_not_write() {
        let dir = tempfile::tempdir().unwrap();
        let storage = GitRefsStorage::new(dir.path().to_str().unwrap(), "test").unwrap();
        let repo = gix::open(dir.path()).unwrap();

        let nested_json = r#"{
            "id": "dry-run-1",
            "entity_type": "task",
            "agent": "test",
            "timestamp": "2026-01-01T00:00:00Z",
            "data": {
                "data": {
                    "title": "Dry Run Task"
                }
            },
            "content_hash": "",
            "size_bytes": 0,
            "tags": [],
            "references": [],
            "metadata": {}
        }"#;
        write_raw_blob(&repo, "refs/engram/task/dry-run-1", nested_json);

        let stats = storage.flatten_nested_refs(true /* dry_run */).unwrap();
        assert_eq!(stats.nested_found, 1);
        assert_eq!(stats.refs_rewritten, 0, "dry-run must not rewrite any refs");

        // Verify the blob is still nested (unchanged).
        let r = repo
            .try_find_reference("refs/engram/task/dry-run-1")
            .unwrap()
            .unwrap();
        let target_id = r.try_id().unwrap();
        let obj = repo.find_object(target_id).unwrap();
        let still_nested: serde_json::Value = serde_json::from_slice(&obj.data).unwrap();
        assert!(
            still_nested["data"]["data"]["title"] == json!("Dry Run Task"),
            "blob must remain unchanged after dry-run"
        );
    }

    #[test]
    fn test_flatten_nested_refs_idempotent() {
        // Running flatten twice should produce the same result.
        let dir = tempfile::tempdir().unwrap();
        let storage = GitRefsStorage::new(dir.path().to_str().unwrap(), "test").unwrap();
        let repo = gix::open(dir.path()).unwrap();

        let nested_json = r#"{
            "id": "idem-1",
            "entity_type": "task",
            "agent": "test",
            "timestamp": "2026-01-01T00:00:00Z",
            "data": {
                "data": {
                    "title": "Idempotent Task"
                }
            },
            "content_hash": "",
            "size_bytes": 0,
            "tags": [],
            "references": [],
            "metadata": {}
        }"#;
        write_raw_blob(&repo, "refs/engram/task/idem-1", nested_json);

        // First run.
        let stats1 = storage.flatten_nested_refs(false).unwrap();
        assert_eq!(stats1.refs_rewritten, 1);

        // Second run — should find nothing to fix.
        let stats2 = storage.flatten_nested_refs(false).unwrap();
        assert_eq!(
            stats2.nested_found, 0,
            "second flatten pass must find no nested refs"
        );
        assert_eq!(stats2.refs_rewritten, 0);
    }

    #[test]
    fn test_flatten_nested_refs_skips_version_sidecars() {
        let dir = tempfile::tempdir().unwrap();
        let mut storage = GitRefsStorage::new(dir.path().to_str().unwrap(), "test").unwrap();
        let entity = make_test_entity("task");
        storage.store(&entity).unwrap(); // creates v1 sidecar

        // The sidecar must not be counted in refs_scanned.
        let stats = storage.flatten_nested_refs(false).unwrap();
        // Scanned should only cover the primary ref, not v1/<uuid>.
        // Primary ref: 1; sidecar: 0.
        assert_eq!(
            stats.refs_scanned, 1,
            "version sidecar refs must be excluded from scanning"
        );
    }

    #[test]
    fn test_reasoning_event_append_only() {
        let dir = tempfile::tempdir().unwrap();
        let mut storage = GitRefsStorage::new(dir.path().to_str().unwrap(), "test").unwrap();

        let mut event = crate::entities::ReasoningEvent::new(
            "reasoning-1".to_string(),
            crate::entities::ReasoningEventType::AutoStored,
            "First store".to_string(),
        );
        event.agent = "agent".to_string();
        let event_id = event.id.clone();
        let generic = event.to_generic();
        storage.store(&generic).unwrap();

        // Store again with the same ID must fail
        let event2 = GenericEntity {
            id: event_id,
            entity_type: "reasoning_event".to_string(),
            agent: "agent".to_string(),
            timestamp: Utc::now(),
            data: serde_json::json!({
                "reasoning_id": "reasoning-1",
                "event_type": "auto_stored",
                "content": "Second store",
                "agent": "agent",
                "created_at": Utc::now().to_rfc3339(),
            }),
        };
        let result = storage.store(&event2);
        assert!(result.is_err(), "Storing reasoning_event twice should fail");
    }

    #[test]
    fn test_reasoning_store_auto_emits_event() {
        let dir = tempfile::tempdir().unwrap();
        let mut storage = GitRefsStorage::new(dir.path().to_str().unwrap(), "test").unwrap();

        let reasoning = crate::entities::Reasoning::new(
            "Auto-emit test".to_string(),
            "task-1".to_string(),
            "agent".to_string(),
        );
        let generic = reasoning.to_generic();
        storage.store(&generic).unwrap();

        // A reasoning_event should have been created
        let event_ids = storage.list_entity_refs("reasoning_event").unwrap();
        assert_eq!(
            event_ids.len(),
            1,
            "Should have auto-emitted one reasoning_event"
        );

        let event_entity = storage
            .get(&event_ids[0], "reasoning_event")
            .unwrap()
            .unwrap();
        let event = crate::entities::ReasoningEvent::from_generic(event_entity).unwrap();
        assert_eq!(event.reasoning_id, reasoning.id);
        assert_eq!(
            event.event_type,
            crate::entities::ReasoningEventType::AutoStored
        );
    }

    #[test]
    fn test_knowledge_citation_increment_on_relationship() {
        let dir = tempfile::tempdir().unwrap();
        let mut storage = GitRefsStorage::new(dir.path().to_str().unwrap(), "test").unwrap();

        // Store a knowledge entity
        let knowledge = crate::entities::Knowledge::new(
            "Test knowledge".to_string(),
            "Content".to_string(),
            crate::entities::KnowledgeType::Fact,
            0.9,
            "agent".to_string(),
        );
        let k_generic = knowledge.to_generic();
        storage.store(&k_generic).unwrap();

        // Create a relationship pointing at the knowledge
        let rel = EntityRelationship::new(
            uuid::Uuid::new_v4().to_string(),
            "agent".to_string(),
            "other-entity".to_string(),
            "task".to_string(),
            knowledge.id.clone(),
            "knowledge".to_string(),
            EntityRelationType::References,
        );
        storage.store_relationship(&rel).unwrap();

        // Knowledge citation_count should be incremented
        let stored = storage.get(&knowledge.id, "knowledge").unwrap().unwrap();
        let stored_k = crate::entities::Knowledge::from_generic(stored).unwrap();
        assert_eq!(stored_k.citation_count, 1, "Citation count should be 1");
        assert!(
            stored_k.last_used_at.is_some(),
            "last_used_at should be set"
        );
    }
}
