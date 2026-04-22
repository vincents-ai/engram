//! Engram-Evo types for trajectory evaluation and self-improvement.
//!
//! Core data structures:
//! - Trajectory: a parsed pi session (from JSONL or live capture)
//! - Turn: a single turn in the conversation (user message + assistant response + tool results)
//! - ToolCall / ToolResult: individual tool invocations and their results
//! - EvalReport: evaluation results from running metrics
//! - MemoryPatch: a specification for engram entities to write

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A parsed pi session trajectory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trajectory {
    /// Unique session identifier
    pub session_id: String,
    /// Working directory for the session
    pub cwd: String,
    /// Model used (e.g., "claude-sonnet-4-6")
    pub model: String,
    /// Provider (e.g., "anthropic", "github-copilot")
    pub provider: String,
    /// All turns in the session
    pub turns: Vec<Turn>,
    /// Original task description (extracted from first user message)
    pub task_description: Option<String>,
    /// Session creation timestamp (ISO 8601 string)
    pub created_at: DateTime<Utc>,
    /// Total token usage
    pub total_tokens: Option<u64>,
}

/// A single turn in the conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Turn {
    /// Turn index (0-based)
    pub index: usize,
    /// User message content (if present)
    pub user_message: Option<String>,
    /// Assistant thinking/reasoning (if present)
    pub assistant_thinking: Option<String>,
    /// Assistant text response (if present)
    pub assistant_text: Option<String>,
    /// Tool calls made by this turn
    pub tool_calls: Vec<ToolCall>,
    /// Tool results received this turn
    pub tool_results: Vec<ToolResult>,
    /// Turn timestamp (Unix epoch ms)
    pub timestamp: i64,
    /// Why the assistant stopped
    pub stopped_reason: StopReason,
}

/// Tool call invocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    /// Unique tool call ID
    pub id: String,
    /// Tool name (e.g., "bash", "read", "edit")
    pub name: String,
    /// Tool arguments (JSON)
    pub arguments: serde_json::Value,
}

/// Tool result from execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    /// Tool call ID this result is for
    pub tool_call_id: String,
    /// Tool name
    pub tool_name: String,
    /// Result content
    pub content: String,
    /// Whether the tool returned an error
    pub is_error: bool,
    /// Exit code (for bash tool)
    pub exit_code: Option<i32>,
    /// Whether output was truncated
    pub truncated: bool,
}

/// Why the assistant stopped
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum StopReason {
    /// Stopped normally
    Stop,
    /// Stopped due to max tokens
    Length,
    /// Stopped due to tool use
    ToolUse,
    /// Stopped due to error
    Error,
    /// Stopped due to abort
    Aborted,
    /// Unknown
    #[default]
    Unknown,
}

/// Evaluation report for a trajectory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalReport {
    /// Trajectory being evaluated
    pub trajectory_id: String,
    /// Session ID source
    pub session_id: String,
    /// Composite scores
    pub scores: EvalScores,
    /// Per-turn scores
    pub turn_scores: Vec<TurnScore>,
    /// Index of the critical failure turn (if identified)
    pub critical_failure_turn: Option<usize>,
    /// Suggestions for improvement
    pub improvement_suggestions: Vec<String>,
    /// Timestamp of evaluation
    pub evaluated_at: DateTime<Utc>,
}

/// Composite evaluation scores
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EvalScores {
    /// Step efficiency: actual vs optimal steps
    pub step_efficiency: f64,
    /// Tool correctness: % of tools returning success
    pub tool_correctness: f64,
    /// Plan adherence: how well execution matched planning
    pub plan_adherence: f64,
    /// Task completion: whether the task was completed
    pub task_completion: f64,
    /// Weighted composite (0.25 * each)
    pub composite: f64,
}

impl EvalScores {
    /// Compute composite as weighted average
    pub fn compute_composite(&mut self) {
        self.composite = 0.25 * (self.step_efficiency
            + self.tool_correctness
            + self.plan_adherence
            + self.task_completion);
    }
}

/// Per-turn evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnScore {
    /// Turn index
    pub turn_index: usize,
    /// Step efficiency for this turn
    pub step_efficiency: f64,
    /// Tool correctness for this turn
    pub tool_correctness: f64,
    /// Whether this turn caused the failure
    pub is_critical_failure: bool,
}

/// Memory patch specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryPatch {
    /// Index of the turn this patch targets
    pub target_failure_turn: usize,
    /// Rationale for this patch
    pub rationale: String,
    /// Entities to create in engram
    pub entities: Vec<PatchEntity>,
}

/// A single entity to add to engram
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchEntity {
    /// Entity type
    pub entity_type: PatchEntityType,
    /// Entity title
    pub title: String,
    /// Entity content
    pub content: String,
    /// Knowledge type (for knowledge entities)
    pub knowledge_type: Option<PatchKnowledgeType>,
    /// Tags
    pub tags: Vec<String>,
    /// Confidence level (0.0-1.0)
    pub confidence: Option<f64>,
}

/// Patch entity type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatchEntityType {
    /// Knowledge entry
    Knowledge,
    /// Lesson learned
    Lesson,
    /// Context entry
    Context,
    /// Reasoning chain
    Reasoning,
}

/// Knowledge type for patch entities
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PatchKnowledgeType {
    /// A concrete fact
    Fact,
    /// A pattern
    Pattern,
    /// An enforceable rule
    Rule,
    /// A domain concept
    Concept,
    /// A procedure
    Procedure,
    /// A rule of thumb
    Heuristic,
    /// A skill
    Skill,
    /// A technique
    Technique,
}