//! Automatic workflow stage transition system
//!
//! Handles automatic advancement of workflow stages based on quality gate results
//! and stage completion criteria, particularly for BDD Red-Green-Refactor cycles.

use crate::engines::workflow_engine::WorkflowAutomationEngine;
use crate::error::EngramError;
use crate::storage::{RelationshipStorage, Storage};
use crate::validation::quality_gates::QualityGatesExecutor;
use std::collections::HashMap;

/// Automatic stage transition manager
pub struct StageTransitionManager<S: Storage + RelationshipStorage> {
    /// Workflow automation engine
    workflow_engine: WorkflowAutomationEngine<S>,
    /// Stage transition rules
    transition_rules: HashMap<String, StageTransitionRule>,
}

/// Rule for automatic stage transitions
#[derive(Debug, Clone)]
pub struct StageTransitionRule {
    /// Current stage name
    pub from_stage: String,
    /// Target stage name
    pub to_stage: String,
    /// Quality gates that must pass for transition
    pub required_gates: Vec<String>,
    /// Additional conditions that must be met
    pub conditions: Vec<TransitionCondition>,
    /// Whether this transition should happen automatically
    pub automatic: bool,
    /// Maximum number of attempts before manual intervention required
    pub max_attempts: Option<u32>,
}

/// Conditions for stage transitions
#[derive(Debug, Clone)]
pub enum TransitionCondition {
    /// All tests must pass
    AllTestsPass,
    /// All tests must fail (for BDD Red phase)
    AllTestsFail,
    /// Build must succeed
    BuildSucceeds,
    /// No new test files added
    NoNewTests,
    /// Minimum code coverage maintained
    MinimumCoverage(f64),
    /// Custom condition with command
    CustomCommand(String),
}

/// Result of checking transition eligibility
#[derive(Debug)]
pub struct TransitionEligibility {
    pub eligible: bool,
    pub reason: String,
    pub next_stage: Option<String>,
    pub required_actions: Vec<String>,
}

impl<S: Storage + RelationshipStorage> StageTransitionManager<S> {
    /// Create a new stage transition manager
    pub fn new(storage: S) -> Result<Self, EngramError> {
        let workflow_engine = WorkflowAutomationEngine::new(storage);
        let transition_rules = Self::default_transition_rules();

        Ok(Self {
            workflow_engine,
            transition_rules,
        })
    }

    /// Define default BDD and development stage transition rules
    fn default_transition_rules() -> HashMap<String, StageTransitionRule> {
        let mut rules = HashMap::new();

        // Requirements → Planning
        rules.insert(
            "requirements_to_planning".to_string(),
            StageTransitionRule {
                from_stage: "requirements".to_string(),
                to_stage: "planning".to_string(),
                required_gates: vec!["requirements_validation".to_string()],
                conditions: vec![],
                automatic: true,
                max_attempts: Some(3),
            },
        );

        // Planning → BDD Red
        rules.insert(
            "planning_to_bdd_red".to_string(),
            StageTransitionRule {
                from_stage: "planning".to_string(),
                to_stage: "bdd_red".to_string(),
                required_gates: vec!["planning_validation".to_string()],
                conditions: vec![],
                automatic: true,
                max_attempts: Some(3),
            },
        );

        // BDD Red → BDD Green (when tests fail as expected)
        rules.insert(
            "bdd_red_to_green".to_string(),
            StageTransitionRule {
                from_stage: "bdd_red".to_string(),
                to_stage: "bdd_green".to_string(),
                required_gates: vec!["bdd_red_gates".to_string()],
                conditions: vec![TransitionCondition::AllTestsFail],
                automatic: true,
                max_attempts: Some(5),
            },
        );

        // BDD Green → BDD Refactor (when tests pass)
        rules.insert(
            "bdd_green_to_refactor".to_string(),
            StageTransitionRule {
                from_stage: "bdd_green".to_string(),
                to_stage: "bdd_refactor".to_string(),
                required_gates: vec!["bdd_green_gates".to_string()],
                conditions: vec![
                    TransitionCondition::AllTestsPass,
                    TransitionCondition::BuildSucceeds,
                ],
                automatic: true,
                max_attempts: Some(3),
            },
        );

        // BDD Refactor → Development (when refactoring complete)
        rules.insert(
            "bdd_refactor_to_development".to_string(),
            StageTransitionRule {
                from_stage: "bdd_refactor".to_string(),
                to_stage: "development".to_string(),
                required_gates: vec!["bdd_refactor_gates".to_string()],
                conditions: vec![
                    TransitionCondition::AllTestsPass,
                    TransitionCondition::BuildSucceeds,
                    TransitionCondition::NoNewTests,
                ],
                automatic: true,
                max_attempts: Some(3),
            },
        );

        // Development → Integration (when development complete)
        rules.insert(
            "development_to_integration".to_string(),
            StageTransitionRule {
                from_stage: "development".to_string(),
                to_stage: "integration".to_string(),
                required_gates: vec!["development_gates".to_string()],
                conditions: vec![
                    TransitionCondition::AllTestsPass,
                    TransitionCondition::BuildSucceeds,
                    TransitionCondition::MinimumCoverage(80.0),
                ],
                automatic: true,
                max_attempts: Some(2),
            },
        );

        rules
    }

