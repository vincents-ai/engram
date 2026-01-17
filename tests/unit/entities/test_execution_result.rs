#[cfg(test)]
mod tests {
    use engram::entities::*;
    use uuid::Uuid;

    #[test]
    fn test_execution_result_creation() {
        let result = ExecutionResult::new(
            Uuid::new_v4().to_string(),
            "development".to_string(),
            "cargo test".to_string(),
            0,
            "test passed".to_string(),
            "".to_string(),
        );

        assert_eq!(result.exit_code, 0);
        assert_eq!(result.command, "cargo test");
        assert_eq!(result.validation_status, ValidationStatus::Passed);
    }

    #[test]
    fn test_execution_result_failure() {
        let result = ExecutionResult::new(
            Uuid::new_v4().to_string(),
            "development".to_string(),
            "cargo test".to_string(),
            1,
            "".to_string(),
            "test failed".to_string(),
        );

        assert_eq!(result.exit_code, 1);
        match result.validation_status {
            ValidationStatus::Failed { reason: _ } => {}
            _ => panic!("Expected failed status"),
        }
    }
}
