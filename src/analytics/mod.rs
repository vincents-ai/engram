//! Analytics module for Engram
//!
//! Provides SPACE framework and DORA metrics calculation
//! for productivity analysis and session management.

use crate::error::EngramError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// SPACE framework metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceMetrics {
    pub satisfaction_score: f64,
    pub performance_score: f64,
    pub activity_score: f64,
    pub communication_score: f64,
    pub efficiency_score: f64,
    pub overall_score: f64,
}

/// DORA metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoraMetrics {
    pub deployment_frequency: u32,
    pub lead_time: f64,
    pub change_failure_rate: f64,
    pub mean_time_to_recover: f64,
}

/// Session analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionAnalytics {
    pub session_id: String,
    pub agent_name: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub space_metrics: SpaceMetrics,
    pub dora_metrics: DoraMetrics,
    pub tasks_completed: u32,
    pub context_items_created: u32,
    pub reasoning_steps_taken: u32,
    pub knowledge_items_added: u32,
}

impl SessionAnalytics {
    pub fn calculate_productivity_score(&self) -> f64 {
        let base_score = self.space_metrics.overall_score;
        let task_weight = 0.3;
        let activity_weight = 0.4;
        let duration_weight = 0.3;

        let duration_factor = if let Some(end_time) = self.end_time {
            let duration = end_time
                .signed_duration_since(self.start_time)
                .num_minutes() as f64;
            if duration > 480.0 {
                0.7
            } else if duration > 240.0 {
                0.85
            } else {
                1.0
            }
        } else {
            0.5
        };

        base_score * (task_weight + activity_weight * duration_factor + duration_weight)
    }

    pub fn generate_summary(&self) -> String {
        let end_status = if let Some(end_time) = self.end_time {
            format!("ended at {}", end_time.format("%Y-%m-%d %H:%M"))
        } else {
            "ongoing".to_string()
        };

        format!(
            "Session {}: {} ({}) | Productivity: {:.2}",
            self.session_id,
            self.agent_name,
            end_status,
            self.calculate_productivity_score()
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FatigueAnalysis {
    pub optimal_session_length: u32,
    pub current_session_length: u32,
    pub fatigue_risk_level: String,
    pub recommendation: String,
}

impl FatigueAnalysis {
    pub fn analyze(&self, session_duration_minutes: u32) -> &str {
        if session_duration_minutes > self.optimal_session_length * 2 {
            "high"
        } else if session_duration_minutes > self.optimal_session_length {
            "medium"
        } else {
            "low"
        }
    }
}
