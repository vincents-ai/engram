use crate::entities::Entity;
use crate::error::EngramError;
use crate::nlq::{ExtractedEntity, ProcessedQuery, QueryIntent, list_skills, list_prompts, search_skills, search_prompts, SkillsQuery, PromptsQuery};
use crate::storage::{GitStorage, RelationshipStorage, Storage};
use serde_json::{json, Value};

pub struct QueryMapper;

impl QueryMapper {
    pub fn new() -> Self {
        Self
    }

    pub async fn execute_query(
        &self,
        processed_query: &ProcessedQuery,
        storage: &dyn Storage,
    ) -> Result<Value, EngramError> {
        match &processed_query.intent {
            QueryIntent::ListTasks => self.handle_list_tasks(processed_query, storage).await,
            QueryIntent::ShowTaskDetails => {
                self.handle_task_details(processed_query, storage).await
            }
            QueryIntent::FindRelationships => {
                self.handle_relationships(processed_query, storage).await
            }
            QueryIntent::SearchContext => {
                self.handle_search_context(processed_query, storage).await
            }
            QueryIntent::AnalyzeWorkflow => {
                self.handle_workflow_analysis(processed_query, storage)
                    .await
            }
            QueryIntent::ListSkills => self.handle_list_skills(processed_query).await,
            QueryIntent::SearchSkills => self.handle_search_skills(processed_query).await,
            QueryIntent::ListPrompts => self.handle_list_prompts(processed_query).await,
            QueryIntent::SearchPrompts => self.handle_search_prompts(processed_query).await,
            QueryIntent::Unknown => Ok(json!({
                "error": "Unable to understand the query",
                "suggestion": "Try queries like 'show my tasks' or 'what skills are available for planning?'"
            })),
        }
    }

    // Skills/Prompts handlers
    async fn handle_list_skills(&self, processed_query: &ProcessedQuery) -> Result<Value, EngramError> {
        let skills = list_skills(&SkillsQuery {
            category: None,
            search_term: None,
            format: "full".to_string(),
        })?;

        let skill_list: Vec<Value> = skills.iter()
            .map(|s| json!({
                "name": s.name,
                "description": s.description,
                "category": s.category,
            }))
            .collect();

        Ok(json!({
            "success": true,
            "type": "skills_list",
            "count": skills.len(),
            "skills": skill_list,
        }))
    }

    async fn handle_search_skills(&self, processed_query: &ProcessedQuery) -> Result<Value, EngramError> {
        let query = &processed_query.original_query;
        let skills = search_skills(query)?;

        let skill_list: Vec<Value> = skills.iter()
            .map(|s| json!({
                "name": s.name,
                "description": s.description,
                "category": s.category,
            }))
            .collect();

        Ok(json!({
            "success": true,
            "type": "skills_search",
            "query": query,
            "count": skills.len(),
            "skills": skill_list,
        }))
    }

    async fn handle_list_prompts(&self, processed_query: &ProcessedQuery) -> Result<Value, EngramError> {
        let prompts = list_prompts(&PromptsQuery {
            category: None,
            search_term: None,
            format: "full".to_string(),
        })?;

        let prompt_list: Vec<Value> = prompts.iter()
            .map(|p| json!({
                "name": p.name,
                "title": p.title,
                "description": p.description,
                "category": p.category,
            }))
            .collect();

        Ok(json!({
            "success": true,
            "type": "prompts_list",
            "count": prompts.len(),
            "prompts": prompt_list,
        }))
    }

    async fn handle_search_prompts(&self, processed_query: &ProcessedQuery) -> Result<Value, EngramError> {
        let query = &processed_query.original_query;
        let prompts = search_prompts(query)?;

        let prompt_list: Vec<Value> = prompts.iter()
            .map(|p| json!({
                "name": p.name,
                "title": p.title,
                "description": p.description,
                "category": p.category,
            }))
            .collect();

        Ok(json!({
            "success": true,
            "type": "prompts_search",
            "query": query,
            "count": prompts.len(),
            "prompts": prompt_list,
        }))
    }

