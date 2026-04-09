use super::{Entity, GenericEntity};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct TaskDurationReport {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "project_path")]
    pub project_path: String,

    #[serde(rename = "computed_at")]
    pub computed_at: DateTime<Utc>,

    #[serde(rename = "agent")]
    pub agent: String,

    #[serde(rename = "total_tasks_analyzed")]
    pub total_tasks_analyzed: u64,

    #[serde(rename = "completed_tasks")]
    pub completed_tasks: u64,

    #[serde(
        rename = "task_durations",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub task_durations: Vec<TaskDurationEntry>,

    #[serde(rename = "median_duration_hours")]
    pub median_duration_hours: f64,

    #[serde(rename = "mean_duration_hours")]
    pub mean_duration_hours: f64,

    #[serde(rename = "min_duration_hours")]
    pub min_duration_hours: f64,

    #[serde(rename = "max_duration_hours")]
    pub max_duration_hours: f64,

    #[serde(
        rename = "metadata",
        skip_serializing_if = "HashMap::is_empty",
        default
    )]
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDurationEntry {
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

    #[serde(rename = "start_time")]
    pub start_time: DateTime<Utc>,

    #[serde(rename = "end_time")]
    pub end_time: Option<DateTime<Utc>>,
}

impl TaskDurationReport {
    pub fn new(project_path: String, agent: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            project_path,
            computed_at: Utc::now(),
            agent,
            total_tasks_analyzed: 0,
            completed_tasks: 0,
            task_durations: Vec::new(),
            median_duration_hours: 0.0,
            mean_duration_hours: 0.0,
            min_duration_hours: 0.0,
            max_duration_hours: 0.0,
            metadata: HashMap::new(),
        }
    }

    pub fn compute<S: crate::storage::Storage>(
        storage: &S,
        repo_path: &std::path::Path,
        agent: &str,
    ) -> crate::Result<Self> {
        let mut report =
            TaskDurationReport::new(repo_path.to_string_lossy().to_string(), agent.to_string());

        let generics = storage.get_all("task")?;

        let mut durations: Vec<f64> = Vec::new();

        for generic in &generics {
            if let Ok(task) = super::Task::from_generic(generic.clone()) {
                report.total_tasks_analyzed += 1;

                let duration_hours = if let Some(end) = task.end_time {
                    let secs = end.signed_duration_since(task.start_time).num_seconds();
                    secs as f64 / 3600.0
                } else {
                    let secs = Utc::now()
                        .signed_duration_since(task.start_time)
                        .num_seconds();
                    secs as f64 / 3600.0
                };

                let status_str = format!("{:?}", task.status).to_lowercase();

                report.task_durations.push(TaskDurationEntry {
                    task_id: task.id.clone(),
                    title: task.title.clone(),
                    status: status_str.clone(),
                    agent: task.agent.clone(),
                    duration_hours,
                    start_time: task.start_time,
                    end_time: task.end_time,
                });

                if status_str == "done" {
                    report.completed_tasks += 1;
                    durations.push(duration_hours);
                }
            }
        }

        report.task_durations.sort_by(|a, b| {
            b.duration_hours
                .partial_cmp(&a.duration_hours)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if !durations.is_empty() {
            durations.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

            let sum: f64 = durations.iter().sum();
            report.mean_duration_hours = sum / durations.len() as f64;
            report.min_duration_hours = durations[0];
            report.max_duration_hours = durations[durations.len() - 1];

            let mid = durations.len() / 2;
            report.median_duration_hours = if durations.len() % 2 == 0 {
                (durations[mid - 1] + durations[mid]) / 2.0
            } else {
                durations[mid]
            };
        }

        Ok(report)
    }
}

impl Entity for TaskDurationReport {
    fn entity_type() -> &'static str {
        "task_duration_report"
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
        if let Err(errors) = <TaskDurationReport as validator::Validate>::validate(self) {
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
                "Failed to deserialize TaskDurationReport: {}",
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
            block_reason: None,
            tags: vec![],
            metadata: HashMap::new(),
        }
    }

    #[test]
    fn test_task_duration_report_new() {
        let report = TaskDurationReport::new("/tmp".to_string(), "agent".to_string());
        assert_eq!(report.project_path, "/tmp");
        assert!(report.task_durations.is_empty());
        assert!(report.validate_entity().is_ok());
    }

    #[test]
    fn test_task_duration_report_entity_trait() {
        let report = TaskDurationReport::new("/tmp".to_string(), "agent".to_string());
        assert_eq!(TaskDurationReport::entity_type(), "task_duration_report");

        let generic = report.to_generic();
        assert_eq!(generic.entity_type, "task_duration_report");

        let restored = TaskDurationReport::from_generic(generic).unwrap();
        assert_eq!(restored.project_path, "/tmp");
    }

    #[test]
    fn test_task_duration_entry_serialization() {
        let entry = TaskDurationEntry {
            task_id: "test-id".to_string(),
            title: "Test".to_string(),
            status: "done".to_string(),
            agent: "agent".to_string(),
            duration_hours: 2.5,
            start_time: Utc::now(),
            end_time: Some(Utc::now()),
        };
        let json = serde_json::to_string(&entry).unwrap();
        let restored: TaskDurationEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.task_id, "test-id");
        assert!((restored.duration_hours - 2.5).abs() < 0.001);
    }

    #[test]
    fn test_compute_empty_task_list() {
        let storage = MockStorage { tasks: vec![] };
        let report =
            TaskDurationReport::compute(&storage, std::path::Path::new("/repo"), "agent").unwrap();
        assert_eq!(report.total_tasks_analyzed, 0);
        assert_eq!(report.completed_tasks, 0);
        assert!(report.task_durations.is_empty());
        assert_eq!(report.median_duration_hours, 0.0);
        assert_eq!(report.mean_duration_hours, 0.0);
        assert_eq!(report.min_duration_hours, 0.0);
        assert_eq!(report.max_duration_hours, 0.0);
    }

    #[test]
    fn test_compute_single_completed_task() {
        let start = Utc::now() - chrono::Duration::hours(3);
        let end = Some(Utc::now());
        let storage = MockStorage {
            tasks: vec![make_task("t1", TaskStatus::Done, start, end)],
        };
        let report =
            TaskDurationReport::compute(&storage, std::path::Path::new("/repo"), "agent").unwrap();
        assert_eq!(report.total_tasks_analyzed, 1);
        assert_eq!(report.completed_tasks, 1);
        assert!((report.mean_duration_hours - 3.0).abs() < 0.01);
        assert!((report.median_duration_hours - 3.0).abs() < 0.01);
        assert!((report.min_duration_hours - 3.0).abs() < 0.01);
        assert!((report.max_duration_hours - 3.0).abs() < 0.01);
    }

    #[test]
    fn test_compute_multiple_tasks_varying_durations() {
        let base = Utc::now();
        let t1 = make_task(
            "t1",
            TaskStatus::Done,
            base - chrono::Duration::hours(1),
            Some(base),
        );
        let t2 = make_task(
            "t2",
            TaskStatus::Done,
            base - chrono::Duration::hours(3),
            Some(base),
        );
        let t3 = make_task(
            "t3",
            TaskStatus::Done,
            base - chrono::Duration::hours(7),
            Some(base),
        );
        let storage = MockStorage {
            tasks: vec![t1, t2, t3],
        };
        let report =
            TaskDurationReport::compute(&storage, std::path::Path::new("/repo"), "agent").unwrap();
        assert_eq!(report.total_tasks_analyzed, 3);
        assert_eq!(report.completed_tasks, 3);
        assert!((report.mean_duration_hours - (1.0 + 3.0 + 7.0) / 3.0).abs() < 0.01);
        assert!((report.median_duration_hours - 3.0).abs() < 0.01);
        assert!((report.min_duration_hours - 1.0).abs() < 0.01);
        assert!((report.max_duration_hours - 7.0).abs() < 0.01);
    }

    #[test]
    fn test_compute_median_even_count() {
        let base = Utc::now();
        let t1 = make_task(
            "t1",
            TaskStatus::Done,
            base - chrono::Duration::hours(2),
            Some(base),
        );
        let t2 = make_task(
            "t2",
            TaskStatus::Done,
            base - chrono::Duration::hours(4),
            Some(base),
        );
        let t3 = make_task(
            "t3",
            TaskStatus::Done,
            base - chrono::Duration::hours(6),
            Some(base),
        );
        let t4 = make_task(
            "t4",
            TaskStatus::Done,
            base - chrono::Duration::hours(8),
            Some(base),
        );
        let storage = MockStorage {
            tasks: vec![t1, t2, t3, t4],
        };
        let report =
            TaskDurationReport::compute(&storage, std::path::Path::new("/repo"), "agent").unwrap();
        let expected_median = (4.0 + 6.0) / 2.0;
        assert!((report.median_duration_hours - expected_median).abs() < 0.01);
    }

    #[test]
    fn test_compute_incomplete_tasks_not_counted_in_stats() {
        let base = Utc::now();
        let done = make_task(
            "d1",
            TaskStatus::Done,
            base - chrono::Duration::hours(5),
            Some(base),
        );
        let in_progress = make_task(
            "ip1",
            TaskStatus::InProgress,
            base - chrono::Duration::hours(10),
            None,
        );
        let storage = MockStorage {
            tasks: vec![done, in_progress],
        };
        let report =
            TaskDurationReport::compute(&storage, std::path::Path::new("/repo"), "agent").unwrap();
        assert_eq!(report.total_tasks_analyzed, 2);
        assert_eq!(report.completed_tasks, 1);
        assert_eq!(report.task_durations.len(), 2);
        assert!((report.mean_duration_hours - 5.0).abs() < 0.01);
        assert!((report.max_duration_hours - 5.0).abs() < 0.01);
    }

    #[test]
    fn test_compute_tasks_sorted_by_duration_descending() {
        let base = Utc::now();
        let t1 = make_task(
            "t1",
            TaskStatus::Done,
            base - chrono::Duration::hours(1),
            Some(base),
        );
        let t2 = make_task(
            "t2",
            TaskStatus::Done,
            base - chrono::Duration::hours(5),
            Some(base),
        );
        let t3 = make_task(
            "t3",
            TaskStatus::Done,
            base - chrono::Duration::hours(3),
            Some(base),
        );
        let storage = MockStorage {
            tasks: vec![t1, t2, t3],
        };
        let report =
            TaskDurationReport::compute(&storage, std::path::Path::new("/repo"), "agent").unwrap();
        assert!(report.task_durations[0].duration_hours >= report.task_durations[1].duration_hours);
        assert!(report.task_durations[1].duration_hours >= report.task_durations[2].duration_hours);
    }

    #[test]
    fn test_compute_incomplete_task_uses_now_for_duration() {
        let start = Utc::now() - chrono::Duration::hours(6);
        let storage = MockStorage {
            tasks: vec![make_task("t1", TaskStatus::InProgress, start, None)],
        };
        let report =
            TaskDurationReport::compute(&storage, std::path::Path::new("/repo"), "agent").unwrap();
        assert_eq!(report.total_tasks_analyzed, 1);
        assert_eq!(report.completed_tasks, 0);
        assert_eq!(report.task_durations.len(), 1);
        assert!((report.task_durations[0].duration_hours - 6.0).abs() < 0.01);
        assert_eq!(report.mean_duration_hours, 0.0);
    }

    #[test]
    fn test_compute_zero_duration_completed_task() {
        let now = Utc::now();
        let storage = MockStorage {
            tasks: vec![make_task("t1", TaskStatus::Done, now, Some(now))],
        };
        let report =
            TaskDurationReport::compute(&storage, std::path::Path::new("/repo"), "agent").unwrap();
        assert_eq!(report.completed_tasks, 1);
        assert!((report.min_duration_hours).abs() < 0.001);
        assert!((report.max_duration_hours).abs() < 0.001);
        assert!((report.mean_duration_hours).abs() < 0.001);
    }

    #[test]
    fn test_compute_single_task_entry_has_correct_fields() {
        let start = Utc::now() - chrono::Duration::hours(2);
        let end = Some(Utc::now());
        let storage = MockStorage {
            tasks: vec![make_task("abc", TaskStatus::Done, start, end)],
        };
        let report =
            TaskDurationReport::compute(&storage, std::path::Path::new("/repo"), "agent").unwrap();
        let entry = &report.task_durations[0];
        assert_eq!(entry.task_id, "abc");
        assert_eq!(entry.title, "Task abc");
        assert_eq!(entry.status, "done");
        assert_eq!(entry.agent, "test-agent");
    }
}
