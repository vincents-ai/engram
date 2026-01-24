use crate::entities::session::{DoraMetrics, SpaceMetrics};
use crate::entities::{Entity, Session, SessionStatus};
use crate::error::EngramError;
use crate::storage::Storage;
use chrono::Utc;
use clap::Subcommand;

/// Session commands
#[derive(Debug, Subcommand)]
pub enum SessionCommands {
    /// Start a new session
    Start {
        /// Agent name
        #[arg(long, short)]
        name: String,

        /// Auto-detect current task
        #[arg(long)]
        auto_detect: bool,
    },
    /// Show session status
    Status {
        /// Session ID
        #[arg(long, short)]
        id: String,

        /// Show detailed metrics
        #[arg(long)]
        metrics: bool,
    },
    /// End current session
    End {
        /// Session ID
        #[arg(long, short)]
        id: String,

        /// Generate summary
        #[arg(long)]
        generate_summary: bool,
    },
    /// List all sessions
    List {
        /// Agent filter
        #[arg(long, short)]
        agent: Option<String>,

        /// Limit results
        #[arg(long, short)]
        limit: Option<usize>,
    },
}

/// Start a new session
pub fn start_session<S: Storage>(
    storage: &mut S,
    agent_name: String,
    auto_detect: bool,
) -> Result<String, EngramError> {
    let title = format!("Session for {}", agent_name);

    let mut goals = Vec::new();
    if auto_detect {
        if is_engram_project() {
            goals.push("Working on Engram project".to_string());
            println!("Auto-detected: Working on Engram project");
            // TODO: Create auto-task for dogfooding when task creation is available
            println!("Note: Auto-task creation for dogfooding will be added in future iteration");
        } else {
            goals.push("General development session".to_string());
        }
    }

    let session = Session::new(title, agent_name.clone(), goals);
    let session_id = session.id.clone();

    let generic = session.to_generic();
    storage.store(&generic)?;

    println!("Session started successfully!");
    println!("Session ID: {}", session_id);
    println!("Agent: {}", agent_name);
    println!("Started at: {}", Utc::now().format("%Y-%m-%d %H:%M:%S"));

    Ok(session_id)
}

/// Check if current directory is Engram project
fn is_engram_project() -> bool {
    let markers = ["rust/Cargo.toml", "AGENTS.md", ".engram/config.yaml"];

    markers
        .iter()
        .any(|marker| std::path::Path::new(marker).exists())
}

/// Show session status
pub fn show_session_status<S: Storage>(
    storage: &S,
    session_id: String,
    show_metrics: bool,
) -> Result<(), EngramError> {
    let generic = storage
        .get(&session_id, Session::entity_type())?
        .ok_or_else(|| EngramError::NotFound(format!("Session not found: {}", session_id)))?;

    let session =
        Session::from_generic(generic).map_err(|e| EngramError::Validation(e.to_string()))?;

    println!("Session Status");
    println!("==============");
    println!("ID: {}", session.id);
    println!("Title: {}", session.title);
    println!("Agent: {}", session.agent);
    println!("Status: {:?}", session.status);
    println!(
        "Started: {}",
        session.start_time.format("%Y-%m-%d %H:%M:%S")
    );

    if let Some(end_time) = session.end_time {
        println!("Ended: {}", end_time.format("%Y-%m-%d %H:%M:%S"));
    }

    if let Some(duration) = session.duration_seconds {
        let hours = duration / 3600;
        let minutes = (duration % 3600) / 60;
        let seconds = duration % 60;
        println!("Duration: {}h {}m {}s", hours, minutes, seconds);
    }

    if !session.goals.is_empty() {
        println!("\nGoals:");
        for goal in &session.goals {
            println!("  - {}", goal);
        }
    }

    if !session.outcomes.is_empty() {
        println!("\nOutcomes:");
        for outcome in &session.outcomes {
            println!("  - {}", outcome);
        }
    }

    if !session.task_ids.is_empty() {
        println!("\nTasks: {}", session.task_ids.join(", "));
    }

    if show_metrics {
        println!("\n--- Metrics ---");

        if let Some(ref space) = session.space_metrics {
            println!("\nSPACE Framework:");
            println!("  Satisfaction: {:.2}", space.satisfaction_score);
            println!("  Performance:  {:.2}", space.performance_score);
            println!("  Activity:     {:.2}", space.activity_score);
            println!("  Communication:{:.2}", space.communication_score);
            println!("  Efficiency:   {:.2}", space.efficiency_score);
            println!("  Overall:      {:.2}", space.overall_score);
        } else {
            println!("\nNo SPACE metrics available");
        }

        if let Some(ref dora) = session.dora_metrics {
            println!("\nDORA Metrics:");
            println!("  Deployment Frequency: {:.2}", dora.deployment_frequency);
            println!("  Lead Time:            {:.2} days", dora.lead_time);
            println!("  Change Failure Rate:  {:.2}%", dora.change_failure_rate);
            println!(
                "  MTTR:                 {:.2} hours",
                dora.mean_time_to_recover
            );
        } else {
            println!("\nNo DORA metrics available");
        }
    }

    Ok(())
}

