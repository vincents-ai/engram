use crate::entities::task::{Task, TaskPriority, TaskStatus};
use crate::entities::Entity;
use crate::storage::Storage;
use crate::EngramError;
use std::collections::HashMap;

pub fn interpolate(template: &str, context: &HashMap<String, String>) -> String {
    let mut result = template.to_string();
    for (key, value) in context {
        let placeholder = format!("{{{{{}}}}}", key);
        result = result.replace(&placeholder, value);
    }
    result
}

pub fn find_next_task<S: Storage>(storage: &S, agent: &str) -> Result<Option<Task>, EngramError> {
    let tasks = storage.query_by_agent(agent, Some("task"))?;

    let mut task_entities: Vec<Task> = Vec::new();

    for entity in tasks {
        if let Ok(task) = Task::from_generic(entity) {
            if task.status != TaskStatus::Done && task.status != TaskStatus::Cancelled {
                task_entities.push(task);
            }
        }
    }

    if task_entities.is_empty() {
        return Ok(None);
    }

    task_entities.sort_by(|a, b| {
        let status_order = |status: &TaskStatus| match status {
            TaskStatus::InProgress => 0,
            TaskStatus::Todo => 1,
            TaskStatus::Blocked => 2,
            _ => 3,
        };

        let status_cmp = status_order(&a.status).cmp(&status_order(&b.status));
        if status_cmp != std::cmp::Ordering::Equal {
            return status_cmp;
        }

        let priority_order = |priority: &TaskPriority| match priority {
            TaskPriority::Critical => 0,
            TaskPriority::High => 1,
            TaskPriority::Medium => 2,
            TaskPriority::Low => 3,
        };

        priority_order(&a.priority).cmp(&priority_order(&b.priority))
    });

    Ok(task_entities.first().cloned())
}

use crate::entities::context::Context;
use crate::entities::workflow::Workflow;

