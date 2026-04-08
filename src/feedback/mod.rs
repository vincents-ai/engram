//! StructuredFeedback trait for machine-readable output
//!
//! Provides a unified interface for result types to produce JSON,
//! ANSI-stripped text, and status codes for consumers like the TUI,
//! LLM context builders, and programmatic API callers.

use crate::engines::action_executor::ActionResult;
use crate::quality_gates::GateResult;
use crate::validation::ValidationResult;
use crate::Entity;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

/// Unified status for all structured feedback types.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FeedbackStatus {
    Success,
    Failed,
    Warning,
    Skipped,
}

impl std::fmt::Display for FeedbackStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FeedbackStatus::Success => write!(f, "success"),
            FeedbackStatus::Failed => write!(f, "failed"),
            FeedbackStatus::Warning => write!(f, "warning"),
            FeedbackStatus::Skipped => write!(f, "skipped"),
        }
    }
}

/// Trait for machine-readable, structured output from result types.
///
/// Designed to coexist with `Display` — use `Display` for human-facing CLI
/// output (with colors, tables, emoji) and `StructuredFeedback` for
/// machine consumers (JSON, ANSI-stripped text, TUI rendering, LLM context).
pub trait StructuredFeedback {
    /// Canonical machine-readable form.
    fn to_json(&self) -> serde_json::Value;

    /// One-line human summary, ANSI-free.
    fn summary(&self) -> String;

    /// Unified status classification.
    fn status_code(&self) -> FeedbackStatus;

    /// ANSI-stripped version of the primary text output.
    ///
    /// Default implementation strips ANSI escape sequences via regex.
    /// Override if the type has non-standard ANSI sources.
    fn ansi_stripped_output(&self) -> String {
        static RE: OnceLock<Regex> = OnceLock::new();
        let re = RE.get_or_init(|| {
            Regex::new(r"[\x1b\x9b][\[()#;?]*(?:[0-9]{1,4}(?:;[0-9]{0,4})*)?[0-9A-ORZcf-nqry=><]")
                .expect("ANSI regex compiles")
        });
        re.replace_all(&self.summary(), "").to_string()
    }

    /// Pretty-printed JSON string.
    fn to_pretty_json(&self) -> String {
        serde_json::to_string_pretty(&self.to_json()).unwrap_or_else(|_| "{}".to_string())
    }
}

// ---------------------------------------------------------------------------
// Blanket impl for all Entity types
// ---------------------------------------------------------------------------

impl<T: Entity + Serialize> StructuredFeedback for T {
    fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }

    fn summary(&self) -> String {
        format!("[{}] {}", Self::entity_type(), self.id())
    }

    fn status_code(&self) -> FeedbackStatus {
        FeedbackStatus::Success
    }
}

// ---------------------------------------------------------------------------
// Targeted impl: ActionResult
// ---------------------------------------------------------------------------

impl StructuredFeedback for ActionResult {
    fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }

    fn summary(&self) -> String {
        let status = if self.success { "OK" } else { "FAIL" };
        let exit = self
            .exit_code
            .map(|c| format!(" (exit {})", c))
            .unwrap_or_default();
        let detail = self
            .error
            .as_deref()
            .or(self.message.strip_prefix("Action completed: "))
            .unwrap_or(&self.message);
        format!("Action {}: {}{}", status, detail, exit)
    }

    fn status_code(&self) -> FeedbackStatus {
        if self.success {
            FeedbackStatus::Success
        } else {
            FeedbackStatus::Failed
        }
    }
}

// ---------------------------------------------------------------------------
// Targeted impl: ValidationResult
// ---------------------------------------------------------------------------

impl StructuredFeedback for ValidationResult {
    fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }

    fn summary(&self) -> String {
        if self.valid {
            let files = self.validated_files.len();
            let rels = self.validated_relationships.len();
            format!(
                "Validation passed: {} file(s), {} relationship(s) checked",
                files, rels
            )
        } else {
            let count = self.errors.len();
            let first = self
                .errors
                .first()
                .map(|e| e.message.as_str())
                .unwrap_or("unknown error");
            format!("Validation failed: {} error(s) — {}", count, first)
        }
    }

    fn status_code(&self) -> FeedbackStatus {
        if self.valid {
            FeedbackStatus::Success
        } else {
            FeedbackStatus::Failed
        }
    }
}

// ---------------------------------------------------------------------------
// Targeted impl: GateResult
// ---------------------------------------------------------------------------

impl StructuredFeedback for GateResult {
    fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }

    fn summary(&self) -> String {
        let score_str = self
            .score
            .map(|s| format!(" (score: {:.1})", s))
            .unwrap_or_default();
        if self.success {
            format!("Gate [{}] passed{}", self.gate_type, score_str)
        } else {
            let recs = if self.recommendations.is_empty() {
                String::new()
            } else {
                format!(" — {}", self.recommendations.join("; "))
            };
            format!("Gate [{}] failed{}{}", self.gate_type, score_str, recs)
        }
    }

    fn status_code(&self) -> FeedbackStatus {
        if self.success {
            FeedbackStatus::Success
        } else if let Some(score) = self.score {
            if score > 0.5 {
                FeedbackStatus::Warning
            } else {
                FeedbackStatus::Failed
            }
        } else {
            FeedbackStatus::Failed
        }
    }
}

// ---------------------------------------------------------------------------
// Targeted impl: WorkflowExecutionResult
// ---------------------------------------------------------------------------