/// End a session
pub fn end_session<S: Storage>(
    storage: &mut S,
    session_id: String,
    generate_summary: bool,
) -> Result<(), EngramError> {
    let generic = storage
        .get(&session_id, Session::entity_type())?
        .ok_or_else(|| EngramError::NotFound(format!("Session not found: {}", session_id)))?;

    let mut session =
        Session::from_generic(generic).map_err(|e| EngramError::Validation(e.to_string()))?;

    if session.status == SessionStatus::Completed || session.status == SessionStatus::Cancelled {
        return Err(EngramError::Validation(format!(
            "Session already ended: {:?}",
            session.status
        )));
    }

    let outcomes = vec!["Session completed".to_string()];
    session.complete(outcomes);

    if session.space_metrics.is_none() {
        let metrics = calculate_basic_space_metrics(&session);
        session.set_space_metrics(metrics);
    }

    if session.dora_metrics.is_none() {
        let metrics = calculate_basic_dora_metrics(&session);
        session.set_dora_metrics(metrics);
    }

    let generic = session.to_generic();
    storage.store(&generic)?;

    println!("Session ended successfully!");
    println!("Session ID: {}", session.id);
    println!(
        "Duration: {} seconds",
        session.duration_seconds.unwrap_or(0)
    );

    if generate_summary {
        println!("\n--- Session Summary ---");
        println!("Agent: {}", session.agent);
        println!(
            "Started: {}",
            session.start_time.format("%Y-%m-%d %H:%M:%S")
        );
        println!(
            "Ended: {}",
            session.end_time.unwrap().format("%Y-%m-%d %H:%M:%S")
        );

        if let Some(duration) = session.duration_seconds {
            let hours = duration / 3600;
            let minutes = (duration % 3600) / 60;
            println!("Total Duration: {}h {}m", hours, minutes);
        }

        println!("\nActivity:");
        println!("  Tasks: {}", session.task_ids.len());
        println!("  Context Items: {}", session.context_ids.len());
        println!("  Knowledge Items: {}", session.knowledge_ids.len());

        if let Some(ref space) = session.space_metrics {
            println!("\nProductivity Score: {:.2}/100", space.overall_score);
        }
    }

    Ok(())
}

/// Calculate basic SPACE metrics for a session
fn calculate_basic_space_metrics(session: &Session) -> SpaceMetrics {
    let activity_score =
        (session.task_ids.len() + session.context_ids.len() + session.knowledge_ids.len()) as f64
            * 10.0;
    let activity_score = activity_score.min(100.0);

    let performance_score = if !session.task_ids.is_empty() {
        70.0
    } else {
        50.0
    };

    let satisfaction_score = 80.0;
    let communication_score = 50.0;
    let efficiency_score = activity_score * 0.8;

    let overall_score = (satisfaction_score
        + performance_score
        + activity_score
        + communication_score
        + efficiency_score)
        / 5.0;

    SpaceMetrics {
        satisfaction_score,
        performance_score,
        activity_score,
        communication_score,
        efficiency_score,
        overall_score,
    }
}

/// Calculate basic DORA metrics for a session
fn calculate_basic_dora_metrics(session: &Session) -> DoraMetrics {
    DoraMetrics {
        deployment_frequency: 0.0,
        lead_time: if let Some(duration) = session.duration_seconds {
            duration as f64 / 86400.0
        } else {
            0.0
        },
        change_failure_rate: 0.0,
        mean_time_to_recover: 0.0,
    }
}

