use crate::entities::{Entity, GenericEntity};
use crate::error::EngramError;
use crate::storage::Storage;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlakinessConfig {
    pub window_size: usize,
    pub failure_rate_threshold: f64,
}

impl Default for FlakinessConfig {
    fn default() -> Self {
        Self {
            window_size: 10,
            failure_rate_threshold: 0.3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlakinessBlacklistEntry {
    pub id: String,
    pub gate_name: String,
    pub blacklisted_at: DateTime<Utc>,
    pub failure_rate: f64,
    pub window_size: usize,
    pub failure_count: usize,
    pub total_runs: usize,
    pub agent: String,
    pub reason: String,
}

impl FlakinessBlacklistEntry {
    pub fn new(
        gate_name: String,
        failure_rate: f64,
        window_size: usize,
        failure_count: usize,
        total_runs: usize,
        agent: String,
    ) -> Self {
        Self {
            id: format!(
                "flaky_blacklist_{}",
                gate_name.replace(['/', ' ', '.'], "_")
            ),
            gate_name,
            blacklisted_at: Utc::now(),
            failure_rate,
            window_size,
            failure_count,
            total_runs,
            agent,
            reason: format!(
                "Auto-blacklisted: {}/{} failures in last {} runs ({:.1}%)",
                failure_count,
                total_runs,
                window_size,
                failure_rate * 100.0
            ),
        }
    }
}

impl Entity for FlakinessBlacklistEntry {
    fn entity_type() -> &'static str {
        "flakiness_blacklist"
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn agent(&self) -> &str {
        &self.agent
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.blacklisted_at
    }

    fn validate_entity(&self) -> crate::Result<()> {
        if self.gate_name.is_empty() {
            return Err(EngramError::Validation(
                "Gate name cannot be empty".to_string(),
            ));
        }
        if self.failure_rate < 0.0 || self.failure_rate > 1.0 {
            return Err(EngramError::Validation(
                "Failure rate must be between 0.0 and 1.0".to_string(),
            ));
        }
        if self.window_size == 0 {
            return Err(EngramError::Validation(
                "Window size must be greater than 0".to_string(),
            ));
        }
        Ok(())
    }

    fn to_generic(&self) -> GenericEntity {
        GenericEntity {
            id: self.id.clone(),
            entity_type: Self::entity_type().to_string(),
            agent: self.agent.clone(),
            timestamp: self.blacklisted_at,
            data: serde_json::to_value(self).unwrap_or_default(),
        }
    }

    fn from_generic(entity: GenericEntity) -> crate::Result<Self>
    where
        Self: Sized,
    {
        serde_json::from_value(entity.data).map_err(|e| {
            EngramError::Deserialization(format!(
                "Failed to deserialize FlakinessBlacklistEntry: {}",
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

pub struct FlakinessTracker {
    config: FlakinessConfig,
}

#[derive(Debug, Clone)]
pub struct FlakinessAssessment {
    pub gate_name: String,
    pub total_runs: usize,
    pub failures: usize,
    pub failure_rate: f64,
    pub is_flaky: bool,
}

impl FlakinessTracker {
    pub fn new() -> Self {
        Self {
            config: FlakinessConfig::default(),
        }
    }

    pub fn with_config(config: FlakinessConfig) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &FlakinessConfig {
        &self.config
    }

    pub fn is_blacklisted<S: Storage>(&self, storage: &S, gate_name: &str) -> bool {
        let id = format!(
            "flaky_blacklist_{}",
            gate_name.replace(['/', ' ', '.'], "_")
        );
        match storage.get(&id, "flakiness_blacklist") {
            Ok(Some(_)) => true,
            _ => false,
        }
    }

    pub fn get_blacklist_entry<S: Storage>(
        &self,
        storage: &S,
        gate_name: &str,
    ) -> Result<Option<FlakinessBlacklistEntry>, EngramError> {
        let id = format!(
            "flaky_blacklist_{}",
            gate_name.replace(['/', ' ', '.'], "_")
        );
        match storage.get(&id, "flakiness_blacklist") {
            Ok(Some(entity)) => Ok(Some(FlakinessBlacklistEntry::from_generic(entity)?)),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn get_all_blacklisted<S: Storage>(
        &self,
        storage: &S,
    ) -> Result<Vec<FlakinessBlacklistEntry>, EngramError> {
        let results = storage.get_all("flakiness_blacklist")?;
        let mut entries = Vec::new();
        for entity in results {
            if let Ok(entry) = FlakinessBlacklistEntry::from_generic(entity) {
                entries.push(entry);
            }
        }
        Ok(entries)
    }

    pub fn unblacklist<S: Storage>(
        &self,
        storage: &mut S,
        gate_name: &str,
    ) -> Result<(), EngramError> {
        let id = format!(
            "flaky_blacklist_{}",
            gate_name.replace(['/', ' ', '.'], "_")
        );
        storage.delete(&id, "flakiness_blacklist")?;
        Ok(())
    }

    pub fn record_and_evaluate<S: Storage>(
        &self,
        storage: &mut S,
        gate_name: &str,
        passed: bool,
    ) -> Result<FlakinessAssessment, EngramError> {
        let history = self.get_gate_history(storage, gate_name)?;
        let assessment = self.assess_flakiness(gate_name, &history);

        if !assessment.is_flaky {
            if let Some(mut updated_history) = history {
                updated_history.push_back(passed);
                if updated_history.len() > self.config.window_size {
                    updated_history.pop_front();
                }
                self.store_gate_history(storage, gate_name, &updated_history)?;
            } else {
                let mut new_history = VecDeque::with_capacity(self.config.window_size);
                new_history.push_back(passed);
                self.store_gate_history(storage, gate_name, &new_history)?;
            }
        }

        Ok(assessment)
    }

    pub fn evaluate_gate<S: Storage>(
        &self,
        storage: &S,
        gate_name: &str,
    ) -> Result<FlakinessAssessment, EngramError> {
        let history = self.get_gate_history(storage, gate_name)?;
        Ok(self.assess_flakiness(gate_name, &history))
    }

    fn assess_flakiness(
        &self,
        gate_name: &str,
        history: &Option<VecDeque<bool>>,
    ) -> FlakinessAssessment {
        match history {
            None => FlakinessAssessment {
                gate_name: gate_name.to_string(),
                total_runs: 0,
                failures: 0,
                failure_rate: 0.0,
                is_flaky: false,
            },
            Some(h) if h.is_empty() => FlakinessAssessment {
                gate_name: gate_name.to_string(),
                total_runs: 0,
                failures: 0,
                failure_rate: 0.0,
                is_flaky: false,
            },
            Some(h) => {
                let total = h.len();
                let failures = h.iter().filter(|&&r| !r).count();
                let rate = failures as f64 / total as f64;
                let has_both = h.iter().any(|&r| r) && h.iter().any(|&r| !r);
                let is_flaky = total >= self.config.window_size
                    && has_both
                    && rate >= self.config.failure_rate_threshold;

                FlakinessAssessment {
                    gate_name: gate_name.to_string(),
                    total_runs: total,
                    failures,
                    failure_rate: rate,
                    is_flaky,
                }
            }
        }
    }

    pub fn blacklist_if_flaky<S: Storage>(
        &self,
        storage: &mut S,
        gate_name: &str,
        agent: &str,
    ) -> Result<bool, EngramError> {
        if self.is_blacklisted(storage, gate_name) {
            return Ok(true);
        }

        let assessment = self.evaluate_gate(storage, gate_name)?;
        if assessment.is_flaky {
            let entry = FlakinessBlacklistEntry::new(
                gate_name.to_string(),
                assessment.failure_rate,
                assessment.total_runs,
                assessment.failures,
                assessment.total_runs,
                agent.to_string(),
            );
            let generic = entry.to_generic();
            storage.store(&generic)?;
            return Ok(true);
        }

        Ok(false)
    }

    fn get_gate_history<S: Storage>(
        &self,
        storage: &S,
        gate_name: &str,
    ) -> Result<Option<VecDeque<bool>>, EngramError> {
        let history_id = format!("flaky_history_{}", gate_name.replace(['/', ' ', '.'], "_"));
        match storage.get(&history_id, "flakiness_history") {
            Ok(Some(entity)) => {
                let data = &entity.data;
                let runs: Vec<bool> = serde_json::from_value(
                    data.get("runs")
                        .cloned()
                        .unwrap_or(serde_json::Value::Array(vec![])),
                )
                .unwrap_or_default();
                Ok(Some(VecDeque::from(runs)))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn store_gate_history<S: Storage>(
        &self,
        storage: &mut S,
        gate_name: &str,
        history: &VecDeque<bool>,
    ) -> Result<(), EngramError> {
        let history_id = format!("flaky_history_{}", gate_name.replace(['/', ' ', '.'], "_"));
        let runs: Vec<bool> = history.iter().copied().collect();

        let mut data = serde_json::Map::new();
        data.insert(
            "gate_name".to_string(),
            serde_json::Value::String(gate_name.to_string()),
        );
        data.insert(
            "runs".to_string(),
            serde_json::to_value(&runs).unwrap_or_default(),
        );
        data.insert(
            "updated_at".to_string(),
            serde_json::Value::String(Utc::now().to_rfc3339()),
        );

        let entity = GenericEntity {
            id: history_id,
            entity_type: "flakiness_history".to_string(),
            agent: "system".to_string(),
            timestamp: Utc::now(),
            data: serde_json::Value::Object(data),
        };

        storage.store(&entity)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::MemoryStorage;

    fn make_tracker() -> FlakinessTracker {
        FlakinessTracker::with_config(FlakinessConfig {
            window_size: 5,
            failure_rate_threshold: 0.3,
        })
    }

    #[test]
    fn test_no_history_is_not_flaky() {
        let tracker = make_tracker();
        let storage = MemoryStorage::new("test-agent");
        let assessment = tracker.evaluate_gate(&storage, "unit_tests").unwrap();
        assert!(!assessment.is_flaky);
        assert_eq!(assessment.total_runs, 0);
    }

    #[test]
    fn test_consistent_pass_is_not_flaky() {
        let tracker = make_tracker();
        let mut storage = MemoryStorage::new("test-agent");
        for _ in 0..6 {
            tracker
                .record_and_evaluate(&mut storage, "unit_tests", true)
                .unwrap();
        }
        let assessment = tracker.evaluate_gate(&storage, "unit_tests").unwrap();
        assert!(!assessment.is_flaky);
        assert_eq!(assessment.failures, 0);
    }

    #[test]
    fn test_consistent_fail_is_not_flaky() {
        let tracker = make_tracker();
        let mut storage = MemoryStorage::new("test-agent");
        for _ in 0..6 {
            tracker
                .record_and_evaluate(&mut storage, "unit_tests", false)
                .unwrap();
        }
        let assessment = tracker.evaluate_gate(&storage, "unit_tests").unwrap();
        assert!(!assessment.is_flaky);
    }

    #[test]
    fn test_mixed_results_are_flaky() {
        let tracker = make_tracker();
        let mut storage = MemoryStorage::new("test-agent");
        let results = [true, false, true, false, true, false];
        for &passed in &results {
            tracker
                .record_and_evaluate(&mut storage, "integration_tests", passed)
                .unwrap();
        }
        let assessment = tracker
            .evaluate_gate(&storage, "integration_tests")
            .unwrap();
        assert!(assessment.is_flaky);
        assert_eq!(assessment.total_runs, 5);
        assert_eq!(assessment.failures, 2);
        assert!(assessment.failure_rate >= 0.3);
    }

    #[test]
    fn test_blacklist_if_flaky() {
        let tracker = make_tracker();
        let mut storage = MemoryStorage::new("test-agent");
        let results = [true, false, true, false, true, false];
        for &passed in &results {
            tracker
                .record_and_evaluate(&mut storage, "security_scan", passed)
                .unwrap();
        }
        let blacklisted = tracker
            .blacklist_if_flaky(&mut storage, "security_scan", "agent")
            .unwrap();
        assert!(blacklisted);
        assert!(tracker.is_blacklisted(&storage, "security_scan"));
    }

    #[test]
    fn test_not_blacklisted_when_stable() {
        let tracker = make_tracker();
        let mut storage = MemoryStorage::new("test-agent");
        for _ in 0..6 {
            tracker
                .record_and_evaluate(&mut storage, "syntax_check", true)
                .unwrap();
        }
        let blacklisted = tracker
            .blacklist_if_flaky(&mut storage, "syntax_check", "agent")
            .unwrap();
        assert!(!blacklisted);
        assert!(!tracker.is_blacklisted(&storage, "syntax_check"));
    }

    #[test]
    fn test_unblacklist() {
        let tracker = make_tracker();
        let mut storage = MemoryStorage::new("test-agent");
        let results = [true, false, true, false, true, false];
        for &passed in &results {
            tracker
                .record_and_evaluate(&mut storage, "perf_test", passed)
                .unwrap();
        }
        tracker
            .blacklist_if_flaky(&mut storage, "perf_test", "agent")
            .unwrap();
        assert!(tracker.is_blacklisted(&storage, "perf_test"));

        tracker.unblacklist(&mut storage, "perf_test").unwrap();
        assert!(!tracker.is_blacklisted(&storage, "perf_test"));
    }

    #[test]
    fn test_blacklist_entry_entity_roundtrip() {
        let entry = FlakinessBlacklistEntry::new(
            "my_gate".to_string(),
            0.4,
            10,
            4,
            10,
            "agent".to_string(),
        );
        assert!(entry.validate_entity().is_ok());

        let generic = entry.to_generic();
        assert_eq!(generic.entity_type, "flakiness_blacklist");

        let restored = FlakinessBlacklistEntry::from_generic(generic).unwrap();
        assert_eq!(restored.gate_name, "my_gate");
        assert_eq!(restored.failure_rate, 0.4);
    }

    #[test]
    fn test_blacklist_entry_validation() {
        let mut entry =
            FlakinessBlacklistEntry::new("gate".to_string(), 0.5, 10, 5, 10, "agent".to_string());
        assert!(entry.validate_entity().is_ok());

        entry.gate_name = String::new();
        assert!(entry.validate_entity().is_err());

        entry.gate_name = "gate".to_string();
        entry.failure_rate = 1.5;
        assert!(entry.validate_entity().is_err());
    }

    #[test]
    fn test_gate_name_with_special_chars() {
        let tracker = make_tracker();
        let mut storage = MemoryStorage::new("test-agent");
        for _ in 0..6 {
            tracker
                .record_and_evaluate(&mut storage, "cargo test --lib quality", true)
                .unwrap();
        }
        assert!(!tracker.is_blacklisted(&storage, "cargo test --lib quality"));

        let assessment = tracker
            .evaluate_gate(&storage, "cargo test --lib quality")
            .unwrap();
        assert!(!assessment.is_flaky);
    }

    #[test]
    fn test_already_blacklisted_returns_true() {
        let tracker = make_tracker();
        let mut storage = MemoryStorage::new("test-agent");
        let results = [true, false, true, false, true, false];
        for &passed in &results {
            tracker
                .record_and_evaluate(&mut storage, "my_gate", passed)
                .unwrap();
        }
        tracker
            .blacklist_if_flaky(&mut storage, "my_gate", "agent")
            .unwrap();

        let blacklisted = tracker
            .blacklist_if_flaky(&mut storage, "my_gate", "agent")
            .unwrap();
        assert!(blacklisted);
    }

    #[test]
    fn test_get_blacklist_entry() {
        let tracker = make_tracker();
        let mut storage = MemoryStorage::new("test-agent");
        let results = [true, false, true, false, true, false];
        for &passed in &results {
            tracker
                .record_and_evaluate(&mut storage, "dep_check", passed)
                .unwrap();
        }
        tracker
            .blacklist_if_flaky(&mut storage, "dep_check", "agent")
            .unwrap();

        let entry = tracker.get_blacklist_entry(&storage, "dep_check").unwrap();
        assert!(entry.is_some());
        let entry = entry.unwrap();
        assert_eq!(entry.gate_name, "dep_check");
        assert!(entry.reason.contains("Auto-blacklisted"));

        let none = tracker
            .get_blacklist_entry(&storage, "nonexistent")
            .unwrap();
        assert!(none.is_none());
    }

    #[test]
    fn test_get_all_blacklisted() {
        let tracker = make_tracker();
        let mut storage = MemoryStorage::new("test-agent");

        let gate_a: Vec<bool> = [true, false, true, false, true, false].to_vec();
        for &passed in &gate_a {
            tracker
                .record_and_evaluate(&mut storage, "gate_a", passed)
                .unwrap();
        }
        tracker
            .blacklist_if_flaky(&mut storage, "gate_a", "agent")
            .unwrap();

        let gate_b: Vec<bool> = [true, false, true, false, true, false].to_vec();
        for &passed in &gate_b {
            tracker
                .record_and_evaluate(&mut storage, "gate_b", passed)
                .unwrap();
        }
        tracker
            .blacklist_if_flaky(&mut storage, "gate_b", "agent")
            .unwrap();

        let all = tracker.get_all_blacklisted(&storage).unwrap();
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn test_history_window_respects_config() {
        let config = FlakinessConfig {
            window_size: 3,
            failure_rate_threshold: 0.3,
        };
        let tracker = FlakinessTracker::with_config(config);
        let mut storage = MemoryStorage::new("test-agent");

        for _ in 0..5 {
            tracker
                .record_and_evaluate(&mut storage, "small_window", true)
                .unwrap();
        }
        let assessment = tracker.evaluate_gate(&storage, "small_window").unwrap();
        assert_eq!(assessment.total_runs, 3);
    }

    #[test]
    fn test_record_and_evaluate_does_not_mutate_when_flaky() {
        let tracker = make_tracker();
        let mut storage = MemoryStorage::new("test-agent");
        let results = [true, false, true, false, true, false];
        for &passed in &results {
            tracker
                .record_and_evaluate(&mut storage, "frozen_gate", passed)
                .unwrap();
        }
        let assessment = tracker
            .record_and_evaluate(&mut storage, "frozen_gate", true)
            .unwrap();
        assert!(assessment.is_flaky);
    }
}