    async fn handle_list_tasks(
        &self,
        processed_query: &ProcessedQuery,
        storage: &dyn Storage,
    ) -> Result<Value, EngramError> {
        let agent = self.extract_agent_or_default(&processed_query.entities);
        let status = self.extract_status(&processed_query.entities);
        let priority = self.extract_priority(&processed_query.entities);
        let title_search = self.extract_title_search(&processed_query.original_query);

        let tasks = storage.query_by_agent(&agent, Some("task"))?;

        let mut filtered_tasks = Vec::new();
        for task_entity in tasks {
            if let Ok(task) = crate::entities::Task::from_generic(task_entity) {
                let mut include_task = true;

                // Filter by status if specified
                if let Some(status_filter) = &status {
                    let task_status = format!("{:?}", task.status).to_lowercase();
                    let status_lower = status_filter.to_lowercase();
                    let normalized_filter = match status_lower.as_str() {
                        "completed" | "finished" => "done",
                        "pending" | "open" => "todo",
                        "current" | "inprogress" | "in progress" => "inprogress",
                        other => other,
                    };
                    include_task = task_status == normalized_filter;
                }

                // Filter by priority if specified
                if include_task {
                    if let Some(priority_filter) = &priority {
                        let task_priority = format!("{:?}", task.priority).to_lowercase();
                        let priority_lower = priority_filter.to_lowercase();
                        let normalized_priority = match priority_lower.as_str() {
                            "critical" | "urgent" => "high",
                            other => other,
                        };
                        include_task = task_priority == normalized_priority;
                    }
                }

                // Filter by title search if specified
                if include_task && !title_search.is_empty() {
                    let title_lower = task.title.to_lowercase();
                    let search_lower = title_search.to_lowercase();
                    include_task = title_lower.contains(&search_lower);
                }

                if include_task {
                    filtered_tasks.push(json!({
                        "id": task.id,
                        "title": task.title,
                        "status": format!("{:?}", task.status),
                        "priority": format!("{:?}", task.priority),
                    }));
                }
            }
        }

        Ok(json!({
            "tasks": filtered_tasks,
            "count": filtered_tasks.len(),
            "agent": agent,
            "status_filter": status,
            "priority_filter": priority,
            "title_search": if title_search.is_empty() { None } else { Some(title_search) }
        }))
    }

    async fn handle_task_details(
        &self,
        processed_query: &ProcessedQuery,
        storage: &dyn Storage,
    ) -> Result<Value, EngramError> {
        if let Some(task_id) = self.extract_task_id(&processed_query.entities) {
            if let Some(task_entity) = storage.get(&task_id, "task")? {
                if let Ok(task) = crate::entities::Task::from_generic(task_entity) {
                    return Ok(json!({
                        "task": {
                            "id": task.id,
                            "title": task.title,
                            "description": task.description,
                            "status": format!("{:?}", task.status),
                            "priority": format!("{:?}", task.priority),
                            "agent": task.agent,
                            "created": task.start_time,
                            "outcome": task.outcome
                        }
                    }));
                }
            }
        }

        Ok(json!({
            "error": "Task not found or no task ID provided"
        }))
    }

    async fn handle_relationships(
        &self,
        processed_query: &ProcessedQuery,
        storage: &dyn Storage,
    ) -> Result<Value, EngramError> {
        if let Some(task_id) = self.extract_task_id(&processed_query.entities) {
            if let Some(git_storage) = storage.as_any().downcast_ref::<GitStorage>() {
                let relationships = git_storage.get_entity_relationships(&task_id)?;

                let mut related_entities = Vec::new();
                for rel in relationships {
                    related_entities.push(json!({
                        "type": format!("{:?}", rel.relationship_type),
                        "source": rel.source_id,
                        "target": rel.target_id,
                        "strength": format!("{:?}", rel.strength)
                    }));
                }

                return Ok(json!({
                    "task_id": task_id,
                    "relationships": related_entities,
                    "count": related_entities.len()
                }));
            } else {
                let mut related_entities = Vec::new();
                related_entities.push(json!({
                    "type": "placeholder",
                    "source": task_id.clone(),
                    "target": "storage_type_not_supported",
                    "strength": "Medium"
                }));

                return Ok(json!({
                    "task_id": task_id,
                    "relationships": related_entities,
                    "count": related_entities.len()
                }));
            }
        }

        Ok(json!({
            "error": "No task ID found in query"
        }))
    }

    async fn handle_search_context(
        &self,
        processed_query: &ProcessedQuery,
        storage: &dyn Storage,
    ) -> Result<Value, EngramError> {
        let agent = self.extract_agent_or_default(&processed_query.entities);
        let search_term =
            self.extract_search_term(&processed_query.entities, &processed_query.original_query);
        let contexts = storage.query_by_agent(&agent, Some("context"))?;
        let mut context_list = Vec::new();

        for context_entity in contexts.into_iter().take(10) {
            if let Ok(context) = crate::entities::Context::from_generic(context_entity) {
                // Filter contexts based on search term
                if search_term.is_empty()
                    || context
                        .title
                        .to_lowercase()
                        .contains(&search_term.to_lowercase())
                    || context
                        .content
                        .to_lowercase()
                        .contains(&search_term.to_lowercase())
                {
                    context_list.push(json!({
                        "id": context.id,
                        "title": context.title,
                        "relevance": format!("{:?}", context.relevance),
                    }));
                }
            }
        }

        Ok(json!({
            "contexts": context_list,
            "count": context_list.len(),
            "agent": agent,
            "search_term": search_term
        }))
    }

