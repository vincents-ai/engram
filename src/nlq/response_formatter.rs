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
                let name = skill.get("name").and_then(|v| v.as_str()).unwrap_or("unknown");
                let desc = skill.get("description").and_then(|v| v.as_str()).unwrap_or("(no description)");
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
                let name = skill.get("name").and_then(|v| v.as_str()).unwrap_or("unknown");
                let desc = skill.get("description").and_then(|v| v.as_str()).unwrap_or("(no description)");
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
                return Ok("No prompts found. Set ENGRAM_PROMPTS_PATH to enable prompts.".to_string());
            }

            let mut output = format!("Found {} prompt(s):\n\n", count);
            for prompt in prompts {
                let name = prompt.get("name").and_then(|v| v.as_str()).unwrap_or("unknown");
                let title = prompt.get("title").and_then(|v| v.as_str()).unwrap_or("(no title)");
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
                let name = prompt.get("name").and_then(|v| v.as_str()).unwrap_or("unknown");
                let title = prompt.get("title").and_then(|v| v.as_str()).unwrap_or("(no title)");
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
    use crate::nlq::QueryIntent;
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
}
