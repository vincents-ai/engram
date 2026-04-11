//! Schema generation and publishing commands

use crate::entities::{
    adr::ADR, compliance::Compliance, context::Context, doc_fragment::DocFragment,
    knowledge::Knowledge, reasoning::Reasoning, session::Session,
    state_reflection::StateReflection, task::Task, theory::Theory, workflow::Workflow,
};
use crate::storage::Storage;
use clap::Subcommand;
use schemars::schema_for;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct EntitySchemaWrapper {
    id: String,
    namespace_pattern: String,
    title: String,
    version: String,
    schema: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    ui: Option<serde_json::Value>,
}

fn entity_schema<T: Serialize + schemars::JsonSchema>(
    id: &str,
    namespace_pattern: &str,
    title: &str,
) -> EntitySchemaWrapper {
    let schema = schema_for!(T);
    EntitySchemaWrapper {
        id: id.to_string(),
        namespace_pattern: namespace_pattern.to_string(),
        title: title.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        schema: serde_json::to_value(schema).unwrap_or_default(),
        ui: None,
    }
}

fn all_entity_schemas() -> Vec<EntitySchemaWrapper> {
    vec![
        entity_schema::<Task>("engram-task", "refs/engram/task/*", "Engram Task"),
        entity_schema::<Context>("engram-context", "refs/engram/context/*", "Engram Context"),
        entity_schema::<Reasoning>(
            "engram-reasoning",
            "refs/engram/reasoning/*",
            "Engram Reasoning",
        ),
        entity_schema::<Knowledge>(
            "engram-knowledge",
            "refs/engram/knowledge/*",
            "Engram Knowledge",
        ),
        entity_schema::<Session>("engram-session", "refs/engram/session/*", "Engram Session"),
        entity_schema::<ADR>("engram-adr", "refs/engram/adr/*", "Engram ADR"),
        entity_schema::<Theory>("engram-theory", "refs/engram/theory/*", "Engram Theory"),
        entity_schema::<StateReflection>(
            "engram-state-reflection",
            "refs/engram/state_reflection/*",
            "Engram State Reflection",
        ),
        entity_schema::<DocFragment>(
            "engram-doc-fragment",
            "refs/engram/doc_fragment/*",
            "Engram DocFragment",
        ),
        entity_schema::<Compliance>(
            "engram-compliance",
            "refs/engram/compliance/*",
            "Engram Compliance",
        ),
        entity_schema::<Workflow>(
            "engram-workflow",
            "refs/engram/workflow/*",
            "Engram Workflow",
        ),
    ]
}

fn write_entity_schema_to_storage<S: Storage>(
    storage: &mut S,
    wrapper: &EntitySchemaWrapper,
) -> crate::Result<()> {
    let entity_type = "schema";
    let entity_id = &wrapper.id;

    let generic = crate::entities::GenericEntity {
        id: entity_id.clone(),
        entity_type: entity_type.to_string(),
        agent: "engram".to_string(),
        timestamp: chrono::Utc::now(),
        data: serde_json::to_value(wrapper).map_err(crate::EngramError::Serialization)?,
    };

    storage.store(&generic)?;
    Ok(())
}

#[derive(Subcommand)]
pub enum SchemaCommands {
    /// Generate JSON Schema for a specific entity type
    Generate {
        /// Entity type (task, context, reasoning, knowledge, session, adr, theory, state_reflection, doc_fragment, compliance, workflow)
        #[arg(long, short)]
        entity: String,

        /// Output file path (prints to stdout if not specified)
        #[arg(long, short)]
        output: Option<String>,
    },

    /// Publish all entity schemas as refs in the engram store
    Publish,

    /// Generate JSON Schema for workflow entity (deprecated, use --entity workflow)
    Workflow {
        /// Output file path (prints to stdout if not specified)
        #[arg(long, short)]
        output: Option<String>,
    },
}

