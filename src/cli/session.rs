use crate::entities::dora_metrics_report::DoraMetricsCalculator;
use crate::entities::session::{DoraMetrics, SpaceMetrics};
use crate::entities::{Entity, Session, SessionStatus};
use crate::error::EngramError;
use crate::storage::Storage;
use chrono::{Duration, Utc};
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

        /// Only show sessions started after this date/time
        /// Formats: 2024-01-01, 2024-01-01T12:00:00, 24h, 7d, 30d
        #[arg(long)]
        since: Option<String>,

        /// Limit results
        #[arg(long, short)]
        limit: Option<usize>,

        /// Show all results (no limit)
        #[arg(long, conflicts_with = "limit")]
        all: bool,

        /// Offset for pagination
        #[arg(long, short)]
        offset: Option<usize>,
    },
    /// Detect zombie sessions (started but never ended beyond a threshold)
    Zombies {
        /// Max age in hours before a session is considered a zombie (default: 24)
        #[arg(long, default_value = "24")]
        max_age_hours: i64,

        /// Check if git commits were made since session started (detects abandoned sessions)
        #[arg(long)]
        check_git: bool,
    },
    /// Summarize recent sessions with goals, outcomes, duration, and task count
    Summaries {
        /// Filter by agent name
        #[arg(long, short)]
        agent: Option<String>,

        /// Only show sessions started on or after this date (YYYY-MM-DD)
        #[arg(long)]
        since: Option<String>,

        /// Limit results
        #[arg(long, short)]
        limit: Option<usize>,

        /// Show all results (no limit)
        #[arg(long, conflicts_with = "limit")]
        all: bool,
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
        let metrics = calculate_basic_dora_metrics(storage, &session);
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

/// Calculate DORA metrics from git history and engram entities.
///
/// Computes real metrics and persists the result as a DoraMetricsReport entity.
/// Falls back to session-duration-based lead time if git history is unavailable.
fn calculate_basic_dora_metrics<S: Storage>(storage: &mut S, session: &Session) -> DoraMetrics {
    let current_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));

    match DoraMetricsCalculator::compute(storage, &current_dir, &session.agent, 30) {
        Ok(report) => {
            let _ = storage.store(&report.to_generic());
            report.to_session_dora_metrics()
        }
        Err(_) => DoraMetrics {
            deployment_frequency: 0.0,
            lead_time: if let Some(duration) = session.duration_seconds {
                duration as f64 / 86400.0
            } else {
                0.0
            },
            change_failure_rate: 0.0,
            mean_time_to_recover: 0.0,
        },
    }
}

use crate::cli::utils::{create_table, truncate};
use chrono::{DateTime, NaiveDate, NaiveDateTime};
use prettytable::row;

fn parse_since(input: &str) -> Result<DateTime<Utc>, EngramError> {
    let input = input.trim();

    if let Some(rest) = input.strip_suffix('h') {
        let hours: i64 = rest.parse().map_err(|_| {
            EngramError::Validation(format!(
                "Invalid hours format: '{}'. Expected e.g. 24h",
                input
            ))
        })?;
        return Ok(Utc::now() - Duration::hours(hours));
    }

    if let Some(rest) = input.strip_suffix('d') {
        let days: i64 = rest.parse().map_err(|_| {
            EngramError::Validation(format!(
                "Invalid days format: '{}'. Expected e.g. 7d",
                input
            ))
        })?;
        return Ok(Utc::now() - Duration::days(days));
    }

    if let Ok(dt) = NaiveDateTime::parse_from_str(input, "%Y-%m-%dT%H:%M:%S") {
        return Ok(dt.and_utc());
    }

    if let Ok(date) = NaiveDate::parse_from_str(input, "%Y-%m-%d") {
        return Ok(date.and_hms_opt(0, 0, 0).unwrap().and_utc());
    }

    Err(EngramError::Validation(format!(
        "Invalid --since format: '{}'. Supported: 2024-01-01, 2024-01-01T12:00:00, 24h, 7d, 30d",
        input
    )))
}

