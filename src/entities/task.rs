//! Task entity implementation

use super::{Entity, GenericEntity};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

/// Task status variants
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Todo,
    // agentic-loop historically wrote both `in_progress` and `inprogress`.
    #[serde(alias = "in_progress")]
    InProgress,
    Done,
    Blocked,
    Cancelled,
    /// Awaiting review. This is a distinct state, never coerced to another one.
    Review,
}

/// Task priority variants
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// One period during which the task was blocked.
///
/// A closed period records both ends; the open period (while the task is
/// blocked) is held in [`Task::blocked_at`] until it is closed, so the two
/// never describe the same block twice.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
pub struct BlockedInterval {
    /// When the block started
    #[serde(rename = "blocked_at")]
    pub blocked_at: DateTime<Utc>,

    /// When the block ended; `None` only for the open period
    #[serde(rename = "unblocked_at", skip_serializing_if = "Option::is_none", default)]
    pub unblocked_at: Option<DateTime<Utc>>,

    /// The reason recorded for this block, kept verbatim
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none", default)]
    pub reason: Option<String>,
}

/// Task entity representing a work item with status tracking
#[derive(Debug, Clone, Serialize, Deserialize, Validate, schemars::JsonSchema)]
pub struct Task {
    /// Unique identifier
    #[serde(rename = "id")]
    pub id: String,

    /// Task title
    #[serde(rename = "title")]
    pub title: String,

    /// Detailed description
    #[serde(rename = "description")]
    pub description: String,

    /// Current status
    #[serde(rename = "status")]
    pub status: TaskStatus,

    /// Priority level
    #[serde(rename = "priority")]
    pub priority: TaskPriority,

    /// Assigned agent
    #[serde(rename = "agent")]
    pub agent: String,

    /// Start time
    #[serde(rename = "start_time")]
    pub start_time: DateTime<Utc>,

    /// End time
    #[serde(rename = "end_time")]
    pub end_time: Option<DateTime<Utc>>,

    /// Parent task ID
    #[serde(rename = "parent", skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,

    /// Child task IDs
    #[serde(rename = "children", skip_serializing_if = "Vec::is_empty", default)]
    pub children: Vec<String>,

    /// Tags for categorization
    #[serde(rename = "tags", skip_serializing_if = "Vec::is_empty", default)]
    pub tags: Vec<String>,

    /// Associated context IDs
    #[serde(rename = "context_ids", skip_serializing_if = "Vec::is_empty", default)]
    pub context_ids: Vec<String>,

    /// Knowledge items
    #[serde(rename = "knowledge", skip_serializing_if = "Vec::is_empty", default)]
    pub knowledge: Vec<String>,

    /// Related files
    #[serde(rename = "files", skip_serializing_if = "Vec::is_empty", default)]
    pub files: Vec<String>,

    /// Task outcome
    #[serde(rename = "outcome", skip_serializing_if = "Option::is_none")]
    pub outcome: Option<String>,

    /// Reason for blocking the task
    #[serde(rename = "block_reason", skip_serializing_if = "Option::is_none")]
    pub block_reason: Option<String>,

    /// When the current blocked period started.
    ///
    /// Present only while the task is blocked. Set when a task *enters*
    /// `Blocked`; never inferred for a task that was already blocked before
    /// this field existed, because that timestamp is unknown.
    #[serde(rename = "blocked_at", skip_serializing_if = "Option::is_none", default)]
    pub blocked_at: Option<DateTime<Utc>>,

    /// Closed blocked periods, oldest first.
    ///
    /// Together with [`Task::blocked_at`] this is the block history that
    /// active duration is computed from: elapsed time minus blocked time.
    #[serde(rename = "blocked_intervals", skip_serializing_if = "Vec::is_empty", default)]
    pub blocked_intervals: Vec<BlockedInterval>,

    #[serde(rename = "workflow_id", skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,

    #[serde(rename = "workflow_state", skip_serializing_if = "Option::is_none")]
    pub workflow_state: Option<String>,

