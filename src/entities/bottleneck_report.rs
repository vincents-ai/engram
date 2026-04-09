use super::{Entity, GenericEntity};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct BottleneckReport {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "project_path")]
    pub project_path: String,

    #[serde(rename = "computed_at")]
    pub computed_at: DateTime<Utc>,

    #[serde(rename = "agent")]
    pub agent: String,

    #[serde(
        rename = "slowest_tasks",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub slowest_tasks: Vec<BottleneckEntry>,

    #[serde(
        rename = "blocked_tasks",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub blocked_tasks: Vec<BottleneckEntry>,

    #[serde(rename = "total_analyzed")]
    pub total_analyzed: u64,

    #[serde(rename = "blocked_count")]
    pub blocked_count: u64,

    #[serde(
        rename = "metadata",
        skip_serializing_if = "HashMap::is_empty",
        default
    )]
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BottleneckEntry {
    #[serde(rename = "task_id")]
    pub task_id: String,

    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "status")]
    pub status: String,

    #[serde(rename = "agent")]
    pub agent: String,

    #[serde(rename = "duration_hours")]
    pub duration_hours: f64,

    #[serde(rename = "block_reason")]
    pub block_reason: Option<String>,

    #[serde(rename = "start_time")]
    pub start_time: DateTime<Utc>,

    #[serde(rename = "end_time")]
    pub end_time: Option<DateTime<Utc>>,
}

