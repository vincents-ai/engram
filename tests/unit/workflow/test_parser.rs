#[cfg(test)]
mod tests {
    use crate::workflow::parser::WorkflowParser;

    #[test]
    fn test_parse_simple_workflow() {
        let yaml = r#"
name: "Test Workflow"
description: "A test workflow"
stages:
  - name: "development"
    description: "Development stage"
    commit_policy: "code_with_tests"
    quality_gates:
      - command: "cargo test"
        required: true
transitions:
  - from: "development"
    to: "integration"
    trigger: "auto"
"#;

        let workflow = WorkflowParser::parse(yaml).unwrap();

        assert_eq!(workflow.name, "Test Workflow");
        assert_eq!(workflow.stages.len(), 1);
        assert_eq!(workflow.transitions.len(), 1);
    }

    #[test]
    fn test_parse_bdd_workflow() {
        let yaml = r#"
name: "Feature Development"
description: "Complete BDD workflow"
stages:
  - name: "bdd"
    description: "Write failing tests"
    commit_policy: "tests_only"
    quality_gates:
      - command: "cargo test"
        required: true
        expected_result: "failure"
        failure_message: "Tests should fail in BDD phase"
"#;

        let workflow = WorkflowParser::parse(yaml).unwrap();

        assert_eq!(
            workflow.stages[0].quality_gates[0].expected_result,
            Some("failure".to_string())
        );
    }
}