pub fn handle_schema_command<S: Storage>(
    command: SchemaCommands,
    storage: &mut S,
) -> crate::Result<()> {
    match command {
        SchemaCommands::Generate { entity, output } => {
            let all = all_entity_schemas();
            let found = all
                .iter()
                .find(|s| s.id == entity || s.id == format!("engram-{}", entity));

            let schema = match found {
                Some(s) => s,
                None => {
                    let available: Vec<String> = all.iter().map(|s| s.id.clone()).collect();
                    return Err(crate::EngramError::Validation(format!(
                        "Unknown entity type '{}'. Available: {}",
                        entity,
                        available.join(", ")
                    )));
                }
            };

            let schema_json =
                serde_json::to_string_pretty(&schema).map_err(crate::EngramError::Serialization)?;

            if let Some(output_path) = output {
                std::fs::write(&output_path, schema_json).map_err(crate::EngramError::Io)?;
                println!("Schema written to: {}", output_path);
            } else {
                println!("{}", schema_json);
            }

            Ok(())
        }
        SchemaCommands::Publish => {
            let all = all_entity_schemas();
            let mut published = Vec::new();

            for wrapper in &all {
                write_entity_schema_to_storage(storage, wrapper)?;
                published.push(wrapper.id.clone());
            }

            for id in &published {
                println!("Published schema: {}", id);
            }

            println!(
                "\n{} schemas published to refs/engram/schema/*",
                published.len()
            );
            Ok(())
        }
        SchemaCommands::Workflow { output } => {
            let schema = schema_for!(Workflow);
            let schema_json =
                serde_json::to_string_pretty(&schema).map_err(crate::EngramError::Serialization)?;

            if let Some(output_path) = output {
                std::fs::write(&output_path, schema_json).map_err(crate::EngramError::Io)?;
                println!("Schema written to: {}", output_path);
            } else {
                println!("{}", schema_json);
            }

            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    use tempfile::NamedTempFile;

    #[test]
    fn test_all_entity_schemas_count() {
        let schemas = all_entity_schemas();
        assert_eq!(schemas.len(), 11);
    }

    #[test]
    fn test_all_entity_schemas_have_required_fields() {
        let schemas = all_entity_schemas();
        for s in &schemas {
            assert!(!s.id.is_empty(), "schema {} missing id", s.id);
            assert!(
                s.namespace_pattern.starts_with("refs/"),
                "schema {} bad namespace_pattern: {}",
                s.id,
                s.namespace_pattern
            );
            assert!(!s.title.is_empty(), "schema {} missing title", s.id);
            assert!(!s.version.is_empty(), "schema {} missing version", s.id);
            let schema_val = &s.schema;
            assert!(
                schema_val.get("$schema").is_some(),
                "schema {} missing $schema",
                s.id
            );
        }
    }

    #[test]
    fn test_entity_schema_serialization() {
        let schema = entity_schema::<Task>("engram-task", "refs/engram/task/*", "Engram Task");
        let json = serde_json::to_value(&schema).unwrap();
        assert_eq!(json["id"], "engram-task");
        assert_eq!(json["namespace_pattern"], "refs/engram/task/*");
        assert_eq!(json["title"], "Engram Task");
        assert!(json["schema"]["$schema"].is_string());
    }

    #[test]
    fn test_generate_unknown_entity() {
        let cmd = SchemaCommands::Generate {
            entity: "nonexistent".to_string(),
            output: None,
        };
        let result = handle_schema_command(cmd, &mut crate::storage::MemoryStorage::new("test"));
        assert!(result.is_err());
        let err_msg = format!("{}", result.unwrap_err());
        assert!(
            err_msg.contains("nonexistent"),
            "error should mention the bad entity: {}",
            err_msg
        );
    }

    #[test]
    fn test_generate_task_entity_to_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap().to_string();

        let cmd = SchemaCommands::Generate {
            entity: "task".to_string(),
            output: Some(path.clone()),
        };
        let result = handle_schema_command(cmd, &mut crate::storage::MemoryStorage::new("test"));
        assert!(result.is_ok());

        let mut file = std::fs::File::open(&path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        let json: serde_json::Value = serde_json::from_str(&content).expect("Should be valid JSON");
        assert_eq!(json["title"], "Engram Task");
        assert!(json["schema"]["$schema"].is_string());
    }

    #[test]
    fn test_workflow_backward_compat() {
        let cmd = SchemaCommands::Workflow { output: None };
        let result = handle_schema_command(cmd, &mut crate::storage::MemoryStorage::new("test"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_publish_schemas() {
        let cmd = SchemaCommands::Publish;
        let mut storage = crate::storage::MemoryStorage::new("test");
        let result = handle_schema_command(cmd, &mut storage);
        assert!(result.is_ok());

        let all = storage.get_all("schema").unwrap();
        assert_eq!(all.len(), 11, "should have published 11 schemas");
    }
}