    /// Additional metadata
    #[serde(
        rename = "metadata",
        skip_serializing_if = "HashMap::is_empty",
        default
    )]
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Task {
    /// Create a new task
    pub fn new(
        title: String,
        description: String,
        agent: String,
        priority: TaskPriority,
        workflow_id: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            description,
            status: TaskStatus::Todo,
            priority,
            agent,
            start_time: now,
            end_time: None,
            parent: None,
            children: Vec::new(),
            tags: Vec::new(),
            context_ids: Vec::new(),
            knowledge: Vec::new(),
            files: Vec::new(),
            outcome: None,
            workflow_id,
            workflow_state: None,
            block_reason: None,
            blocked_at: None,
            blocked_intervals: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Mark task as in progress
    ///
    /// Leaving `Blocked` for any other state closes the open blocked period.
    pub fn start(&mut self) {
        self.end_block();
        self.status = TaskStatus::InProgress;
        self.block_reason = None;
    }

    /// Block the task
    ///
    /// Entering `Blocked` from another state records the start of a blocked
    /// period. Blocking an already-blocked task updates the reason only: it
    /// does not restart the clock, and it does not invent a start time for a
    /// task that was blocked before this field was recorded.
    pub fn block(&mut self, reason: String) {
        if self.status != TaskStatus::Blocked {
            let now = Utc::now();
            self.blocked_at = Some(now);
        }
        self.status = TaskStatus::Blocked;
        self.block_reason = Some(reason);
    }

    /// Close the open blocked period, if there is one.
    ///
    /// Called whenever a task leaves `Blocked` by a route that does not go
    /// through [`Task::start`], [`Task::complete`] or [`Task::cancel`].
    pub fn end_block(&mut self) {
        if let Some(blocked_at) = self.blocked_at.take() {
            self.blocked_intervals.push(BlockedInterval {
                blocked_at,
                unblocked_at: Some(Utc::now()),
                reason: self.block_reason.clone(),
            });
        }
    }

    /// Every blocked period this task records, closed and open, oldest first.
    ///
    /// Empty for a task that was blocked before block history was recorded:
    /// nothing is invented.
    pub fn blocked_periods(&self) -> Vec<BlockedInterval> {
        let mut periods = self.blocked_intervals.clone();
        if let Some(blocked_at) = self.blocked_at {
            periods.push(BlockedInterval {
                blocked_at,
                unblocked_at: None,
                reason: self.block_reason.clone(),
            });
        }
        periods.sort_by_key(|period| period.blocked_at);
        periods
    }

    /// Update workflow state
    pub fn update_workflow_state(&mut self, state: String) {
        self.workflow_state = Some(state);
    }

    /// Complete the task
    pub fn complete(&mut self, outcome: String) {
        self.end_block();
        self.status = TaskStatus::Done;
        self.end_time = Some(Utc::now());
        self.outcome = Some(outcome);
    }

    /// Cancel the task, closing any open blocked period
    pub fn cancel(&mut self, note: Option<String>) {
        self.end_block();
        self.status = TaskStatus::Cancelled;
        if let Some(note) = note {
            if self.outcome.is_none() {
                self.outcome = Some(note);
            }
        }
    }

    /// Add a child task
    pub fn add_child(&mut self, child_id: String) {
        if !self.children.contains(&child_id) {
            self.children.push(child_id);
        }
    }

    /// Add a tag
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }
}

impl Entity for Task {
    fn entity_type() -> &'static str {
        "task"
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn agent(&self) -> &str {
        &self.agent
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.start_time
    }

    fn validate_entity(&self) -> crate::Result<()> {
        if self.title.is_empty() {
            return Err(crate::EngramError::Validation(
                "Task title cannot be empty".to_string(),
            ));
        }

        if self.agent.is_empty() {
            return Err(crate::EngramError::Validation(
                "Task agent cannot be empty".to_string(),
            ));
        }

        Ok(())
    }

    fn to_generic(&self) -> GenericEntity {
        GenericEntity {
            id: self.id.clone(),
            entity_type: Self::entity_type().to_string(),
            agent: self.agent.clone(),
            timestamp: self.start_time,
            data: serde_json::to_value(self).unwrap_or_default(),
        }
    }