impl BottleneckReport {
    pub fn new(project_path: String, agent: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            project_path,
            computed_at: Utc::now(),
            agent,
            slowest_tasks: Vec::new(),
            blocked_tasks: Vec::new(),
            total_analyzed: 0,
            blocked_count: 0,
            metadata: HashMap::new(),
        }
    }

    pub fn compute<S: crate::storage::Storage>(
        storage: &S,
        repo_path: &std::path::Path,
        agent: &str,
        top_n: usize,
    ) -> crate::Result<Self> {
        let mut report =
            BottleneckReport::new(repo_path.to_string_lossy().to_string(), agent.to_string());

        let generics = storage.get_all("task")?;

        let mut all_entries: Vec<BottleneckEntry> = Vec::new();

        for generic in &generics {
            if let Ok(task) = super::Task::from_generic(generic.clone()) {
                report.total_analyzed += 1;

                let duration_hours = if let Some(end) = task.end_time {
                    end.signed_duration_since(task.start_time).num_seconds() as f64 / 3600.0
                } else {
                    Utc::now()
                        .signed_duration_since(task.start_time)
                        .num_seconds() as f64
                        / 3600.0
                };

                let status_str = format!("{:?}", task.status).to_lowercase();

                let entry = BottleneckEntry {
                    task_id: task.id.clone(),
                    title: task.title.clone(),
                    status: status_str.clone(),
                    agent: task.agent.clone(),
                    duration_hours,
                    block_reason: task.block_reason.clone(),
                    start_time: task.start_time,
                    end_time: task.end_time,
                };

                if status_str == "blocked" {
                    report.blocked_count += 1;
                    report.blocked_tasks.push(entry.clone());
                }

                all_entries.push(entry);
            }
        }

        all_entries.sort_by(|a, b| {
            b.duration_hours
                .partial_cmp(&a.duration_hours)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        report.slowest_tasks = all_entries.into_iter().take(top_n).collect();

        report.blocked_tasks.sort_by(|a, b| {
            b.duration_hours
                .partial_cmp(&a.duration_hours)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(report)
    }
}

impl Entity for BottleneckReport {
    fn entity_type() -> &'static str {
        "bottleneck_report"
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn agent(&self) -> &str {
        &self.agent
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.computed_at
    }

    fn validate_entity(&self) -> crate::Result<()> {
        if let Err(errors) = <BottleneckReport as validator::Validate>::validate(self) {
            let msgs: Vec<String> = errors
                .field_errors()
                .values()
                .flat_map(|fe| fe.iter())
                .map(|e| e.message.clone().unwrap_or_default().to_string())
                .collect();
            return Err(crate::EngramError::Validation(msgs.join(", ")));
        }
        Ok(())
    }

    fn to_generic(&self) -> GenericEntity {
        GenericEntity {
            id: self.id.clone(),
            entity_type: Self::entity_type().to_string(),
            agent: self.agent.clone(),
            timestamp: self.computed_at,
            data: serde_json::to_value(self).unwrap_or_default(),
        }
    }

    fn from_generic(entity: GenericEntity) -> crate::Result<Self> {
        serde_json::from_value(entity.data).map_err(|e| {
            crate::EngramError::Deserialization(format!(
                "Failed to deserialize BottleneckReport: {}",
                e
            ))
        })
    }

    fn as_any(&self) -> &dyn std::any::Any
    where
        Self: Sized,
    {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::task::{Task, TaskPriority, TaskStatus};
    use crate::storage::{GitCommit, QueryFilter, QueryResult, Storage, StorageStats};
    use serde_json::Value;
    use std::collections::HashMap as StdHashMap;

    struct MockStorage {
        tasks: Vec<Task>,
    }

    impl Storage for MockStorage {
        fn get(
            &self,
            _: &str,
            _: &str,
        ) -> Result<Option<crate::entities::GenericEntity>, crate::EngramError> {
            Ok(None)
        }
        fn store(&mut self, _: &crate::entities::GenericEntity) -> Result<(), crate::EngramError> {
            Ok(())
        }
        fn delete(&mut self, _: &str, _: &str) -> Result<(), crate::EngramError> {
            Ok(())
        }
        fn query(&self, _: &QueryFilter) -> Result<QueryResult, crate::EngramError> {
            Ok(QueryResult {
                entities: vec![],
                total_count: 0,
                has_more: false,
            })
        }
        fn query_by_agent(
            &self,
            _: &str,
            _: Option<&str>,
        ) -> Result<Vec<crate::entities::GenericEntity>, crate::EngramError> {
            Ok(vec![])
        }
        fn query_by_time_range(
            &self,
            _: chrono::DateTime<chrono::Utc>,
            _: chrono::DateTime<chrono::Utc>,
        ) -> Result<Vec<crate::entities::GenericEntity>, crate::EngramError> {
            Ok(vec![])
        }
        fn query_by_type(
            &self,
            _: &str,
            _: Option<&StdHashMap<String, Value>>,
            _: Option<usize>,
            _: Option<usize>,
        ) -> Result<QueryResult, crate::EngramError> {
            Ok(QueryResult {
                entities: vec![],
                total_count: 0,
                has_more: false,
            })
        }
        fn text_search(
            &self,
            _: &str,
            _: Option<&[String]>,
            _: Option<usize>,
        ) -> Result<Vec<crate::entities::GenericEntity>, crate::EngramError> {
            Ok(vec![])
        }
        fn count(&self, _: &QueryFilter) -> Result<usize, crate::EngramError> {
            Ok(0)
        }
        fn list_ids(&self, _: &str) -> Result<Vec<String>, crate::EngramError> {
            Ok(vec![])
        }
        fn get_all(
            &self,
            _: &str,
        ) -> Result<Vec<crate::entities::GenericEntity>, crate::EngramError> {
            Ok(self.tasks.iter().map(|t| t.to_generic()).collect())
        }
        fn sync(&mut self) -> Result<(), crate::EngramError> {
            Ok(())
        }
        fn current_branch(&self) -> Result<String, crate::EngramError> {
            Ok("main".to_string())
        }
        fn create_branch(&mut self, _: &str) -> Result<(), crate::EngramError> {
            Ok(())
        }
        fn switch_branch(&mut self, _: &str) -> Result<(), crate::EngramError> {
            Ok(())
        }
        fn merge_branches(&mut self, _: &str, _: &str) -> Result<(), crate::EngramError> {
            Ok(())
        }
        fn history(&self, _: Option<usize>) -> Result<Vec<GitCommit>, crate::EngramError> {
            Ok(vec![])
        }
        fn bulk_store(
            &mut self,
            _: &[crate::entities::GenericEntity],
        ) -> Result<(), crate::EngramError> {
            Ok(())
        }
        fn get_stats(&self) -> Result<StorageStats, crate::EngramError> {
            Ok(StorageStats::default())
        }
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    fn make_task(
        id: &str,
        status: TaskStatus,
        start: DateTime<Utc>,
        end: Option<DateTime<Utc>>,
    ) -> Task {
        let block_reason = if status == TaskStatus::Blocked {
            Some("waiting".to_string())
        } else {
            None
        };
        Task {
            id: id.to_string(),
            title: format!("Task {}", id),
            description: "desc".to_string(),
            status,
            priority: TaskPriority::Medium,
            agent: "test-agent".to_string(),
            start_time: start,
            end_time: end,
            parent: None,
            children: vec![],
            context_ids: vec![],
            knowledge: vec![],
            files: vec![],
            outcome: None,
            workflow_id: None,
            workflow_state: None,
            block_reason,
            tags: vec![],
            metadata: HashMap::new(),
        }
    }

    #[test]
    fn test_bottleneck_report_new() {
        let report = BottleneckReport::new("/tmp".to_string(), "agent".to_string());
        assert_eq!(report.project_path, "/tmp");
        assert!(report.slowest_tasks.is_empty());
        assert!(report.blocked_tasks.is_empty());
        assert!(report.validate_entity().is_ok());
    }

    #[test]
    fn test_bottleneck_report_entity_trait() {
        let report = BottleneckReport::new("/tmp".to_string(), "agent".to_string());
        assert_eq!(BottleneckReport::entity_type(), "bottleneck_report");

        let generic = report.to_generic();
        assert_eq!(generic.entity_type, "bottleneck_report");

        let restored = BottleneckReport::from_generic(generic).unwrap();
        assert_eq!(restored.project_path, "/tmp");
    }

    #[test]
    fn test_bottleneck_entry_serialization() {
        let entry = BottleneckEntry {
            task_id: "test-id".to_string(),
            title: "Test".to_string(),
            status: "blocked".to_string(),
            agent: "agent".to_string(),
            duration_hours: 48.0,
            block_reason: Some("waiting".to_string()),
            start_time: Utc::now(),
            end_time: None,
        };
        let json = serde_json::to_string(&entry).unwrap();
        let restored: BottleneckEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.block_reason, Some("waiting".to_string()));
        assert!((restored.duration_hours - 48.0).abs() < 0.001);
    }

    #[test]
    fn test_compute_no_tasks() {
        let storage = MockStorage { tasks: vec![] };
        let report =
            BottleneckReport::compute(&storage, std::path::Path::new("/repo"), "agent", 5).unwrap();
        assert_eq!(report.total_analyzed, 0);
        assert!(report.slowest_tasks.is_empty());
        assert!(report.blocked_tasks.is_empty());
        assert_eq!(report.blocked_count, 0);
    }

    #[test]
    fn test_compute_single_task() {
        let start = Utc::now() - chrono::Duration::hours(2);
        let end = Some(Utc::now());
        let storage = MockStorage {
            tasks: vec![make_task("t1", TaskStatus::Done, start, end)],
        };
        let report =
            BottleneckReport::compute(&storage, std::path::Path::new("/repo"), "agent", 5).unwrap();
        assert_eq!(report.total_analyzed, 1);
        assert_eq!(report.slowest_tasks.len(), 1);
        assert_eq!(report.slowest_tasks[0].task_id, "t1");
        assert!((report.slowest_tasks[0].duration_hours - 2.0).abs() < 0.01);
        assert_eq!(report.blocked_count, 0);
    }

    #[test]
    fn test_compute_multiple_tasks_ordering() {
        let base = Utc::now();
        let t1 = make_task(
            "fast",
            TaskStatus::Done,
            base - chrono::Duration::hours(1),
            Some(base),
        );
        let t2 = make_task(
            "medium",
            TaskStatus::Done,
            base - chrono::Duration::hours(5),
            Some(base),
        );
        let t3 = make_task(
            "slow",
            TaskStatus::Done,
            base - chrono::Duration::hours(10),
            Some(base),
        );
        let storage = MockStorage {
            tasks: vec![t1, t2, t3],
        };
        let report =
            BottleneckReport::compute(&storage, std::path::Path::new("/repo"), "agent", 5).unwrap();
        assert_eq!(report.slowest_tasks.len(), 3);
        assert_eq!(report.slowest_tasks[0].task_id, "slow");
        assert_eq!(report.slowest_tasks[1].task_id, "medium");
        assert_eq!(report.slowest_tasks[2].task_id, "fast");
    }

    #[test]
    fn test_compute_top_n_limit() {
        let base = Utc::now();
        let tasks: Vec<Task> = (0..5)
            .map(|i| {
                make_task(
                    &format!("t{}", i),
                    TaskStatus::Done,
                    base - chrono::Duration::hours(i as i64 + 1),
                    Some(base),
                )
            })
            .collect();
        let storage = MockStorage { tasks };
        let report =
            BottleneckReport::compute(&storage, std::path::Path::new("/repo"), "agent", 2).unwrap();
        assert_eq!(report.total_analyzed, 5);
        assert_eq!(report.slowest_tasks.len(), 2);
        assert_eq!(report.slowest_tasks[0].task_id, "t4");
        assert_eq!(report.slowest_tasks[1].task_id, "t3");
    }

    #[test]
    fn test_compute_blocked_tasks_identification() {
        let base = Utc::now();
        let blocked = make_task(
            "b1",
            TaskStatus::Blocked,
            base - chrono::Duration::hours(3),
            None,
        );
        let done = make_task(
            "d1",
            TaskStatus::Done,
            base - chrono::Duration::hours(1),
            Some(base),
        );
        let storage = MockStorage {
            tasks: vec![blocked, done],
        };
        let report =
            BottleneckReport::compute(&storage, std::path::Path::new("/repo"), "agent", 5).unwrap();
        assert_eq!(report.blocked_count, 1);
        assert_eq!(report.blocked_tasks.len(), 1);
        assert_eq!(report.blocked_tasks[0].task_id, "b1");
        assert_eq!(
            report.blocked_tasks[0].block_reason,
            Some("waiting".to_string())
        );
    }

    #[test]
    fn test_compute_incomplete_task_uses_now() {
        let start = Utc::now() - chrono::Duration::hours(4);
        let storage = MockStorage {
            tasks: vec![make_task("t1", TaskStatus::InProgress, start, None)],
        };
        let report =
            BottleneckReport::compute(&storage, std::path::Path::new("/repo"), "agent", 5).unwrap();
        assert_eq!(report.total_analyzed, 1);
        assert!((report.slowest_tasks[0].duration_hours - 4.0).abs() < 0.01);
    }

    #[test]
    fn test_compute_all_same_duration() {
        let base = Utc::now();
        let t1 = make_task(
            "a",
            TaskStatus::Done,
            base - chrono::Duration::hours(3),
            Some(base),
        );
        let t2 = make_task(
            "b",
            TaskStatus::Done,
            base - chrono::Duration::hours(3),
            Some(base),
        );
        let t3 = make_task(
            "c",
            TaskStatus::Done,
            base - chrono::Duration::hours(3),
            Some(base),
        );
        let storage = MockStorage {
            tasks: vec![t1, t2, t3],
        };
        let report =
            BottleneckReport::compute(&storage, std::path::Path::new("/repo"), "agent", 5).unwrap();
        assert_eq!(report.slowest_tasks.len(), 3);
        for entry in &report.slowest_tasks {
            assert!((entry.duration_hours - 3.0).abs() < 0.01);
        }
    }

    #[test]
    fn test_compute_zero_duration_task() {
        let now = Utc::now();
        let storage = MockStorage {
            tasks: vec![make_task("t1", TaskStatus::Done, now, Some(now))],
        };
        let report =
            BottleneckReport::compute(&storage, std::path::Path::new("/repo"), "agent", 5).unwrap();
        assert_eq!(report.slowest_tasks.len(), 1);
        assert!((report.slowest_tasks[0].duration_hours).abs() < 0.001);
    }

    #[test]
    fn test_compute_blocked_tasks_sorted_longest_first() {
        let base = Utc::now();
        let b1 = make_task(
            "b1",
            TaskStatus::Blocked,
            base - chrono::Duration::hours(1),
            None,
        );
        let b2 = make_task(
            "b2",
            TaskStatus::Blocked,
            base - chrono::Duration::hours(8),
            None,
        );
        let storage = MockStorage {
            tasks: vec![b1, b2],
        };
        let report =
            BottleneckReport::compute(&storage, std::path::Path::new("/repo"), "agent", 5).unwrap();
        assert_eq!(report.blocked_tasks.len(), 2);
        assert_eq!(report.blocked_tasks[0].task_id, "b2");
        assert_eq!(report.blocked_tasks[1].task_id, "b1");
    }
}