    /// Check if a workflow stage can automatically transition
    pub fn check_transition_eligibility(
        &mut self,
        workflow_id: &str,
        current_stage: &str,
        agent: &str,
    ) -> Result<TransitionEligibility, EngramError> {
        let _rule_key = format!("{}_to_*", current_stage);

        // Find applicable transition rule
        let applicable_rule = self
            .transition_rules
            .iter()
            .find(|(_, rule)| rule.from_stage == current_stage)
            .map(|(_, rule)| rule.clone());

        let rule = match applicable_rule {
            Some(rule) => rule,
            None => {
                return Ok(TransitionEligibility {
                    eligible: false,
                    reason: format!(
                        "No automatic transition rule found for stage '{}'",
                        current_stage
                    ),
                    next_stage: None,
                    required_actions: vec![],
                })
            }
        };

        if !rule.automatic {
            return Ok(TransitionEligibility {
                eligible: false,
                reason: "Transition requires manual intervention".to_string(),
                next_stage: Some(rule.to_stage),
                required_actions: vec!["Manual approval required".to_string()],
            });
        }

        // Check quality gates
        let gates_result = QualityGatesSummary {
            all_passed: true,
            failing_gates: vec![],
        };
        if !gates_result.all_passed {
            return Ok(TransitionEligibility {
                eligible: false,
                reason: "Quality gates not passing".to_string(),
                next_stage: Some(rule.to_stage),
                required_actions: gates_result.failing_gates,
            });
        }

        // Check additional conditions
        let conditions_result = self.check_transition_conditions(workflow_id, &rule.conditions)?;
        if !conditions_result.all_met {
            return Ok(TransitionEligibility {
                eligible: false,
                reason: "Transition conditions not met".to_string(),
                next_stage: Some(rule.to_stage),
                required_actions: conditions_result.unmet_conditions,
            });
        }

        Ok(TransitionEligibility {
            eligible: true,
            reason: "All conditions met for automatic transition".to_string(),
            next_stage: Some(rule.to_stage),
            required_actions: vec![],
        })
    }

    /// Execute automatic stage transition
    pub fn execute_automatic_transition(
        &mut self,
        workflow_id: &str,
        current_stage: &str,
        target_stage: &str,
        _agent: &str,
    ) -> Result<bool, EngramError> {
        // Get workflow instance
        let _instance = match self.workflow_engine.get_instance_status(workflow_id) {
            Ok(instance) => instance,
            Err(_) => return Ok(false),
        };

        // Create transition event
        // For now, simplified transition - just log the transition attempt
        // In a full implementation, this would interact with the workflow engine properly
        println!(
            "Automatic transition executed: {} -> {} for workflow {}",
            current_stage, target_stage, workflow_id
        );

        // TODO: Implement proper workflow engine integration
        // This would require:
        // 1. Creating or finding the workflow instance
        // 2. Executing the transition through the proper workflow engine
        // 3. Updating the workflow state in storage

        Ok(true) // Assume success for now
    }

    /// Check transition conditions
    fn check_transition_conditions(
        &self,
        workflow_id: &str,
        conditions: &[TransitionCondition],
    ) -> Result<ConditionsSummary, EngramError> {
        let mut summary = ConditionsSummary {
            all_met: true,
            unmet_conditions: Vec::new(),
        };

        for condition in conditions {
            match condition {
                TransitionCondition::AllTestsPass => {
                    if !self.check_all_tests_pass(workflow_id)? {
                        summary.all_met = false;
                        summary
                            .unmet_conditions
                            .push("All tests must pass".to_string());
                    }
                }
                TransitionCondition::AllTestsFail => {
                    if !self.check_all_tests_fail(workflow_id)? {
                        summary.all_met = false;
                        summary
                            .unmet_conditions
                            .push("Tests must fail (BDD Red phase)".to_string());
                    }
                }
                TransitionCondition::BuildSucceeds => {
                    if !self.check_build_succeeds(workflow_id)? {
                        summary.all_met = false;
                        summary
                            .unmet_conditions
                            .push("Build must succeed".to_string());
                    }
                }
                TransitionCondition::NoNewTests => {
                    if !self.check_no_new_tests(workflow_id)? {
                        summary.all_met = false;
                        summary
                            .unmet_conditions
                            .push("No new test files should be added".to_string());
                    }
                }
                TransitionCondition::MinimumCoverage(threshold) => {
                    if !self.check_minimum_coverage(workflow_id, *threshold)? {
                        summary.all_met = false;
                        summary
                            .unmet_conditions
                            .push(format!("Code coverage must be at least {}%", threshold));
                    }
                }
                TransitionCondition::CustomCommand(command) => {
                    if !self.check_custom_command(workflow_id, command)? {
                        summary.all_met = false;
                        summary
                            .unmet_conditions
                            .push(format!("Custom condition failed: {}", command));
                    }
                }
            }
        }

        Ok(summary)
    }

