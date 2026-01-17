use crate::entities::{
    CommitPolicy, QualityGate, TransitionTrigger, Workflow, WorkflowStage, WorkflowTransition,
};
use crate::error::EngramError;
use serde::{Deserialize, Serialize};
use serde_yaml;

#[derive(Debug, Serialize, Deserialize)]
struct WorkflowDefinition {
    name: String,
    description: String,
    stages: Vec<StageDefinition>,
    transitions: Vec<TransitionDefinition>,
}

#[derive(Debug, Serialize, Deserialize)]
struct StageDefinition {
    name: String,
    description: String,
    commit_policy: String,
    quality_gates: Vec<QualityGateDefinition>,
}

#[derive(Debug, Serialize, Deserialize)]
struct QualityGateDefinition {
    command: String,
    required: bool,
    expected_result: Option<String>,
    failure_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TransitionDefinition {
    from: String,
    to: String,
    trigger: String,
}

pub struct WorkflowParser;

impl WorkflowParser {
    pub fn parse(yaml_content: &str) -> Result<Workflow, EngramError> {
        let definition: WorkflowDefinition = serde_yaml::from_str(yaml_content)
            .map_err(|e| EngramError::Validation(format!("Invalid YAML: {}", e)))?;

        let mut workflow = Workflow::new(definition.name, definition.description);

        // Parse stages
        for stage_def in definition.stages {
            let commit_policy = Self::parse_commit_policy(&stage_def.commit_policy)?;
            let quality_gates = stage_def
                .quality_gates
                .into_iter()
                .map(Self::parse_quality_gate)
                .collect();

            let stage = WorkflowStage {
                name: stage_def.name,
                description: stage_def.description,
                commit_policy,
                quality_gates,
            };

            workflow.add_stage(stage);
        }

        // Parse transitions
        for transition_def in definition.transitions {
            let trigger = Self::parse_transition_trigger(&transition_def.trigger)?;

            let transition = WorkflowTransition {
                from: transition_def.from,
                to: transition_def.to,
                trigger,
            };

            workflow.add_transition(transition);
        }

        Ok(workflow)
    }

    fn parse_commit_policy(policy: &str) -> Result<CommitPolicy, EngramError> {
        match policy {
            "engram_only" => Ok(CommitPolicy::EngramOnly),
            "research_artifacts" => Ok(CommitPolicy::ResearchArtifacts),
            "tests_only" => Ok(CommitPolicy::TestsOnly),
            "code_with_tests" => Ok(CommitPolicy::CodeWithTests),
            "full_validation" => Ok(CommitPolicy::FullValidation),
            _ => Err(EngramError::Validation(format!(
                "Unknown commit policy: {}",
                policy
            ))),
        }
    }

    fn parse_transition_trigger(trigger: &str) -> Result<TransitionTrigger, EngramError> {
        match trigger {
            "manual" => Ok(TransitionTrigger::Manual),
            "auto" => Ok(TransitionTrigger::Auto),
            _ => Err(EngramError::Validation(format!(
                "Unknown transition trigger: {}",
                trigger
            ))),
        }
    }

    fn parse_quality_gate(gate_def: QualityGateDefinition) -> QualityGate {
        QualityGate {
            command: gate_def.command,
            required: gate_def.required,
            expected_result: gate_def.expected_result,
            failure_message: gate_def.failure_message,
        }
    }
}
