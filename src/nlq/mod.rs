//! Natural Language Query Engine for Engram
//!
//! Provides conversational access to Engram's memory system through pattern matching
//! and entity extraction. Maps natural language queries to structured Engram operations.

pub mod deep_walk;
pub mod entity_extractor;
pub mod intent_classifier;
pub mod query_mapper;
pub mod response_formatter;
pub mod skills_prompts_handler;

use crate::error::EngramError;
use crate::storage::Storage;
use serde::{Deserialize, Serialize};

pub use deep_walk::{ConnectedEntity, DeepWalker, DeepWalkResult};
pub use entity_extractor::EntityExtractor;
pub use intent_classifier::IntentClassifier;
pub use query_mapper::QueryMapper;
pub use response_formatter::ResponseFormatter;
pub use skills_prompts_handler::{
    list_prompts, list_skills, search_prompts, search_skills, PromptInfo, PromptsQuery, SkillInfo,
    SkillsQuery,
};

/// Main Natural Language Query Engine
pub struct NLQEngine {
    intent_classifier: IntentClassifier,
    entity_extractor: EntityExtractor,
    query_mapper: QueryMapper,
    response_formatter: ResponseFormatter,
}

/// Represents a processed natural language query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedQuery {
    pub original_query: String,
    pub intent: QueryIntent,
    pub entities: Vec<ExtractedEntity>,
    pub context: Option<String>,
    pub confidence: f64,
}

/// Supported query intents
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryIntent {
    ListTasks,
    ShowTaskDetails,
    FindRelationships,
    SearchContext,
    AnalyzeWorkflow,
    ListSkills,
    SearchSkills,
    ListPrompts,
    SearchPrompts,
    /// Free-text search across all entity types (tasks, context, reasoning)
    FullTextSearch,
    Unknown,
}

/// Extracted entities from natural language
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedEntity {
    pub entity_type: String,
    pub value: String,
    pub confidence: f64,
    pub position: Option<(usize, usize)>,
}

/// Query execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub success: bool,
    pub data: serde_json::Value,
    pub formatted_response: String,
    pub execution_time_ms: u64,
}

impl NLQEngine {
    /// Create a new NLQ engine instance
    pub fn new() -> Self {
        Self {
            intent_classifier: IntentClassifier::new(),
            entity_extractor: EntityExtractor::new(),
            query_mapper: QueryMapper::new(),
            response_formatter: ResponseFormatter::new(),
        }
    }

    /// Process a natural language query and return results
    pub async fn process_query(
        &self,
        query: &str,
        context: Option<String>,
        storage: &dyn Storage,
    ) -> Result<QueryResult, EngramError> {
        self.process_query_with_deep(query, context, storage, false, None).await
    }

    /// Process a natural language query with optional deep relationship walking
    pub async fn process_query_with_deep(
        &self,
        query: &str,
        context: Option<String>,
        storage: &dyn Storage,
        deep: bool,
        max_depth: Option<usize>,
    ) -> Result<QueryResult, EngramError> {
        let start_time = std::time::Instant::now();

        // Step 1: Classify intent
        let intent = self.intent_classifier.classify(query)?;

        // Step 2: Extract entities
        let entities = self.entity_extractor.extract(query)?;

        // Step 3: Create processed query
        let processed_query = ProcessedQuery {
            original_query: query.to_string(),
            intent: intent.clone(),
            entities,
            context,
            confidence: 0.8,
        };

        // Step 4: Map to storage query and execute
        let data = self
            .query_mapper
            .execute_query(&processed_query, storage)
            .await?;

        // Step 5: Deep walk if requested
        let data = if deep {
            self.perform_deep_walk(&data, storage, max_depth)?
        } else {
            data
        };

        // Step 6: Format response
        let formatted_response = self.response_formatter.format(&processed_query, &data)?;

        let execution_time = start_time.elapsed().as_millis() as u64;

        Ok(QueryResult {
            success: true,
            data,
            formatted_response,
            execution_time_ms: execution_time,
        })
    }

    fn perform_deep_walk(
        &self,
        data: &serde_json::Value,
        storage: &dyn Storage,
        max_depth: Option<usize>,
    ) -> Result<serde_json::Value, EngramError> {
        if let Some(git_refs_storage) = storage.as_any().downcast_ref::<crate::storage::GitRefsStorage>() {
            let entity_ids = DeepWalker::resolve_entity_ids(data);
            if entity_ids.is_empty() {
                return Ok(data.clone());
            }

            let walk_result = DeepWalker::walk_from_entities(git_refs_storage, &entity_ids, max_depth)?;

            let mut enriched = data.clone();
            let connected_json: Vec<serde_json::Value> = walk_result
                .connected_entities
                .iter()
                .map(|ce| {
                    serde_json::json!({
                        "entity_id": ce.entity_id,
                        "entity_type": ce.entity_type,
                        "depth": ce.depth,
                        "relationship_type": ce.relationship_type,
                        "direction": ce.direction,
                    })
                })
                .collect();

            enriched["deep_walk"] = serde_json::json!({
                "enabled": true,
                "seed_count": walk_result.seed_entities.len(),
                "max_depth": walk_result.max_depth,
                "total_connected": walk_result.total_connected,
                "connected_entities": connected_json,
            });

            Ok(enriched)
        } else {
            Ok(data.clone())
        }
    }

    /// Get supported query patterns for help/documentation
    pub fn get_supported_patterns(&self) -> Vec<String> {
        vec![
            "show my tasks".to_string(),
            "list tasks for agent X".to_string(),
            "what tasks depend on task Y?".to_string(),
            "find context for task Z".to_string(),
            "show workflow status".to_string(),
        ]
    }
}

impl Default for NLQEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nlq_engine_creation() {
        let engine = NLQEngine::new();
        let patterns = engine.get_supported_patterns();
        assert!(!patterns.is_empty());
    }

    #[test]
    fn test_query_intent_serialization() {
        let intent = QueryIntent::ListTasks;
        let json = serde_json::to_string(&intent).unwrap();
        let deserialized: QueryIntent = serde_json::from_str(&json).unwrap();
        assert_eq!(intent, deserialized);
    }
}
