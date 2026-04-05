//! Analytics module for Engram
//!
//! Provides SPACE framework and DORA metrics calculation
//! for productivity analysis and session management.

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

/// Task duration analytics report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDurationReport {
    pub period_days: u32,
    pub total_tasks: u32,
    pub completed_tasks: u32,
    pub avg_completion_time_seconds: f64,
    pub median_completion_time_seconds: f64,
    pub min_completion_time_seconds: f64,
    pub max_completion_time_seconds: f64,
    pub tasks_by_agent: Vec<AgentTaskMetrics>,
    pub tasks_by_priority: Vec<PriorityTaskMetrics>,
    pub throughput_per_day: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTaskMetrics {
    pub agent: String,
    pub tasks_completed: u32,
    pub avg_duration_seconds: f64,
    pub total_seconds: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorityTaskMetrics {
    pub priority: String,
    pub tasks_completed: u32,
    pub avg_duration_seconds: f64,
}

/// Workflow stage analytics report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStageReport {
    pub workflow_id: String,
    pub total_instances: u32,
    pub completed_instances: u32,
    pub failed_instances: u32,
    pub avg_cycle_time_seconds: f64,
    pub stage_dwell_times: Vec<StageDwellMetrics>,
    pub transition_counts: Vec<TransitionMetrics>,
    pub completion_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageDwellMetrics {
    pub stage_name: String,
    pub avg_time_seconds: f64,
    pub min_time_seconds: f64,
    pub max_time_seconds: f64,
    pub total_entries: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionMetrics {
    pub from_state: String,
    pub to_state: String,
    pub count: u32,
    pub avg_transition_time_seconds: f64,
}

/// Bottleneck analysis report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BottleneckReport {
    pub generated_at: DateTime<Utc>,
    pub period_days: u32,
    pub overall_metrics: BottleneckOverallMetrics,
    pub task_bottlenecks: Vec<TaskBottleneck>,
    pub workflow_bottlenecks: Vec<WorkflowBottleneck>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BottleneckOverallMetrics {
    pub total_tasks_created: u32,
    pub total_tasks_completed: u32,
    pub completion_rate: f64,
    pub avg_cycle_time_seconds: f64,
    pub blocked_task_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskBottleneck {
    pub task_id: String,
    pub title: String,
    pub agent: String,
    pub time_in_current_status_seconds: f64,
    pub status: String,
    pub priority: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowBottleneck {
    pub workflow_id: String,
    pub stage_name: String,
    pub avg_dwell_time_seconds: f64,
    pub total_instances_stuck: u32,
    pub severity: String,
}

use crate::entities::{Entity, Task, TaskStatus};
use crate::storage::Storage;
use std::collections::HashMap;

impl TaskDurationReport {
    pub fn generate<S: Storage>(storage: &S, days: u32) -> Result<Self, String> {
        let cutoff = Utc::now() - chrono::Duration::days(days as i64);
        let task_ids = storage.list_ids("task").map_err(|e| e.to_string())?;

        let mut durations: Vec<f64> = Vec::new();
        let mut agent_metrics: HashMap<String, (u32, f64)> = HashMap::new();
        let mut priority_metrics: HashMap<String, (u32, f64)> = HashMap::new();
        let mut total_tasks = 0;
        let mut completed_tasks = 0;

        for id in task_ids {
            if let Ok(Some(entity)) = storage.get(&id, "task") {
                if let Ok(task) = Task::from_generic(entity) {
                    total_tasks += 1;

                    if let Some(end_time) = task.end_time {
                        if end_time > cutoff {
                            completed_tasks += 1;
                            let duration_seconds = end_time
                                .signed_duration_since(task.start_time)
                                .num_seconds()
                                as f64;
                            durations.push(duration_seconds);

                            agent_metrics
                                .entry(task.agent.clone())
                                .and_modify(|(count, total)| {
                                    *count += 1;
                                    *total += duration_seconds;
                                })
                                .or_insert((1, duration_seconds));

                            let priority = format!("{:?}", task.priority).to_lowercase();
                            priority_metrics
                                .entry(priority)
                                .and_modify(|(count, total)| {
                                    *count += 1;
                                    *total += duration_seconds;
                                })
                                .or_insert((1, duration_seconds));
                        }
                    }
                }
            }
        }

        durations.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let avg = durations.iter().sum::<f64>() / durations.len().max(1) as f64;
        let median = if durations.is_empty() {
            0.0
        } else {
            durations[durations.len() / 2]
        };
        let min = durations.first().copied().unwrap_or(0.0);
        let max = durations.last().copied().unwrap_or(0.0);

        let tasks_by_agent: Vec<AgentTaskMetrics> = agent_metrics
            .into_iter()
            .map(|(agent, (count, total))| AgentTaskMetrics {
                agent,
                tasks_completed: count,
                avg_duration_seconds: total / count.max(1) as f64,
                total_seconds: total,
            })
            .collect();

        let tasks_by_priority: Vec<PriorityTaskMetrics> = priority_metrics
            .into_iter()
            .map(|(priority, (count, total))| PriorityTaskMetrics {
                priority,
                tasks_completed: count,
                avg_duration_seconds: total / count.max(1) as f64,
            })
            .collect();

        Ok(TaskDurationReport {
            period_days: days,
            total_tasks,
            completed_tasks,
            avg_completion_time_seconds: avg,
            median_completion_time_seconds: median,
            min_completion_time_seconds: min,
            max_completion_time_seconds: max,
            tasks_by_agent,
            tasks_by_priority,
            throughput_per_day: completed_tasks as f64 / days.max(1) as f64,
        })
    }
}

impl WorkflowStageReport {
    pub fn generate<S: Storage + 'static>(
        storage: S,
        workflow_id: &str,
        days: u32,
    ) -> Result<Self, String> {
        use crate::engines::workflow_engine::{WorkflowAutomationEngine, WorkflowStatus};

        let cutoff = Utc::now() - chrono::Duration::days(days as i64);
        let engine = WorkflowAutomationEngine::new(storage);
        let instances = engine.list_active_instances();

        let filtered_instances: Vec<_> = instances
            .into_iter()
            .filter(|i| i.workflow_id == workflow_id && i.started_at > cutoff)
            .collect();

        let completed_count = filtered_instances
            .iter()
            .filter(|i| matches!(i.status, WorkflowStatus::Completed))
            .count() as u32;
        let failed_count = filtered_instances
            .iter()
            .filter(|i| matches!(i.status, WorkflowStatus::Failed(_)))
            .count() as u32;

        let mut cycle_times: Vec<f64> = Vec::new();
        let mut stage_times: HashMap<String, Vec<f64>> = HashMap::new();
        let mut transitions: HashMap<(String, String), u32> = HashMap::new();

        for instance in &filtered_instances {
            if let Some(completed) = instance.completed_at {
                cycle_times.push(
                    completed
                        .signed_duration_since(instance.started_at)
                        .num_seconds() as f64,
                );
            }

            let _prev_state: Option<String> = None;
            for event in &instance.execution_history {
                if let Some(to) = &event.to_state {
                    let dwell = event
                        .timestamp
                        .signed_duration_since(instance.started_at)
                        .num_seconds() as f64;
                    stage_times
                        .entry(to.clone())
                        .and_modify(|v| v.push(dwell))
                        .or_insert_with(|| vec![dwell]);
                }

                if let (Some(from), Some(to)) = (&event.from_state, &event.to_state) {
                    *transitions.entry((from.clone(), to.clone())).or_insert(0) += 1;
                }
            }
        }

        let avg_cycle = cycle_times.iter().sum::<f64>() / cycle_times.len().max(1) as f64;

        let stage_dwell_times: Vec<StageDwellMetrics> = stage_times
            .into_iter()
            .map(|(stage, times)| {
                let mut sorted = times.clone();
                sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
                StageDwellMetrics {
                    stage_name: stage,
                    avg_time_seconds: times.iter().sum::<f64>() / times.len().max(1) as f64,
                    min_time_seconds: sorted.first().copied().unwrap_or(0.0),
                    max_time_seconds: sorted.last().copied().unwrap_or(0.0),
                    total_entries: times.len() as u32,
                }
            })
            .collect();

        let transition_counts: Vec<TransitionMetrics> = transitions
            .into_iter()
            .map(|((from, to), count)| TransitionMetrics {
                from_state: from,
                to_state: to,
                count,
                avg_transition_time_seconds: 0.0,
            })
            .collect();

        Ok(WorkflowStageReport {
            workflow_id: workflow_id.to_string(),
            total_instances: filtered_instances.len() as u32,
            completed_instances: completed_count,
            failed_instances: failed_count,
            avg_cycle_time_seconds: avg_cycle,
            stage_dwell_times,
            transition_counts,
            completion_rate: if filtered_instances.is_empty() {
                0.0
            } else {
                completed_count as f64 / filtered_instances.len() as f64
            },
        })
    }
}

impl Default for BottleneckReport {
    fn default() -> Self {
        Self {
            generated_at: Utc::now(),
            period_days: 0,
            overall_metrics: BottleneckOverallMetrics {
                total_tasks_created: 0,
                total_tasks_completed: 0,
                completion_rate: 0.0,
                avg_cycle_time_seconds: 0.0,
                blocked_task_rate: 0.0,
            },
            task_bottlenecks: vec![],
            workflow_bottlenecks: vec![],
            recommendations: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_productivity_score_calculation() {
        let metrics = SessionAnalytics {
            session_id: "test".to_string(),
            agent_name: "test".to_string(),
            start_time: Utc::now(),
            end_time: Some(Utc::now() + chrono::Duration::minutes(60)),
            space_metrics: SpaceMetrics {
                satisfaction_score: 0.8,
                performance_score: 0.8,
                activity_score: 0.8,
                communication_score: 0.8,
                efficiency_score: 0.8,
                overall_score: 0.8,
            },
            dora_metrics: DoraMetrics {
                deployment_frequency: 0,
                lead_time: 0.0,
                change_failure_rate: 0.0,
                mean_time_to_recover: 0.0,
            },
            tasks_completed: 5,
            context_items_created: 10,
            reasoning_steps_taken: 50,
            knowledge_items_added: 2,
        };

        let score = metrics.calculate_productivity_score();
        assert!(score > 0.0);
    }

    #[test]
    fn test_fatigue_analysis() {
        let analysis = FatigueAnalysis {
            optimal_session_length: 60,
            current_session_length: 0,
            fatigue_risk_level: "low".to_string(),
            recommendation: "none".to_string(),
        };

        assert_eq!(analysis.analyze(30), "low");
        assert_eq!(analysis.analyze(70), "medium");
        assert_eq!(analysis.analyze(130), "high");
    }

    #[test]
    fn test_productivity_score_duration_impact() {
        let mut metrics = SessionAnalytics {
            session_id: "test".to_string(),
            agent_name: "test".to_string(),
            start_time: Utc::now(),
            end_time: Some(Utc::now() + chrono::Duration::minutes(60)), // 1 hour (factor 1.0)
            space_metrics: SpaceMetrics {
                satisfaction_score: 1.0,
                performance_score: 1.0,
                activity_score: 1.0,
                communication_score: 1.0,
                efficiency_score: 1.0,
                overall_score: 1.0,
            },
            dora_metrics: DoraMetrics {
                deployment_frequency: 0,
                lead_time: 0.0,
                change_failure_rate: 0.0,
                mean_time_to_recover: 0.0,
            },
            tasks_completed: 5,
            context_items_created: 10,
            reasoning_steps_taken: 50,
            knowledge_items_added: 2,
        };

        // Duration factor 1.0 -> 1.0 * (0.3 + 0.4*1.0 + 0.3) = 1.0
        let score_normal = metrics.calculate_productivity_score();
        assert!((score_normal - 1.0).abs() < 0.001);

        // > 4 hours (factor 0.85)
        metrics.end_time = Some(Utc::now() + chrono::Duration::minutes(300));
        let score_long = metrics.calculate_productivity_score();
        // 1.0 * (0.3 + 0.4*0.85 + 0.3) = 1.0 * (0.6 + 0.34) = 0.94
        assert!((score_long - 0.94).abs() < 0.001);

        // > 8 hours (factor 0.7)
        metrics.end_time = Some(Utc::now() + chrono::Duration::minutes(500));
        let score_very_long = metrics.calculate_productivity_score();
        // 1.0 * (0.3 + 0.4*0.7 + 0.3) = 1.0 * (0.6 + 0.28) = 0.88
        assert!((score_very_long - 0.88).abs() < 0.001);
    }

    #[test]
    fn test_productivity_score_no_end_time() {
        let metrics = SessionAnalytics {
            session_id: "test".to_string(),
            agent_name: "test".to_string(),
            start_time: Utc::now(),
            end_time: None,
            space_metrics: SpaceMetrics {
                satisfaction_score: 1.0,
                performance_score: 1.0,
                activity_score: 1.0,
                communication_score: 1.0,
                efficiency_score: 1.0,
                overall_score: 1.0,
            },
            dora_metrics: DoraMetrics {
                deployment_frequency: 0,
                lead_time: 0.0,
                change_failure_rate: 0.0,
                mean_time_to_recover: 0.0,
            },
            tasks_completed: 5,
            context_items_created: 10,
            reasoning_steps_taken: 50,
            knowledge_items_added: 2,
        };

        let score = metrics.calculate_productivity_score();
        let expected = 1.0 * (0.3 + 0.4 * 0.5 + 0.3);
        assert!((score - expected).abs() < 0.001);
    }

    #[test]
    fn test_generate_summary_ongoing() {
        let metrics = SessionAnalytics {
            session_id: "sess-1".to_string(),
            agent_name: "agent-a".to_string(),
            start_time: Utc::now(),
            end_time: None,
            space_metrics: SpaceMetrics {
                satisfaction_score: 0.5,
                performance_score: 0.5,
                activity_score: 0.5,
                communication_score: 0.5,
                efficiency_score: 0.5,
                overall_score: 0.5,
            },
            dora_metrics: DoraMetrics {
                deployment_frequency: 0,
                lead_time: 0.0,
                change_failure_rate: 0.0,
                mean_time_to_recover: 0.0,
            },
            tasks_completed: 0,
            context_items_created: 0,
            reasoning_steps_taken: 0,
            knowledge_items_added: 0,
        };

        let summary = metrics.generate_summary();
        assert!(summary.contains("sess-1"));
        assert!(summary.contains("agent-a"));
        assert!(summary.contains("ongoing"));
    }

    #[test]
    fn test_generate_summary_ended() {
        let end = Utc::now();
        let metrics = SessionAnalytics {
            session_id: "sess-2".to_string(),
            agent_name: "agent-b".to_string(),
            start_time: end - chrono::Duration::minutes(30),
            end_time: Some(end),
            space_metrics: SpaceMetrics {
                satisfaction_score: 0.7,
                performance_score: 0.7,
                activity_score: 0.7,
                communication_score: 0.7,
                efficiency_score: 0.7,
                overall_score: 0.7,
            },
            dora_metrics: DoraMetrics {
                deployment_frequency: 0,
                lead_time: 0.0,
                change_failure_rate: 0.0,
                mean_time_to_recover: 0.0,
            },
            tasks_completed: 3,
            context_items_created: 5,
            reasoning_steps_taken: 10,
            knowledge_items_added: 1,
        };

        let summary = metrics.generate_summary();
        assert!(summary.contains("ended at"));
        assert!(!summary.contains("ongoing"));
    }

    #[test]
    fn test_fatigue_analysis_boundary() {
        let analysis = FatigueAnalysis {
            optimal_session_length: 60,
            current_session_length: 0,
            fatigue_risk_level: "low".to_string(),
            recommendation: "none".to_string(),
        };

        assert_eq!(analysis.analyze(60), "low");
        assert_eq!(analysis.analyze(61), "medium");
        assert_eq!(analysis.analyze(120), "medium");
        assert_eq!(analysis.analyze(121), "high");
    }

    #[test]
    fn test_fatigue_analysis_zero_optimal() {
        let analysis = FatigueAnalysis {
            optimal_session_length: 0,
            current_session_length: 0,
            fatigue_risk_level: "low".to_string(),
            recommendation: "none".to_string(),
        };

        assert_eq!(analysis.analyze(0), "low");
        assert_eq!(analysis.analyze(1), "high");
    }

    #[test]
    fn test_bottleneck_report_default() {
        let report = BottleneckReport::default();
        assert_eq!(report.period_days, 0);
        assert_eq!(report.overall_metrics.total_tasks_created, 0);
        assert!(report.task_bottlenecks.is_empty());
        assert!(report.workflow_bottlenecks.is_empty());
        assert!(report.recommendations.is_empty());
    }

    use crate::entities::task::{TaskPriority, TaskStatus};
    use crate::entities::Task;
    use crate::storage::MemoryStorage;

    #[test]
    fn test_task_duration_report_empty_storage() {
        let storage = MemoryStorage::new("test-agent");
        let report = TaskDurationReport::generate(&storage, 7).unwrap();
        assert_eq!(report.period_days, 7);
        assert_eq!(report.total_tasks, 0);
        assert_eq!(report.completed_tasks, 0);
        assert_eq!(report.avg_completion_time_seconds, 0.0);
        assert_eq!(report.median_completion_time_seconds, 0.0);
        assert_eq!(report.min_completion_time_seconds, 0.0);
        assert_eq!(report.max_completion_time_seconds, 0.0);
        assert!(report.tasks_by_agent.is_empty());
        assert!(report.tasks_by_priority.is_empty());
    }

    #[test]
    fn test_task_duration_report_with_tasks() {
        let mut storage = MemoryStorage::new("test-agent");

        let mut task1 = Task::new(
            "Task 1".to_string(),
            "Desc".to_string(),
            "agent-a".to_string(),
            TaskPriority::High,
            None,
        );
        task1.start_time = Utc::now() - chrono::Duration::minutes(30);
        task1.status = TaskStatus::Done;
        task1.end_time = Some(Utc::now() - chrono::Duration::minutes(10));
        let _ = storage.store(&task1.to_generic());

        let mut task2 = Task::new(
            "Task 2".to_string(),
            "Desc".to_string(),
            "agent-b".to_string(),
            TaskPriority::Medium,
            None,
        );
        task2.start_time = Utc::now() - chrono::Duration::minutes(20);
        task2.status = TaskStatus::Done;
        task2.end_time = Some(Utc::now() - chrono::Duration::minutes(5));
        let _ = storage.store(&task2.to_generic());

        let mut task3 = Task::new(
            "Task 3".to_string(),
            "Desc".to_string(),
            "agent-a".to_string(),
            TaskPriority::Low,
            None,
        );
        task3.start_time = Utc::now() - chrono::Duration::minutes(60);
        let _ = storage.store(&task3.to_generic());

        let report = TaskDurationReport::generate(&storage, 1).unwrap();
        assert_eq!(report.total_tasks, 3);
        assert_eq!(report.completed_tasks, 2);
        assert_eq!(report.tasks_by_agent.len(), 2);
        assert_eq!(report.tasks_by_priority.len(), 2);
        assert!(report.throughput_per_day > 0.0);
        assert!(report.max_completion_time_seconds >= report.min_completion_time_seconds);
    }

    #[test]
    fn test_task_duration_report_no_completed() {
        let mut storage = MemoryStorage::new("test-agent");

        let task = Task::new(
            "Todo Task".to_string(),
            "Desc".to_string(),
            "agent-a".to_string(),
            TaskPriority::High,
            None,
        );
        let _ = storage.store(&task.to_generic());

        let report = TaskDurationReport::generate(&storage, 7).unwrap();
        assert_eq!(report.total_tasks, 1);
        assert_eq!(report.completed_tasks, 0);
        assert_eq!(report.avg_completion_time_seconds, 0.0);
    }

    #[test]
    fn test_bottleneck_report_generate_empty() {
        let storage = MemoryStorage::new("test-agent");
        let report = BottleneckReport::generate(storage, 7).unwrap();
        assert_eq!(report.period_days, 7);
        assert_eq!(report.overall_metrics.total_tasks_created, 0);
        assert_eq!(report.overall_metrics.total_tasks_completed, 0);
        assert!(report.recommendations.is_empty());
    }

    #[test]
    fn test_bottleneck_report_with_done_tasks() {
        let mut storage = MemoryStorage::new("test-agent");

        let mut task1 = Task::new(
            "Done Task 1".to_string(),
            "Desc".to_string(),
            "agent-a".to_string(),
            TaskPriority::High,
            None,
        );
        task1.start_time = Utc::now() - chrono::Duration::minutes(60);
        task1.status = TaskStatus::Done;
        task1.end_time = Some(Utc::now() - chrono::Duration::minutes(30));
        let _ = storage.store(&task1.to_generic());

        let report = BottleneckReport::generate(storage, 1).unwrap();
        assert_eq!(report.overall_metrics.total_tasks_created, 1);
        assert_eq!(report.overall_metrics.total_tasks_completed, 1);
        assert!((report.overall_metrics.completion_rate - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_bottleneck_report_with_blocked_tasks() {
        let mut storage = MemoryStorage::new("test-agent");

        let mut blocked_task = Task::new(
            "Blocked Task".to_string(),
            "Desc".to_string(),
            "agent-a".to_string(),
            TaskPriority::High,
            None,
        );
        blocked_task.start_time = Utc::now() - chrono::Duration::days(2);
        blocked_task.status = TaskStatus::Blocked;
        let _ = storage.store(&blocked_task.to_generic());

        let report = BottleneckReport::generate(storage, 7).unwrap();
        assert_eq!(report.overall_metrics.total_tasks_created, 1);
        assert_eq!(report.overall_metrics.total_tasks_completed, 0);
        assert!(report.overall_metrics.blocked_task_rate > 0.0);
    }

    #[test]
    fn test_bottleneck_report_with_old_in_progress() {
        let mut storage = MemoryStorage::new("test-agent");

        let mut old_task = Task::new(
            "Old InProgress".to_string(),
            "Desc".to_string(),
            "agent-a".to_string(),
            TaskPriority::Medium,
            None,
        );
        old_task.start_time = Utc::now() - chrono::Duration::days(4);
        old_task.status = TaskStatus::InProgress;
        let _ = storage.store(&old_task.to_generic());

        let report = BottleneckReport::generate(storage, 7).unwrap();
        assert!(!report.task_bottlenecks.is_empty());
        assert_eq!(report.task_bottlenecks[0].status, "InProgress");
    }

    #[test]
    fn test_bottleneck_report_truncates_to_10() {
        let mut storage = MemoryStorage::new("test-agent");

        for i in 0..15 {
            let mut task = Task::new(
                format!("Old Task {}", i),
                "Desc".to_string(),
                "agent-a".to_string(),
                TaskPriority::High,
                None,
            );
            task.start_time = Utc::now() - chrono::Duration::days(5);
            task.status = TaskStatus::InProgress;
            let _ = storage.store(&task.to_generic());
        }

        let report = BottleneckReport::generate(storage, 7).unwrap();
        assert!(report.task_bottlenecks.len() <= 10);
    }

    #[test]
    fn test_space_metrics_serialization() {
        let metrics = SpaceMetrics {
            satisfaction_score: 0.9,
            performance_score: 0.8,
            activity_score: 0.7,
            communication_score: 0.6,
            efficiency_score: 0.5,
            overall_score: 0.7,
        };

        let json = serde_json::to_string(&metrics).unwrap();
        let parsed: SpaceMetrics = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.satisfaction_score, 0.9);
        assert_eq!(parsed.overall_score, 0.7);
    }

    #[test]
    fn test_dora_metrics_serialization() {
        let metrics = DoraMetrics {
            deployment_frequency: 5,
            lead_time: 3600.0,
            change_failure_rate: 0.1,
            mean_time_to_recover: 7200.0,
        };

        let json = serde_json::to_string(&metrics).unwrap();
        let parsed: DoraMetrics = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.deployment_frequency, 5);
        assert!((parsed.lead_time - 3600.0).abs() < 0.001);
    }

    #[test]
    fn test_task_duration_report_median_even_count() {
        let mut storage = MemoryStorage::new("test-agent");

        let durations: Vec<i64> = vec![100, 200, 300, 400];
        for (i, &dur) in durations.iter().enumerate() {
            let mut task = Task::new(
                format!("Task {}", i),
                "Desc".to_string(),
                "agent-a".to_string(),
                TaskPriority::High,
                None,
            );
            task.start_time = Utc::now() - chrono::Duration::seconds(dur);
            task.status = TaskStatus::Done;
            task.end_time = Some(Utc::now());
            let _ = storage.store(&task.to_generic());
        }

        let report = TaskDurationReport::generate(&storage, 1).unwrap();
        assert_eq!(report.completed_tasks, 4);
        assert_eq!(report.median_completion_time_seconds, 300.0);
    }

    #[test]
    fn test_task_duration_report_throughput_calculation() {
        let mut storage = MemoryStorage::new("test-agent");

        for i in 0..3 {
            let mut task = Task::new(
                format!("Task {}", i),
                "Desc".to_string(),
                "agent-a".to_string(),
                TaskPriority::Medium,
                None,
            );
            task.start_time = Utc::now() - chrono::Duration::minutes(10);
            task.status = TaskStatus::Done;
            task.end_time = Some(Utc::now());
            let _ = storage.store(&task.to_generic());
        }

        let report = TaskDurationReport::generate(&storage, 3).unwrap();
        assert!((report.throughput_per_day - 1.0).abs() < 0.001);
    }
}

impl BottleneckReport {
    pub fn generate<S: Storage + 'static>(storage: S, days: u32) -> Result<Self, String> {
        let cutoff = Utc::now() - chrono::Duration::days(days as i64);
        let task_ids = storage.list_ids("task").map_err(|e| e.to_string())?;

        let mut total_created = 0;
        let mut total_completed = 0;
        let mut cycle_times: Vec<f64> = Vec::new();
        let mut blocked_count = 0;
        let mut task_bottlenecks: Vec<TaskBottleneck> = Vec::new();

        for id in task_ids {
            if let Ok(Some(entity)) = storage.get(&id, "task") {
                if let Ok(task) = Task::from_generic(entity) {
                    if task.start_time > cutoff {
                        total_created += 1;

                        match task.status {
                            TaskStatus::Blocked => {
                                blocked_count += 1;
                                let seconds_stuck = Utc::now()
                                    .signed_duration_since(task.start_time)
                                    .num_seconds()
                                    as f64;
                                if seconds_stuck > 86400.0 {
                                    task_bottlenecks.push(TaskBottleneck {
                                        task_id: task.id,
                                        title: task.title,
                                        agent: task.agent,
                                        time_in_current_status_seconds: seconds_stuck,
                                        status: format!("{:?}", task.status),
                                        priority: format!("{:?}", task.priority),
                                    });
                                }
                            }
                            TaskStatus::Done | TaskStatus::Cancelled => {
                                total_completed += 1;
                                if let Some(end_time) = task.end_time {
                                    cycle_times.push(
                                        end_time
                                            .signed_duration_since(task.start_time)
                                            .num_seconds()
                                            as f64,
                                    );
                                }
                            }
                            _ => {
                                let seconds_in_status = Utc::now()
                                    .signed_duration_since(task.start_time)
                                    .num_seconds()
                                    as f64;
                                if seconds_in_status > 259200.0 {
                                    task_bottlenecks.push(TaskBottleneck {
                                        task_id: task.id,
                                        title: task.title,
                                        agent: task.agent,
                                        time_in_current_status_seconds: seconds_in_status,
                                        status: format!("{:?}", task.status),
                                        priority: format!("{:?}", task.priority),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        task_bottlenecks.sort_by(|a, b| {
            b.time_in_current_status_seconds
                .partial_cmp(&a.time_in_current_status_seconds)
                .unwrap()
        });
        task_bottlenecks.truncate(10);

        let avg_cycle = cycle_times.iter().sum::<f64>() / cycle_times.len().max(1) as f64;

        let mut recommendations: Vec<String> = Vec::new();
        if blocked_count as f64 / total_created.max(1) as f64 > 0.1 {
            recommendations.push(
                "High blocked task rate detected. Review blockers and dependencies.".to_string(),
            );
        }
        if avg_cycle > 172800.0 {
            recommendations.push(
                "Average cycle time exceeds 2 days. Consider breaking down tasks.".to_string(),
            );
        }
        if task_bottlenecks.len() > 3 {
            recommendations.push(
                "Multiple long-running tasks detected. Consider reassessment or escalation."
                    .to_string(),
            );
        }

        Ok(BottleneckReport {
            generated_at: Utc::now(),
            period_days: days,
            overall_metrics: BottleneckOverallMetrics {
                total_tasks_created: total_created,
                total_tasks_completed: total_completed,
                completion_rate: total_completed as f64 / total_created.max(1) as f64,
                avg_cycle_time_seconds: avg_cycle,
                blocked_task_rate: blocked_count as f64 / total_created.max(1) as f64,
            },
            task_bottlenecks,
            workflow_bottlenecks: Vec::new(),
            recommendations,
        })
    }
}
