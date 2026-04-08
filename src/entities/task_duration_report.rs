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
}
