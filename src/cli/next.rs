use crate::entities::task::{Task, TaskPriority, TaskStatus};
use crate::entities::{Entity, GenericEntity};
use crate::storage::{GitCommit, QueryFilter, QueryResult, Storage, StorageStats};
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::task::{Task, TaskPriority, TaskStatus};
    use crate::entities::GenericEntity;
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
}
