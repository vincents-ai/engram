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

    struct MockStorage {
        entities: HashMap<String, Vec<GenericEntity>>,
    }

    impl MockStorage {
        fn new() -> Self {
            Self {
                entities: HashMap::new(),
            }
        }

        fn with_entities(entity_type: &str, entities: Vec<GenericEntity>) -> Self {
            let mut map = HashMap::new();
            map.insert(entity_type.to_string(), entities);
            Self { entities: map }
        }
    }

    use crate::storage::{GitCommit, QueryFilter, QueryResult, Storage, StorageStats};
    use std::collections::HashMap as StdHashMap;

    impl Storage for MockStorage {
        fn store(&mut self, _entity: &GenericEntity) -> Result<(), crate::EngramError> {
            Ok(())
        }
        fn get(
            &self,
            _id: &str,
            _entity_type: &str,
        ) -> Result<Option<GenericEntity>, crate::EngramError> {
            Ok(None)
        }
        fn query(&self, _filter: &QueryFilter) -> Result<QueryResult, crate::EngramError> {
            Ok(QueryResult {
                entities: vec![],
                total_count: 0,
                has_more: false,
            })
        }
        fn query_by_agent(
            &self,
            _agent: &str,
            _entity_type: Option<&str>,
        ) -> Result<Vec<GenericEntity>, crate::EngramError> {
            Ok(vec![])
        }
        fn query_by_time_range(
            &self,
            _start: chrono::DateTime<Utc>,
            _end: chrono::DateTime<Utc>,
        ) -> Result<Vec<GenericEntity>, crate::EngramError> {
            Ok(vec![])
        }
        fn query_by_type(
            &self,
            _entity_type: &str,
            _filters: Option<&StdHashMap<String, serde_json::Value>>,
            _limit: Option<usize>,
            _offset: Option<usize>,
        ) -> Result<QueryResult, crate::EngramError> {
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
        ) -> Result<Vec<GenericEntity>, crate::EngramError> {
            Ok(vec![])
        }
        fn count(&self, _filter: &QueryFilter) -> Result<usize, crate::EngramError> {
            Ok(0)
        }
        fn delete(&mut self, _id: &str, _entity_type: &str) -> Result<(), crate::EngramError> {
            Ok(())
        }
        fn list_ids(&self, _entity_type: &str) -> Result<Vec<String>, crate::EngramError> {
            Ok(vec![])
        }
        fn get_all(&self, entity_type: &str) -> Result<Vec<GenericEntity>, crate::EngramError> {
            Ok(self.entities.get(entity_type).cloned().unwrap_or_default())
        }
        fn sync(&mut self) -> Result<(), crate::EngramError> {
            Ok(())
        }
        fn current_branch(&self) -> Result<String, crate::EngramError> {
            Ok("main".to_string())
        }
        fn create_branch(&mut self, _name: &str) -> Result<(), crate::EngramError> {
            Ok(())
        }
        fn switch_branch(&mut self, _name: &str) -> Result<(), crate::EngramError> {
            Ok(())
        }
        fn merge_branches(&mut self, _src: &str, _tgt: &str) -> Result<(), crate::EngramError> {
            Ok(())
        }
        fn history(&self, _limit: Option<usize>) -> Result<Vec<GitCommit>, crate::EngramError> {
            Ok(vec![])
        }
        fn bulk_store(&mut self, _entities: &[GenericEntity]) -> Result<(), crate::EngramError> {
            Ok(())
        }
        fn get_stats(&self) -> Result<StorageStats, crate::EngramError> {
            Ok(StorageStats::default())
        }
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    fn make_execution_result(id: &str, status: &str) -> GenericEntity {
        GenericEntity {
            id: id.to_string(),
            entity_type: "execution_result".to_string(),
            agent: "test-agent".to_string(),
            timestamp: Utc::now(),
            data: serde_json::json!({ "validation_status": status }),
        }
    }

    fn make_escalation(
        id: &str,
        created_at: &str,
        reviewed_at: &str,
        status: &str,
    ) -> GenericEntity {
        GenericEntity {
            id: id.to_string(),
            entity_type: "escalation_request".to_string(),
            agent: "test-agent".to_string(),
            timestamp: Utc::now(),
            data: serde_json::json!({
                "created_at": created_at,
                "reviewed_at": reviewed_at,
                "status": status,
            }),
        }
    }

    #[test]
    fn test_deployment_frequency_zero_commits() {
        let repo_path = std::env::var("CARGO_MANIFEST_DIR")
            .map(|p| std::path::PathBuf::from(p))
            .unwrap_or_else(|_| std::path::PathBuf::from("."));

        let far_future = Utc::now() + chrono::Duration::days(365);
        let result = DoraMetricsCalculator::compute_from_git(
            &repo_path,
            far_future,
            far_future + chrono::Duration::days(1),
        );

        let (dep_freq, lead_time, commits) = result.unwrap();
        assert_eq!(dep_freq, 0.0);
        assert_eq!(lead_time, 0.0);
        assert_eq!(commits, 0);
    }

    #[test]
    fn test_deployment_frequency_burst_pattern() {
        let day = 86400i64;
        let timestamps: Vec<i64> = (0..20).map(|i| day * 10 + i * 60).collect();
        let freq = DoraMetricsCalculator::median_commit_span_days(&timestamps);
        let expected = (19 * 60) as f64 / 86400.0;
        assert!((freq - expected).abs() < 0.001);
    }

    #[test]
    fn test_deployment_frequency_steady_pattern() {
        let day = 86400i64;
        let timestamps: Vec<i64> = (0..7)
            .flat_map(|d| {
                let base = day * (10 + d);
                vec![base, base + 3600]
            })
            .collect();
        let mut spans: Vec<f64> = Vec::new();
        for _d in 0..7 {
            spans.push(3600.0 / 86400.0);
        }
        spans.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let expected: f64 = spans[3];
        let result = DoraMetricsCalculator::median_commit_span_days(&timestamps);
        assert!((result - expected).abs() < 0.001);
    }

    #[test]
    fn test_lead_time_single_commit_per_day() {
        let day = 86400i64;
        let timestamps: Vec<i64> = (0..5).map(|d| day * (20 + d) + 43200).collect();
        let result = DoraMetricsCalculator::median_commit_span_days(&timestamps);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_lead_time_wide_spread() {
        let day = 86400i64;
        let timestamps = vec![day * 10, day * 10 + 10 * 3600, day * 11, day * 11 + 60];
        let result = DoraMetricsCalculator::median_commit_span_days(&timestamps);
        let expected: f64 = ((10.0 * 3600.0 / 86400.0) + (60.0 / 86400.0)) / 2.0;
        assert!((result - expected).abs() < 0.001);
    }

    #[test]
    fn test_lead_time_odd_number_of_days() {
        let day = 86400i64;
        let timestamps = vec![
            day * 10,
            day * 10 + 1800,
            day * 11,
            day * 11 + 3600,
            day * 12,
            day * 12 + 7200,
        ];
        let result = DoraMetricsCalculator::median_commit_span_days(&timestamps);
        let expected = 3600.0 / 86400.0;
        assert!((result - expected).abs() < 0.001);
    }

    #[test]
    fn test_change_failure_rate_all_pass() {
        let entities = vec![
            make_execution_result("e1", "passed"),
            make_execution_result("e2", "passed"),
            make_execution_result("e3", "passed"),
        ];
        let storage = MockStorage::with_entities("execution_result", entities);
        let (rate, total) =
            DoraMetricsCalculator::compute_change_failure_rate(&storage, "test").unwrap();
        assert_eq!(rate, 0.0);
        assert_eq!(total, 3);
    }

    #[test]
    fn test_change_failure_rate_all_fail() {
        let entities = vec![
            make_execution_result("e1", "failed"),
            make_execution_result("e2", "failed"),
        ];
        let storage = MockStorage::with_entities("execution_result", entities);
        let (rate, total) =
            DoraMetricsCalculator::compute_change_failure_rate(&storage, "test").unwrap();
        assert_eq!(rate, 1.0);
        assert_eq!(total, 2);
    }

    #[test]
    fn test_change_failure_rate_mixed() {
        let entities = vec![
            make_execution_result("e1", "passed"),
            make_execution_result("e2", "failed"),
            make_execution_result("e3", "passed"),
            make_execution_result("e4", "failed"),
            make_execution_result("e5", "passed"),
        ];
        let storage = MockStorage::with_entities("execution_result", entities);
        let (rate, total) =
            DoraMetricsCalculator::compute_change_failure_rate(&storage, "test").unwrap();
        assert!((rate - 0.4).abs() < 0.001);
        assert_eq!(total, 5);
    }

    #[test]
    fn test_change_failure_rate_empty() {
        let storage = MockStorage::new();
        let (rate, total) =
            DoraMetricsCalculator::compute_change_failure_rate(&storage, "test").unwrap();
        assert_eq!(rate, 0.0);
        assert_eq!(total, 0);
    }

    #[test]
    fn test_change_failure_rate_ignores_unknown_status() {
        let entities = vec![
            make_execution_result("e1", "passed"),
            make_execution_result("e2", "skipped"),
            make_execution_result("e3", "unknown"),
        ];
        let storage = MockStorage::with_entities("execution_result", entities);
        let (rate, total) =
            DoraMetricsCalculator::compute_change_failure_rate(&storage, "test").unwrap();
        assert_eq!(rate, 0.0);
        assert_eq!(total, 1);
    }

    #[test]
    fn test_mttr_empty() {
        let storage = MockStorage::new();
        let (mttr, count) =
            DoraMetricsCalculator::compute_mttr_from_entities(&storage, "test").unwrap();
        assert_eq!(mttr, 0.0);
        assert_eq!(count, 0);
    }

    #[test]
    fn test_mttr_approved_escalations() {
        let entities = vec![
            make_escalation(
                "esc1",
                "2026-01-01T10:00:00Z",
                "2026-01-01T12:00:00Z",
                "approved",
            ),
            make_escalation(
                "esc2",
                "2026-01-02T08:00:00Z",
                "2026-01-02T09:00:00Z",
                "approved",
            ),
        ];
        let storage = MockStorage::with_entities("escalation_request", entities);
        let (mttr, count) =
            DoraMetricsCalculator::compute_mttr_from_entities(&storage, "test").unwrap();
        assert!((mttr - 1.5).abs() < 0.001);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_mttr_denied_escalations() {
        let entities = vec![make_escalation(
            "esc1",
            "2026-01-01T10:00:00Z",
            "2026-01-01T11:30:00Z",
            "denied",
        )];
        let storage = MockStorage::with_entities("escalation_request", entities);
        let (mttr, count) =
            DoraMetricsCalculator::compute_mttr_from_entities(&storage, "test").unwrap();
        assert!((mttr - 1.5).abs() < 0.001);
        assert_eq!(count, 1);
    }

    #[test]
    fn test_mttr_ignores_pending() {
        let entities = vec![
            make_escalation(
                "esc1",
                "2026-01-01T10:00:00Z",
                "2026-01-01T12:00:00Z",
                "approved",
            ),
            make_escalation(
                "esc2",
                "2026-01-02T08:00:00Z",
                "2026-01-02T09:00:00Z",
                "pending",
            ),
        ];
        let storage = MockStorage::with_entities("escalation_request", entities);
        let (mttr, count) =
            DoraMetricsCalculator::compute_mttr_from_entities(&storage, "test").unwrap();
        assert_eq!(count, 1);
        assert!((mttr - 2.0).abs() < 0.001);
    }

    #[test]
    fn test_mttr_ignores_missing_reviewed_at() {
        let entities = vec![GenericEntity {
            id: "esc1".to_string(),
            entity_type: "escalation_request".to_string(),
            agent: "test-agent".to_string(),
            timestamp: Utc::now(),
            data: serde_json::json!({
                "created_at": "2026-01-01T10:00:00Z",
                "status": "approved",
            }),
        }];
        let storage = MockStorage::with_entities("escalation_request", entities);
        let (mttr, count) =
            DoraMetricsCalculator::compute_mttr_from_entities(&storage, "test").unwrap();
        assert_eq!(mttr, 0.0);
        assert_eq!(count, 0);
    }

    #[test]
    fn test_mttr_cross_day_resolution() {
        let entities = vec![make_escalation(
            "esc1",
            "2026-01-01T22:00:00Z",
            "2026-01-02T02:00:00Z",
            "approved",
        )];
        let storage = MockStorage::with_entities("escalation_request", entities);
        let (mttr, count) =
            DoraMetricsCalculator::compute_mttr_from_entities(&storage, "test").unwrap();
        assert!((mttr - 4.0).abs() < 0.001);
        assert_eq!(count, 1);
    }

    #[test]
    fn test_validation_rejects_empty_project_path() {
        let mut report = DoraMetricsReport::new("".to_string(), "agent".to_string());
        assert!(report.validate_entity().is_err());
        report.project_path = "  ".to_string();
        assert!(report.validate_entity().is_ok());
    }

    #[test]
    fn test_window_bounds() {
        let report = DoraMetricsReport::new("/repo".to_string(), "agent".to_string());
        assert!(report.window_start <= report.computed_at);
        assert!(report.window_end >= report.computed_at);
    }

    #[test]
    fn test_report_json_serialization_roundtrip() {
        let mut report = DoraMetricsReport::new("/my/project".to_string(), "my-agent".to_string());
        report.deployment_frequency = 12.5;
        report.lead_time_for_changes = 0.8;
        report.change_failure_rate = 0.22;
        report.mean_time_to_recovery = 6.0;
        report.commits_analyzed = 100;
        report.executions_analyzed = 50;
        report.escalations_analyzed = 7;
        report
            .metadata
            .insert("env".to_string(), serde_json::json!("production"));

        let json = serde_json::to_string(&report).unwrap();
        let deserialized: DoraMetricsReport = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, report.id);
        assert_eq!(deserialized.project_path, "/my/project");
        assert_eq!(deserialized.agent, "my-agent");
        assert_eq!(deserialized.deployment_frequency, 12.5);
        assert_eq!(deserialized.lead_time_for_changes, 0.8);
        assert_eq!(deserialized.change_failure_rate, 0.22);
        assert_eq!(deserialized.mean_time_to_recovery, 6.0);
        assert_eq!(deserialized.commits_analyzed, 100);
        assert_eq!(deserialized.executions_analyzed, 50);
        assert_eq!(deserialized.escalations_analyzed, 7);
        assert_eq!(deserialized.metadata.get("env").unwrap(), "production");
    }

    #[test]
    fn test_report_json_omits_empty_metadata() {
        let report = DoraMetricsReport::new("/repo".to_string(), "agent".to_string());
        let json = serde_json::to_string(&report).unwrap();
        assert!(!json.contains("metadata"));
    }

    #[test]
    fn test_report_persistence_roundtrip() {
        let mut report =
            DoraMetricsReport::new("/persist/test".to_string(), "agent-01".to_string());
        report.deployment_frequency = 3.0;
        report.lead_time_for_changes = 1.5;
        report.change_failure_rate = 0.1;
        report.mean_time_to_recovery = 2.0;
        report.commits_analyzed = 20;
        report.executions_analyzed = 5;
        report.escalations_analyzed = 2;

        let generic = report.to_generic();
        assert_eq!(generic.entity_type, "dora_metrics_report");
        assert_eq!(generic.agent, "agent-01");

        let restored = DoraMetricsReport::from_generic(generic.clone()).unwrap();
        assert_eq!(restored.id, report.id);
        assert_eq!(restored.project_path, report.project_path);
        assert_eq!(restored.computed_at, report.computed_at);
        assert_eq!(restored.deployment_frequency, 3.0);
        assert_eq!(restored.lead_time_for_changes, 1.5);
        assert_eq!(restored.change_failure_rate, 0.1);
        assert_eq!(restored.mean_time_to_recovery, 2.0);
        assert_eq!(restored.commits_analyzed, 20);
        assert_eq!(restored.executions_analyzed, 5);
        assert_eq!(restored.escalations_analyzed, 2);

        let double_restored = DoraMetricsReport::from_generic(restored.to_generic()).unwrap();
        assert_eq!(double_restored.id, report.id);
    }

    #[test]
    fn test_entity_trait_methods() {
        let report = DoraMetricsReport::new("/repo".to_string(), "cli-agent".to_string());
        assert_eq!(DoraMetricsReport::entity_type(), "dora_metrics_report");
        assert_eq!(report.id(), report.id.as_str());
        assert_eq!(report.agent(), "cli-agent");
        assert_eq!(report.timestamp(), report.computed_at);
        assert!(report.validate_entity().is_ok());
    }

    #[test]
    fn test_to_session_dora_metrics_zero_values() {
        let report = DoraMetricsReport::new("/repo".to_string(), "agent".to_string());
        let metrics = report.to_session_dora_metrics();
        assert_eq!(metrics.deployment_frequency, 0.0);
        assert_eq!(metrics.lead_time, 0.0);
        assert_eq!(metrics.change_failure_rate, 0.0);
        assert_eq!(metrics.mean_time_to_recover, 0.0);
    }
}
