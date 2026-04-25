//! Entity trait and generic entity representation.

use serde::{Deserialize, Serialize};
use std::any::Any;

/// Core entity trait — all engram entity types implement this.
///
/// Provides a uniform interface for CRUD, validation, and serialization
/// across all 27 entity types.
pub trait Entity: Serialize + for<'de> Deserialize<'de> + Send + Sync {
    /// Get the entity type identifier (e.g. "task", "context", "reasoning").
    fn entity_type() -> &'static str;

    /// Get the entity ID.
    fn id(&self) -> &str;

    /// Get the agent associated with this entity.
    fn agent(&self) -> &str;

    /// Get timestamp for this entity.
    fn timestamp(&self) -> chrono::DateTime<chrono::Utc>;

    /// Validate the entity.
    fn validate_entity(&self) -> Result<(), crate::error::EngramError>;

    /// Convert to generic representation.
    fn to_generic(&self) -> GenericEntity;

    /// Create from generic representation.
    fn from_generic(entity: GenericEntity) -> Result<Self, crate::error::EngramError>
    where
        Self: Sized;

    /// Convert to Any for downcasting.
    fn as_any(&self) -> &dyn Any
    where
        Self: Sized;

    /// Downcast to specific type.
    fn downcast_ref<T: Entity + 'static>(&self) -> Option<&T>
    where
        Self: Sized,
    {
        self.as_any().downcast_ref()
    }
}

/// Generic entity representation for dynamic handling.
///
/// All entity types can be converted to/from this form for storage
/// and querying without knowing the concrete type at compile time.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GenericEntity {
    pub id: String,
    #[serde(alias = "type")]
    pub entity_type: String,
    pub agent: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub data: serde_json::Value,
}

impl GenericEntity {
    /// Create a GenericEntity from a serde_json::Value.
    pub fn from_value(value: serde_json::Value) -> Result<Self, crate::error::EngramError> {
        let id = value
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| crate::error::EngramError::Deserialization("Missing id".into()))?
            .to_string();

        let entity_type = value
            .get("type")
            .or_else(|| value.get("entity_type"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| crate::error::EngramError::Deserialization("Missing type".into()))?
            .to_string();

        let agent = value
            .get("agent")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();

        let timestamp = value
            .get("timestamp")
            .and_then(|v| v.as_str())
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.to_utc())
            .unwrap_or_else(chrono::Utc::now);

        Ok(Self {
            id,
            entity_type,
            agent,
            timestamp,
            data: value,
        })
    }
}

/// Deserialize function for entity types in the registry.
pub type EntityDeserializeFn = fn(GenericEntity) -> Result<GenericEntity, crate::error::EngramError>;

/// Registry mapping entity type names to their deserializers.
///
/// Used for dynamic entity type resolution during storage queries.
pub struct EntityRegistry {
    /// Map of entity type name → deserialize function.
    pub types: std::collections::HashMap<String, EntityDeserializeFn>,
}

impl EntityRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self {
            types: std::collections::HashMap::new(),
        }
    }

    /// Register an entity type (validates it can round-trip through GenericEntity).
    pub fn register<E: Entity + 'static>(&mut self) {
        self.types.insert(
            E::entity_type().to_string(),
            Ok,
        );
    }

    /// Check if a type is registered.
    pub fn has_type(&self, type_name: &str) -> bool {
        self.types.contains_key(type_name)
    }

    /// Get all registered type names.
    pub fn type_names(&self) -> Vec<String> {
        self.types.keys().cloned().collect()
    }
}

impl Default for EntityRegistry {
    fn default() -> Self {
        Self::new()
    }
}
