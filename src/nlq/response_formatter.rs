use crate::error::EngramError;
use crate::nlq::{ProcessedQuery, QueryIntent};
use serde_json::Value;

pub struct ResponseFormatter;

impl ResponseFormatter {
    pub fn new() -> Self {
        Self
    }

    pub fn format(&self, query: &ProcessedQuery, data: &Value) -> Result<String, EngramError> {
        match &query.intent {
            QueryIntent::ListTasks => self.format_task_list(data),
            QueryIntent::ShowTaskDetails => self.format_task_details(data),
            QueryIntent::FindRelationships => self.format_relationships(data),
            QueryIntent::SearchContext => self.format_context_results(data),
            QueryIntent::AnalyzeWorkflow => self.format_workflow_status(data),
            QueryIntent::ListSkills => self.format_skills_list(data),
            QueryIntent::SearchSkills => self.format_skills_search(data),
            QueryIntent::ListPrompts => self.format_prompts_list(data),
            QueryIntent::SearchPrompts => self.format_prompts_search(data),
            QueryIntent::FullTextSearch => self.format_full_text_search(data),
            QueryIntent::Unknown => self.format_unknown(data),
        }
    }

    // Skills/Prompts formatters
    fn format_skills_list(&self, data: &Value) -> Result<String, EngramError> {
        if let Some(skills) = data.get("skills").and_then(|v| v.as_array()) {
            let count = skills.len();
            if count == 0 {
                return Ok("No skills found. Set ENGRAM_SKILLS_PATH to enable skills.".to_string());
            }

            let mut output = format!("Found {} skill(s):\n\n", count);
            for skill in skills {
                let name = skill
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown");
                let desc = skill
                    .get("description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("(no description)");
                output.push_str(&format!("[{}]\n  {}\n\n", name, desc));
            }
            return Ok(output);
        }
        Ok("Skills data not available".to_string())
    }

    fn format_skills_search(&self, data: &Value) -> Result<String, EngramError> {
        let query = data.get("query").and_then(|v| v.as_str()).unwrap_or("");
        if let Some(skills) = data.get("skills").and_then(|v| v.as_array()) {
            let count = skills.len();
            if count == 0 {
                return Ok(format!("No skills found for query: '{}'", query));
            }

            let mut output = format!("Found {} skill(s) matching '{}':\n\n", count, query);
            for skill in skills {
                let name = skill
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown");
                let desc = skill
                    .get("description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("(no description)");
                output.push_str(&format!("[{}]\n  {}\n\n", name, desc));
            }
            return Ok(output);
        }
        Ok("Skills search data not available".to_string())
    }

    fn format_prompts_list(&self, data: &Value) -> Result<String, EngramError> {
        if let Some(prompts) = data.get("prompts").and_then(|v| v.as_array()) {
            let count = prompts.len();
            if count == 0 {
                return Ok(
                    "No prompts found. Set ENGRAM_PROMPTS_PATH to enable prompts.".to_string(),
                );
            }

            let mut output = format!("Found {} prompt(s):\n\n", count);
            for prompt in prompts {
                let name = prompt
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown");
                let title = prompt
                    .get("title")
                    .and_then(|v| v.as_str())
                    .unwrap_or("(no title)");
                output.push_str(&format!("[{}]\n  {}\n\n", name, title));
            }
            return Ok(output);
        }
        Ok("Prompts data not available".to_string())
    }

    fn format_prompts_search(&self, data: &Value) -> Result<String, EngramError> {
        let query = data.get("query").and_then(|v| v.as_str()).unwrap_or("");
        if let Some(prompts) = data.get("prompts").and_then(|v| v.as_array()) {
            let count = prompts.len();
            if count == 0 {
                return Ok(format!("No prompts found for query: '{}'", query));
            }

            let mut output = format!("Found {} prompt(s) matching '{}':\n\n", count, query);
            for prompt in prompts {
                let name = prompt
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown");
                let title = prompt
                    .get("title")
                    .and_then(|v| v.as_str())
                    .unwrap_or("(no title)");
                output.push_str(&format!("[{}]\n  {}\n\n", name, title));
            }
            return Ok(output);
        }
        Ok("Prompts search data not available".to_string())
    }

    fn format_task_list(&self, data: &Value) -> Result<String, EngramError> {
        if let Some(error) = data.get("error") {
            return Ok(format!(
                "Error: {}",
                error.as_str().unwrap_or("Unknown error")
            ));
        }

        let empty_vec = vec![];
        let tasks = data["tasks"].as_array().unwrap_or(&empty_vec);
        let count = data["count"].as_u64().unwrap_or(0);
        let agent = data["agent"].as_str().unwrap_or("default");

        if count == 0 {
            return Ok(format!("No tasks found for agent '{}'", agent));
        }

        let mut response = format!("Found {} task(s) for agent '{}':\n\n", count, agent);

        for (i, task) in tasks.iter().enumerate() {
            let title = task["title"].as_str().unwrap_or("Untitled");
            let status = task["status"].as_str().unwrap_or("Unknown");
            let priority = task["priority"].as_str().unwrap_or("Unknown");

            response.push_str(&format!(
                "{}. {} [{}] ({})\n",
                i + 1,
                title,
                status,
                priority
            ));
        }

        Ok(response)
    }

    fn format_task_details(&self, data: &Value) -> Result<String, EngramError> {
        if let Some(error) = data.get("error") {
            return Ok(format!(
                "Error: {}",
                error.as_str().unwrap_or("Unknown error")
            ));
        }

        if let Some(task) = data.get("task") {
            let title = task["title"].as_str().unwrap_or("Untitled");
            let description = task["description"].as_str().unwrap_or("No description");
            let status = task["status"].as_str().unwrap_or("Unknown");
            let priority = task["priority"].as_str().unwrap_or("Unknown");
            let agent = task["agent"].as_str().unwrap_or("Unknown");

            let mut response = format!("Task Details:\n");
            response.push_str(&format!("Title: {}\n", title));
            response.push_str(&format!("Status: {}\n", status));
            response.push_str(&format!("Priority: {}\n", priority));
            response.push_str(&format!("Agent: {}\n", agent));

            if !description.is_empty() {
                response.push_str(&format!("Description: {}\n", description));
            }

            if let Some(outcome) = task.get("outcome") {
                if let Some(outcome_str) = outcome.as_str() {
                    response.push_str(&format!("Outcome: {}\n", outcome_str));
                }
            }

            return Ok(response);
        }

        Ok("No task details available".to_string())
    }

    fn format_relationships(&self, data: &Value) -> Result<String, EngramError> {
        if let Some(error) = data.get("error") {
            return Ok(format!(
                "Error: {}",
                error.as_str().unwrap_or("Unknown error")
            ));
        }

        let task_id = data["task_id"].as_str().unwrap_or("unknown");
        let empty_vec = vec![];
        let relationships = data["relationships"].as_array().unwrap_or(&empty_vec);
        let count = data["count"].as_u64().unwrap_or(0);

        if count == 0 {
            return Ok(format!("No relationships found for task {}", task_id));
        }

        let mut response = format!("Found {} relationship(s) for task {}:\n\n", count, task_id);

        for (i, rel) in relationships.iter().enumerate() {
            let rel_type = rel["type"].as_str().unwrap_or("Unknown");
            let target = rel["target"].as_str().unwrap_or("Unknown");
            let strength = rel["strength"].as_str().unwrap_or("Unknown");

            response.push_str(&format!(
                "{}. {} -> {} ({})\n",
                i + 1,
                rel_type,
                target,
                strength
            ));
        }

        Ok(response)
    }

    fn format_context_results(&self, data: &Value) -> Result<String, EngramError> {
        let empty_vec = vec![];
        let contexts = data["contexts"].as_array().unwrap_or(&empty_vec);
        let count = data["count"].as_u64().unwrap_or(0);

        if count == 0 {
            return Ok("No context information found".to_string());
        }

        let mut response = format!("Found {} context item(s):\n\n", count);

        for (i, context) in contexts.iter().enumerate() {
            let title = context["title"].as_str().unwrap_or("Untitled");
            let relevance = context["relevance"].as_f64().unwrap_or(0.0);

            response.push_str(&format!(
                "{}. {} (relevance: {:.2})\n",
                i + 1,
                title,
                relevance
            ));
        }

        Ok(response)
    }

    fn format_workflow_status(&self, data: &Value) -> Result<String, EngramError> {
        let empty_vec = vec![];
        let workflows = data["workflows"].as_array().unwrap_or(&empty_vec);
        let count = data["count"].as_u64().unwrap_or(0);

        if count == 0 {
            return Ok("No workflows found".to_string());
        }

        let mut response = format!("Found {} workflow(s):\n\n", count);

        for (i, workflow) in workflows.iter().enumerate() {
            let title = workflow["title"].as_str().unwrap_or("Untitled");
            let current_state = workflow["current_state"].as_str().unwrap_or("Unknown");
            let status = workflow["status"].as_str().unwrap_or("Unknown");

            response.push_str(&format!(
                "{}. {} - {} ({})\n",
                i + 1,
                title,
                current_state,
                status
            ));
        }

        Ok(response)
    }

    fn format_full_text_search(&self, data: &Value) -> Result<String, EngramError> {
        let query = data["query"].as_str().unwrap_or("");
        let total = data["total_matches"].as_u64().unwrap_or(0);

        if total == 0 {
            return Ok(format!("No results found for '{}'", query));
        }

        let empty_vec = vec![];
        let mut response = format!("Found {} result(s) for '{}':", total, query);

        let tasks = data["tasks"].as_array().unwrap_or(&empty_vec);
        if !tasks.is_empty() {
            response.push_str("\n\nTasks:\n");
            for (i, task) in tasks.iter().enumerate() {
                let title = task["title"].as_str().unwrap_or("Untitled");
                let id = task["id"].as_str().unwrap_or("");
                let status = task["status"].as_str().unwrap_or("Unknown");
                response.push_str(&format!(
                    "  {}. [{}] {} ({})\n",
                    i + 1,
                    &id[..8.min(id.len())],
                    title,
                    status
                ));
            }
        }

        let contexts = data["contexts"].as_array().unwrap_or(&empty_vec);
        if !contexts.is_empty() {
            response.push_str("\n\nContext:\n");
            for (i, ctx) in contexts.iter().enumerate() {
                let title = ctx["title"].as_str().unwrap_or("Untitled");
                let id = ctx["id"].as_str().unwrap_or("");
                response.push_str(&format!(
                    "  {}. [{}] {}\n",
                    i + 1,
                    &id[..8.min(id.len())],
                    title
                ));
            }
        }

        let reasoning = data["reasoning"].as_array().unwrap_or(&empty_vec);
        if !reasoning.is_empty() {
            response.push_str("\n\nReasoning:\n");
            for (i, rsn) in reasoning.iter().enumerate() {
                let title = rsn["title"].as_str().unwrap_or("Untitled");
                let id = rsn["id"].as_str().unwrap_or("");
                response.push_str(&format!(
                    "  {}. [{}] {}\n",
                    i + 1,
                    &id[..8.min(id.len())],
                    title
                ));
            }
        }

        let knowledge = data["knowledge"].as_array().unwrap_or(&empty_vec);
        if !knowledge.is_empty() {
            response.push_str("\n\nKnowledge:\n");
            for (i, k) in knowledge.iter().enumerate() {
                let title = k["title"].as_str().unwrap_or("Untitled");
                let id = k["id"].as_str().unwrap_or("");
                response.push_str(&format!(
                    "  {}. [{}] {}\n",
                    i + 1,
                    &id[..8.min(id.len())],
                    title
                ));
            }
        }

        let rules = data["rules"].as_array().unwrap_or(&empty_vec);
        if !rules.is_empty() {
            response.push_str("\n\nRules:\n");
            for (i, r) in rules.iter().enumerate() {
                let title = r["title"].as_str().unwrap_or("Untitled");
                let id = r["id"].as_str().unwrap_or("");
                response.push_str(&format!(
                    "  {}. [{}] {}\n",
                    i + 1,
                    &id[..8.min(id.len())],
                    title
                ));
            }
        }

        let standards = data["standards"].as_array().unwrap_or(&empty_vec);
        if !standards.is_empty() {
            response.push_str("\n\nStandards:\n");
            for (i, s) in standards.iter().enumerate() {
                let title = s["title"].as_str().unwrap_or("Untitled");
                let id = s["id"].as_str().unwrap_or("");
                response.push_str(&format!(
                    "  {}. [{}] {}\n",
                    i + 1,
                    &id[..8.min(id.len())],
                    title
                ));
            }
        }

        let adrs = data["adrs"].as_array().unwrap_or(&empty_vec);
        if !adrs.is_empty() {
            response.push_str("\n\nADRs:\n");
            for (i, adr) in adrs.iter().enumerate() {
                let title = adr["title"].as_str().unwrap_or("Untitled");
                let id = adr["id"].as_str().unwrap_or("");
                let number = adr["number"].as_u64().unwrap_or(0);
                response.push_str(&format!(
                    "  {}. [{}] ADR-{}: {}\n",
                    i + 1,
                    &id[..8.min(id.len())],
                    number,
                    title
                ));
            }
        }

        let theories = data["theories"].as_array().unwrap_or(&empty_vec);
        if !theories.is_empty() {
            response.push_str("\n\nTheories:\n");
            for (i, t) in theories.iter().enumerate() {
                let title = t["title"].as_str().unwrap_or("Untitled");
                let id = t["id"].as_str().unwrap_or("");
                response.push_str(&format!(
                    "  {}. [{}] {}\n",
                    i + 1,
                    &id[..8.min(id.len())],
                    title
                ));
            }
        }

        let compliance = data["compliance"].as_array().unwrap_or(&empty_vec);
        if !compliance.is_empty() {
            response.push_str("\n\nCompliance:\n");
            for (i, c) in compliance.iter().enumerate() {
                let title = c["title"].as_str().unwrap_or("Untitled");
                let id = c["id"].as_str().unwrap_or("");
                response.push_str(&format!(
                    "  {}. [{}] {}\n",
                    i + 1,
                    &id[..8.min(id.len())],
                    title
                ));
            }
        }

        let sessions = data["sessions"].as_array().unwrap_or(&empty_vec);
        if !sessions.is_empty() {
            response.push_str("\n\nSessions:\n");
            for (i, s) in sessions.iter().enumerate() {
                let title = s["title"].as_str().unwrap_or("Untitled");
                let id = s["id"].as_str().unwrap_or("");
                response.push_str(&format!(
                    "  {}. [{}] {}\n",
                    i + 1,
                    &id[..8.min(id.len())],
                    title
                ));
            }
        }

        let reflections = data["state_reflections"].as_array().unwrap_or(&empty_vec);
        if !reflections.is_empty() {
            response.push_str("\n\nState Reflections:\n");
            for (i, r) in reflections.iter().enumerate() {
                let title = r["title"].as_str().unwrap_or("Untitled");
                let id = r["id"].as_str().unwrap_or("");
                response.push_str(&format!(
                    "  {}. [{}] {}\n",
                    i + 1,
                    &id[..8.min(id.len())],
                    title
                ));
            }
        }

        let workflows = data["workflows"].as_array().unwrap_or(&empty_vec);
        if !workflows.is_empty() {
            response.push_str("\n\nWorkflows:\n");
            for (i, w) in workflows.iter().enumerate() {
                let title = w["title"].as_str().unwrap_or("Untitled");
                let id = w["id"].as_str().unwrap_or("");
                response.push_str(&format!(
                    "  {}. [{}] {}\n",
                    i + 1,
                    &id[..8.min(id.len())],
                    title
                ));
            }
        }

        let instances = data["workflow_instances"].as_array().unwrap_or(&empty_vec);
        if !instances.is_empty() {
            response.push_str("\n\nWorkflow Instances:\n");
            for (i, wi) in instances.iter().enumerate() {
                let title = wi["title"].as_str().unwrap_or("Untitled");
                let id = wi["id"].as_str().unwrap_or("");
                response.push_str(&format!(
                    "  {}. [{}] {}\n",
                    i + 1,
                    &id[..8.min(id.len())],
                    title
                ));
            }
        }

        let sandboxes = data["agent_sandboxes"].as_array().unwrap_or(&empty_vec);
        if !sandboxes.is_empty() {
            response.push_str("\n\nAgent Sandboxes:\n");
            for (i, sb) in sandboxes.iter().enumerate() {
                let title = sb["title"].as_str().unwrap_or("Untitled");
                let id = sb["id"].as_str().unwrap_or("");
                response.push_str(&format!(
                    "  {}. [{}] {}\n",
                    i + 1,
                    &id[..8.min(id.len())],
                    title
                ));
            }
        }

        let escalations = data["escalation_requests"].as_array().unwrap_or(&empty_vec);
        if !escalations.is_empty() {
            response.push_str("\n\nEscalation Requests:\n");
            for (i, er) in escalations.iter().enumerate() {
                let title = er["title"].as_str().unwrap_or("Untitled");
                let id = er["id"].as_str().unwrap_or("");
                response.push_str(&format!(
                    "  {}. [{}] {}\n",
                    i + 1,
                    &id[..8.min(id.len())],
                    title
                ));
            }
        }

        let exec_results = data["execution_results"].as_array().unwrap_or(&empty_vec);
        if !exec_results.is_empty() {
            response.push_str("\n\nExecution Results:\n");
            for (i, er) in exec_results.iter().enumerate() {
                let title = er["title"].as_str().unwrap_or("Untitled");
                let id = er["id"].as_str().unwrap_or("");
                response.push_str(&format!(
                    "  {}. [{}] {}\n",
                    i + 1,
                    &id[..8.min(id.len())],
                    title
                ));
            }
        }

        let gate_configs = data["progressive_gate_configs"]
            .as_array()
            .unwrap_or(&empty_vec);
        if !gate_configs.is_empty() {
            response.push_str("\n\nProgressive Gate Configs:\n");
            for (i, pgc) in gate_configs.iter().enumerate() {
                let title = pgc["title"].as_str().unwrap_or("Untitled");
                let id = pgc["id"].as_str().unwrap_or("");
                response.push_str(&format!(
                    "  {}. [{}] {}\n",
                    i + 1,
                    &id[..8.min(id.len())],
                    title
                ));
            }
        }

        let doc_fragments = data["doc_fragments"].as_array().unwrap_or(&empty_vec);
        if !doc_fragments.is_empty() {
            response.push_str("\n\nDoc Fragments:\n");
            for (i, df) in doc_fragments.iter().enumerate() {
                let title = df["title"].as_str().unwrap_or("Untitled");
                let id = df["id"].as_str().unwrap_or("");
                let topic = df["topic"].as_str().unwrap_or("");
                let stale = df["stale"].as_bool().unwrap_or(false);
                let stale_marker = if stale { " [stale]" } else { "" };
                response.push_str(&format!(
                    "  {}. [{}] {} (topic: {}){}\n",
                    i + 1,
                    &id[..8.min(id.len())],
                    title,
                    topic,
                    stale_marker
                ));
            }
        }

        if let Some(deep_walk) = data.get("deep_walk") {
            response.push_str(&self.format_deep_walk_section(deep_walk)?);
        }

        Ok(response)
    }

    fn format_deep_walk_section(&self, deep_walk: &Value) -> Result<String, EngramError> {
        let total = deep_walk["total_connected"].as_u64().unwrap_or(0);
        if total == 0 {
            return Ok(String::new());
        }

        let max_depth = deep_walk["max_depth"].as_u64().unwrap_or(2);
        let seed_count = deep_walk["seed_count"].as_u64().unwrap_or(0);
        let empty_vec = vec![];
        let connected = deep_walk["connected_entities"]
            .as_array()
            .unwrap_or(&empty_vec);

        let mut section = format!(
            "\n\nConnected Entities (depth {} from {} seeds):\n",
            max_depth, seed_count
        );

        for (i, entity) in connected.iter().enumerate() {
            let eid = entity["entity_id"].as_str().unwrap_or("");
            let etype = entity["entity_type"].as_str().unwrap_or("unknown");
            let depth = entity["depth"].as_u64().unwrap_or(0);
            let rel_type = entity["relationship_type"].as_str().unwrap_or("Unknown");
            let direction = entity["direction"].as_str().unwrap_or("unknown");
            let arrow = if direction == "outbound" { "->" } else { "<-" };

            section.push_str(&format!(
                "  {}. [{}] {} {} {} ({})\n",
                i + 1,
                &eid[..8.min(eid.len())],
                arrow,
                etype,
                rel_type,
                depth
            ));
        }

        Ok(section)
    }

    fn format_unknown(&self, data: &Value) -> Result<String, EngramError> {
        if let Some(error) = data.get("error") {
            let error_msg = error.as_str().unwrap_or("Unknown error");
            if let Some(suggestion) = data.get("suggestion") {
                let suggestion_msg = suggestion.as_str().unwrap_or("");
                return Ok(format!("{}\n\n{}", error_msg, suggestion_msg));
            }
            return Ok(error_msg.to_string());
        }

        Ok(
            "I don't understand that query. Try asking about tasks, relationships, or workflows."
                .to_string(),
        )
    }
}

impl Default for ResponseFormatter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

    #[test]
    fn test_format_empty_task_list() {
        let formatter = ResponseFormatter::new();
        let data = json!({
            "tasks": [],
            "count": 0,
            "agent": "alice"
        });

        let result = formatter.format_task_list(&data).unwrap();
        assert!(result.contains("No tasks found for agent 'alice'"));
    }

    #[test]
    fn test_format_task_list_with_tasks() {
        let formatter = ResponseFormatter::new();
        let data = json!({
            "tasks": [
                {
                    "title": "Test task",
                    "status": "Todo",
                    "priority": "High"
                }
            ],
            "count": 1,
            "agent": "bob"
        });

        let result = formatter.format_task_list(&data).unwrap();
        assert!(result.contains("Found 1 task(s) for agent 'bob'"));
        assert!(result.contains("Test task"));
    }

    #[test]
    fn test_format_error() {
        let formatter = ResponseFormatter::new();
        let data = json!({
            "error": "Task not found"
        });

        let result = formatter.format_task_details(&data).unwrap();
        assert!(result.contains("Error: Task not found"));
    }

    #[test]
    fn test_format_deep_walk_section() {
        let formatter = ResponseFormatter::new();
        let data = json!({
            "query": "test query",
            "total_matches": 1,
            "tasks": [{"id": "task-1", "title": "Test"}],
            "deep_walk": {
                "enabled": true,
                "seed_count": 1,
                "max_depth": 2,
                "total_connected": 2,
                "connected_entities": [
                    {
                        "entity_id": "ctx-abc",
                        "entity_type": "context",
                        "depth": 1,
                        "relationship_type": "Explains",
                        "direction": "outbound"
                    },
                    {
                        "entity_id": "rsn-def",
                        "entity_type": "reasoning",
                        "depth": 2,
                        "relationship_type": "DependsOn",
                        "direction": "inbound"
                    }
                ]
            }
        });

        let result = formatter.format_full_text_search(&data).unwrap();
        assert!(result.contains("Connected Entities"));
        assert!(result.contains("depth 2 from 1 seeds"));
        assert!(result.contains("-> context Explains (1)"));
        assert!(result.contains("<- reasoning DependsOn (2)"));
    }

    #[test]
    fn test_format_deep_walk_empty() {
        let formatter = ResponseFormatter::new();
        let data = json!({
            "query": "test query",
            "total_matches": 1,
            "tasks": [{"id": "task-1", "title": "Test"}],
            "deep_walk": {
                "enabled": true,
                "seed_count": 1,
                "max_depth": 2,
                "total_connected": 0,
                "connected_entities": []
            }
        });

        let result = formatter.format_full_text_search(&data).unwrap();
        assert!(!result.contains("Connected Entities"));
    }

    #[test]
    fn test_format_no_deep_walk() {
        let formatter = ResponseFormatter::new();
        let data = json!({
            "query": "test query",
            "total_matches": 1,
            "tasks": [{"id": "task-1", "title": "Test"}]
        });

        let result = formatter.format_full_text_search(&data).unwrap();
        assert!(!result.contains("Connected Entities"));
    }
}
