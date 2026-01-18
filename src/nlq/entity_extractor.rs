use crate::error::EngramError;
use crate::nlq::ExtractedEntity;
use regex::Regex;
use std::collections::HashMap;

pub struct EntityExtractor {
    extractors: HashMap<String, Vec<Regex>>,
}

impl EntityExtractor {
    pub fn new() -> Self {
        let mut extractors = HashMap::new();

        extractors.insert(
            "agent".to_string(),
            vec![
                Regex::new(r"(?i)agent\s+([a-zA-Z0-9_-]+)").unwrap(),
                Regex::new(r"(?i)for\s+([a-zA-Z0-9_-]+)").unwrap(),
            ],
        );

        extractors.insert(
            "task_id".to_string(),
            vec![
                Regex::new(r"task\s+([a-f0-9-]{36})").unwrap(),
                Regex::new(r"([a-f0-9-]{36})").unwrap(),
            ],
        );

        extractors.insert(
            "status".to_string(),
            vec![
                Regex::new(r"(?i)status\s+(todo|done|in_progress|blocked|cancelled)").unwrap(),
                Regex::new(r"(?i)(todo|done|in_progress|blocked|cancelled)\s+tasks?").unwrap(),
                Regex::new(r"(?i)(completed|finished)\s+tasks?").unwrap(),
                Regex::new(r"(?i)(pending|open)\s+tasks?").unwrap(),
                Regex::new(r"(?i)(current|inprogress|in\s*progress)\s+tasks?").unwrap(),
            ],
        );

        extractors.insert(
            "priority".to_string(),
            vec![
                Regex::new(r"(?i)priority\s+(high|medium|low|critical|urgent)").unwrap(),
                Regex::new(r"(?i)(high|medium|low|critical|urgent)\s+priority\s+tasks?").unwrap(),
                Regex::new(r"(?i)(high|medium|low|critical|urgent)\s+tasks?").unwrap(),
            ],
        );

        extractors.insert(
            "time_period".to_string(),
            vec![
                Regex::new(r"(?i)(today|yesterday|this\s+week|last\s+week)").unwrap(),
                Regex::new(r"(?i)in\s+the\s+last\s+(\d+)\s+(days?|weeks?|months?)").unwrap(),
            ],
        );

        Self { extractors }
    }

    pub fn extract(&self, query: &str) -> Result<Vec<ExtractedEntity>, EngramError> {
        let mut entities = Vec::new();

        for (entity_type, regexes) in &self.extractors {
            for regex in regexes {
                for captures in regex.captures_iter(query) {
                    if let Some(matched) = captures.get(1) {
                        entities.push(ExtractedEntity {
                            entity_type: entity_type.clone(),
                            value: matched.as_str().to_string(),
                            confidence: 0.8,
                            position: Some((matched.start(), matched.end())),
                        });
                    }
                }
            }
        }

        Ok(entities)
    }

    pub fn extract_specific(
        &self,
        query: &str,
        entity_type: &str,
    ) -> Result<Vec<ExtractedEntity>, EngramError> {
        let mut entities = Vec::new();

        if let Some(regexes) = self.extractors.get(entity_type) {
            for regex in regexes {
                for captures in regex.captures_iter(query) {
                    if let Some(matched) = captures.get(1) {
                        entities.push(ExtractedEntity {
                            entity_type: entity_type.to_string(),
                            value: matched.as_str().to_string(),
                            confidence: 0.8,
                            position: Some((matched.start(), matched.end())),
                        });
                    }
                }
            }
        }

        Ok(entities)
    }
}

impl Default for EntityExtractor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_extraction() {
        let extractor = EntityExtractor::new();
        let entities = extractor.extract("show tasks for alice").unwrap();

        assert_eq!(entities.len(), 1);
        assert_eq!(entities[0].entity_type, "agent");
        assert_eq!(entities[0].value, "alice");
    }

    #[test]
    fn test_status_extraction() {
        let extractor = EntityExtractor::new();
        let entities = extractor.extract("show done tasks").unwrap();

        assert!(!entities.is_empty());
        let status_entity = entities.iter().find(|e| e.entity_type == "status").unwrap();
        assert_eq!(status_entity.value, "done");
    }

    #[test]
    fn test_task_id_extraction() {
        let extractor = EntityExtractor::new();
        let task_id = "550e8400-e29b-41d4-a716-446655440000";
        let query = format!("show task {}", task_id);
        let entities = extractor.extract(&query).unwrap();

        let task_entity = entities
            .iter()
            .find(|e| e.entity_type == "task_id")
            .unwrap();
        assert_eq!(task_entity.value, task_id);
    }

    #[test]
    fn test_specific_extraction() {
        let extractor = EntityExtractor::new();
        let entities = extractor
            .extract_specific("show tasks for bob", "agent")
            .unwrap();

        assert_eq!(entities.len(), 1);
        assert_eq!(entities[0].entity_type, "agent");
        assert_eq!(entities[0].value, "bob");
    }
}
