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
}