    fn from_generic(entity: GenericEntity) -> crate::Result<Self> {
        serde_json::from_value(entity.data).map_err(|e| {
            crate::EngramError::Deserialization(format!("Failed to deserialize Task: {}", e))
        })
    }

    fn as_any(&self) -> &dyn std::any::Any
    where
        Self: Sized,
    {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_creation() {
        let task = Task::new(
            "Test Task".to_string(),
            "Description".to_string(),
            "agent-1".to_string(),
            TaskPriority::High,
            None,
        );

        assert_eq!(task.title, "Test Task");
        assert_eq!(task.status, TaskStatus::Todo);
        assert_eq!(task.priority, TaskPriority::High);
    }

    #[test]
    fn test_task_lifecycle() {
        let mut task = Task::new(
            "Test".to_string(),
            "Desc".to_string(),
            "agent".to_string(),
            TaskPriority::Medium,
            None,
        );

        // Start
        task.start();
        assert_eq!(task.status, TaskStatus::InProgress);

        // Block
        task.block("Waiting".to_string());
        assert_eq!(task.status, TaskStatus::Blocked);
        assert_eq!(task.block_reason, Some("Waiting".to_string()));

        // Complete
        task.complete("Done".to_string());
        assert_eq!(task.status, TaskStatus::Done);
        assert!(task.end_time.is_some());
        assert_eq!(task.outcome, Some("Done".to_string()));
    }

    #[test]
    fn task_status_round_trips_agentic_loop_vocabulary() {
        // Both spellings agentic-loop has written must deserialize to InProgress.
        for input in ["in_progress", "inprogress"] {
            let deserialized: TaskStatus =
                serde_json::from_str(&format!("\"{}\"", input)).unwrap();
            assert_eq!(deserialized, TaskStatus::InProgress);
            assert_eq!(
                serde_json::to_string(&deserialized).unwrap(),
                "\"inprogress\""
            );
        }

        // Every accepted vocabulary value round-trips.
        let vocabulary = [
            ("todo", TaskStatus::Todo),
            ("inprogress", TaskStatus::InProgress),
            ("review", TaskStatus::Review),
            ("done", TaskStatus::Done),
            ("blocked", TaskStatus::Blocked),
            ("cancelled", TaskStatus::Cancelled),
        ];
        for (input, expected) in vocabulary {
            let deserialized: TaskStatus =
                serde_json::from_str(&format!("\"{}\"", input)).unwrap();
            assert_eq!(deserialized, expected);
            assert_eq!(
                serde_json::to_string(&deserialized).unwrap(),
                format!("\"{}\"", input)
            );
        }

        // `review` must stay review, never coerced to another state.
        let review: TaskStatus = serde_json::from_str("\"review\"").unwrap();
        assert_eq!(review, TaskStatus::Review);
        assert_eq!(serde_json::to_string(&review).unwrap(), "\"review\"");

        // Genuinely unknown values are rejected.
        for unknown in ["\"archived\"", "\"pending\"", "\"REVIEW\"", "\"\""] {
            assert!(
                serde_json::from_str::<TaskStatus>(unknown).is_err(),
                "expected {:?} to be rejected",
                unknown
            );
        }
    }

    #[test]
    fn test_task_validation() {
        let mut task = Task::new(
            "".to_string(), // Invalid empty title
            "Desc".to_string(),
            "agent".to_string(),
            TaskPriority::Low,
            None,
        );

        assert!(task.validate_entity().is_err());

        task.title = "Valid Title".to_string();
        assert!(task.validate_entity().is_ok());

        task.agent = "".to_string(); // Invalid empty agent
        assert!(task.validate_entity().is_err());
    }

    fn new_task() -> Task {
        Task::new(
            "Test".to_string(),
            "Desc".to_string(),
            "agent".to_string(),
            TaskPriority::Medium,
            None,
        )
    }

    #[test]
    fn blocking_records_the_start_and_unblocking_closes_the_interval() {
        let mut task = new_task();
        task.start();
        task.block("Waiting".to_string());

        let blocked_at = task.blocked_at.expect("blocking records a start");
        assert_eq!(task.status, TaskStatus::Blocked);
        assert!(
            task.blocked_intervals.is_empty(),
            "the open block is held in blocked_at, not yet in the closed list"
        );

        task.start();

        assert!(task.blocked_at.is_none(), "the open block is closed");
        assert_eq!(task.blocked_intervals.len(), 1);
        let interval = &task.blocked_intervals[0];
        assert_eq!(interval.blocked_at, blocked_at);
        assert!(interval.unblocked_at.is_some());
        assert_eq!(interval.reason.as_deref(), Some("Waiting"));
        assert_eq!(task.blocked_periods(), task.blocked_intervals);
    }

    #[test]
    fn a_second_block_while_blocked_does_not_restart_the_clock() {
        let mut task = new_task();
        task.start();
        task.block("Waiting on CI".to_string());
        let blocked_at = task.blocked_at.expect("blocking records a start");

        task.block("Waiting on a human".to_string());

        assert_eq!(
            task.blocked_at,
            Some(blocked_at),
            "re-blocking must not move the start of the period"
        );
        assert_eq!(task.block_reason.as_deref(), Some("Waiting on a human"));
        assert!(
            task.blocked_intervals.is_empty(),
            "re-blocking must not close the period it is still inside"
        );
    }

    #[test]
    fn completing_a_blocked_task_closes_the_interval() {
        let mut task = new_task();
        task.start();
        task.block("Waiting".to_string());
        let blocked_at = task.blocked_at.expect("blocking records a start");

        task.complete("Done anyway".to_string());

        assert_eq!(task.status, TaskStatus::Done);
        assert!(task.blocked_at.is_none());
        assert_eq!(task.blocked_intervals.len(), 1);
        assert_eq!(task.blocked_intervals[0].blocked_at, blocked_at);
        assert!(task.blocked_intervals[0].unblocked_at.is_some());
    }

    #[test]
    fn two_blocked_periods_are_kept_oldest_first() {
        let mut task = new_task();
        task.start();
        task.block("First".to_string());
        let first = task.blocked_at.expect("first block recorded");
        task.start();
        task.block("Second".to_string());
        let second = task.blocked_at.expect("second block recorded");
        assert!(second >= first);

        let periods = task.blocked_periods();
        assert_eq!(periods.len(), 2, "closed plus open, no duplicates");
        assert_eq!(periods[0].blocked_at, first);
        assert_eq!(periods[0].reason.as_deref(), Some("First"));
        assert!(periods[0].unblocked_at.is_some());
        assert_eq!(periods[1].blocked_at, second);
        assert_eq!(periods[1].reason.as_deref(), Some("Second"));
        assert!(periods[1].unblocked_at.is_none());
        assert_eq!(task.blocked_intervals.len(), 1);
    }

    #[test]
    fn a_task_blocked_before_history_existed_has_no_invented_timestamp() {
        let mut task = new_task();
        // A legacy payload: blocked, but no blocked_at and no history.
        task.status = TaskStatus::Blocked;
        task.block_reason = Some("Legacy".to_string());

        assert!(task.blocked_at.is_none());
        assert_eq!(task.blocked_periods(), Vec::new());

        // Leaving the legacy block records nothing either.
        task.start();
        assert!(task.blocked_at.is_none());
        assert!(task.blocked_intervals.is_empty());
    }

    #[test]
    fn block_history_round_trips_through_a_generic_entity() {
        let mut task = new_task();
        task.id = "task-block-history".to_string();
        task.start();
        task.block("First".to_string());
        task.start();
        task.block("Second".to_string());

        let generic = task.to_generic();
        let restored = Task::from_generic(generic).expect("task round-trips");

        assert_eq!(restored.blocked_at, task.blocked_at);
        assert_eq!(restored.blocked_intervals, task.blocked_intervals);
        assert_eq!(restored.blocked_periods(), task.blocked_periods());
        assert_eq!(restored.block_reason, task.block_reason);
    }
}