/// List sessions
pub fn list_sessions<S: Storage>(
    writer: &mut dyn std::io::Write,
    storage: &S,
    agent_filter: Option<String>,
    since_filter: Option<String>,
    limit: Option<usize>,
    all: bool,
    offset: Option<usize>,
) -> Result<(), EngramError> {
    let since_time = since_filter.as_deref().map(parse_since).transpose()?;

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
                if let Some(ref since) = since_time {
                    if session.start_time < *since {
                        continue;
                    }
                }
                sessions.push(session);
            }
        }
    }

    sessions.sort_by(|a, b| b.start_time.cmp(&a.start_time));

    let total_count = sessions.len();

    if let Some(off) = offset {
        sessions = sessions.into_iter().skip(off).collect();
    }

    if !all {
        if let Some(lim) = limit {
            sessions.truncate(lim);
        }
    }

    if sessions.is_empty() {
        writeln!(writer, "No sessions found")?;
        return Ok(());
    }

    writeln!(
        writer,
        "Found {} sessions (showing {} of {})",
        total_count,
        sessions.len(),
        total_count
    )?;
    writeln!(writer)?;

    let mut table = create_table();
    table.set_titles(row!["ID", "St", "Agent", "Started", "Ended", "Duration"]);

    for session in &sessions {
        let status_symbol = match session.status {
            SessionStatus::Active => "🟢",
            SessionStatus::Paused => "⏸️",
            SessionStatus::Completed => "✅",
            SessionStatus::Cancelled => "❌",
            SessionStatus::Reflecting => "🔄",
        };

        let duration_str = if let Some(duration) = session.duration_seconds {
            let hours = duration / 3600;
            let minutes = (duration % 3600) / 60;
            format!("{}h {}m", hours, minutes)
        } else {
            "-".to_string()
        };

        let end_time = session
            .end_time
            .map(|t| t.format("%Y-%m-%d %H:%M").to_string())
            .unwrap_or_else(|| "-".to_string());

        table.add_row(row![
            &session.id[..8],
            status_symbol,
            truncate(&session.agent, 15),
            session.start_time.format("%Y-%m-%d %H:%M"),
            end_time,
            duration_str
        ]);
    }

    table.print(writer)?;
    writeln!(writer)?;

    if total_count > sessions.len() {
        writeln!(
            writer,
            "(More results available — use --all, --offset N, or --limit N)"
        )?;
    }

    Ok(())
}

/// Result of zombie session detection for a single session
struct ZombieInfo {
    session: Session,
    elapsed_hours: i64,
    has_recent_commits: Option<bool>,
}

/// Detect and display zombie sessions
pub fn detect_zombie_sessions<S: Storage>(
    writer: &mut dyn std::io::Write,
    storage: &S,
    max_age_hours: i64,
    check_git: bool,
) -> Result<(), EngramError> {
    let entity_ids = storage.list_ids(Session::entity_type())?;

    let mut zombies: Vec<ZombieInfo> = Vec::new();
    for id in entity_ids {
        if let Some(generic) = storage.get(&id, Session::entity_type())? {
            if let Ok(session) = Session::from_generic(generic) {
                if !session.is_zombie(max_age_hours) {
                    continue;
                }

                let elapsed = Utc::now()
                    .signed_duration_since(session.start_time)
                    .num_hours();

                let has_recent_commits = if check_git {
                    Some(commits_since(&session.start_time))
                } else {
                    None
                };

                zombies.push(ZombieInfo {
                    session,
                    elapsed_hours: elapsed,
                    has_recent_commits,
                });
            }
        }
    }

    zombies.sort_by(|a, b| b.elapsed_hours.cmp(&a.elapsed_hours));

    if zombies.is_empty() {
        writeln!(
            writer,
            "No zombie sessions found (threshold: {}h)",
            max_age_hours
        )?;
        return Ok(());
    }

    writeln!(
        writer,
        "Found {} zombie session{} (threshold: {}h)",
        zombies.len(),
        if zombies.len() == 1 { "" } else { "s" },
        max_age_hours
    )?;
    writeln!(writer)?;

    let mut table = create_table();
    if check_git {
        table.set_titles(row![
            "ID",
            "St",
            "Agent",
            "Started",
            "Age",
            "Tasks",
            "Commits Since?"
        ]);
    } else {
        table.set_titles(row!["ID", "St", "Agent", "Started", "Age", "Tasks"]);
    }

    for z in &zombies {
        let status_symbol = match z.session.status {
            SessionStatus::Active => "\u{1f7e2}",
            SessionStatus::Paused => "\u{23f8}\u{fe0f}",
            SessionStatus::Reflecting => "\u{1f504}",
            SessionStatus::Completed | SessionStatus::Cancelled => unreachable!(),
        };

        let age_str = format!("{}h", z.elapsed_hours);

        let tasks_str = z.session.task_ids.len().to_string();

        if check_git {
            let git_str = match z.has_recent_commits {
                Some(true) => "YES",
                Some(false) => "no",
                None => "-",
            };
            table.add_row(row![
                &z.session.id[..8],
                status_symbol,
                truncate(&z.session.agent, 15),
                z.session.start_time.format("%Y-%m-%d %H:%M"),
                age_str,
                tasks_str,
                git_str,
            ]);
        } else {
            table.add_row(row![
                &z.session.id[..8],
                status_symbol,
                truncate(&z.session.agent, 15),
                z.session.start_time.format("%Y-%m-%d %H:%M"),
                age_str,
                tasks_str,
            ]);
        }
    }

    table.print(writer)?;
    writeln!(writer)?;

    if check_git {
        let abandoned: Vec<_> = zombies
            .iter()
            .filter(|z| z.has_recent_commits == Some(true))
            .collect();
        if !abandoned.is_empty() {
            writeln!(
                writer,
                "\u{26a0}\u{fe0f}  {} session{} show commits after start — likely abandoned",
                abandoned.len(),
                if abandoned.len() == 1 { "" } else { "s" }
            )?;
        }
    }

    writeln!(
        writer,
        "\nTip: close zombie sessions with: engram session end --id <UUID>"
    )?;

    Ok(())
}

