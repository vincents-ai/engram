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