    async fn handle_workflow_analysis(
        &self,
        processed_query: &ProcessedQuery,
        storage: &dyn Storage,
    ) -> Result<Value, EngramError> {
        let agent = self.extract_agent_or_default(&processed_query.entities);
        let workflows = storage.query_by_agent(&agent, Some("workflow"))?;
        let mut workflow_status = Vec::new();

        for workflow_entity in workflows {
            if let Ok(workflow) = crate::entities::Workflow::from_generic(workflow_entity) {
                workflow_status.push(json!({
                    "id": workflow.id,
                    "title": workflow.title,
                    "current_state": workflow.initial_state.clone(),
                    "status": format!("{:?}", workflow.status)
                }));
            }
        }

        Ok(json!({
            "workflows": workflow_status,
            "count": workflow_status.len()
        }))
    }

    fn extract_agent_or_default(&self, entities: &[ExtractedEntity]) -> String {
        entities
            .iter()
            .find(|e| e.entity_type == "agent")
            .map(|e| e.value.clone())
            .unwrap_or_else(|| "default".to_string())
    }

    fn extract_status(&self, entities: &[ExtractedEntity]) -> Option<String> {
        entities
            .iter()
            .find(|e| e.entity_type == "status")
            .map(|e| e.value.clone())
    }

    fn extract_priority(&self, entities: &[ExtractedEntity]) -> Option<String> {
        entities
            .iter()
            .find(|e| e.entity_type == "priority")
            .map(|e| e.value.clone())
    }

    fn extract_task_id(&self, entities: &[ExtractedEntity]) -> Option<String> {
        entities
            .iter()
            .find(|e| e.entity_type == "task_id")
            .map(|e| e.value.clone())
    }

    fn extract_search_term(&self, entities: &[ExtractedEntity], query: &str) -> String {
        let lower_query = query.to_lowercase();

        if let Some(start) = lower_query.find("about ") {
            let start_idx = start + 6;
            let remaining = &query[start_idx..];

            if let Some(for_idx) = remaining.find(" for ") {
                remaining[..for_idx].trim().to_string()
            } else {
                remaining.trim().to_string()
            }
        } else if let Some(context_entity) = entities.iter().find(|e| e.entity_type == "context") {
            context_entity.value.clone()
        } else {
            String::new()
        }
    }

    fn extract_title_search(&self, query: &str) -> String {
        let lower_query = query.to_lowercase();

        let search_keywords = ["about", "with", "containing", "titled", "called"];
        for keyword in &search_keywords {
            if let Some(start) = lower_query.find(&format!("{} ", keyword)) {
                let start_idx = start + keyword.len() + 1;
                let remaining = &query[start_idx..];

                if let Some(for_idx) = remaining.find(" for ") {
                    return remaining[..for_idx].trim().to_string();
                } else {
                    return remaining.trim().to_string();
                }
            }
        }

        String::new()
    }
}

impl Default for QueryMapper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nlq::QueryIntent;

    #[test]
    fn test_agent_extraction() {
        let mapper = QueryMapper::new();
        let entities = vec![ExtractedEntity {
            entity_type: "agent".to_string(),
            value: "alice".to_string(),
            confidence: 0.8,
            position: None,
        }];

        let agent = mapper.extract_agent_or_default(&entities);
        assert_eq!(agent, "alice");
    }

    #[test]
    fn test_default_agent_extraction() {
        let mapper = QueryMapper::new();
        let entities = vec![];

        let agent = mapper.extract_agent_or_default(&entities);
        assert_eq!(agent, "default");
    }

    #[test]
    fn test_task_id_extraction() {
        let mapper = QueryMapper::new();
        let task_id = "550e8400-e29b-41d4-a716-446655440000";
        let entities = vec![ExtractedEntity {
            entity_type: "task_id".to_string(),
            value: task_id.to_string(),
            confidence: 0.8,
            position: None,
        }];

        let extracted = mapper.extract_task_id(&entities);
        assert_eq!(extracted, Some(task_id.to_string()));
    }
}
