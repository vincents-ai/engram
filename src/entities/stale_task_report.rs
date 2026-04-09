use crate::entities::{Entity, Task, TaskStatus};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaleTaskEntry {
    #[serde(rename = "task_id")]
    pub task_id: String,

    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "agent")]
    pub agent: String,

    #[serde(rename = "status")]
    pub status: String,

    #[serde(rename = "start_time")]
    pub start_time: DateTime<Utc>,

    #[serde(rename = "age_hours")]
    pub age_hours: f64,

    #[serde(rename = "last_git_commit")]
    pub last_git_commit: Option<DateTime<Utc>>,

    #[serde(rename = "hours_since_last_commit")]
    pub hours_since_last_commit: Option<f64>,

    #[serde(rename = "staleness_reason")]
    pub staleness_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaleTaskReport {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "computed_at")]
    pub computed_at: DateTime<Utc>,

    #[serde(rename = "stale_threshold_hours")]
    pub stale_threshold_hours: i64,

    #[serde(rename = "total_in_progress")]
    pub total_in_progress: usize,

    #[serde(rename = "total_stale")]
    pub total_stale: usize,

    #[serde(rename = "stale_tasks")]
    pub stale_tasks: Vec<StaleTaskEntry>,

    #[serde(rename = "metadata")]
    pub metadata: HashMap<String, serde_json::Value>,
}

