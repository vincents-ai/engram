use super::{GateContext, QualityGateError, QualityGateResult};
use crate::entities::progressive_config::GateLevel;

pub struct LevelSelector;

impl LevelSelector {
    pub fn select_level<'a>(
        context: &GateContext,
        available_levels: &'a [GateLevel],
    ) -> QualityGateResult<&'a GateLevel> {
        for level in available_levels {
            if Self::matches_threshold(context, &level.threshold) {
                return Ok(level);
            }
        }

        available_levels
            .first()
            .ok_or_else(|| QualityGateError::ConfigError("No gate levels available".to_string()))
    }

    fn matches_threshold(
        context: &GateContext,
        threshold: &crate::entities::progressive_config::ChangeThreshold,
    ) -> bool {
        let lines_changed = context.changed_files.len() as u32 * 50;
        let files_affected = context.changed_files.len() as u32;

        lines_changed <= threshold.max_lines_changed
            && files_affected <= threshold.max_files_affected
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::progressive_config::{ChangeThreshold, GateLevel};
    use crate::entities::task::{TaskPriority, TaskStatus};
    use crate::validation::quality_gates::GateContext;
    use std::collections::HashMap;

    fn make_task() -> crate::entities::Task {
        crate::entities::Task {
            id: "test-1".to_string(),
            title: "Test".to_string(),
            description: "desc".to_string(),
            status: TaskStatus::Todo,
            priority: TaskPriority::Medium,
            agent: "test".to_string(),
            start_time: chrono::Utc::now(),
            end_time: None,
            parent: None,
            children: Vec::new(),
            tags: Vec::new(),
            context_ids: Vec::new(),
            knowledge: Vec::new(),
            files: Vec::new(),
            outcome: None,
            block_reason: None,
            workflow_id: None,
            workflow_state: None,
            metadata: HashMap::new(),
        }
    }

    fn make_context(changed_files: Vec<String>) -> GateContext {
        GateContext {
            task: make_task(),
            changed_files,
            commit_message: None,
            branch_name: None,
            metadata: HashMap::new(),
        }
    }

    fn make_level(max_lines: u32, max_files: u32) -> GateLevel {
        use crate::entities::progressive_config::*;
        use std::time::Duration;
        GateLevel {
            name: "test-level".to_string(),
            threshold: ChangeThreshold {
                max_lines_changed: max_lines,
                max_files_affected: max_files,
                max_complexity_delta: 1.0,
                allowed_change_types: vec![ChangeType::Feature],
                risk_level_limit: ProgressiveRiskLevel::Low,
                file_patterns: vec![],
            },
            required_gates: vec![],
            optional_gates: vec![],
            max_execution_time: Duration::from_secs(300),
            parallelization: ParallelizationStrategy::Sequential,
            failure_handling: FailureHandling {
                continue_on_optional_failure: false,
                fail_fast: false,
                escalate_on_failure: false,
                retry_failed_gates: false,
                notification_channels: vec![],
            },
            enabled: true,
            priority: 0,
        }
    }

    #[test]
    fn test_select_level_matching_threshold() {
        let context = make_context(vec!["file1.rs".to_string()]);
        let levels = vec![make_level(25, 1), make_level(100, 5)];

        let result = LevelSelector::select_level(&context, &levels).unwrap();
        assert_eq!(result.name, "test-level");
    }

    #[test]
    fn test_select_level_no_matching_falls_back_to_first() {
        let context = make_context(vec!["a.rs".to_string(), "b.rs".to_string()]);
        let levels = vec![make_level(25, 1), make_level(100, 5)];

        let result = LevelSelector::select_level(&context, &levels).unwrap();
        assert_eq!(result.name, "test-level");
    }

    #[test]
    fn test_select_level_empty_list_returns_error() {
        let context = make_context(vec!["file1.rs".to_string()]);
        let levels: Vec<GateLevel> = vec![];

        let result = LevelSelector::select_level(&context, &levels);
        assert!(result.is_err());
    }

    #[test]
    fn test_matches_threshold_boundary_exact() {
        let context = make_context(vec!["file1.rs".to_string()]);
        let threshold = ChangeThreshold {
            max_lines_changed: 50,
            max_files_affected: 1,
            max_complexity_delta: 1.0,
            allowed_change_types: vec![],
            risk_level_limit: crate::entities::progressive_config::ProgressiveRiskLevel::Low,
            file_patterns: vec![],
        };

        assert!(LevelSelector::matches_threshold(&context, &threshold));
    }

    #[test]
    fn test_matches_threshold_boundary_exceeded() {
        let context = make_context(vec!["a.rs".to_string(), "b.rs".to_string()]);
        let threshold = ChangeThreshold {
            max_lines_changed: 50,
            max_files_affected: 1,
            max_complexity_delta: 1.0,
            allowed_change_types: vec![],
            risk_level_limit: crate::entities::progressive_config::ProgressiveRiskLevel::Low,
            file_patterns: vec![],
        };

        assert!(!LevelSelector::matches_threshold(&context, &threshold));
    }

    #[test]
    fn test_select_level_with_empty_changed_files() {
        let context = make_context(vec![]);
        let levels = vec![make_level(50, 1)];

        let result = LevelSelector::select_level(&context, &levels).unwrap();
        assert_eq!(result.name, "test-level");
    }
}
