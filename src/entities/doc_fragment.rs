//! DocFragment entity implementation

use super::{Entity, GenericEntity};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// DocFragment entity representing a named documentation content chunk
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct DocFragment {
    /// Unique identifier
    #[serde(rename = "id")]
    pub id: String,

    /// Topic this fragment belongs to (e.g., "adrs", "knowledge", "overview")
    #[serde(rename = "topic")]
    pub topic: String,

    /// Chunk identifier within the topic (named by the LLM agent)
    #[serde(rename = "chunk_id")]
    pub chunk_id: String,

    /// Human-readable title for this fragment
    #[serde(rename = "title")]
    pub title: String,

    /// Markdown content of this fragment
    #[serde(rename = "content")]
    pub content: String,

    /// Ordering position within the topic for assembly
    #[serde(rename = "order")]
    pub order: u32,

    /// Timestamp when this fragment was written
    #[serde(rename = "written_at")]
    pub written_at: DateTime<Utc>,

    /// Agent that wrote this fragment
    #[serde(rename = "agent")]
    pub agent: String,

    /// Entity IDs that this chunk references (source material)
    #[serde(
        rename = "source_entity_ids",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub source_entity_ids: Vec<String>,

    /// Whether this fragment is stale (source entities changed since written_at)
    #[serde(rename = "stale")]
    pub stale: bool,
}

impl DocFragment {
    pub fn new(
        topic: String,
        chunk_id: String,
        title: String,
        content: String,
        order: u32,
        agent: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            topic,
            chunk_id,
            title,
            content,
            order,
            written_at: Utc::now(),
            agent,
            source_entity_ids: Vec::new(),
            stale: false,
        }
    }

    pub fn update_content(&mut self, content: String) {
        self.content = content;
        self.written_at = Utc::now();
        self.stale = false;
    }

    pub fn mark_stale(&mut self) {
        self.stale = true;
    }

    pub fn add_source_entity(&mut self, entity_id: String) {
        if !self.source_entity_ids.contains(&entity_id) {
            self.source_entity_ids.push(entity_id);
        }
    }
}

impl Entity for DocFragment {
    fn entity_type() -> &'static str {
        "doc_fragment"
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn agent(&self) -> &str {
        &self.agent
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.written_at
    }

    fn validate_entity(&self) -> crate::Result<()> {
        if let Err(errors) = <DocFragment as validator::Validate>::validate(self) {
            let error_messages: Vec<String> = errors
                .field_errors()
                .values()
                .flat_map(|field_errors| field_errors.iter())
                .map(|error| {
                    error
                        .message
                        .clone()
                        .map(|s| s.to_string())
                        .unwrap_or_default()
                })
                .collect();
            return Err(crate::EngramError::Validation(error_messages.join(", ")));
        }

        if self.topic.is_empty() {
            return Err(crate::EngramError::Validation(
                "DocFragment topic cannot be empty".to_string(),
            ));
        }

        if self.chunk_id.is_empty() {
            return Err(crate::EngramError::Validation(
                "DocFragment chunk_id cannot be empty".to_string(),
            ));
        }

        if self.title.is_empty() {
            return Err(crate::EngramError::Validation(
                "DocFragment title cannot be empty".to_string(),
            ));
        }

        if self.content.is_empty() {
            return Err(crate::EngramError::Validation(
                "DocFragment content cannot be empty".to_string(),
            ));
        }

        Ok(())
    }

    fn to_generic(&self) -> GenericEntity {
        GenericEntity {
            id: self.id.clone(),
            entity_type: Self::entity_type().to_string(),
            agent: self.agent.clone(),
            timestamp: self.written_at,
            data: serde_json::to_value(self).unwrap_or_default(),
        }
    }

    fn from_generic(entity: GenericEntity) -> crate::Result<Self> {
        serde_json::from_value(entity.data).map_err(|e| {
            crate::EngramError::Deserialization(format!("Failed to deserialize DocFragment: {}", e))
        })
    }

