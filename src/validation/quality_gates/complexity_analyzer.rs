use crate::entities::Task;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ComplexityLevel {
    Low,
    Medium,
    High,
    Critical,
}

pub struct ComplexityAnalyzer;

impl ComplexityAnalyzer {
    pub fn analyze_task(task: &Task) -> ComplexityLevel {
        let mut complexity_score = 0;

        if task.description.len() > 200 {
            complexity_score += 1;
        }

        if task.description.contains("refactor") || task.description.contains("migration") {
            complexity_score += 2;
        }

        if task.description.contains("architecture") || task.description.contains("security") {
            complexity_score += 3;
        }

        match complexity_score {
            0..=1 => ComplexityLevel::Low,
            2..=3 => ComplexityLevel::Medium,
            4..=5 => ComplexityLevel::High,
            _ => ComplexityLevel::Critical,
        }
    }

    pub fn analyze_change_context(
        changed_files: &[String],
        commit_message: &Option<String>,
    ) -> ComplexityLevel {
        let mut complexity_score = 0;

        if changed_files.len() > 10 {
            complexity_score += 2;
        }

        if let Some(message) = commit_message {
            if message.contains("breaking") || message.contains("BREAKING") {
                complexity_score += 3;
            }

            if message.contains("fix") && message.contains("critical") {
                complexity_score += 2;
            }
        }

        for file in changed_files {
            if file.contains("migration") || file.contains("schema") {
                complexity_score += 2;
                break;
            }

            if file.ends_with(".sql") || file.contains("config") {
                complexity_score += 1;
            }
        }

        match complexity_score {
            0..=1 => ComplexityLevel::Low,
            2..=4 => ComplexityLevel::Medium,
            5..=7 => ComplexityLevel::High,
            _ => ComplexityLevel::Critical,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::task::{Task, TaskPriority, TaskStatus};
    use std::collections::HashMap;

    fn make_task(description: &str) -> Task {
        Task {
            id: "test-1".to_string(),
            title: "Test".to_string(),
            description: description.to_string(),
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

    #[test]
    fn test_analyze_task_low() {
        let task = make_task("fix a small bug");
        assert_eq!(
            ComplexityAnalyzer::analyze_task(&task),
            ComplexityLevel::Low
        );
    }

    #[test]
    fn test_analyze_task_medium() {
        let task = make_task("refactor the login module");
        assert_eq!(
            ComplexityAnalyzer::analyze_task(&task),
            ComplexityLevel::Medium
        );
    }

    #[test]
    fn test_analyze_task_high() {
        let task = make_task("refactor the architecture of the system");
        assert_eq!(
            ComplexityAnalyzer::analyze_task(&task),
            ComplexityLevel::High
        );
    }

    #[test]
    fn test_analyze_task_critical() {
        let task = make_task("security migration architecture refactor with a very long description that definitely exceeds two hundred characters to push the complexity score well beyond the high threshold into critical territory for real this time now please");
        assert_eq!(
            ComplexityAnalyzer::analyze_task(&task),
            ComplexityLevel::Critical
        );
    }

    #[test]
    fn test_analyze_task_empty_description() {
        let task = make_task("");
        assert_eq!(
            ComplexityAnalyzer::analyze_task(&task),
            ComplexityLevel::Low
        );
    }

    #[test]
    fn test_analyze_task_migration_keyword() {
        let task = make_task("database migration to new schema");
        assert_eq!(
            ComplexityAnalyzer::analyze_task(&task),
            ComplexityLevel::Medium
        );
    }

    #[test]
    fn test_analyze_task_security_keyword() {
        let task = make_task("update security headers");
        assert_eq!(
            ComplexityAnalyzer::analyze_task(&task),
            ComplexityLevel::Medium
        );
    }

    #[test]
    fn test_analyze_change_context_low() {
        let result = ComplexityAnalyzer::analyze_change_context(&["file.rs".to_string()], &None);
        assert_eq!(result, ComplexityLevel::Low);
    }

    #[test]
    fn test_analyze_change_context_medium() {
        let files: Vec<String> = (0..12).map(|i| format!("{}.rs", i)).collect();
        let result = ComplexityAnalyzer::analyze_change_context(&files, &None);
        assert_eq!(result, ComplexityLevel::Medium);
    }

    #[test]
    fn test_analyze_change_context_high() {
        let mut files: Vec<String> = (0..12).map(|i| format!("{}.rs", i)).collect();
        files.push("migrations/001.sql".to_string());
        let result = ComplexityAnalyzer::analyze_change_context(
            &files,
            &Some("fix critical bug".to_string()),
        );
        assert_eq!(result, ComplexityLevel::High);
    }

    #[test]
    fn test_analyze_change_context_critical() {
        let mut files: Vec<String> = (0..12).map(|i| format!("{}.rs", i)).collect();
        files.push("migrations/001.sql".to_string());
        let result = ComplexityAnalyzer::analyze_change_context(
            &files,
            &Some("BREAKING: fix critical API change".to_string()),
        );
        assert_eq!(result, ComplexityLevel::Critical);
    }

    #[test]
    fn test_analyze_change_context_no_commit_message() {
        let files = vec!["config.rs".to_string()];
        let result = ComplexityAnalyzer::analyze_change_context(&files, &None);
        assert_eq!(result, ComplexityLevel::Low);
    }

    #[test]
    fn test_analyze_change_context_migration_file() {
        let files = vec!["migrations/001.sql".to_string()];
        let result = ComplexityAnalyzer::analyze_change_context(&files, &None);
        assert_eq!(result, ComplexityLevel::Medium);
    }

    #[test]
    fn test_analyze_change_context_schema_file() {
        let files = vec!["schema.sql".to_string()];
        let result = ComplexityAnalyzer::analyze_change_context(&files, &None);
        assert_eq!(result, ComplexityLevel::Medium);
    }

    #[test]
    fn test_analyze_change_context_empty_files() {
        let result = ComplexityAnalyzer::analyze_change_context(&[], &None);
        assert_eq!(result, ComplexityLevel::Low);
    }

    #[test]
    fn test_analyze_change_context_breaking_keyword() {
        let result = ComplexityAnalyzer::analyze_change_context(
            &["file.rs".to_string()],
            &Some("feat: breaking change in API".to_string()),
        );
        assert_eq!(result, ComplexityLevel::Medium);
    }
}