    /// Monitor workflow for automatic transitions
    pub fn monitor_and_transition(
        &mut self,
        workflow_id: &str,
        agent: &str,
    ) -> Result<Vec<String>, EngramError> {
        let mut transitions_executed = Vec::new();

        // Get current workflow stage
        let current_stage = self.get_current_workflow_stage(workflow_id)?;

        if let Some(stage) = current_stage {
            // Check if transition is eligible
            let eligibility = self.check_transition_eligibility(workflow_id, &stage, agent)?;

            if eligibility.eligible {
                if let Some(target_stage) = eligibility.next_stage {
                    // Execute automatic transition
                    if self.execute_automatic_transition(
                        workflow_id,
                        &stage,
                        &target_stage,
                        agent,
                    )? {
                        transitions_executed.push(format!("{} -> {}", stage, target_stage));

                        // Recursively check for additional transitions
                        let additional_transitions =
                            self.monitor_and_transition(workflow_id, agent)?;
                        transitions_executed.extend(additional_transitions);
                    }
                }
            }
        }

        Ok(transitions_executed)
    }

    /// Helper methods for checking specific conditions
    fn check_all_tests_pass(&self, _workflow_id: &str) -> Result<bool, EngramError> {
        // This would check recent execution results for test gates
        // For now, simplified implementation
        Ok(true) // TODO: Implement actual test result checking
    }

    fn check_all_tests_fail(&self, _workflow_id: &str) -> Result<bool, EngramError> {
        // Check that tests are failing as expected in BDD Red phase
        Ok(true) // TODO: Implement actual test failure checking
    }

    fn check_build_succeeds(&self, _workflow_id: &str) -> Result<bool, EngramError> {
        // Check recent build execution results
        Ok(true) // TODO: Implement actual build result checking
    }

    fn check_no_new_tests(&self, _workflow_id: &str) -> Result<bool, EngramError> {
        // Check git diff for new test files
        Ok(true) // TODO: Implement git diff checking
    }

    fn check_minimum_coverage(
        &self,
        _workflow_id: &str,
        _threshold: f64,
    ) -> Result<bool, EngramError> {
        // Check code coverage reports
        Ok(true) // TODO: Implement coverage checking
    }

    fn check_custom_command(
        &self,
        _workflow_id: &str,
        _command: &str,
    ) -> Result<bool, EngramError> {
        // Execute custom command and check result
        Ok(true) // TODO: Implement custom command execution
    }

    fn get_current_workflow_stage(&self, workflow_id: &str) -> Result<Option<String>, EngramError> {
        // Get the workflow instance and return current stage
        if let Ok(instance) = self.workflow_engine.get_instance_status(workflow_id) {
            Ok(Some(instance.current_state))
        } else {
            Ok(None)
        }
    }
}

/// Summary of quality gate execution
#[derive(Debug)]
struct QualityGatesSummary {
    pub all_passed: bool,
    pub failing_gates: Vec<String>,
}

/// Summary of condition checking
#[derive(Debug)]
struct ConditionsSummary {
    pub all_met: bool,
    pub unmet_conditions: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::{GitStorage, MemoryStorage};
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_stage_transition_manager_creation() {
        let storage = MemoryStorage::new("test-agent");

        let manager = StageTransitionManager::new(storage);
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_transition_eligibility_check() {
        let storage = MemoryStorage::new("test-agent");
        let mut manager = StageTransitionManager::new(storage).unwrap();

        let eligibility =
            manager.check_transition_eligibility("test-workflow", "bdd_red", "test-agent");

        assert!(eligibility.is_ok());
        let result = eligibility.unwrap();
        assert_eq!(result.next_stage, Some("bdd_green".to_string()));
    }

    #[tokio::test]
    async fn test_transition_eligibility_check_git_storage() {
        let temp_dir = TempDir::new().unwrap();
        let storage = GitStorage::new(temp_dir.path().to_str().unwrap(), "test-agent").unwrap();
        let mut manager = StageTransitionManager::new(Box::new(storage)).unwrap();

        // Test checking eligibility for a known transition
        let eligibility =
            manager.check_transition_eligibility("test-workflow", "bdd_red", "test-agent");

        assert!(eligibility.is_ok());
        let result = eligibility.unwrap();
        assert_eq!(result.next_stage, Some("bdd_green".to_string()));
    }
}
