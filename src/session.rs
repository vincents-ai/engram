//! Session management for Engram
//!
//! Provides agent session lifecycle management with real-time analytics
//! and productivity tracking using SPACE framework.

use crate::analytics::{DoraMetrics, FatigueAnalysis, SessionAnalytics, SpaceMetrics};
use crate::error::EngramError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Active session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub agent_name: String,
    pub start_time: DateTime<Utc>,
    pub status: SessionStatus,
    pub current_task: Option<String>,
    pub context_stack: Vec<String>,
    pub analytics: SessionAnalytics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionStatus {
    Active,
    Paused,
    Completed,
    Failed,
}

impl Session {
    pub fn new(agent_name: String) -> Self {
        let now = Utc::now();
        let session_id = uuid::Uuid::new_v4().to_string();
        Self {
            id: session_id.clone(),
            agent_name: agent_name.clone(),
            start_time: now,
            status: SessionStatus::Active,
            current_task: None,
            context_stack: vec![],
            analytics: SessionAnalytics {
                session_id,
                agent_name,
                start_time: now,
                end_time: None,
                space_metrics: SpaceMetrics {
                    satisfaction_score: 8.0,
                    performance_score: 7.0,
                    activity_score: 8.0,
                    communication_score: 5.0,
                    efficiency_score: 8.0,
                    overall_score: 7.5,
                },
                dora_metrics: DoraMetrics {
                    deployment_frequency: 0,
                    lead_time: 2.5,
                    change_failure_rate: 0.0,
                    mean_time_to_recover: 0.0,
                },
                tasks_completed: 0,
                context_items_created: 0,
                reasoning_steps_taken: 0,
                knowledge_items_added: 0,
            },
        }
    }

    pub fn end_session(&mut self) {
        self.status = SessionStatus::Completed;
        self.analytics.end_time = Some(Utc::now());
    }

    pub fn pause_session(&mut self) {
        self.status = SessionStatus::Paused;
    }

    pub fn resume_session(&mut self) {
        self.status = SessionStatus::Active;
    }

    pub fn update_task(&mut self, task: String) {
        self.current_task = Some(task);
    }

    pub fn add_context(&mut self, context: String) {
        self.context_stack.push(context);
        self.analytics.context_items_created += 1;
    }

    pub fn complete_task(&mut self) {
        if self.current_task.is_some() {
            self.analytics.tasks_completed += 1;
            self.current_task = None;
        }
    }

    pub fn add_reasoning_step(&mut self) {
        self.analytics.reasoning_steps_taken += 1;
    }

    pub fn add_knowledge_item(&mut self) {
        self.analytics.knowledge_items_added += 1;
    }
}