impl StructuredFeedback for crate::engines::workflow_engine::WorkflowExecutionResult {
    fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }

    fn summary(&self) -> String {
        let status = if self.success { "completed" } else { "failed" };
        format!(
            "Workflow {}: {} — {} ({} event(s))",
            status,
            self.instance_id,
            self.message,
            self.events.len()
        )
    }

    fn status_code(&self) -> FeedbackStatus {
        if self.success {
            FeedbackStatus::Success
        } else {
            FeedbackStatus::Failed
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // -- FeedbackStatus --

    #[test]
    fn feedback_status_display_roundtrip() {
        assert_eq!(FeedbackStatus::Success.to_string(), "success");
        assert_eq!(FeedbackStatus::Failed.to_string(), "failed");
        assert_eq!(FeedbackStatus::Warning.to_string(), "warning");
        assert_eq!(FeedbackStatus::Skipped.to_string(), "skipped");
    }

    #[test]
    fn feedback_status_serializes_lowercase() {
        let json = serde_json::to_value(FeedbackStatus::Warning).unwrap();
        assert_eq!(json.as_str(), Some("warning"));
    }

    // -- ActionResult --

    #[test]
    fn action_result_success_feedback() {
        let r = ActionResult {
            success: true,
            message: "Action completed: all good".into(),
            output: Some("stdout here".into()),
            error: None,
            exit_code: Some(0),
            metadata: HashMap::new(),
        };
        assert_eq!(r.status_code(), FeedbackStatus::Success);
        assert!(r.summary().contains("OK"));
        let json = r.to_json();
        assert_eq!(json["success"], true);
        assert!(r.to_pretty_json().contains("success"));
    }

    #[test]
    fn action_result_failure_feedback() {
        let r = ActionResult {
            success: false,
            message: "Action completed: oops".into(),
            output: None,
            error: Some("boom".into()),
            exit_code: Some(1),
            metadata: HashMap::new(),
        };
        assert_eq!(r.status_code(), FeedbackStatus::Failed);
        assert!(r.summary().contains("FAIL"));
        assert!(r.summary().contains("boom"));
    }

    // -- ValidationResult --

    #[test]
    fn validation_result_passed_feedback() {
        let r = ValidationResult {
            valid: true,
            errors: vec![],
            task_id: Some("task-123".into()),
            validated_relationships: vec!["rel-1".into()],
            validated_files: vec!["foo.rs".into()],
            validation_time_ms: 5,
        };
        assert_eq!(r.status_code(), FeedbackStatus::Success);
        assert!(r.summary().contains("passed"));
        assert!(r.summary().contains("1 file(s)"));
    }

    #[test]
    fn validation_result_failed_feedback() {
        use crate::validation::{ValidationError, ValidationErrorType};
        let r = ValidationResult {
            valid: false,
            errors: vec![ValidationError {
                error_type: ValidationErrorType::NoTaskReference,
                message: "no task ref".into(),
                suggestion: None,
            }],
            task_id: None,
            validated_relationships: vec![],
            validated_files: vec![],
            validation_time_ms: 1,
        };
        assert_eq!(r.status_code(), FeedbackStatus::Failed);
        assert!(r.summary().contains("1 error(s)"));
    }

    // -- GateResult --

    #[test]
    fn gate_result_success_feedback() {
        let r = GateResult {
            gate_type: "complexity".into(),
            success: true,
            score: Some(0.95),
            details: HashMap::new(),
            execution_time_ms: 10,
            recommendations: vec![],
        };
        assert_eq!(r.status_code(), FeedbackStatus::Success);
        assert!(r.summary().contains("passed"));
        assert!(r.summary().contains("0.9"));
    }

    #[test]
    fn gate_result_failed_low_score_feedback() {
        let r = GateResult {
            gate_type: "coverage".into(),
            success: false,
            score: Some(0.3),
            details: HashMap::new(),
            execution_time_ms: 8,
            recommendations: vec!["add more tests".into()],
        };
        assert_eq!(r.status_code(), FeedbackStatus::Failed);
        assert!(r.summary().contains("failed"));
    }

    #[test]
    fn gate_result_failed_medium_score_is_warning() {
        let r = GateResult {
            gate_type: "lint".into(),
            success: false,
            score: Some(0.7),
            details: HashMap::new(),
            execution_time_ms: 3,
            recommendations: vec![],
        };
        assert_eq!(r.status_code(), FeedbackStatus::Warning);
    }

    // -- WorkflowExecutionResult --

    #[test]
    fn workflow_execution_result_success_feedback() {
        use crate::engines::workflow_engine::WorkflowExecutionResult;
        let r = WorkflowExecutionResult {
            success: true,
            instance_id: "inst-abc".into(),
            current_state: "done".into(),
            message: "all good".into(),
            events: vec![],
            variables_changed: HashMap::new(),
        };
        assert_eq!(r.status_code(), FeedbackStatus::Success);
        assert!(r.summary().contains("completed"));
        assert!(r.summary().contains("inst-abc"));
    }

    // -- ANSI stripping --

    #[test]
    fn ansi_stripped_output_removes_escapes() {
        let r = ActionResult {
            success: false,
            message: "\x1b[31merror\x1b[0m: something broke".into(),
            output: None,
            error: None,
            exit_code: Some(1),
            metadata: HashMap::new(),
        };
        let stripped = r.ansi_stripped_output();
        assert!(!stripped.contains('\x1b'));
        assert!(stripped.contains("error: something broke"));
    }

    #[test]
    fn to_pretty_json_is_valid_json() {
        let r = ActionResult {
            success: true,
            message: "ok".into(),
            output: None,
            error: None,
            exit_code: Some(0),
            metadata: HashMap::new(),
        };
        let pretty = r.to_pretty_json();
        let _: serde_json::Value =
            serde_json::from_str(&pretty).expect("pretty output must be valid JSON");
    }
}
