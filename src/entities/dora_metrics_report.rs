//! DoraMetricsReport entity - persisted DORA metrics computation results
//!
//! Stores computed DORA metrics (Deployment Frequency, Lead Time for Changes,
//! Change Failure Rate, Mean Time to Recovery) as a first-class engram entity
//! so results persist and sync via GitRefsStorage.

use super::{Entity, GenericEntity};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

/// Persisted DORA metrics computation result
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct DoraMetricsReport {
    /// Unique identifier
    #[serde(rename = "id")]
    pub id: String,

    /// Project/repo path this report covers
    #[serde(rename = "project_path")]
    pub project_path: String,

    /// Computation timestamp
    #[serde(rename = "computed_at")]
    pub computed_at: DateTime<Utc>,

    /// Time window start
    #[serde(rename = "window_start")]
    pub window_start: DateTime<Utc>,

    /// Time window end
    #[serde(rename = "window_end")]
    pub window_end: DateTime<Utc>,

    /// Number of commits analyzed
    #[serde(rename = "commits_analyzed")]
    pub commits_analyzed: u64,

    /// Number of execution results analyzed
    #[serde(rename = "executions_analyzed")]
    pub executions_analyzed: u64,

    /// Number of escalation requests analyzed
    #[serde(rename = "escalations_analyzed")]
    pub escalations_analyzed: u64,

    /// Deployment frequency (deployments per week)
    #[serde(rename = "deployment_frequency")]
    pub deployment_frequency: f64,

    /// Lead time for changes (in days)
    #[serde(rename = "lead_time_for_changes")]
    pub lead_time_for_changes: f64,

    /// Change failure rate (0.0 - 1.0)
    #[serde(rename = "change_failure_rate")]
    pub change_failure_rate: f64,

    /// Mean time to recovery (in hours)
    #[serde(rename = "mean_time_to_recovery")]
    pub mean_time_to_recovery: f64,

    /// Associated agent
    #[serde(rename = "agent")]
    pub agent: String,

    /// Additional metadata
    #[serde(
        rename = "metadata",
        skip_serializing_if = "HashMap::is_empty",
        default
    )]
    pub metadata: HashMap<String, serde_json::Value>,
}

impl DoraMetricsReport {
    pub fn new(project_path: String, agent: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            project_path,
            computed_at: now,
            window_start: now,
            window_end: now,
            commits_analyzed: 0,
            executions_analyzed: 0,
            escalations_analyzed: 0,
            deployment_frequency: 0.0,
            lead_time_for_changes: 0.0,
            change_failure_rate: 0.0,
            mean_time_to_recovery: 0.0,
            agent,
            metadata: HashMap::new(),
        }
    }

    /// Convert to the session's DoraMetrics struct for backward compat
    pub fn to_session_dora_metrics(&self) -> super::session::DoraMetrics {
        super::session::DoraMetrics {
            deployment_frequency: self.deployment_frequency,
            lead_time: self.lead_time_for_changes,
            change_failure_rate: self.change_failure_rate * 100.0,
            mean_time_to_recover: self.mean_time_to_recovery,
        }
    }
}

impl Entity for DoraMetricsReport {
    fn entity_type() -> &'static str {
        "dora_metrics_report"
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
        if let Err(errors) = <DoraMetricsReport as validator::Validate>::validate(self) {
            let error_messages: Vec<String> = errors
                .field_errors()
                .values()
                .flat_map(|field_errors| field_errors.iter())
                .map(|error| {
                    error
                        .message
                        .clone()
                        .map(|s| s.to_string())
                        .unwrap_or_default()
                })
                .collect();
            return Err(crate::EngramError::Validation(error_messages.join(", ")));
        }

