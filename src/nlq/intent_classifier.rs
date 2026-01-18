use crate::error::EngramError;
use crate::nlq::QueryIntent;
use regex::Regex;
use std::collections::HashMap;

/// Intent classifier using pattern matching for natural language queries
pub struct IntentClassifier {
    patterns: HashMap<QueryIntent, Vec<Regex>>,
}

impl IntentClassifier {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();

        // List tasks patterns
        patterns.insert(
            QueryIntent::ListTasks,
            vec![
                Regex::new(r"(?i)^(show|list|get)\s+(my\s+)?tasks?").unwrap(),
                Regex::new(r"(?i)^what\s+tasks?\s+(do\s+i\s+have|am\s+i\s+working\s+on)").unwrap(),
                Regex::new(r"(?i)^tasks?\s+for\s+").unwrap(),
                // Status-based patterns
                Regex::new(r"(?i)^(show|list|get)\s+(done|completed|finished)\s+tasks?").unwrap(),
                Regex::new(r"(?i)^(show|list|get)\s+(todo|pending|open)\s+tasks?").unwrap(),
                Regex::new(r"(?i)^(show|list|get)\s+(in\s*progress|inprogress|current)\s+tasks?")
                    .unwrap(),
                // Priority-based patterns
                Regex::new(r"(?i)^(show|list|get)\s+(high|medium|low)\s+priority\s+tasks?")
                    .unwrap(),
                Regex::new(r"(?i)^(show|list|get)\s+(urgent|critical)\s+tasks?").unwrap(),
            ],
        );

        // Show task details patterns
        patterns.insert(
            QueryIntent::ShowTaskDetails,
            vec![
                Regex::new(r"(?i)^(show|get|details?\s+of)\s+task\s+").unwrap(),
                Regex::new(r"(?i)^what\s+(is|about)\s+task\s+").unwrap(),
            ],
        );

        // Find relationships patterns
        patterns.insert(
            QueryIntent::FindRelationships,
            vec![
                Regex::new(r"(?i)what\s+tasks?\s+(depend\s+on|are\s+related\s+to)").unwrap(),
                Regex::new(r"(?i)(dependencies|dependents)\s+(of|for)\s+").unwrap(),
                Regex::new(r"(?i)^(show|find|get)\s+(relationship|dependencies|dependents)")
                    .unwrap(),
            ],
        );

        // Search context patterns
        patterns.insert(
            QueryIntent::SearchContext,
            vec![
                Regex::new(r"(?i)^(find|search|get)\s+(context|background)").unwrap(),
                Regex::new(r"(?i)what\s+(context|information|background)").unwrap(),
            ],
        );

        // Analyze workflow patterns
        patterns.insert(
            QueryIntent::AnalyzeWorkflow,
            vec![
                Regex::new(r"(?i)^(show|get)\s+(workflow|status)").unwrap(),
                Regex::new(r"(?i)workflow\s+(status|state)").unwrap(),
            ],
        );

        Self { patterns }
    }

    /// Classify the intent of a natural language query
    pub fn classify(&self, query: &str) -> Result<QueryIntent, EngramError> {
        let trimmed_query = query.trim();

        // Check patterns in order of specificity (most specific first)
        let intent_order = vec![
            QueryIntent::ShowTaskDetails,
            QueryIntent::FindRelationships,
            QueryIntent::SearchContext,
            QueryIntent::AnalyzeWorkflow,
            QueryIntent::ListTasks, // More general, check last
        ];

        for intent in intent_order {
            if let Some(regexes) = self.patterns.get(&intent) {
                for regex in regexes {
                    if regex.is_match(trimmed_query) {
                        return Ok(intent.clone());
                    }
                }
            }
        }

        Ok(QueryIntent::Unknown)
    }

    /// Get confidence score for a classification (0.0 to 1.0)
    pub fn get_confidence(&self, query: &str, intent: &QueryIntent) -> f64 {
        let trimmed_query = query.trim();

        if let Some(regexes) = self.patterns.get(intent) {
            for regex in regexes {
                if regex.is_match(trimmed_query) {
                    // Simple confidence based on pattern specificity
                    return if regex.as_str().len() > 20 { 0.9 } else { 0.7 };
                }
            }
        }

        0.0
    }
}

impl Default for IntentClassifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_tasks_classification() {
        let classifier = IntentClassifier::new();

        assert_eq!(
            classifier.classify("show my tasks").unwrap(),
            QueryIntent::ListTasks
        );
        assert_eq!(
            classifier.classify("list tasks").unwrap(),
            QueryIntent::ListTasks
        );
        assert_eq!(
            classifier.classify("what tasks do i have").unwrap(),
            QueryIntent::ListTasks
        );
    }

    #[test]
    fn test_relationship_classification() {
        let classifier = IntentClassifier::new();

        assert_eq!(
            classifier
                .classify("what tasks depend on task-123")
                .unwrap(),
            QueryIntent::FindRelationships
        );
        assert_eq!(
            classifier.classify("show dependencies").unwrap(),
            QueryIntent::FindRelationships
        );
    }

    #[test]
    fn test_unknown_classification() {
        let classifier = IntentClassifier::new();

        assert_eq!(
            classifier.classify("random unrelated question").unwrap(),
            QueryIntent::Unknown
        );
    }

    #[test]
    fn test_confidence_scoring() {
        let classifier = IntentClassifier::new();

        let confidence = classifier.get_confidence("show my tasks", &QueryIntent::ListTasks);
        assert!(confidence > 0.0);

        let no_confidence = classifier.get_confidence("show my tasks", &QueryIntent::SearchContext);
        assert_eq!(no_confidence, 0.0);
    }
}
