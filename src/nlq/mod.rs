//! Natural Language Query Engine for Engram
//!
//! Provides conversational access to Engram's memory system through pattern matching
//! and entity extraction. Maps natural language queries to structured Engram operations.

pub mod entity_extractor;
pub mod intent_classifier;
pub mod query_mapper;
pub mod response_formatter;

use crate::error::EngramError;
use crate::storage::Storage;
use serde::{Deserialize, Serialize};

pub use entity_extractor::EntityExtractor;
pub use intent_classifier::IntentClassifier;
pub use query_mapper::QueryMapper;
pub use response_formatter::ResponseFormatter;

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
            confidence: 0.8, // TODO: Calculate actual confidence
        };

        // Step 4: Map to storage query and execute
        let data = self
            .query_mapper
            .execute_query(&processed_query, storage)
            .await?;

        // Step 5: Format response
        let formatted_response = self.response_formatter.format(&processed_query, &data)?;

        let execution_time = start_time.elapsed().as_millis() as u64;

        Ok(QueryResult {
            success: true,
            data,
            formatted_response,
            execution_time_ms: execution_time,
        })
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