pub fn handle_next_command<S: Storage>(
    storage: &mut S,
    id: Option<String>,
    format: String,
) -> Result<(), EngramError> {
    // 1. Identify Task
    let task = if let Some(task_id) = id {
        if let Some(entity) = storage.get(&task_id, "task")? {
            Task::from_generic(entity).map_err(|e| EngramError::Validation(e.to_string()))?
        } else {
            return Err(EngramError::NotFound(format!("Task {} not found", task_id)));
        }
    } else {
        // Default agent to "default" for CLI usage, typically would come from config/auth
        if let Some(t) = find_next_task(storage, "default")? {
            t
        } else {
            println!("No pending tasks found.");
            return Ok(());
        }
    };

    // 2. Load associated Workflow (if any)
    let workflow = if let Some(workflow_id) = &task.workflow_id {
        if let Some(entity) = storage.get(workflow_id, "workflow")? {
            Some(
                Workflow::from_generic(entity)
                    .map_err(|e| EngramError::Validation(e.to_string()))?,
            )
        } else {
            None
        }
    } else {
        None
    };

    // 3. Build Context Map
    let mut prompt_context = HashMap::new();
    prompt_context.insert("TASK_ID".to_string(), task.id.clone());
    prompt_context.insert("TASK_TITLE".to_string(), task.title.clone());
    prompt_context.insert("TASK_DESCRIPTION".to_string(), task.description.clone());

    // Load related Context entities
    let mut context_content = String::new();
    for context_id in &task.context_ids {
        if let Some(entity) = storage.get(context_id, "context")? {
            let context = Context::from_generic(entity)
                .map_err(|e| EngramError::Validation(e.to_string()))?;
            context_content.push_str(&format!("\n- {}: {}", context.title, context.content));
        }
    }
    prompt_context.insert("CONTEXT".to_string(), context_content);

    // 4. Select Prompts
    let (system_prompt, user_prompt) = if let Some(ref wf) = workflow {
        if let Some(state_name) = &task.workflow_state {
            if let Some(state) = wf.states.iter().find(|s| &s.name == state_name) {
                if let Some(prompts) = &state.prompts {
                    (
                        prompts
                            .system
                            .clone()
                            .unwrap_or_else(|| "You are an AI assistant.".to_string()),
                        prompts.user.clone().unwrap_or_else(|| {
                            "Task: {{TASK_TITLE}}\nDescription: {{TASK_DESCRIPTION}}".to_string()
                        }),
                    )
                } else {
                    (
                        "You are an AI assistant.".to_string(),
                        "Task: {{TASK_TITLE}}\nDescription: {{TASK_DESCRIPTION}}".to_string(),
                    )
                }
            } else {
                (
                    "You are an AI assistant.".to_string(),
                    "Task: {{TASK_TITLE}}\nDescription: {{TASK_DESCRIPTION}}".to_string(),
                )
            }
        } else {
            (
                "You are an AI assistant.".to_string(),
                "Task: {{TASK_TITLE}}\nDescription: {{TASK_DESCRIPTION}}".to_string(),
            )
        }
    } else {
        (
            "You are an AI assistant.".to_string(),
            "Task: {{TASK_TITLE}}\nDescription: {{TASK_DESCRIPTION}}".to_string(),
        )
    };

    // 5. Interpolate
    let final_system = interpolate(&system_prompt, &prompt_context);
    let final_user = interpolate(&user_prompt, &prompt_context);

    let task_management_instructions = format!(
        r#"
## Task Management with Engram

**Current Task**: {} ({})
**Status**: {:?} | **Priority**: {:?}

### Required Workflow Actions:

1. **Update Task Status**:
   ```bash
   # Mark task as in progress
   engram task update {} --status in_progress
   
   # Mark task as done when complete
   engram task update {} --status done
   ```

2. **Create Context and Reasoning**:
   ```bash
   # Create context for important findings
   engram context create --title "Context title" --content "Details..."
   
   # Create reasoning for decisions made
   engram reasoning create --title "Why we chose X" --content "Because..."
   
   # Link to task
   engram relationship create --source-id {} --source-type task \
     --target-id <context-id> --target-type context --relationship-type references
   ```

3. **Workflow Management** (if applicable):
   {}

4. **Before Committing Code**:
   - All commits MUST reference this task: `[{}]`
   - Task must have both context and reasoning relationships
   - Example: `git commit -m "feat: implement feature [{}]"`

### Query Task Details:
```bash
engram task show {}
engram relationship connected --entity-id {}
```
"#,
        task.title,
        task.id,
        task.status,
        task.priority,
        task.id,
        task.id,
        task.id,
        if workflow.is_some() {
            format!(
                "This task is part of a workflow. Use:\n   engram workflow status <instance-id>\n   engram workflow transition <instance-id> --transition <name> --agent default"
            )
        } else {
            "No active workflow for this task.".to_string()
        },
        task.id,
        task.id,
        task.id,
        task.id
    );

    // 6. Output
    if format == "json" {
        let output = serde_json::json!({
            "task_id": task.id,
            "system_prompt": final_system,
            "user_prompt": final_user,
            "task_management": task_management_instructions
        });
        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    } else {
        println!(
            "## System Prompt\n\n{}\n\n## User Prompt\n\n{}\n\n{}",
            final_system, final_user, task_management_instructions
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::task::{Task, TaskPriority, TaskStatus};
    use crate::entities::GenericEntity;
    use crate::storage::{GitCommit, QueryFilter, QueryResult, StorageStats};
    use chrono::Utc;
    use serde_json::Value;

    struct MockStorage {
        tasks: Vec<Task>,
    }

    impl Storage for MockStorage {
        fn get(&self, _id: &str, _entity_type: &str) -> Result<Option<GenericEntity>, EngramError> {
            Ok(None)
        }

        fn store(&mut self, _entity: &GenericEntity) -> Result<(), EngramError> {
            Ok(())
        }

        fn delete(&mut self, _id: &str, _entity_type: &str) -> Result<(), EngramError> {
            Ok(())
        }

        fn query_by_agent(
            &self,
            _agent: &str,
            _entity_type: Option<&str>,
        ) -> Result<Vec<GenericEntity>, EngramError> {
            let mut result: Vec<GenericEntity> = Vec::new();
            for task in &self.tasks {
                result.push(task.to_generic());
            }
            Ok(result)
        }

        fn query(&self, _filter: &QueryFilter) -> Result<QueryResult, EngramError> {
            Ok(QueryResult {
                entities: vec![],
                total_count: 0,
                has_more: false,
            })
        }

        fn query_by_time_range(
            &self,
            _start: chrono::DateTime<Utc>,
            _end: chrono::DateTime<Utc>,
        ) -> Result<Vec<GenericEntity>, EngramError> {
            Ok(vec![])
        }

        fn query_by_type(
            &self,
            _entity_type: &str,
            _filters: Option<&HashMap<String, Value>>,
            _limit: Option<usize>,
            _offset: Option<usize>,
        ) -> Result<QueryResult, EngramError> {
            Ok(QueryResult {
                entities: vec![],
                total_count: 0,
                has_more: false,
            })
        }

        fn text_search(
            &self,
            _query: &str,
            _entity_types: Option<&[String]>,
            _limit: Option<usize>,
        ) -> Result<Vec<GenericEntity>, EngramError> {
            Ok(vec![])
        }

        fn count(&self, _filter: &QueryFilter) -> Result<usize, EngramError> {
            Ok(0)
        }
        fn list_ids(&self, _entity_type: &str) -> Result<Vec<String>, EngramError> {
            Ok(vec![])
        }
        fn get_all(&self, _entity_type: &str) -> Result<Vec<GenericEntity>, EngramError> {
            Ok(vec![])
        }
        fn sync(&mut self) -> Result<(), EngramError> {
            Ok(())
        }
        fn current_branch(&self) -> Result<String, EngramError> {
            Ok("main".to_string())
        }
        fn create_branch(&mut self, _branch_name: &str) -> Result<(), EngramError> {
            Ok(())
        }
        fn switch_branch(&mut self, _branch_name: &str) -> Result<(), EngramError> {
            Ok(())
        }
        fn merge_branches(&mut self, _source: &str, _target: &str) -> Result<(), EngramError> {
            Ok(())
        }
        fn history(&self, _limit: Option<usize>) -> Result<Vec<GitCommit>, EngramError> {
            Ok(vec![])
        }
        fn bulk_store(&mut self, _entities: &[GenericEntity]) -> Result<(), EngramError> {
            Ok(())
        }
        fn get_stats(&self) -> Result<StorageStats, EngramError> {
            Ok(StorageStats::default())
        }
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    fn create_test_task(id: &str, status: TaskStatus, priority: TaskPriority) -> Task {
        Task {
            id: id.to_string(),
            title: format!("Task {}", id),
            description: "desc".to_string(),
            status,
            priority,
            agent: "test-agent".to_string(),
            start_time: Utc::now(),
            end_time: None,
            parent: None,
            children: vec![],
            context_ids: vec![],
            knowledge: vec![],
            files: vec![],
            outcome: None,
            workflow_id: None,
            workflow_state: None,
            tags: vec![],
            metadata: HashMap::new(),
        }
    }

    #[test]
    fn test_interpolation() {
        let template = "Hello {{AGENT_NAME}}, working on {{TASK_ID}}";
        let mut context = HashMap::new();
        context.insert("AGENT_NAME".to_string(), "Alice".to_string());
        context.insert("TASK_ID".to_string(), "123".to_string());

        let result = interpolate(template, &context);
        assert_eq!(result, "Hello Alice, working on 123");
    }

    #[test]
    fn test_find_next_task_selection() {
        let t1 = create_test_task("1", TaskStatus::Todo, TaskPriority::High);
        let t2 = create_test_task("2", TaskStatus::InProgress, TaskPriority::Medium);
        let t3 = create_test_task("3", TaskStatus::Todo, TaskPriority::Critical);
        let t4 = create_test_task("4", TaskStatus::Done, TaskPriority::Critical);

        let storage = MockStorage {
            tasks: vec![t1, t2.clone(), t3, t4],
        };

        let next = find_next_task(&storage, "test-agent").unwrap();
        assert!(next.is_some());
        assert_eq!(next.unwrap().id, "2");
    }

    #[test]
    fn test_find_next_task_priority_tiebreaker() {
        let t1 = create_test_task("1", TaskStatus::Todo, TaskPriority::Low);
        let t2 = create_test_task("2", TaskStatus::Todo, TaskPriority::High);

        let storage = MockStorage {
            tasks: vec![t1, t2.clone()],
        };

        let next = find_next_task(&storage, "test-agent").unwrap();
        assert!(next.is_some());
        assert_eq!(next.unwrap().id, "2");
    }

    #[test]
    fn test_find_next_task_empty() {
        let storage = MockStorage { tasks: vec![] };
        let next = find_next_task(&storage, "test-agent").unwrap();
        assert!(next.is_none());
    }

    #[test]
    fn test_handle_next_command_task_not_found() {
        let mut storage = MockStorage { tasks: vec![] };
        let result = handle_next_command(
            &mut storage,
            Some("missing".to_string()),
            "text".to_string(),
        );
        assert!(matches!(result, Err(EngramError::NotFound(_))));
    }

    #[test]
    fn test_handle_next_command_no_pending() {
        // We need a MockStorage that behaves properly for handle_next_command which uses
        // storage.query_by_agent inside find_next_task.
        // The MockStorage implementation in this file already implements query_by_agent
        // by returning all tasks.
        let mut storage = MockStorage { tasks: vec![] };

        // This should print "No pending tasks found" and return Ok(())
        let result = handle_next_command(&mut storage, None, "text".to_string());
        assert!(result.is_ok());
    }
}
