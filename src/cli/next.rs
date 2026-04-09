use crate::entities::task::{Task, TaskPriority, TaskStatus};
use crate::entities::Entity;
use crate::storage::Storage;
use crate::EngramError;
use chrono::Utc;
use std::collections::HashMap;

pub fn interpolate(template: &str, context: &HashMap<String, String>) -> String {
    let mut result = template.to_string();
    for (key, value) in context {
        let placeholder = format!("{{{{{}}}}}", key);
        result = result.replace(&placeholder, value);
    }
    result
}

pub struct NextScope {
    pub parent: Option<String>,
    pub agent: Option<String>,
    pub session: Option<String>,
    pub tag: Option<String>,
}

pub fn find_next_task<S: Storage>(
    storage: &S,
    agent: &str,
    scope: &NextScope,
) -> Result<Option<Task>, EngramError> {
    let tasks = storage.query_by_agent(agent, Some("task"))?;

    let mut task_entities: Vec<Task> = Vec::new();

    for entity in tasks {
        if let Ok(task) = Task::from_generic(entity) {
            if task.status != TaskStatus::Done && task.status != TaskStatus::Cancelled {
                if let Some(ref parent_id) = scope.parent {
                    if task.parent.as_deref() != Some(parent_id.as_str()) {
                        continue;
                    }
                }
                if let Some(ref scope_agent) = scope.agent {
                    if task.agent != *scope_agent {
                        continue;
                    }
                }
                if let Some(ref session_id) = scope.session {
                    if task.metadata.get("session_id").and_then(|v| v.as_str())
                        != Some(session_id.as_str())
                    {
                        continue;
                    }
                }
                if let Some(ref tag) = scope.tag {
                    if !task.tags.iter().any(|t| t == tag) {
                        continue;
                    }
                }
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
use crate::entities::session::{Session, SessionStatus};
use crate::entities::workflow::Workflow;

fn find_active_session<S: Storage>(storage: &S) -> Result<Option<Session>, EngramError> {
    let session_ids = storage.list_ids(Session::entity_type())?;
    let mut active_sessions: Vec<Session> = Vec::new();

    for id in session_ids {
        if let Some(entity) = storage.get(&id, Session::entity_type())? {
            if let Ok(session) = Session::from_generic(entity) {
                match session.status {
                    SessionStatus::Active | SessionStatus::Paused | SessionStatus::Reflecting => {}
                    SessionStatus::Completed | SessionStatus::Cancelled => continue,
                }
                active_sessions.push(session);
            }
        }
    }

    active_sessions.sort_by(|a, b| b.start_time.cmp(&a.start_time));
    Ok(active_sessions.into_iter().next())
}

pub fn handle_next_command<S: Storage>(
    storage: &mut S,
    id: Option<String>,
    format: String,
    agent: Option<String>,
    parent: Option<String>,
    scope_agent: Option<String>,
    session: Option<String>,
    tag: Option<String>,
) -> Result<(), EngramError> {
    let scope = NextScope {
        parent,
        agent: scope_agent,
        session,
        tag,
    };

    // 1. Identify Task
    let task = if let Some(task_id) = id {
        if let Some(entity) = storage.get(&task_id, "task")? {
            Task::from_generic(entity).map_err(|e| EngramError::Validation(e.to_string()))?
        } else {
            return Err(EngramError::NotFound(format!("Task {} not found", task_id)));
        }
    } else {
        if let Some(t) = find_next_task(storage, "default", &scope)? {
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

    // 5. Resolve persona system prompt prefix (if agent config specifies one)
    let persona_prefix = agent
        .as_deref()
        .and_then(|agent_name| {
            let agent_file =
                std::path::PathBuf::from(".engram/agents").join(format!("{}.yaml", agent_name));
            std::fs::read_to_string(&agent_file).ok()
        })
        .and_then(|yaml| {
            serde_yaml::from_str::<crate::config::agent_config::AgentConfig>(&yaml).ok()
        })
        .and_then(|cfg| cfg.persona)
        .and_then(|persona_name| {
            let result = crate::personas::find_persona(&persona_name);
            if result.is_none() {
                eprintln!(
                    "⚠️  Persona '{}' not found in storage or embedded set",
                    persona_name
                );
            }
            result
        })
        .map(|(_, def)| def.instructions)
        .unwrap_or_default();

    // 6. Interpolate
    let interpolated_system = interpolate(&system_prompt, &prompt_context);
    let final_system = if persona_prefix.is_empty() {
        interpolated_system
    } else {
        format!("{}\n\n{}", persona_prefix, interpolated_system)
    };
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

    // 6. Detect active session
    let active_session = find_active_session(storage)?;

    // 7. Output
    if format == "json" {
        let mut output = serde_json::json!({
            "task_id": task.id,
            "system_prompt": final_system,
            "user_prompt": final_user,
            "task_management": task_management_instructions
        });
        if let Some(ref sess) = active_session {
            let elapsed = Utc::now()
                .signed_duration_since(sess.start_time)
                .num_seconds()
                .max(0) as u64;
            let mut session_json = serde_json::json!({
                "session_id": sess.id,
                "session_title": sess.title,
                "agent": sess.agent,
                "status": format!("{:?}", sess.status),
                "started": sess.start_time.to_rfc3339(),
                "elapsed_seconds": elapsed,
                "goals": sess.goals,
                "task_count": sess.task_ids.len(),
                "context_count": sess.context_ids.len()
            });
            if !sess.outcomes.is_empty() {
                session_json["outcomes"] = serde_json::json!(sess.outcomes);
            }
            output["session"] = session_json;
        }
        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    } else {
        let mut output_parts = Vec::new();

        if let Some(ref sess) = active_session {
            let elapsed = Utc::now()
                .signed_duration_since(sess.start_time)
                .num_seconds()
                .max(0) as u64;
            let hours = elapsed / 3600;
            let minutes = (elapsed % 3600) / 60;

            let mut session_header = format!(
                "## Active Session\n\n**{}** ({})\nAgent: {} | Status: {:?} | Elapsed: {}h {}m\n",
                sess.title,
                &sess.id[..8],
                sess.agent,
                sess.status,
                hours,
                minutes
            );

            if !sess.goals.is_empty() {
                session_header.push_str("\n**Goals:**\n");
                for goal in &sess.goals {
                    session_header.push_str(&format!("  - {}\n", goal));
                }
            }

            session_header.push_str(&format!(
                "\nTasks in session: {} | Context items: {}",
                sess.task_ids.len(),
                sess.context_ids.len()
            ));

            if !sess.outcomes.is_empty() {
                session_header.push_str("\n\n**Outcomes so far:**\n");
                for outcome in &sess.outcomes {
                    session_header.push_str(&format!("  - {}\n", outcome));
                }
            }

            output_parts.push(session_header);
        }

        output_parts.push(format!(
            "## System Prompt\n\n{}\n\n## User Prompt\n\n{}\n\n{}",
            final_system, final_user, task_management_instructions
        ));

        println!("{}", output_parts.join("\n\n---\n\n"));
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
            block_reason: None,
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

        let scope = NextScope {
            parent: None,
            agent: None,
            session: None,
            tag: None,
        };
        let next = find_next_task(&storage, "test-agent", &scope).unwrap();
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

        let scope = NextScope {
            parent: None,
            agent: None,
            session: None,
            tag: None,
        };
        let next = find_next_task(&storage, "test-agent", &scope).unwrap();
        assert!(next.is_some());
        assert_eq!(next.unwrap().id, "2");
    }

    #[test]
    fn test_find_next_task_empty() {
        let storage = MockStorage { tasks: vec![] };
        let scope = NextScope {
            parent: None,
            agent: None,
            session: None,
            tag: None,
        };
        let next = find_next_task(&storage, "test-agent", &scope).unwrap();
        assert!(next.is_none());
    }

    #[test]
    fn test_handle_next_command_task_not_found() {
        let mut storage = MockStorage { tasks: vec![] };
        let result = handle_next_command(
            &mut storage,
            Some("missing".to_string()),
            "text".to_string(),
            None,
            None,
            None,
            None,
            None,
        );
        assert!(matches!(result, Err(EngramError::NotFound(_))));
    }

    #[test]
    fn test_handle_next_command_no_pending() {
        let mut storage = MockStorage { tasks: vec![] };

        let result = handle_next_command(
            &mut storage,
            None,
            "text".to_string(),
            None,
            None,
            None,
            None,
            None,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_find_next_task_scope_parent() {
        let mut t1 = create_test_task("1", TaskStatus::Todo, TaskPriority::High);
        t1.parent = Some("parent-1".to_string());
        let mut t2 = create_test_task("2", TaskStatus::InProgress, TaskPriority::Medium);
        t2.parent = Some("parent-2".to_string());
        let t3 = create_test_task("3", TaskStatus::Todo, TaskPriority::Critical);

        let storage = MockStorage {
            tasks: vec![t1, t2.clone(), t3],
        };

        let scope = NextScope {
            parent: Some("parent-1".to_string()),
            agent: None,
            session: None,
            tag: None,
        };
        let next = find_next_task(&storage, "test-agent", &scope).unwrap();
        assert!(next.is_some());
        assert_eq!(next.unwrap().id, "1");
    }

    #[test]
    fn test_find_next_task_scope_agent() {
        let mut t1 = create_test_task("1", TaskStatus::Todo, TaskPriority::High);
        t1.agent = "alice".to_string();
        let mut t2 = create_test_task("2", TaskStatus::InProgress, TaskPriority::Low);
        t2.agent = "bob".to_string();

        let storage = MockStorage {
            tasks: vec![t1, t2.clone()],
        };

        let scope = NextScope {
            parent: None,
            agent: Some("bob".to_string()),
            session: None,
            tag: None,
        };
        let next = find_next_task(&storage, "test-agent", &scope).unwrap();
        assert!(next.is_some());
        assert_eq!(next.unwrap().id, "2");
    }

    #[test]
    fn test_find_next_task_scope_tag() {
        let mut t1 = create_test_task("1", TaskStatus::Todo, TaskPriority::High);
        t1.tags = vec!["frontend".to_string()];
        let mut t2 = create_test_task("2", TaskStatus::InProgress, TaskPriority::Low);
        t2.tags = vec!["backend".to_string()];

        let storage = MockStorage {
            tasks: vec![t1, t2.clone()],
        };

        let scope = NextScope {
            parent: None,
            agent: None,
            session: None,
            tag: Some("backend".to_string()),
        };
        let next = find_next_task(&storage, "test-agent", &scope).unwrap();
        assert!(next.is_some());
        assert_eq!(next.unwrap().id, "2");
    }

    #[test]
    fn test_find_next_task_scope_session() {
        let mut t1 = create_test_task("1", TaskStatus::Todo, TaskPriority::High);
        t1.metadata.insert(
            "session_id".to_string(),
            serde_json::Value::String("sess-1".to_string()),
        );
        let mut t2 = create_test_task("2", TaskStatus::InProgress, TaskPriority::Low);
        t2.metadata.insert(
            "session_id".to_string(),
            serde_json::Value::String("sess-2".to_string()),
        );

        let storage = MockStorage {
            tasks: vec![t1, t2.clone()],
        };

        let scope = NextScope {
            parent: None,
            agent: None,
            session: Some("sess-2".to_string()),
            tag: None,
        };
        let next = find_next_task(&storage, "test-agent", &scope).unwrap();
        assert!(next.is_some());
        assert_eq!(next.unwrap().id, "2");
    }

    #[test]
    fn test_find_next_task_scope_combined() {
        let mut t1 = create_test_task("1", TaskStatus::Todo, TaskPriority::High);
        t1.parent = Some("p1".to_string());
        t1.tags = vec!["bug".to_string()];
        let mut t2 = create_test_task("2", TaskStatus::Todo, TaskPriority::Critical);
        t2.parent = Some("p1".to_string());
        t2.tags = vec!["feature".to_string()];
        let mut t3 = create_test_task("3", TaskStatus::Todo, TaskPriority::Critical);
        t3.parent = Some("p1".to_string());
        t3.tags = vec!["bug".to_string()];

        let storage = MockStorage {
            tasks: vec![t1, t2, t3],
        };

        let scope = NextScope {
            parent: Some("p1".to_string()),
            agent: None,
            session: None,
            tag: Some("bug".to_string()),
        };
        let next = find_next_task(&storage, "test-agent", &scope).unwrap();
        assert!(next.is_some());
        let id = next.unwrap().id;
        assert!(id == "1" || id == "3");
    }
}