        if self.project_path.is_empty() {
            return Err(crate::EngramError::Validation(
                "project_path cannot be empty".to_string(),
            ));
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
                "Failed to deserialize DoraMetricsReport: {}",
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

/// Computes DORA metrics from git history and engram entities.
pub struct DoraMetricsCalculator;

impl DoraMetricsCalculator {
    /// Compute all DORA metrics for a repository path.
    ///
    /// - **Deployment Frequency**: commits per week over the time window
    /// - **Lead Time for Changes**: median time between first and last commit per day (days)
    /// - **Change Failure Rate**: fraction of ExecutionResult entities with Failed status
    /// - **MTTR**: mean time from creation to resolution of EscalationRequest entities (hours)
    pub fn compute<S: crate::storage::Storage>(
        storage: &S,
        repo_path: &std::path::Path,
        agent: &str,
        window_days: i64,
    ) -> crate::Result<DoraMetricsReport> {
        let now = Utc::now();
        let window_start = now - chrono::Duration::days(window_days);

        let mut report =
            DoraMetricsReport::new(repo_path.to_string_lossy().to_string(), agent.to_string());
        report.window_start = window_start;
        report.window_end = now;

        let (dep_freq, lead_time, commits_analyzed) =
            Self::compute_from_git(repo_path, window_start, now)?;

        report.deployment_frequency = dep_freq;
        report.lead_time_for_changes = lead_time;
        report.commits_analyzed = commits_analyzed;

        let (cfr, executions_analyzed) = Self::compute_change_failure_rate(storage, agent)?;

        report.change_failure_rate = cfr;
        report.executions_analyzed = executions_analyzed;

        let (mttr, escalations_analyzed) = Self::compute_mttr_from_entities(storage, agent)?;

        report.mean_time_to_recovery = mttr;
        report.escalations_analyzed = escalations_analyzed;

        Ok(report)
    }

    /// Deployment Frequency and Lead Time from git history.
    fn compute_from_git(
        repo_path: &std::path::Path,
        window_start: DateTime<Utc>,
        window_end: DateTime<Utc>,
    ) -> crate::Result<(f64, f64, u64)> {
        let repo = git2::Repository::discover(repo_path).map_err(|e| {
            crate::EngramError::Git(format!("Failed to open git repo at {:?}: {}", repo_path, e))
        })?;

        let start_time = git2::Time::new(window_start.timestamp(), 0);
        let end_time = git2::Time::new(window_end.timestamp(), 0);

        let mut revwalk = repo
            .revwalk()
            .map_err(|e| crate::EngramError::Git(format!("revwalk failed: {}", e)))?;

        revwalk
            .push_head()
            .map_err(|e| crate::EngramError::Git(format!("push_head failed: {}", e)))?;

        revwalk
            .set_sorting(git2::Sort::TOPOLOGICAL | git2::Sort::REVERSE)
            .map_err(|e| crate::EngramError::Git(format!("set_sorting failed: {}", e)))?;

        let mut commit_timestamps: Vec<i64> = Vec::new();

        for oid_result in revwalk {
            let oid = oid_result
                .map_err(|e| crate::EngramError::Git(format!("revwalk iteration failed: {}", e)))?;
            let commit = repo
                .find_commit(oid)
                .map_err(|e| crate::EngramError::Git(format!("find_commit failed: {}", e)))?;

            let time = commit.time();
            if time.seconds() >= start_time.seconds() && time.seconds() <= end_time.seconds() {
                commit_timestamps.push(time.seconds());
            }
        }

        commit_timestamps.sort();

        let commits_analyzed = commit_timestamps.len() as u64;

        if commits_analyzed == 0 {
            return Ok((0.0, 0.0, 0));
        }

        let window_seconds = (window_end - window_start).num_seconds() as f64;
        let weeks = window_seconds / (7.0 * 24.0 * 3600.0);
        let deployment_frequency = commits_analyzed as f64 / weeks.max(1.0);

        let lead_time = Self::median_commit_span_days(&commit_timestamps);

        Ok((deployment_frequency, lead_time, commits_analyzed))
    }

    /// Median time span per calendar day of commits, in days.
    ///
    /// For each day that has commits, compute the time between the first
    /// and last commit of that day. Return the median of those spans.
    /// This approximates "lead time for changes" — how long a change
    /// sits in progress before being finalized.
    fn median_commit_span_days(timestamps: &[i64]) -> f64 {
        use std::collections::BTreeMap;

        let mut by_day: BTreeMap<i64, Vec<i64>> = BTreeMap::new();
        for &ts in timestamps {
            let day = ts / 86400;
            by_day.entry(day).or_default().push(ts);
        }

        if by_day.is_empty() {
            return 0.0;
        }

        let mut spans: Vec<f64> = by_day
            .values()
            .filter(|ts_list| ts_list.len() >= 2)
            .map(|ts_list| {
                let first = *ts_list.first().unwrap();
                let last = *ts_list.last().unwrap();
                (last - first) as f64 / 86400.0
            })
            .collect();

        if spans.is_empty() {
            return 0.0;
        }

        spans.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mid = spans.len() / 2;
        if spans.len() % 2 == 0 {
            (spans[mid - 1] + spans[mid]) / 2.0
        } else {
            spans[mid]
        }
    }

    /// Change Failure Rate from ExecutionResult entities.
    ///
    /// Returns (rate, total_analyzed). Rate is 0.0-1.0.
    fn compute_change_failure_rate<S: crate::storage::Storage>(
        storage: &S,
        _agent: &str,
    ) -> crate::Result<(f64, u64)> {
        let generics = storage.get_all("execution_result")?;

        let mut total = 0u64;
        let mut failed = 0u64;

        for generic in &generics {
            if let Some(data) = generic.data.get("validation_status") {
                if let Some(status) = data.as_str() {
                    if status == "passed" || status == "failed" {
                        total += 1;
                        if status == "failed" {
                            failed += 1;
                        }
                    }
                }
            }
        }

        let rate = if total > 0 {
            failed as f64 / total as f64
        } else {
            0.0
        };
        Ok((rate, total))
    }

    /// Mean Time to Recovery from EscalationRequest entities.
    ///
    /// Measures the mean time from escalation creation to its resolution
    /// (approved or denied). Returns (mttr_hours, total_analyzed).
    fn compute_mttr_from_entities<S: crate::storage::Storage>(
        storage: &S,
        _agent: &str,
    ) -> crate::Result<(f64, u64)> {
        let generics = storage.get_all("escalation_request")?;

        let mut total_duration_hours: f64 = 0.0;
        let mut count = 0u64;

        for generic in &generics {
            let data = &generic.data;

            let created = data
                .get("created_at")
                .and_then(|v| v.as_str())
                .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&Utc));

            let reviewed = data
                .get("reviewed_at")
                .and_then(|v| v.as_str())
                .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&Utc));