    fn as_any(&self) -> &dyn std::any::Any
    where
        Self: Sized,
    {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doc_fragment_creation() {
        let fragment = DocFragment::new(
            "adrs".to_string(),
            "database-choice".to_string(),
            "Database Selection".to_string(),
            "# Database\nWe chose Postgres.".to_string(),
            1,
            "writer".to_string(),
        );

        assert_eq!(fragment.topic, "adrs");
        assert_eq!(fragment.chunk_id, "database-choice");
        assert_eq!(fragment.title, "Database Selection");
        assert_eq!(fragment.order, 1);
        assert!(!fragment.stale);
        assert!(fragment.source_entity_ids.is_empty());
        assert!(!fragment.id.is_empty());
    }

    #[test]
    fn test_doc_fragment_update_content() {
        let mut fragment = DocFragment::new(
            "knowledge".to_string(),
            "auth-flow".to_string(),
            "Auth Flow".to_string(),
            "Original".to_string(),
            2,
            "writer".to_string(),
        );

        fragment.mark_stale();
        assert!(fragment.stale);

        fragment.update_content("Updated content".to_string());
        assert_eq!(fragment.content, "Updated content");
        assert!(!fragment.stale);
    }

    #[test]
    fn test_doc_fragment_source_entities() {
        let mut fragment = DocFragment::new(
            "overview".to_string(),
            "summary".to_string(),
            "Summary".to_string(),
            "Content".to_string(),
            0,
            "writer".to_string(),
        );

        fragment.add_source_entity("entity-1".to_string());
        fragment.add_source_entity("entity-2".to_string());
        fragment.add_source_entity("entity-1".to_string());

        assert_eq!(fragment.source_entity_ids.len(), 2);
        assert!(fragment.source_entity_ids.contains(&"entity-1".to_string()));
        assert!(fragment.source_entity_ids.contains(&"entity-2".to_string()));
    }

    #[test]
    fn test_doc_fragment_validation() {
        let fragment = DocFragment::new(
            "".to_string(),
            "chunk".to_string(),
            "Title".to_string(),
            "Content".to_string(),
            0,
            "writer".to_string(),
        );
        assert!(fragment.validate_entity().is_err());

        let fragment = DocFragment::new(
            "topic".to_string(),
            "".to_string(),
            "Title".to_string(),
            "Content".to_string(),
            0,
            "writer".to_string(),
        );
        assert!(fragment.validate_entity().is_err());

        let fragment = DocFragment::new(
            "topic".to_string(),
            "chunk".to_string(),
            "".to_string(),
            "Content".to_string(),
            0,
            "writer".to_string(),
        );
        assert!(fragment.validate_entity().is_err());

        let fragment = DocFragment::new(
            "topic".to_string(),
            "chunk".to_string(),
            "Title".to_string(),
            "".to_string(),
            0,
            "writer".to_string(),
        );
        assert!(fragment.validate_entity().is_err());

        let fragment = DocFragment::new(
            "topic".to_string(),
            "chunk".to_string(),
            "Title".to_string(),
            "Content".to_string(),
            0,
            "writer".to_string(),
        );
        assert!(fragment.validate_entity().is_ok());
    }

    #[test]
    fn test_doc_fragment_serde_roundtrip() {
        let fragment = DocFragment::new(
            "adrs".to_string(),
            "api-design".to_string(),
            "API Design".to_string(),
            "# API\nREST endpoints.".to_string(),
            3,
            "agent".to_string(),
        );

        let json = serde_json::to_string(&fragment).expect("serialize");
        let deserialized: DocFragment = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(deserialized.id, fragment.id);
        assert_eq!(deserialized.topic, fragment.topic);
        assert_eq!(deserialized.chunk_id, fragment.chunk_id);
        assert_eq!(deserialized.order, fragment.order);
    }

    #[test]
    fn test_doc_fragment_generic_roundtrip() {
        let fragment = DocFragment::new(
            "reasoning".to_string(),
            "debug-notes".to_string(),
            "Debug Notes".to_string(),
            "Findings...".to_string(),
            1,
            "agent".to_string(),
        );

        let generic = fragment.to_generic();
        assert_eq!(generic.entity_type, "doc_fragment");

        let restored = DocFragment::from_generic(generic).expect("from_generic");
        assert_eq!(restored.id, fragment.id);
        assert_eq!(restored.topic, fragment.topic);
        assert_eq!(restored.chunk_id, fragment.chunk_id);
    }

    #[test]
    fn test_staleness_report_has_stale() {
        let report = StalenessReport {
            total_fragments: 5,
            stale_count: 2,
            fresh_count: 3,
            stale_chunks: vec![],
            checked_at: Utc::now(),
        };
        assert!(report.has_stale());

        let report = StalenessReport {
            total_fragments: 5,
            stale_count: 0,
            fresh_count: 5,
            stale_chunks: vec![],
            checked_at: Utc::now(),
        };
        assert!(!report.has_stale());
    }

    #[test]
    fn test_stale_chunk_and_outdated_source_serde() {
        let stale = StaleChunk {
            fragment_id: "frag-1".to_string(),
            topic: "adrs".to_string(),
            chunk_id: "db-choice".to_string(),
            written_at: Utc::now(),
            outdated_by: vec![OutdatedSource {
                entity_id: "task-42".to_string(),
                entity_type: "task".to_string(),
                source_timestamp: Utc::now(),
            }],
        };

        let json = serde_json::to_string(&stale).expect("serialize");
        let deserialized: StaleChunk = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(deserialized.fragment_id, "frag-1");
        assert_eq!(deserialized.outdated_by.len(), 1);
        assert_eq!(deserialized.outdated_by[0].entity_id, "task-42");
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaleChunk {
    pub fragment_id: String,
    pub topic: String,
    pub chunk_id: String,
    pub written_at: DateTime<Utc>,
    pub outdated_by: Vec<OutdatedSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutdatedSource {
    pub entity_id: String,
    pub entity_type: String,
    pub source_timestamp: DateTime<Utc>,
}

/// Result of a staleness check across all doc fragments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StalenessReport {
    pub total_fragments: usize,
    pub stale_count: usize,
    pub fresh_count: usize,
    pub stale_chunks: Vec<StaleChunk>,
    pub checked_at: DateTime<Utc>,
}

impl StalenessReport {
    pub fn has_stale(&self) -> bool {
        self.stale_count > 0
    }
}

/// Check staleness for all doc fragments in storage.
///
/// For each fragment, looks up its `source_entity_ids`. If any source entity's
/// `timestamp` (in its GenericEntity) is strictly after the fragment's
/// `written_at`, the fragment is considered stale.
pub fn check_staleness(storage: &dyn crate::storage::Storage) -> crate::Result<StalenessReport> {
    let all = storage.get_all("doc_fragment")?;

    let mut stale_chunks = Vec::new();
    let mut stale_count = 0usize;

    for generic in &all {
        let frag = match DocFragment::from_generic(generic.clone()) {
            Ok(f) => f,
            Err(_) => continue,
        };

        let mut outdated_by = Vec::new();

        for src_id in &frag.source_entity_ids {
            let found = try_get_entity_any_type(storage, src_id);
            if let Some(src_generic) = found {
                let src_ts = src_generic.timestamp;
                if src_ts > frag.written_at {
                    outdated_by.push(OutdatedSource {
                        entity_id: src_id.clone(),
                        entity_type: src_generic.entity_type.clone(),
                        source_timestamp: src_ts,
                    });
                }
            }
        }

        if !outdated_by.is_empty() {
            stale_count += 1;
            stale_chunks.push(StaleChunk {
                fragment_id: frag.id.clone(),
                topic: frag.topic.clone(),
                chunk_id: frag.chunk_id.clone(),
                written_at: frag.written_at,
                outdated_by,
            });
        }
    }

    let fresh_count = all.len() - stale_count;

    Ok(StalenessReport {
        total_fragments: all.len(),
        stale_count,
        fresh_count,
        stale_chunks,
        checked_at: Utc::now(),
    })
}

/// Try to retrieve an entity by ID, searching across all known entity types.
fn try_get_entity_any_type(
    storage: &dyn crate::storage::Storage,
    entity_id: &str,
) -> Option<crate::entities::GenericEntity> {
    let types = [
        "task",
        "context",
        "reasoning",
        "knowledge",
        "adr",
        "session",
        "standard",
        "rule",
        "workflow",
        "workflow_instance",
        "theory",
        "compliance",
        "escalation_request",
        "state_reflection",
        "doc_fragment",
        "bottleneck_report",
        "dora_metrics_report",
        "execution_result",
        "task_duration_report",
    ];

    for entity_type in &types {
        if let Ok(Some(entity)) = storage.get(entity_id, entity_type) {
            return Some(entity);
        }
    }

    None
}
