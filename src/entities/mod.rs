//! Core entity types for the Engram system
//!
//! This module provides extensible entity types that can be dynamically
//! configured and extended through the configuration system.

pub mod adr;
pub mod compliance;
pub mod context;
pub mod execution_result;
pub mod knowledge;
pub mod reasoning;
pub mod relationship;
pub mod rule;
pub mod session;
pub mod standard;
pub mod task;
pub mod workflow;

// Re-export all entity types
pub use adr::*;
pub use compliance::*;
pub use context::*;
pub use execution_result::*;
pub use knowledge::*;
pub use reasoning::*;
pub use relationship::*;
pub use rule::*;
pub use session::*;
pub use standard::*;
pub use task::*;
pub use workflow::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Trait for extensible entities
pub trait Entity: Serialize + for<'de> Deserialize<'de> + Send + Sync {
    /// Get the entity type identifier
    fn entity_type() -> &'static str;

    /// Get the entity ID
    fn id(&self) -> &str;

    /// Get the agent associated with this entity
    fn agent(&self) -> &str;

    /// Get timestamp for this entity
    fn timestamp(&self) -> chrono::DateTime<chrono::Utc>;

    /// Validate the entity
    fn validate_entity(&self) -> Result<()>;

    /// Convert to generic representation
    fn to_generic(&self) -> GenericEntity;

    /// Create from generic representation
    fn from_generic(entity: GenericEntity) -> Result<Self>
    where
        Self: Sized;

    /// Get entity type identifier (associated function)
    fn get_entity_type() -> &'static str
    where
        Self: Sized,
    {
        Self::entity_type()
    }

    /// Get the entity ID (associated function)
    fn get_id(entity: &Self) -> String
    where
        Self: Sized,
    {
        entity.id().to_string()
    }

    /// Get the agent associated with this entity (associated function)
    fn get_agent(entity: &Self) -> String
    where
        Self: Sized,
    {
        entity.agent().to_string()
    }

    /// Get timestamp for this entity (associated function)
    fn get_timestamp(entity: &Self) -> chrono::DateTime<chrono::Utc>
    where
        Self: Sized,
    {
        entity.timestamp()
    }

    /// Validate the entity (associated function)
    fn validate_entity_static(entity: &Self) -> Result<()>
    where
        Self: Sized,
    {
        entity.validate_entity()
    }

    /// Convert to generic representation (associated function)
    fn to_generic_entity(entity: &Self) -> GenericEntity
    where
        Self: Sized,
    {
        entity.to_generic()
    }

    /// Convert to Any for downcasting
    fn as_any(&self) -> &dyn std::any::Any
    where
        Self: Sized;

    /// Downcast to specific type
    fn downcast_ref<T: Entity + 'static>(&self) -> Option<&T>
    where
        Self: Sized,
    {
        self.as_any().downcast_ref()
    }
}

/// Generic entity representation for dynamic handling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericEntity {
    pub id: String,
    pub entity_type: String,
    pub agent: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub data: serde_json::Value,
}

impl GenericEntity {
    /// Create a GenericEntity from a serde_json::Value
    pub fn from_value(value: serde_json::Value) -> Result<Self> {
        serde_json::from_value(value)
            .map_err(|e| format!("Failed to deserialize GenericEntity: {}", e))
    }
}

/// Registry for entity types
pub struct EntityRegistry {
    entities: HashMap<String, EntityFactory>,
}

type EntityFactory = Box<dyn Fn(GenericEntity) -> Result<GenericEntity> + Send + Sync>;

impl EntityRegistry {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
        }
    }

    pub fn register<T>(&mut self)
    where
        T: Entity + 'static + for<'de> Deserialize<'de> + Serialize,
    {
        let factory = Box::new(|entity: GenericEntity| -> Result<GenericEntity> {
            T::from_generic(entity.clone()).map(|t| t.to_generic())
        });
        self.entities.insert(T::entity_type().to_string(), factory);
    }

    pub fn create(&self, entity: GenericEntity) -> Result<GenericEntity> {
        let factory = self
            .entities
            .get(&entity.entity_type)
            .ok_or_else(|| format!("Unknown entity type: {}", entity.entity_type))?;
        factory(entity)
    }

    pub fn list_types(&self) -> Vec<&str> {
        self.entities.keys().map(|k| k.as_str()).collect()
    }
}

impl Default for EntityRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Result type for entity operations
pub type Result<T> = std::result::Result<T, String>;