impl StaleTaskReport {
    pub fn new(stale_threshold_hours: i64) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            computed_at: Utc::now(),
            stale_threshold_hours,
            total_in_progress: 0,
            total_stale: 0,
            stale_tasks: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn detect<S: crate::storage::Storage>(
        storage: &S,
        stale_threshold_hours: i64,
    ) -> crate::Result<Self> {
        let mut report = Self::new(stale_threshold_hours);
        let generics = storage.get_all("task")?;
        let now = Utc::now();
        let threshold = chrono::Duration::hours(stale_threshold_hours);

        for generic in &generics {
            if let Ok(task) = Task::from_generic(generic.clone()) {
                if task.status != TaskStatus::InProgress {
                    continue;
                }

                report.total_in_progress += 1;

                let age = now.signed_duration_since(task.start_time);
                let age_hours = age.num_hours() as f64 + (age.num_minutes() % 60) as f64 / 60.0;

                let last_commit = find_last_commit_for_task(&task.id);
                let hours_since_commit = last_commit.map(|t| {
                    let delta = now.signed_duration_since(t);
                    delta.num_hours() as f64 + (delta.num_minutes() % 60) as f64 / 60.0
                });

                let is_stale_by_age = age > threshold;
                let is_stale_by_git = hours_since_commit
                    .map(|h| h > stale_threshold_hours as f64)
                    .unwrap_or(age > threshold);

                if is_stale_by_age || is_stale_by_git {
                    report.total_stale += 1;

                    let reason = if is_stale_by_git {
                        match hours_since_commit {
                            Some(h) => format!(
                                "No git activity in {:.1}h (threshold: {}h)",
                                h, stale_threshold_hours
                            ),
                            None => format!(
                                "No git commits found; task age {:.1}h (threshold: {}h)",
                                age_hours, stale_threshold_hours
                            ),
                        }
                    } else {
                        format!(
                            "Task age {:.1}h exceeds threshold {}h",
                            age_hours, stale_threshold_hours
                        )
                    };

                    report.stale_tasks.push(StaleTaskEntry {
                        task_id: task.id.clone(),
                        title: task.title.clone(),
                        agent: task.agent.clone(),
                        status: "in_progress".to_string(),
                        start_time: task.start_time,
                        age_hours,
                        last_git_commit: last_commit,
                        hours_since_last_commit: hours_since_commit,
                        staleness_reason: reason,
                    });
                }
            }
        }

        report.stale_tasks.sort_by(|a, b| {
            b.age_hours
                .partial_cmp(&a.age_hours)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(report)
    }
}

fn find_last_commit_for_task(task_id: &str) -> Option<DateTime<Utc>> {
    let short_id = &task_id[..8];
    let full_id = task_id;

    let output = std::process::Command::new("git")
        .args(["log", "--all", "--format=%aI", &format!("[{}]", short_id)])
        .output()
        .ok()?;

    if !output.status.success() || output.stdout.is_empty() {
        let output = std::process::Command::new("git")
            .args(["log", "--all", "--format=%aI", &format!("[{}]", full_id)])
            .output()
            .ok()?;

        if !output.status.success() || output.stdout.is_empty() {
            return None;
        }

        let line = std::str::from_utf8(&output.stdout).ok()?.lines().next()?;
        DateTime::parse_from_rfc3339(line)
            .ok()
            .map(|dt| dt.with_timezone(&Utc))
    } else {
        let line = std::str::from_utf8(&output.stdout).ok()?.lines().next()?;
        DateTime::parse_from_rfc3339(line)
            .ok()
            .map(|dt| dt.with_timezone(&Utc))
    }
}

impl crate::feedback::StructuredFeedback for StaleTaskReport {
    fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }

    fn summary(&self) -> String {
        if self.total_stale == 0 {
            format!(
                "Health OK: {}/{} in-progress tasks are active (threshold: {}h)",
                self.total_in_progress, self.total_in_progress, self.stale_threshold_hours
            )
        } else {
            format!(
                "Health warning: {}/{} in-progress tasks are stale (threshold: {}h)",
                self.total_stale, self.total_in_progress, self.stale_threshold_hours
            )
        }
    }

    fn status_code(&self) -> crate::feedback::FeedbackStatus {
        if self.total_stale == 0 {
            crate::feedback::FeedbackStatus::Success
        } else {
            crate::feedback::FeedbackStatus::Warning
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::feedback::StructuredFeedback;

    #[test]
    fn test_stale_task_report_new() {
        let report = StaleTaskReport::new(24);
        assert_eq!(report.stale_threshold_hours, 24);
        assert_eq!(report.total_stale, 0);
        assert!(report.stale_tasks.is_empty());
    }

    #[test]
    fn test_stale_task_entry_serialization() {
        let entry = StaleTaskEntry {
            task_id: "test-id".to_string(),
            title: "Test Task".to_string(),
            agent: "agent".to_string(),
            status: "in_progress".to_string(),
            start_time: Utc::now(),
            age_hours: 48.5,
            last_git_commit: None,
            hours_since_last_commit: None,
            staleness_reason: "Task age 48.5h exceeds threshold 24h".to_string(),
        };
        let json = serde_json::to_string(&entry).unwrap();
        let restored: StaleTaskEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.task_id, "test-id");
        assert!((restored.age_hours - 48.5).abs() < 0.001);
    }

    #[test]
    fn test_stale_task_report_feedback_no_stale() {
        let report = StaleTaskReport::new(24);
        assert_eq!(
            report.status_code(),
            crate::feedback::FeedbackStatus::Success
        );
        assert!(report.summary().contains("Health OK"));
    }

    #[test]
    fn test_stale_task_report_feedback_with_stale() {
        let mut report = StaleTaskReport::new(24);
        report.total_in_progress = 3;
        report.total_stale = 1;
        assert_eq!(
            report.status_code(),
            crate::feedback::FeedbackStatus::Warning
        );
        assert!(report.summary().contains("Health warning"));
        assert!(report.summary().contains("1/3"));
    }

    #[test]
    fn test_stale_task_report_entity_trait() {
        let report = StaleTaskReport::new(24);
        let json = serde_json::to_string(&report).unwrap();
        let restored: StaleTaskReport = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.stale_threshold_hours, 24);
    }

    #[test]
    fn test_find_last_commit_for_task_no_git() {
        let result = find_last_commit_for_task("nonexistent-task-id-that-does-not-exist");
        assert!(result.is_none());
    }
}