            let status = data.get("status").and_then(|v| v.as_str());

            if let (Some(created_at), Some(reviewed_at)) = (created, reviewed) {
                if status == Some("approved") || status == Some("denied") {
                    let duration = reviewed_at.signed_duration_since(created_at);
                    total_duration_hours += duration.num_seconds() as f64 / 3600.0;
                    count += 1;
                }
            }
        }

        let mttr = if count > 0 {
            total_duration_hours / count as f64
        } else {
            0.0
        };

        Ok((mttr, count))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dora_metrics_report_new() {
        let report = DoraMetricsReport::new("/tmp/project".to_string(), "agent".to_string());
        assert_eq!(report.project_path, "/tmp/project");
        assert_eq!(report.agent, "agent");
        assert_eq!(report.deployment_frequency, 0.0);
        assert_eq!(report.change_failure_rate, 0.0);
        assert!(report.validate_entity().is_ok());
    }

    #[test]
    fn test_dora_metrics_report_entity_trait() {
        let report = DoraMetricsReport::new("/tmp/project".to_string(), "agent".to_string());
        assert_eq!(DoraMetricsReport::entity_type(), "dora_metrics_report");

        let generic = report.to_generic();
        assert_eq!(generic.entity_type, "dora_metrics_report");

        let restored = DoraMetricsReport::from_generic(generic).unwrap();
        assert_eq!(restored.project_path, "/tmp/project");
    }

    #[test]
    fn test_dora_metrics_report_roundtrip_with_values() {
        let mut report = DoraMetricsReport::new("/tmp/project".to_string(), "agent".to_string());
        report.deployment_frequency = 5.2;
        report.lead_time_for_changes = 1.3;
        report.change_failure_rate = 0.15;
        report.mean_time_to_recovery = 4.5;
        report.commits_analyzed = 42;
        report.executions_analyzed = 10;
        report.escalations_analyzed = 3;

        let generic = report.to_generic();
        let restored = DoraMetricsReport::from_generic(generic).unwrap();

        assert_eq!(restored.deployment_frequency, 5.2);
        assert_eq!(restored.lead_time_for_changes, 1.3);
        assert_eq!(restored.change_failure_rate, 0.15);
        assert_eq!(restored.mean_time_to_recovery, 4.5);
        assert_eq!(restored.commits_analyzed, 42);
    }

    #[test]
    fn test_to_session_dora_metrics() {
        let mut report = DoraMetricsReport::new("/tmp".to_string(), "agent".to_string());
        report.deployment_frequency = 5.0;
        report.lead_time_for_changes = 2.0;
        report.change_failure_rate = 0.15;
        report.mean_time_to_recovery = 3.0;

        let session_metrics = report.to_session_dora_metrics();
        assert_eq!(session_metrics.deployment_frequency, 5.0);
        assert_eq!(session_metrics.lead_time, 2.0);
        assert_eq!(session_metrics.change_failure_rate, 15.0);
        assert_eq!(session_metrics.mean_time_to_recover, 3.0);
    }

    #[test]
    fn test_median_commit_span_days_empty() {
        assert_eq!(DoraMetricsCalculator::median_commit_span_days(&[]), 0.0);
    }

    #[test]
    fn test_median_commit_span_days_single_day_single_commit() {
        let ts = vec![100000];
        assert_eq!(DoraMetricsCalculator::median_commit_span_days(&ts), 0.0);
    }

    #[test]
    fn test_median_commit_span_days_single_day_multiple_commits() {
        let base = 86400i64 * 10;
        let ts = vec![base, base + 3600, base + 7200];
        let result = DoraMetricsCalculator::median_commit_span_days(&ts);
        let expected = 7200.0 / 86400.0;
        assert!((result - expected).abs() < 0.001);
    }

    #[test]
    fn test_median_commit_span_days_multiple_days() {
        let ts = vec![
            86400 * 10,         // day 10, 0:00
            86400 * 10 + 7200,  // day 10, 2:00
            86400 * 11,         // day 11, 0:00
            86400 * 11 + 14400, // day 11, 4:00
        ];
        let spans: Vec<f64> = vec![7200.0 / 86400.0, 14400.0 / 86400.0];
        let expected = (spans[0] + spans[1]) / 2.0;
        let result = DoraMetricsCalculator::median_commit_span_days(&ts);
        assert!((result - expected).abs() < 0.001);
    }

    #[test]
    fn test_compute_from_real_repo() {
        let repo_path = std::env::var("CARGO_MANIFEST_DIR")
            .map(|p| std::path::PathBuf::from(p))
            .unwrap_or_else(|_| std::path::PathBuf::from("."));

        let result = DoraMetricsCalculator::compute_from_git(
            &repo_path,
            Utc::now() - chrono::Duration::days(30),
            Utc::now(),
        );

        match result {
            Ok((dep_freq, lead_time, commits)) => {
                assert!(dep_freq >= 0.0);
                assert!(lead_time >= 0.0);
                assert!(commits > 0, "Expected at least 1 commit in the repo");
            }
            Err(e) => {
                panic!("compute_from_git failed on real repo: {}", e);
            }
        }
    }
}