/// List sessions
pub fn list_sessions<S: Storage>(
    storage: &S,
    agent_filter: Option<String>,
    limit: Option<usize>,
) -> Result<(), EngramError> {
    let entity_ids = storage.list_ids(Session::entity_type())?;

    let mut sessions: Vec<Session> = Vec::new();
    for id in entity_ids {
        if let Some(generic) = storage.get(&id, Session::entity_type())? {
            if let Some(ref agent) = agent_filter {
                if generic.agent != *agent {
                    continue;
                }
            }
            if let Ok(session) = Session::from_generic(generic) {
                sessions.push(session);
            }
        }
    }

    sessions.sort_by(|a, b| b.start_time.cmp(&a.start_time));

    if let Some(lim) = limit {
        sessions.truncate(lim);
    }

    if sessions.is_empty() {
        println!("No sessions found");
        return Ok(());
    }

    println!("Sessions ({})", sessions.len());
    println!("========================================");

    for session in sessions {
        let status_str = match session.status {
            SessionStatus::Active => "Active",
            SessionStatus::Paused => "Paused",
            SessionStatus::Completed => "Completed",
            SessionStatus::Cancelled => "Cancelled",
        };

        println!("ID: {}", session.id);
        println!("  Agent: {} | Status: {}", session.agent, status_str);
        println!(
            "  Started: {}",
            session.start_time.format("%Y-%m-%d %H:%M").to_string()
        );
        if let Some(end_time) = session.end_time {
            println!("  Ended: {}", end_time.format("%Y-%m-%d %H:%M").to_string());
        }
        println!();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::MemoryStorage;

    fn create_test_storage() -> MemoryStorage {
        MemoryStorage::new("default")
    }

    #[test]
    fn test_start_session() {
        let mut storage = create_test_storage();
        let session_id = start_session(&mut storage, "agent1".to_string(), false).unwrap();

        let generic = storage.get(&session_id, "session").unwrap().unwrap();
        let session = Session::from_generic(generic).unwrap();

        assert_eq!(session.agent, "agent1");
        assert_eq!(session.status, SessionStatus::Active);
    }

    #[test]
    fn test_end_session() {
        let mut storage = create_test_storage();
        let session_id = start_session(&mut storage, "agent1".to_string(), false).unwrap();

        end_session(&mut storage, session_id.clone(), false).unwrap();

        let generic = storage.get(&session_id, "session").unwrap().unwrap();
        let session = Session::from_generic(generic).unwrap();

        assert_eq!(session.status, SessionStatus::Completed);
        assert!(session.end_time.is_some());
    }

    #[test]
    fn test_end_session_already_ended() {
        let mut storage = create_test_storage();
        let session_id = start_session(&mut storage, "agent1".to_string(), false).unwrap();

        end_session(&mut storage, session_id.clone(), false).unwrap();
        let result = end_session(&mut storage, session_id, false);

        assert!(matches!(result, Err(EngramError::Validation(_))));
    }

    #[test]
    fn test_list_sessions() {
        let mut storage = create_test_storage();
        start_session(&mut storage, "agent1".to_string(), false).unwrap();
        start_session(&mut storage, "agent2".to_string(), false).unwrap();

        list_sessions(&storage, None, None).unwrap();
        list_sessions(&storage, Some("agent1".to_string()), None).unwrap();
    }

    #[test]
    fn test_show_session_status() {
        let mut storage = create_test_storage();
        let session_id = start_session(&mut storage, "agent1".to_string(), false).unwrap();

        assert!(show_session_status(&storage, session_id.clone(), true).is_ok());
        assert!(show_session_status(&storage, "non-existent".to_string(), false).is_err());
    }

    #[test]
    fn test_space_metrics_calculation() {
        let mut storage = create_test_storage();
        let session_id = start_session(&mut storage, "agent1".to_string(), false).unwrap();
        end_session(&mut storage, session_id.clone(), false).unwrap();

        let generic = storage.get(&session_id, "session").unwrap().unwrap();
        let session = Session::from_generic(generic).unwrap();

        assert!(session.space_metrics.is_some());
        let metrics = session.space_metrics.unwrap();
        assert!(metrics.overall_score > 0.0);
    }

    #[test]
    fn test_end_session_not_found() {
        let mut storage = create_test_storage();
        let result = end_session(&mut storage, "non-existent".to_string(), false);
        assert!(matches!(result, Err(EngramError::NotFound(_))));
    }
}