/// Check if any git commits were made since the given timestamp.
/// Returns true if at least one commit exists after `since`.
fn commits_since(since: &DateTime<Utc>) -> bool {
    let since_epoch = since.timestamp();
    let output = std::process::Command::new("git")
        .args([
            "log",
            "--format=%ct",
            "-1",
            &format!("--after={}", since_epoch),
        ])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            !stdout.trim().is_empty()
        }
        _ => false,
    }
}

fn format_duration(seconds: u64) -> String {
    if seconds < 60 {
        return format!("{}s", seconds);
    }
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    }
}

pub fn summarize_sessions<S: Storage>(
    writer: &mut dyn std::io::Write,
    storage: &S,
    agent_filter: Option<String>,
    since_filter: Option<String>,
    limit: Option<usize>,
    all: bool,
) -> Result<(), EngramError> {
    let since_time = since_filter.as_deref().map(parse_since).transpose()?;

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
                if let Some(ref since) = since_time {
                    if session.start_time < *since {
                        continue;
                    }
                }
                sessions.push(session);
            }
        }
    }

    sessions.sort_by(|a, b| b.start_time.cmp(&a.start_time));

    let total_count = sessions.len();

    if !all {
        if let Some(lim) = limit {
            sessions.truncate(lim);
        }
    }

    if sessions.is_empty() {
        writeln!(writer, "No sessions found")?;
        return Ok(());
    }

    writeln!(
        writer,
        "Session Summaries ({} of {})",
        sessions.len(),
        total_count
    )?;
    writeln!(writer)?;

    let mut table = create_table();
    table.set_titles(row![
        "ID", "St", "Agent", "Started", "Duration", "Tasks", "Goals", "Outcomes"
    ]);

    for session in &sessions {
        let status_symbol = match session.status {
            SessionStatus::Active => "\u{1f7e2}",
            SessionStatus::Paused => "\u{23f8}\u{fe0f}",
            SessionStatus::Completed => "\u{2705}",
            SessionStatus::Cancelled => "\u{274c}",
            SessionStatus::Reflecting => "\u{1f504}",
        };

        let duration_str = if let Some(dur) = session.duration_seconds {
            format_duration(dur)
        } else if let Some(end) = session.end_time {
            let elapsed = end
                .signed_duration_since(session.start_time)
                .num_seconds()
                .max(0) as u64;
            format_duration(elapsed)
        } else {
            let elapsed = Utc::now()
                .signed_duration_since(session.start_time)
                .num_seconds()
                .max(0) as u64;
            format!("{} (active)", format_duration(elapsed))
        };

        let goals_str = if session.goals.is_empty() {
            "-".to_string()
        } else {
            truncate(&session.goals.join("; "), 30)
        };

        let outcomes_str = if session.outcomes.is_empty() {
            "-".to_string()
        } else {
            truncate(&session.outcomes.join("; "), 30)
        };

        table.add_row(row![
            &session.id[..8],
            status_symbol,
            truncate(&session.agent, 12),
            session.start_time.format("%Y-%m-%d %H:%M"),
            duration_str,
            session.task_ids.len().to_string(),
            goals_str,
            outcomes_str,
        ]);
    }

    table.print(writer)?;
    writeln!(writer)?;

    if total_count > sessions.len() {
        writeln!(writer, "(More results available — use --all or --limit N)")?;
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

        let mut buffer = Vec::new();
        list_sessions(&mut buffer, &storage, None, None, None, false, None).unwrap();
        let output = String::from_utf8(buffer).unwrap();

        assert!(output.contains("Found 2 sessions"));
        assert!(output.contains("agent1"));
        assert!(output.contains("agent2"));

        let mut buffer_filtered = Vec::new();
        list_sessions(
            &mut buffer_filtered,
            &storage,
            Some("agent1".to_string()),
            None,
            None,
            false,
            None,
        )
        .unwrap();
        let output_filtered = String::from_utf8(buffer_filtered).unwrap();

        assert!(output_filtered.contains("Found 1 sessions"));
        assert!(output_filtered.contains("agent1"));
        assert!(!output_filtered.contains("agent2"));
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
