use crate::entities::Entity;
use crate::error::EngramError;
use crate::nlq::{
    list_prompts, list_skills, search_prompts, search_skills, ExtractedEntity, ProcessedQuery,
    PromptsQuery, QueryIntent, SkillsQuery,
};
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
            QueryIntent::FullTextSearch => {
                self.handle_full_text_search(processed_query, storage).await
            }
            QueryIntent::Unknown => Ok(json!({
                "error": "Unable to understand the query",
                "suggestion": "Try queries like 'show my tasks' or 'what skills are available for planning?'"
            })),
        }
    }

    // Skills/Prompts handlers
    async fn handle_list_skills(
        &self,
        _processed_query: &ProcessedQuery,
    ) -> Result<Value, EngramError> {
        let skills = list_skills(&SkillsQuery {
            category: None,
            search_term: None,
            format: "full".to_string(),
        })?;

        let skill_list: Vec<Value> = skills
            .iter()
            .map(|s| {
                json!({
                    "name": s.name,
                    "description": s.description,
                    "category": s.category,
                })
            })
            .collect();

        Ok(json!({
            "success": true,
            "type": "skills_list",
            "count": skills.len(),
            "skills": skill_list,
        }))
    }

    async fn handle_search_skills(
        &self,
        processed_query: &ProcessedQuery,
    ) -> Result<Value, EngramError> {
        let query = &processed_query.original_query;
        let skills = search_skills(query)?;

        let skill_list: Vec<Value> = skills
            .iter()
            .map(|s| {
                json!({
                    "name": s.name,
                    "description": s.description,
                    "category": s.category,
                })
            })
            .collect();

        Ok(json!({
            "success": true,
            "type": "skills_search",
            "query": query,
            "count": skills.len(),
            "skills": skill_list,
        }))
    }

    async fn handle_list_prompts(
        &self,
        _processed_query: &ProcessedQuery,
    ) -> Result<Value, EngramError> {
        let prompts = list_prompts(&PromptsQuery {
            category: None,
            search_term: None,
            format: "full".to_string(),
        })?;

        let prompt_list: Vec<Value> = prompts
            .iter()
            .map(|p| {
                json!({
                    "name": p.name,
                    "title": p.title,
                    "description": p.description,
                    "category": p.category,
                })
            })
            .collect();

        Ok(json!({
            "success": true,
            "type": "prompts_list",
            "count": prompts.len(),
            "prompts": prompt_list,
        }))
    }

    async fn handle_search_prompts(
        &self,
        processed_query: &ProcessedQuery,
    ) -> Result<Value, EngramError> {
        let query = &processed_query.original_query;
        let prompts = search_prompts(query)?;

        let prompt_list: Vec<Value> = prompts
            .iter()
            .map(|p| {
                json!({
                    "name": p.name,
                    "title": p.title,
                    "description": p.description,
                    "category": p.category,
                })
            })
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

    /// Full-text search across tasks, context, and reasoning for any free-form query.
    async fn handle_full_text_search(
        &self,
        processed_query: &ProcessedQuery,
        storage: &dyn Storage,
    ) -> Result<Value, EngramError> {
        let query = processed_query.original_query.trim().to_lowercase();
        let agent = self.extract_agent_or_default(&processed_query.entities);

        // Search tasks by title
        let all_tasks = storage.query_by_agent(&agent, Some("task"))?;
        let mut matching_tasks = Vec::new();
        for entity in all_tasks {
            if let Ok(task) = crate::entities::Task::from_generic(entity) {
                if task.title.to_lowercase().contains(&query)
                    || task.description.to_lowercase().contains(&query)
                {
                    matching_tasks.push(json!({
                        "id": task.id,
                        "title": task.title,
                        "status": format!("{:?}", task.status),
                        "priority": format!("{:?}", task.priority),
                    }));
                }
            }
        }

        // Search context by title + content
        let all_contexts = storage.query_by_agent(&agent, Some("context"))?;
        let mut matching_contexts = Vec::new();
        for entity in all_contexts.into_iter().take(50) {
            if let Ok(ctx) = crate::entities::Context::from_generic(entity) {
                if ctx.title.to_lowercase().contains(&query)
                    || ctx.content.to_lowercase().contains(&query)
                {
                    matching_contexts.push(json!({
                        "id": ctx.id,
                        "title": ctx.title,
                    }));
                }
            }
        }

        // Search reasoning by title + conclusion + step descriptions
        let all_reasoning = storage.query_by_agent(&agent, Some("reasoning"))?;
        let mut matching_reasoning = Vec::new();
        for entity in all_reasoning.into_iter().take(50) {
            if let Ok(rsn) = crate::entities::Reasoning::from_generic(entity) {
                let matches = rsn.title.to_lowercase().contains(&query)
                    || rsn.conclusion.to_lowercase().contains(&query)
                    || rsn.steps.iter().any(|s| {
                        s.description.to_lowercase().contains(&query)
                            || s.conclusion.to_lowercase().contains(&query)
                    });
                if matches {
                    matching_reasoning.push(json!({
                        "id": rsn.id,
                        "title": rsn.title,
                    }));
                }
            }
        }

        // Search knowledge by title + content
        let all_knowledge = storage.query_by_agent(&agent, Some("knowledge"))?;
        let mut matching_knowledge = Vec::new();
        for entity in all_knowledge.into_iter().take(50) {
            if let Ok(k) = crate::entities::Knowledge::from_generic(entity) {
                if k.title.to_lowercase().contains(&query)
                    || k.content.to_lowercase().contains(&query)
                {
                    matching_knowledge.push(json!({
                        "id": k.id,
                        "title": k.title,
                    }));
                }
            }
        }

        // Search rules by title + description
        let all_rules = storage.query_by_agent(&agent, Some("rule"))?;
        let mut matching_rules = Vec::new();
        for entity in all_rules.into_iter().take(50) {
            if let Ok(r) = crate::entities::Rule::from_generic(entity) {
                if r.title.to_lowercase().contains(&query)
                    || r.description.to_lowercase().contains(&query)
                {
                    matching_rules.push(json!({
                        "id": r.id,
                        "title": r.title,
                    }));
                }
            }
        }

        // Search standards by title + description
        let all_standards = storage.query_by_agent(&agent, Some("standard"))?;
        let mut matching_standards = Vec::new();
        for entity in all_standards.into_iter().take(50) {
            if let Ok(s) = crate::entities::Standard::from_generic(entity) {
                if s.title.to_lowercase().contains(&query)
                    || s.description.to_lowercase().contains(&query)
                {
                    matching_standards.push(json!({
                        "id": s.id,
                        "title": s.title,
                    }));
                }
            }
        }

        // Search ADRs by title + context + decision
        let all_adrs = storage.query_by_agent(&agent, Some("adr"))?;
        let mut matching_adrs = Vec::new();
        for entity in all_adrs.into_iter().take(50) {
            if let Ok(adr) = crate::entities::ADR::from_generic(entity) {
                if adr.title.to_lowercase().contains(&query)
                    || adr.context.to_lowercase().contains(&query)
                    || adr.decision.to_lowercase().contains(&query)
                {
                    matching_adrs.push(json!({
                        "id": adr.id,
                        "title": adr.title,
                        "number": adr.number,
                    }));
                }
            }
        }

        // Search theories by domain_name + concepts + rationale
        let all_theories = storage.query_by_agent(&agent, Some("theory"))?;
        let mut matching_theories = Vec::new();
        for entity in all_theories.into_iter().take(50) {
            if let Ok(t) = crate::entities::Theory::from_generic(entity) {
                let matches = t.domain_name.to_lowercase().contains(&query)
                    || t.conceptual_model.iter().any(|(k, v)| {
                        k.to_lowercase().contains(&query) || v.to_lowercase().contains(&query)
                    })
                    || t.design_rationale.iter().any(|(k, v)| {
                        k.to_lowercase().contains(&query) || v.to_lowercase().contains(&query)
                    })
                    || t.invariants.iter().any(|i| i.to_lowercase().contains(&query));
                if matches {
                    matching_theories.push(json!({
                        "id": t.id,
                        "title": t.domain_name,
                    }));
                }
            }
        }

        // Search compliance by title + description
        let all_compliance = storage.query_by_agent(&agent, Some("compliance"))?;
        let mut matching_compliance = Vec::new();
        for entity in all_compliance.into_iter().take(50) {
            if let Ok(c) = crate::entities::Compliance::from_generic(entity) {
                if c.title.to_lowercase().contains(&query)
                    || c.description.to_lowercase().contains(&query)
                {
                    matching_compliance.push(json!({
                        "id": c.id,
                        "title": c.title,
                    }));
                }
            }
        }

        // Search sessions by title + goals + outcomes
        let all_sessions = storage.query_by_agent(&agent, Some("session"))?;
        let mut matching_sessions = Vec::new();
        for entity in all_sessions.into_iter().take(50) {
            if let Ok(s) = crate::entities::Session::from_generic(entity) {
                let matches = s.title.to_lowercase().contains(&query)
                    || s.goals.iter().any(|g| g.to_lowercase().contains(&query))
                    || s.outcomes.iter().any(|o| o.to_lowercase().contains(&query));
                if matches {
                    matching_sessions.push(json!({
                        "id": s.id,
                        "title": s.title,
                    }));
                }
            }
        }

        // Search state reflections by observed_state + cognitive_dissonance + proposed_theory_updates
        let all_reflections = storage.query_by_agent(&agent, Some("state_reflection"))?;
        let mut matching_reflections = Vec::new();
        for entity in all_reflections.into_iter().take(50) {
            if let Ok(r) = crate::entities::StateReflection::from_generic(entity) {
                let matches = r.observed_state.to_lowercase().contains(&query)
                    || r.cognitive_dissonance
                        .iter()
                        .any(|d| d.to_lowercase().contains(&query))
                    || r.proposed_theory_updates
                        .iter()
                        .any(|u| u.to_lowercase().contains(&query));
                if matches {
                    matching_reflections.push(json!({
                        "id": r.id,
                        "title": format!("Reflection on theory {}", &r.theory_id[..8.min(r.theory_id.len())]),
                    }));
                }
            }
        }

        // Search workflows by title + description
        let all_workflows = storage.query_by_agent(&agent, Some("workflow"))?;
        let mut matching_workflows = Vec::new();
        for entity in all_workflows.into_iter().take(50) {
            if let Ok(w) = crate::entities::Workflow::from_generic(entity) {
                if w.title.to_lowercase().contains(&query)
                    || w.description.to_lowercase().contains(&query)
                {
                    matching_workflows.push(json!({
                        "id": w.id,
                        "title": w.title,
                    }));
                }
            }
        }

        // Search workflow instances by current_state + workflow_id
        let all_instances = storage.query_by_agent(&agent, Some("workflow_instance"))?;
        let mut matching_instances = Vec::new();
        for entity in all_instances.into_iter().take(50) {
            if let Ok(wi) = crate::entities::WorkflowInstance::from_generic(entity) {
                if wi.current_state.to_lowercase().contains(&query)
                    || wi.workflow_id.to_lowercase().contains(&query)
                {
                    matching_instances.push(json!({
                        "id": wi.id,
                        "title": format!("Instance of workflow {} ({})", &wi.workflow_id[..8.min(wi.workflow_id.len())], wi.current_state),
                    }));
                }
            }
        }

        // Search agent sandboxes by agent_id
        let all_sandboxes = storage.query_by_agent(&agent, Some("agent_sandbox"))?;
        let mut matching_sandboxes = Vec::new();
        for entity in all_sandboxes.into_iter().take(50) {
            if let Ok(sb) = crate::entities::AgentSandbox::from_generic(entity) {
                if sb.agent_id.to_lowercase().contains(&query) {
                    matching_sandboxes.push(json!({
                        "id": sb.id,
                        "title": format!("Sandbox for agent {}", sb.agent_id),
                    }));
                }
            }
        }

        // Search escalation requests by justification + impact_if_denied + operation
        let all_escalations = storage.query_by_agent(&agent, Some("escalation_request"))?;
        let mut matching_escalations = Vec::new();
        for entity in all_escalations.into_iter().take(50) {
            if let Ok(er) = crate::entities::EscalationRequest::from_generic(entity) {
                let matches = er.justification.to_lowercase().contains(&query)
                    || er
                        .impact_if_denied
                        .as_deref()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&query)
                    || er
                        .operation_context
                        .operation
                        .to_lowercase()
                        .contains(&query);
                if matches {
                    matching_escalations.push(json!({
                        "id": er.id,
                        "title": format!("Escalation: {}", er.operation_context.operation),
                    }));
                }
            }
        }

        // Search execution results by command + stdout + stderr + workflow_stage
        let all_results = storage.query_by_agent(&agent, Some("execution_result"))?;
        let mut matching_results = Vec::new();
        for entity in all_results.into_iter().take(50) {
            if let Ok(er) = crate::entities::ExecutionResult::from_generic(entity) {
                let matches = er.command.to_lowercase().contains(&query)
                    || er.stdout.to_lowercase().contains(&query)
                    || er.stderr.to_lowercase().contains(&query)
                    || er.workflow_stage.to_lowercase().contains(&query);
                if matches {
                    matching_results.push(json!({
                        "id": er.id,
                        "title": format!("Execution: {}", er.command),
                    }));
                }
            }
        }

        // Search progressive gate configs by name + description
        let all_gate_configs = storage.query_by_agent(&agent, Some("progressive_gate_config"))?;
        let mut matching_gate_configs = Vec::new();
        for entity in all_gate_configs.into_iter().take(50) {
            if let Ok(pgc) = crate::entities::ProgressiveGateConfig::from_generic(entity) {
                if pgc.name.to_lowercase().contains(&query)
                    || pgc.description.to_lowercase().contains(&query)
                {
                    matching_gate_configs.push(json!({
                        "id": pgc.id,
                        "title": pgc.name,
                    }));
                }
            }
        }

        let total = matching_tasks.len()
            + matching_contexts.len()
            + matching_reasoning.len()
            + matching_knowledge.len()
            + matching_rules.len()
            + matching_standards.len()
            + matching_adrs.len()
            + matching_theories.len()
            + matching_compliance.len()
            + matching_sessions.len()
            + matching_reflections.len()
            + matching_workflows.len()
            + matching_instances.len()
            + matching_sandboxes.len()
            + matching_escalations.len()
            + matching_results.len()
            + matching_gate_configs.len();

        Ok(json!({
            "query": processed_query.original_query,
            "total_matches": total,
            "tasks": matching_tasks,
            "contexts": matching_contexts,
            "reasoning": matching_reasoning,
            "knowledge": matching_knowledge,
            "rules": matching_rules,
            "standards": matching_standards,
            "adrs": matching_adrs,
            "theories": matching_theories,
            "compliance": matching_compliance,
            "sessions": matching_sessions,
            "state_reflections": matching_reflections,
            "workflows": matching_workflows,
            "workflow_instances": matching_instances,
            "agent_sandboxes": matching_sandboxes,
            "escalation_requests": matching_escalations,
            "execution_results": matching_results,
            "progressive_gate_configs": matching_gate_configs,
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
