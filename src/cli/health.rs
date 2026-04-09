use std::collections::{HashMap, HashSet};

use crate::cli::utils::{create_table, truncate};
use crate::entities::GenericEntity;
use crate::error::EngramError;
use crate::feedback::{FeedbackStatus, StructuredFeedback};
use crate::storage::{RelationshipStorage, Storage};
use clap::Subcommand;
use prettytable::row;
use serde::{Deserialize, Serialize};

#[derive(Subcommand)]
pub enum HealthCommands {
    /// Run all health checks and compute a score
    Audit {
        /// Store results as a context entity in engram
        #[arg(long)]
        store: bool,
    },
    /// Show files with highest churn (most changes)
    Churn {
        /// Number of files to show (default: 20)
        #[arg(long, default_value = "20")]
        top: usize,
    },
    /// Analyze contributor distribution and bus factor risk
    BusFactor,
    /// Show files with the most bug-related commits
    BugClusters {
        /// Number of files to show (default: 20)
        #[arg(long, default_value = "20")]
        top: usize,
    },
    /// Show commit velocity trend by month
    Velocity,
    /// Show revert/hotfix/rollback frequency
    Firefighting,
    /// Show average commit size (lines changed)
    CommitSize,
    /// Show test-related commit ratio
    TestSignal,
    /// Compute overall health score (0-100)
    Score,
    /// Detect orphaned entities with no relationships
    Orphans,
    /// Check git refs store consistency
    Consistency,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChurnEntry {
    pub count: usize,
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Contributor {
    pub commits: usize,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VelocityEntry {
    pub month: String,
    pub commits: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirefightingEntry {
    pub hash: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitSizeStats {
    pub avg_additions: f64,
    pub avg_deletions: f64,
    pub total_commits: usize,
    pub avg_total_lines: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSignalStats {
    pub test_commits: usize,
    pub total_commits: usize,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthAuditReport {
    pub churn: Vec<ChurnEntry>,
    pub contributors_all_time: Vec<Contributor>,
    pub contributors_recent: Vec<Contributor>,
    pub bug_clusters: Vec<ChurnEntry>,
    pub velocity: Vec<VelocityEntry>,
    pub firefighting: Vec<FirefightingEntry>,
    pub commit_size: Option<CommitSizeStats>,
    pub test_signal: TestSignalStats,
    pub score: HealthScore,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthScore {
    pub overall: u8,
    pub bus_factor: SignalScore,
    pub churn_concentration: SignalScore,
    pub bug_churn_overlap: SignalScore,
    pub velocity_trend: SignalScore,
    pub firefighting: SignalScore,
    pub commit_size: SignalScore,
    pub test_signal: SignalScore,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalScore {
    pub value: f64,
    pub weight: f64,
    pub rating: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrphanedEntity {
    pub id: String,
    pub entity_type: String,
    pub agent: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrphanReport {
    pub total_entities: usize,
    pub total_relationships: usize,
    pub orphaned_count: usize,
    pub orphans_by_type: HashMap<String, usize>,
    pub orphaned_entities: Vec<OrphanedEntity>,
    pub excluded_types: Vec<String>,
}

impl StructuredFeedback for OrphanReport {
    fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }

    fn summary(&self) -> String {
        if self.orphaned_count == 0 {
            format!(
                "Orphan check: clean — {}/{} entities connected",
                self.total_entities - self.excluded_types.len().min(self.total_entities),
                self.total_entities
            )
        } else {
            let type_summary: Vec<String> = self
                .orphans_by_type
                .iter()
                .map(|(t, c)| format!("{} {}", c, t))
                .collect();
            format!(
                "Orphan check: {}/{} entities orphaned — {}",
                self.orphaned_count,
                self.total_entities,
                type_summary.join(", ")
            )
        }
    }

    fn status_code(&self) -> FeedbackStatus {
        if self.orphaned_count == 0 {
            FeedbackStatus::Success
        } else {
            FeedbackStatus::Warning
        }
    }
}

impl StructuredFeedback for HealthAuditReport {
    fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }

    fn summary(&self) -> String {
        let rating = if self.score.overall >= 80 {
            "Healthy"
        } else if self.score.overall >= 60 {
            "Warning"
        } else {
            "Critical"
        };
        format!(
            "Health Audit: {}/100 ({}) — {} churn files, {} contributors, {} firefighting events",
            self.score.overall,
            rating,
            self.churn.len(),
            self.contributors_recent.len(),
            self.firefighting.len(),
        )
    }

    fn status_code(&self) -> FeedbackStatus {
        if self.score.overall >= 80 {
            FeedbackStatus::Success
        } else if self.score.overall >= 60 {
            FeedbackStatus::Warning
        } else {
            FeedbackStatus::Failed
        }
    }
}

impl StructuredFeedback for HealthScore {
    fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }

    fn summary(&self) -> String {
        let rating = if self.overall >= 80 {
            "Healthy"
        } else if self.overall >= 60 {
            "Warning"
        } else {
            "Critical"
        };
        format!("Health Score: {}/100 ({})", self.overall, rating)
    }

    fn status_code(&self) -> FeedbackStatus {
        if self.overall >= 80 {
            FeedbackStatus::Success
        } else if self.overall >= 60 {
            FeedbackStatus::Warning
        } else {
            FeedbackStatus::Failed
        }
    }
}

pub fn handle_health_command<S: Storage + RelationshipStorage>(
    storage: &mut S,
    command: HealthCommands,
) -> Result<(), EngramError> {
    match command {
        HealthCommands::Audit { store } => run_audit(storage, store),
        HealthCommands::Churn { top } => run_churn(top),
        HealthCommands::BusFactor => run_bus_factor(),
        HealthCommands::BugClusters { top } => run_bug_clusters(top),
        HealthCommands::Velocity => run_velocity(),
        HealthCommands::Firefighting => run_firefighting(),
        HealthCommands::CommitSize => run_commit_size(),
        HealthCommands::TestSignal => run_test_signal(),
        HealthCommands::Score => {
            let report = collect_audit_data()?;
            let score = compute_health_score(&report);
            print_health_score(&score);
            Ok(())
        }
        HealthCommands::Orphans => run_orphan_detection(storage),
        HealthCommands::Consistency => run_consistency_check(storage),
    }
}

fn run_git(args: &[&str]) -> Result<String, EngramError> {
    let output = std::process::Command::new("git")
        .args(args)
        .output()
        .map_err(|e| EngramError::Git(format!("Failed to run git: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(EngramError::Git(stderr.to_string()));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn run_git_allow_fail(args: &[&str]) -> String {
    std::process::Command::new("git")
        .args(args)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default()
}

fn parse_count_name_lines(output: &str) -> Vec<ChurnEntry> {
    output
        .lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|line| {
            let line = line.trim();
            let mut parts = line.splitn(2, char::is_whitespace);
            let count: usize = parts.next()?.trim_start().parse().ok()?;
            let file = parts.next()?.trim().to_string();
            if file.is_empty() {
                None
            } else {
                Some(ChurnEntry { count, file })
            }
        })
        .collect()
}

fn parse_shortlog(output: &str) -> Vec<Contributor> {
    output
        .lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|line| {
            let line = line.trim();
            let mut parts = line.splitn(2, '\t');
            let count: usize = parts.next()?.trim().parse().ok()?;
            let name = parts.next()?.trim().to_string();
            if name.is_empty() {
                None
            } else {
                Some(Contributor {
                    commits: count,
                    name,
                })
            }
        })
        .collect()
}

fn parse_velocity(output: &str) -> Vec<VelocityEntry> {
    output
        .lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|line| {
            let line = line.trim();
            let mut parts = line.splitn(2, char::is_whitespace);
            let count: usize = parts.next()?.trim_start().parse().ok()?;
            let month = parts.next()?.trim().to_string();
            if month.is_empty() {
                None
            } else {
                Some(VelocityEntry {
                    month,
                    commits: count,
                })
            }
        })
        .collect()
}

fn parse_numstat(output: &str) -> CommitSizeStats {
    let mut total_add: u64 = 0;
    let mut total_del: u64 = 0;
    let mut commit_count = 0;

    for line in output.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            let add: u64 = parts[0].parse().unwrap_or(0);
            let del: u64 = parts[1].parse().unwrap_or(0);
            if add > 0 || del > 0 {
                total_add += add;
                total_del += del;
                commit_count += 1;
            }
        }
    }

    let n = commit_count.max(1) as f64;
    CommitSizeStats {
        avg_additions: total_add as f64 / n,
        avg_deletions: total_del as f64 / n,
        total_commits: commit_count,
        avg_total_lines: (total_add + total_del) as f64 / n,
    }
}

fn compute_test_signal() -> TestSignalStats {
    let total_output = run_git_allow_fail(&["log", "--oneline", "--since=1 year ago"]);
    let total_commits = total_output
        .lines()
        .filter(|l| !l.trim().is_empty())
        .count();

    let test_commits = total_output
        .lines()
        .filter(|l| {
            let l = l.to_lowercase();
            l.contains("test")
                || l.contains("spec")
                || l.contains("coverage")
                || l.contains("fixture")
        })
        .count();

    let percentage = if total_commits > 0 {
        (test_commits as f64 / total_commits as f64) * 100.0
    } else {
        0.0
    };

    TestSignalStats {
        test_commits,
        total_commits,
        percentage,
    }
}

fn collect_audit_data() -> Result<HealthAuditReport, EngramError> {
    let churn_output = run_git(&[
        "log",
        "--format=format:",
        "--name-only",
        "--since=1 year ago",
    ])?;
    let churn = parse_count_name_lines(&churn_output);
    let churn: Vec<ChurnEntry> = churn.into_iter().take(20).collect();

    let contributors_all = run_git(&["shortlog", "-sn", "--no-merges"])?;
    let contributors_all_time = parse_shortlog(&contributors_all);

    let contributors_recent = run_git(&["shortlog", "-sn", "--no-merges", "--since=6 months ago"])?;
    let contributors_recent = parse_shortlog(&contributors_recent);

    let bug_output = run_git(&[
        "log",
        "-i",
        "-E",
        "--grep=fix|bug|broken|regression",
        "--name-only",
        "--format=",
    ])?;
    let bug_clusters = parse_count_name_lines(&bug_output);
    let bug_clusters: Vec<ChurnEntry> = bug_clusters.into_iter().take(20).collect();

    let velocity_output = run_git(&[
        "log",
        "--format=%ad",
        "--date=format:%Y-%m",
        "--since=12 months ago",
    ])?;
    let velocity = parse_velocity(&velocity_output);

    let firefighting_output = run_git_allow_fail(&["log", "--oneline", "--since=1 year ago"]);
    let firefighting: Vec<FirefightingEntry> = firefighting_output
        .lines()
        .filter(|l| {
            let l = l.to_lowercase();
            l.contains("revert")
                || l.contains("hotfix")
                || l.contains("emergency")
                || l.contains("rollback")
        })
        .map(|line| {
            let parts: Vec<&str> = line.splitn(2, ' ').collect();
            let hash = parts.first().unwrap_or(&"").to_string();
            let message = parts.get(1).unwrap_or(&"").to_string();
            FirefightingEntry { hash, message }
        })
        .collect();

    let numstat_output = run_git(&["log", "--numstat", "--format=", "--since=3 months ago"])?;
    let commit_size = parse_numstat(&numstat_output);
    let commit_size = if commit_size.total_commits > 0 {
        Some(commit_size)
    } else {
        None
    };

    let test_signal = compute_test_signal();

    let score = compute_health_score_from_signals(
        &contributors_recent,
        &churn,
        &bug_clusters,
        &velocity,
        firefighting.len(),
        commit_size.as_ref(),
        &test_signal,
    );

    Ok(HealthAuditReport {
        churn,
        contributors_all_time,
        contributors_recent,
        bug_clusters,
        velocity,
        firefighting,
        commit_size,
        test_signal,
        score,
    })
}

fn compute_health_score(report: &HealthAuditReport) -> HealthScore {
    compute_health_score_from_signals(
        &report.contributors_recent,
        &report.churn,
        &report.bug_clusters,
        &report.velocity,
        report.firefighting.len(),
        report.commit_size.as_ref(),
        &report.test_signal,
    )
}

fn compute_health_score_from_signals(
    contributors_recent: &[Contributor],
    churn: &[ChurnEntry],
    bug_clusters: &[ChurnEntry],
    velocity: &[VelocityEntry],
    firefighting_count: usize,
    commit_size: Option<&CommitSizeStats>,
    test_signal: &TestSignalStats,
) -> HealthScore {
    let total_churn: usize = churn.iter().map(|c| c.count).sum();

    let bus_factor_score = {
        let active = contributors_recent.len();
        let (value, rating, label) = if active >= 3 {
            (
                100.0,
                "Healthy".into(),
                format!("{} active contributors", active),
            )
        } else if active >= 1 {
            (
                40.0,
                "Warning".into(),
                format!("{} active contributor(s)", active),
            )
        } else {
            (0.0, "Critical".into(), "No active contributors".into())
        };
        SignalScore {
            value,
            weight: 20.0,
            rating,
            label,
        }
    };

    let churn_concentration_score = {
        let top_file_pct = if total_churn > 0 && !churn.is_empty() {
            churn[0].count as f64 / total_churn as f64 * 100.0
        } else {
            0.0
        };
        let (value, rating, label) = if top_file_pct <= 5.0 {
            (
                100.0,
                "Healthy".into(),
                format!("Top file: {:.1}% of changes", top_file_pct),
            )
        } else if top_file_pct <= 10.0 {
            (
                50.0,
                "Warning".into(),
                format!("Top file: {:.1}% of changes", top_file_pct),
            )
        } else {
            (
                0.0,
                "Critical".into(),
                format!("Top file: {:.1}% of changes", top_file_pct),
            )
        };
        SignalScore {
            value,
            weight: 15.0,
            rating,
            label,
        }
    };

    let bug_churn_overlap_score = {
        let churn_files: std::collections::HashSet<&str> =
            churn.iter().map(|c| c.file.as_str()).collect();
        let bug_files: std::collections::HashSet<&str> =
            bug_clusters.iter().map(|c| c.file.as_str()).collect();
        let overlap: Vec<_> = churn_files.intersection(&bug_files).collect();
        let overlap_count = overlap.len();
        let (value, rating, label) = if overlap_count == 0 {
            (100.0, "Healthy".into(), "No churn/bug overlap".into())
        } else if overlap_count <= 2 {
            (
                50.0,
                "Warning".into(),
                format!("{} file(s) on both lists", overlap_count),
            )
        } else {
            (
                0.0,
                "Critical".into(),
                format!("{} file(s) on both lists", overlap_count),
            )
        };
        SignalScore {
            value,
            weight: 15.0,
            rating,
            label,
        }
    };

    let velocity_trend_score = {
        let (value, rating, label) = if velocity.len() < 2 {
            (75.0, "Healthy".into(), "Insufficient data".into())
        } else {
            let first_half: usize = velocity[..velocity.len() / 2]
                .iter()
                .map(|v| v.commits)
                .sum();
            let second_half: usize = velocity[velocity.len() / 2..]
                .iter()
                .map(|v| v.commits)
                .sum();
            let change = if first_half > 0 {
                (second_half as f64 - first_half as f64) / first_half as f64 * 100.0
            } else if second_half > 0 {
                100.0
            } else {
                0.0
            };
            if change >= -20.0 {
                (
                    100.0,
                    "Healthy".into(),
                    format!("Velocity change: {change:+.1}%"),
                )
            } else if change >= -40.0 {
                (
                    50.0,
                    "Warning".into(),
                    format!("Velocity change: {change:+.1}%"),
                )
            } else {
                (
                    0.0,
                    "Critical".into(),
                    format!("Velocity decline: {change:+.1}%"),
                )
            }
        };
        SignalScore {
            value,
            weight: 15.0,
            rating,
            label,
        }
    };

    let firefighting_score = {
        let total_commits = test_signal.total_commits;
        let pct = if total_commits > 0 {
            firefighting_count as f64 / total_commits as f64 * 100.0
        } else {
            0.0
        };
        let (value, rating, label) = if pct < 2.0 {
            (
                100.0,
                "Healthy".into(),
                format!("{firefighting_count} events ({pct:.1}%)"),
            )
        } else if pct <= 5.0 {
            (
                50.0,
                "Warning".into(),
                format!("{firefighting_count} events ({pct:.1}%)"),
            )
        } else {
            (
                0.0,
                "Critical".into(),
                format!("{firefighting_count} events ({pct:.1}%)"),
            )
        };
        SignalScore {
            value,
            weight: 15.0,
            rating,
            label,
        }
    };

    let commit_size_score = {
        let avg = commit_size.map(|cs| cs.avg_total_lines).unwrap_or(0.0);
        let (value, rating, label) = if avg == 0.0 {
            (75.0, "Healthy".into(), "No data (new repo?)".into())
        } else if avg < 200.0 {
            (
                100.0,
                "Healthy".into(),
                format!("Avg: {:.0} lines/commit", avg),
            )
        } else if avg <= 400.0 {
            (
                50.0,
                "Warning".into(),
                format!("Avg: {:.0} lines/commit", avg),
            )
        } else {
            (
                0.0,
                "Critical".into(),
                format!("Avg: {:.0} lines/commit", avg),
            )
        };
        SignalScore {
            value,
            weight: 10.0,
            rating,
            label,
        }
    };

    let test_signal_score = {
        let pct = test_signal.percentage;
        let (value, rating, label) = if pct > 10.0 {
            (100.0, "Healthy".into(), format!("{:.1}% test commits", pct))
        } else if pct >= 5.0 {
            (50.0, "Warning".into(), format!("{:.1}% test commits", pct))
        } else {
            (0.0, "Critical".into(), format!("{:.1}% test commits", pct))
        };
        SignalScore {
            value,
            weight: 10.0,
            rating,
            label,
        }
    };

    let overall = ((bus_factor_score.value * bus_factor_score.weight
        + churn_concentration_score.value * churn_concentration_score.weight
        + bug_churn_overlap_score.value * bug_churn_overlap_score.weight
        + velocity_trend_score.value * velocity_trend_score.weight
        + firefighting_score.value * firefighting_score.weight
        + commit_size_score.value * commit_size_score.weight
        + test_signal_score.value * test_signal_score.weight)
        / 100.0)
        .round()
        .clamp(0.0, 100.0) as u8;

    HealthScore {
        overall,
        bus_factor: bus_factor_score,
        churn_concentration: churn_concentration_score,
        bug_churn_overlap: bug_churn_overlap_score,
        velocity_trend: velocity_trend_score,
        firefighting: firefighting_score,
        commit_size: commit_size_score,
        test_signal: test_signal_score,
    }
}

fn run_audit<S: Storage + RelationshipStorage>(
    storage: &mut S,
    store: bool,
) -> Result<(), EngramError> {
    let report = collect_audit_data()?;

    println!("Project Health Audit");
    println!("====================");
    println!();

    print_health_score(&report.score);
    println!();

    print!("Churn Hotspots (top {})", report.churn.len());
    println!();
    if report.churn.is_empty() {
        println!("  No changes in the last year.");
    } else {
        let mut table = create_table();
        table.set_titles(row!["#", "Changes", "File"]);
        for (i, entry) in report.churn.iter().enumerate() {
            table.add_row(row![
                format!("{}", i + 1),
                entry.count,
                truncate(&entry.file, 60),
            ]);
        }
        table.printstd();
    }
    println!();

    println!("Bus Factor");
    println!("----------");
    println!(
        "  All-time contributors: {}",
        report.contributors_all_time.len()
    );
    if !report.contributors_all_time.is_empty() {
        let top = &report.contributors_all_time[0];
        println!(
            "  Top contributor: {} ({} commits, {:.0}%)",
            top.name,
            top.commits,
            top.commits as f64
                / report
                    .contributors_all_time
                    .iter()
                    .map(|c| c.commits as f64)
                    .sum::<f64>()
                * 100.0
        );
    }
    println!(
        "  Recent (6mo) contributors: {}",
        report.contributors_recent.len()
    );
    if !report.contributors_recent.is_empty() {
        let top_recent = &report.contributors_recent[0];
        println!(
            "  Top recent: {} ({} commits)",
            top_recent.name, top_recent.commits
        );
    }
    println!();

    print!("Bug Clusters (top {})", report.bug_clusters.len());
    println!();
    if report.bug_clusters.is_empty() {
        println!("  No bug-related commits found.");
    } else {
        let mut table = create_table();
        table.set_titles(row!["#", "Bugs", "File"]);
        for (i, entry) in report.bug_clusters.iter().enumerate() {
            table.add_row(row![
                format!("{}", i + 1),
                entry.count,
                truncate(&entry.file, 60),
            ]);
        }
        table.printstd();
    }
    println!();

    println!("Velocity Trend (12 months)");
    println!("--------------------------");
    if report.velocity.is_empty() {
        println!("  No commits in the last 12 months.");
    } else {
        let mut table = create_table();
        table.set_titles(row!["Month", "Commits"]);
        for entry in &report.velocity {
            table.add_row(row![&entry.month, entry.commits]);
        }
        table.printstd();
    }
    println!();

    println!("Firefighting (last year)");
    println!("-----------------------");
    println!(
        "  Revert/hotfix/rollback events: {}",
        report.firefighting.len()
    );
    for entry in report.firefighting.iter().take(10) {
        println!("    {} {}", entry.hash, entry.message);
    }
    println!();

    println!("Commit Size (3 months)");
    println!("---------------------");
    if let Some(cs) = &report.commit_size {
        println!(
            "  Avg: +{:.0} -{:.0} lines ({} commits)",
            cs.avg_additions, cs.avg_deletions, cs.total_commits
        );
    } else {
        println!("  No data.");
    }
    println!();

    println!("Test Signal (1 year)");
    println!("-------------------");
    println!(
        "  Test commits: {} / {} ({:.1}%)",
        report.test_signal.test_commits,
        report.test_signal.total_commits,
        report.test_signal.percentage
    );
    println!();

    if store {
        let json =
            serde_json::to_string_pretty(&report).map_err(|e| EngramError::Serialization(e))?;
        let summary = report.summary();

        let content = format!(
            "# Project Health Audit\n\n{}\n\n## Score\n{}\n\n## Raw Data\n```json\n{}\n```",
            summary,
            serde_json::to_string_pretty(&report.score)?,
            json
        );

        let project_name = std::env::current_dir()
            .ok()
            .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
            .unwrap_or_else(|| "unknown".to_string());

        let entity = crate::entities::GenericEntity {
            id: uuid::Uuid::new_v4().to_string(),
            entity_type: "context".to_string(),
            agent: "default".to_string(),
            timestamp: chrono::Utc::now(),
            data: serde_json::json!({
                "title": format!("Project Health Audit: {}", project_name),
                "content": content,
                "source": "project-health",
                "relevance": "project",
                "tags": vec!["project-health", "audit", &project_name],
            }),
        };
        storage.store(&entity)?;
        println!("  Stored as context entity: {}", entity.id);
    }

    Ok(())
}

fn print_health_score(score: &HealthScore) {
    println!("Health Score: {}/100", score.overall);
    println!("===============");
    println!();

    let mut table = create_table();
    table.set_titles(row!["Signal", "Weight", "Rating", "Detail"]);

    let signals = vec![
        ("Bus Factor", &score.bus_factor),
        ("Churn Concentration", &score.churn_concentration),
        ("Bug/Churn Overlap", &score.bug_churn_overlap),
        ("Velocity Trend", &score.velocity_trend),
        ("Firefighting", &score.firefighting),
        ("Commit Size", &score.commit_size),
        ("Test Signal", &score.test_signal),
    ];

    for (name, sig) in signals {
        table.add_row(row![
            name,
            format!("{:.0}%", sig.weight),
            sig.rating,
            sig.label,
        ]);
    }
    table.printstd();
}

fn run_churn(top: usize) -> Result<(), EngramError> {
    let output = run_git(&[
        "log",
        "--format=format:",
        "--name-only",
        "--since=1 year ago",
    ])?;
    let entries = parse_count_name_lines(&output);
    let entries: Vec<_> = entries.into_iter().take(top).collect();

    println!("Churn Hotspots (top {})", entries.len());
    println!("=====================");
    if entries.is_empty() {
        println!("  No changes in the last year.");
    } else {
        let mut table = create_table();
        table.set_titles(row!["#", "Changes", "File"]);
        for (i, entry) in entries.iter().enumerate() {
            table.add_row(row![
                format!("{}", i + 1),
                entry.count,
                truncate(&entry.file, 60),
            ]);
        }
        table.printstd();
    }

    Ok(())
}

fn run_bus_factor() -> Result<(), EngramError> {
    let all_time = run_git(&["shortlog", "-sn", "--no-merges"])?;
    let recent = run_git(&["shortlog", "-sn", "--no-merges", "--since=6 months ago"])?;
    let all_contributors = parse_shortlog(&all_time);
    let recent_contributors = parse_shortlog(&recent);

    println!("Bus Factor Analysis");
    println!("===================");
    println!();

    println!("All-time contributors ({})", all_contributors.len());
    if all_contributors.is_empty() {
        println!("  No contributors found.");
    } else {
        let mut table = create_table();
        table.set_titles(row!["#", "Commits", "Contributor"]);
        for (i, c) in all_contributors.iter().take(15).enumerate() {
            table.add_row(row![format!("{}", i + 1), c.commits, truncate(&c.name, 30),]);
        }
        table.printstd();
    }
    println!();

    println!(
        "Recent contributors — 6 months ({})",
        recent_contributors.len()
    );
    if recent_contributors.is_empty() {
        println!("  No recent contributors.");
    } else {
        let mut table = create_table();
        table.set_titles(row!["#", "Commits", "Contributor"]);
        for (i, c) in recent_contributors.iter().take(15).enumerate() {
            table.add_row(row![format!("{}", i + 1), c.commits, truncate(&c.name, 30),]);
        }
        table.printstd();
    }

    if !all_contributors.is_empty() {
        let total: usize = all_contributors.iter().map(|c| c.commits).sum();
        let top_pct = all_contributors[0].commits as f64 / total as f64 * 100.0;
        println!();
        if top_pct >= 60.0 {
            println!(
                "  WARNING: Top contributor ({}) has {:.0}% of all commits — bus factor risk!",
                all_contributors[0].name, top_pct
            );
        } else {
            println!(
                "  Top contributor ({}) has {:.0}% of commits — distributed.",
                all_contributors[0].name, top_pct
            );
        }
    }

    Ok(())
}

fn run_bug_clusters(top: usize) -> Result<(), EngramError> {
    let output = run_git(&[
        "log",
        "-i",
        "-E",
        "--grep=fix|bug|broken|regression",
        "--name-only",
        "--format=",
    ])?;
    let entries = parse_count_name_lines(&output);
    let entries: Vec<_> = entries.into_iter().take(top).collect();

    println!("Bug Clusters (top {})", entries.len());
    println!("=============");
    if entries.is_empty() {
        println!("  No bug-related commits found.");
    } else {
        let mut table = create_table();
        table.set_titles(row!["#", "Bug Commits", "File"]);
        for (i, entry) in entries.iter().enumerate() {
            table.add_row(row![
                format!("{}", i + 1),
                entry.count,
                truncate(&entry.file, 60),
            ]);
        }
        table.printstd();
    }

    Ok(())
}

fn run_velocity() -> Result<(), EngramError> {
    let output = run_git(&[
        "log",
        "--format=%ad",
        "--date=format:%Y-%m",
        "--since=12 months ago",
    ])?;
    let entries = parse_velocity(&output);

    println!("Velocity Trend (12 months)");
    println!("=========================");
    if entries.is_empty() {
        println!("  No commits in the last 12 months.");
    } else {
        let mut table = create_table();
        table.set_titles(row!["Month", "Commits"]);
        for entry in &entries {
            table.add_row(row![&entry.month, entry.commits]);
        }
        table.printstd();

        let total: usize = entries.iter().map(|v| v.commits).sum();
        let avg = total as f64 / entries.len() as f64;
        println!();
        println!(
            "  Total: {} commits across {} months (avg {:.1}/month)",
            total,
            entries.len(),
            avg
        );
    }

    Ok(())
}

fn run_firefighting() -> Result<(), EngramError> {
    let output = run_git_allow_fail(&["log", "--oneline", "--since=1 year ago"]);
    let entries: Vec<FirefightingEntry> = output
        .lines()
        .filter(|l| {
            let l = l.to_lowercase();
            l.contains("revert")
                || l.contains("hotfix")
                || l.contains("emergency")
                || l.contains("rollback")
        })
        .map(|line| {
            let parts: Vec<&str> = line.splitn(2, ' ').collect();
            let hash = parts.first().unwrap_or(&"").to_string();
            let message = parts.get(1).unwrap_or(&"").to_string();
            FirefightingEntry { hash, message }
        })
        .collect();

    println!("Firefighting (last year)");
    println!("=======================");
    println!("  Events: {}", entries.len());
    println!();
    if entries.is_empty() {
        println!("  No revert/hotfix/rollback commits found.");
    } else {
        let mut table = create_table();
        table.set_titles(row!["Hash", "Message"]);
        for entry in &entries {
            table.add_row(row![truncate(&entry.hash, 8), truncate(&entry.message, 70),]);
        }
        table.printstd();
    }

    Ok(())
}

fn run_commit_size() -> Result<(), EngramError> {
    let output = run_git(&["log", "--numstat", "--format=", "--since=3 months ago"])?;
    let stats = parse_numstat(&output);

    println!("Commit Size Distribution (3 months)");
    println!("====================================");
    if stats.total_commits == 0 {
        println!("  No commits in the last 3 months.");
    } else {
        println!("  Commits analyzed: {}", stats.total_commits);
        println!(
            "  Average: +{:.0} -{:.0} lines ({:.0} total lines changed)",
            stats.avg_additions, stats.avg_deletions, stats.avg_total_lines
        );
        println!();
        if stats.avg_total_lines < 200.0 {
            println!("  Rating: Healthy — commits are small and focused.");
        } else if stats.avg_total_lines <= 400.0 {
            println!("  Rating: Warning — commits may be too large.");
        } else {
            println!("  Rating: Critical — average commit size is very large.");
        }
    }

    Ok(())
}

fn run_test_signal() -> Result<(), EngramError> {
    let stats = compute_test_signal();

    println!("Test Signal (1 year)");
    println!("===================");
    println!(
        "  Test-related commits: {} / {} ({:.1}%)",
        stats.test_commits, stats.total_commits, stats.percentage
    );
    println!();
    if stats.total_commits == 0 {
        println!("  No commits in the last year.");
    } else if stats.percentage > 10.0 {
        println!("  Rating: Healthy — strong test culture.");
    } else if stats.percentage >= 5.0 {
        println!("  Rating: Warning — testing may be secondary.");
    } else {
        println!("  Rating: Critical — testing appears to be an afterthought.");
    }

    Ok(())
}

const EXCLUDED_ENTITY_TYPES: &[&str] = &[
    "session",
    "compliance",
    "escalation_request",
    "agent_sandbox",
    "progressive_config",
    "execution_result",
    "bottleneck_report",
    "dora_metrics_report",
    "task_duration_report",
    "stale_task_report",
    "doc_fragment",
];

const SCANNED_ENTITY_TYPES: &[&str] = &[
    "task",
    "context",
    "reasoning",
    "knowledge",
    "adr",
    "workflow",
    "workflow_instance",
    "rule",
    "standard",
    "lesson",
    "persona",
    "state_reflection",
    "theory",
];

pub struct OrphanDetector;

impl OrphanDetector {
    pub fn detect<S: Storage + RelationshipStorage>(
        storage: &S,
    ) -> Result<OrphanReport, EngramError> {
        let mut all_entities: Vec<GenericEntity> = Vec::new();
        for entity_type in SCANNED_ENTITY_TYPES {
            match storage.get_all(entity_type) {
                Ok(entities) => all_entities.extend(entities),
                Err(_) => continue,
            }
        }

        let filter = crate::entities::RelationshipFilter::new();
        let all_relationships = storage.query_relationships(&filter)?;
        let total_relationships = all_relationships.len();

        let mut connected_ids: HashSet<String> = HashSet::new();
        for rel in &all_relationships {
            connected_ids.insert(rel.source_id.clone());
            connected_ids.insert(rel.target_id.clone());
        }

        let excluded_set: HashSet<&str> = EXCLUDED_ENTITY_TYPES.iter().copied().collect();

        let mut orphaned_entities: Vec<OrphanedEntity> = Vec::new();
        let mut orphans_by_type: HashMap<String, usize> = HashMap::new();

        for entity in &all_entities {
            if connected_ids.contains(&entity.id) {
                continue;
            }
            orphans_by_type
                .entry(entity.entity_type.clone())
                .and_modify(|c| *c += 1)
                .or_insert(1);
            orphaned_entities.push(OrphanedEntity {
                id: entity.id.clone(),
                entity_type: entity.entity_type.clone(),
                agent: entity.agent.clone(),
                created_at: entity.timestamp,
            });
        }

        orphaned_entities.sort_by(|a, b| {
            a.entity_type
                .cmp(&b.entity_type)
                .then_with(|| a.id.cmp(&b.id))
        });

        Ok(OrphanReport {
            total_entities: all_entities.len(),
            total_relationships,
            orphaned_count: orphaned_entities.len(),
            orphans_by_type,
            orphaned_entities,
            excluded_types: excluded_set.into_iter().map(String::from).collect(),
        })
    }
}

fn run_orphan_detection<S: Storage + RelationshipStorage>(storage: &S) -> Result<(), EngramError> {
    let report = OrphanDetector::detect(storage)?;

    println!("Orphaned Entity Detection");
    println!("========================");
    println!();

    println!("  Total entities scanned: {}", report.total_entities);
    println!("  Total relationships:    {}", report.total_relationships);
    println!(
        "  Excluded types:         {}",
        report.excluded_types.join(", ")
    );
    println!();

    if report.orphaned_count == 0 {
        println!("  No orphaned entities found. Knowledge graph is fully connected.");
    } else {
        println!("  Orphaned entities:      {}", report.orphaned_count);
        println!();

        let mut sorted_types: Vec<_> = report.orphans_by_type.iter().collect();
        sorted_types.sort_by(|a, b| b.1.cmp(a.1));

        let mut table = create_table();
        table.set_titles(row!["Entity Type", "Orphan Count"]);
        for (entity_type, count) in sorted_types {
            table.add_row(row![entity_type, count]);
        }
        table.printstd();
        println!();

        println!("Orphaned Entity IDs");
        println!("-------------------");
        let mut current_type = String::new();
        for orphan in &report.orphaned_entities {
            if orphan.entity_type != current_type {
                current_type = orphan.entity_type.clone();
                println!();
                println!("  [{}]", current_type);
            }
            println!("    {} (agent: {})", orphan.id, orphan.agent);
        }
    }

    Ok(())
}

fn run_consistency_check<S: Storage + RelationshipStorage>(storage: &S) -> Result<(), EngramError> {
    use crate::storage::git_refs_storage::GitRefsStorage;

    let git_storage = storage
        .as_any()
        .downcast_ref::<GitRefsStorage>()
        .ok_or_else(|| {
            EngramError::InvalidOperation(
                "Consistency check requires GitRefsStorage backend".to_string(),
            )
        })?;

    let report = git_storage.consistency_check()?;

    println!("Git Refs Store Consistency Check");
    println!("================================");
    println!();
    println!("  Total refs scanned: {}", report.total_refs);
    println!("  Total blobs checked: {}", report.total_blobs_checked);
    println!("  Orphaned blobs: {}", report.orphaned_blobs);
    println!();

    let mut table = create_table();
    table.set_titles(row!["Check", "Status", "Detail"]);

    for check in &report.checks {
        let status_str = match check.status {
            crate::storage::git_refs_storage::ConsistencyCheckStatus::Pass => "PASS",
            crate::storage::git_refs_storage::ConsistencyCheckStatus::Fail => "FAIL",
            crate::storage::git_refs_storage::ConsistencyCheckStatus::Warning => "WARN",
        };
        table.add_row(row![check.name, status_str, truncate(&check.detail, 80)]);
    }
    table.printstd();
    println!();

    if !report.dangling_refs.is_empty() {
        println!("Dangling refs:");
        for r in &report.dangling_refs {
            println!("  {}", r);
        }
        println!();
    }

    if !report.invalid_json_refs.is_empty() {
        println!("Invalid JSON refs:");
        for r in &report.invalid_json_refs {
            println!("  {}", r);
        }
        println!();
    }

    if !report.missing_required_fields.is_empty() {
        println!("Missing required fields:");
        for r in &report.missing_required_fields {
            println!("  {}", r);
        }
        println!();
    }

    if !report.id_path_mismatches.is_empty() {
        println!("ID/path mismatches:");
        for r in &report.id_path_mismatches {
            println!("  {}", r);
        }
        println!();
    }

    if !report.future_timestamps.is_empty() {
        println!("Future timestamps:");
        for r in &report.future_timestamps {
            println!("  {}", r);
        }
        println!();
    }

    println!("  {}", report.summary());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_count_name_lines_basic() {
        let input = "  42 src/main.rs\n  17 src/lib.rs\n   3 Cargo.toml\n";
        let entries = parse_count_name_lines(input);
        assert_eq!(entries.len(), 3);
        assert_eq!(
            entries[0],
            ChurnEntry {
                count: 42,
                file: "src/main.rs".into()
            }
        );
        assert_eq!(
            entries[1],
            ChurnEntry {
                count: 17,
                file: "src/lib.rs".into()
            }
        );
        assert_eq!(
            entries[2],
            ChurnEntry {
                count: 3,
                file: "Cargo.toml".into()
            }
        );
    }

    #[test]
    fn parse_count_name_lines_empty() {
        let entries = parse_count_name_lines("");
        assert!(entries.is_empty());
    }

    #[test]
    fn parse_count_name_lines_skips_blank_lines() {
        let input = "  10 foo.rs\n\n  5 bar.rs\n";
        let entries = parse_count_name_lines(input);
        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn parse_shortlog_basic() {
        let input = "  42\tAlice\n  17\tBob\n   3\tCarol\n";
        let contributors = parse_shortlog(input);
        assert_eq!(contributors.len(), 3);
        assert_eq!(
            contributors[0],
            Contributor {
                commits: 42,
                name: "Alice".into()
            }
        );
        assert_eq!(
            contributors[1],
            Contributor {
                commits: 17,
                name: "Bob".into()
            }
        );
    }

    #[test]
    fn health_score_perfect() {
        let contributors = vec![
            Contributor {
                commits: 10,
                name: "A".into(),
            },
            Contributor {
                commits: 8,
                name: "B".into(),
            },
            Contributor {
                commits: 5,
                name: "C".into(),
            },
        ];
        let churn = (1..=20)
            .map(|i| ChurnEntry {
                count: 1,
                file: format!("file{}.rs", i),
            })
            .collect::<Vec<_>>();
        let bugs = vec![ChurnEntry {
            count: 5,
            file: "other.rs".into(),
        }];
        let velocity = vec![
            VelocityEntry {
                month: "2025-01".into(),
                commits: 50,
            },
            VelocityEntry {
                month: "2025-02".into(),
                commits: 55,
            },
            VelocityEntry {
                month: "2025-03".into(),
                commits: 52,
            },
            VelocityEntry {
                month: "2025-04".into(),
                commits: 58,
            },
        ];
        let commit_size = CommitSizeStats {
            avg_additions: 50.0,
            avg_deletions: 30.0,
            total_commits: 10,
            avg_total_lines: 80.0,
        };
        let test_signal = TestSignalStats {
            test_commits: 20,
            total_commits: 100,
            percentage: 20.0,
        };

        let score = compute_health_score_from_signals(
            &contributors,
            &churn,
            &bugs,
            &velocity,
            1,
            Some(&commit_size),
            &test_signal,
        );
        assert_eq!(score.overall, 100);
    }

    #[test]
    fn health_score_critical() {
        let contributors = vec![Contributor {
            commits: 50,
            name: "Solo".into(),
        }];
        let churn = vec![ChurnEntry {
            count: 100,
            file: "monster.rs".into(),
        }];
        let bugs = vec![ChurnEntry {
            count: 50,
            file: "monster.rs".into(),
        }];
        let velocity = vec![
            VelocityEntry {
                month: "2025-01".into(),
                commits: 100,
            },
            VelocityEntry {
                month: "2025-02".into(),
                commits: 10,
            },
        ];
        let commit_size = CommitSizeStats {
            avg_additions: 500.0,
            avg_deletions: 300.0,
            total_commits: 10,
            avg_total_lines: 800.0,
        };
        let test_signal = TestSignalStats {
            test_commits: 2,
            total_commits: 100,
            percentage: 2.0,
        };

        let score = compute_health_score_from_signals(
            &contributors,
            &churn,
            &bugs,
            &velocity,
            10,
            Some(&commit_size),
            &test_signal,
        );
        assert!(
            score.overall < 20,
            "Expected critical score, got {}",
            score.overall
        );
    }

    #[test]
    fn parse_velocity_basic() {
        let input = "  42 2025-01\n  38 2025-02\n  55 2025-03\n";
        let entries = parse_velocity(input);
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].month, "2025-01");
        assert_eq!(entries[0].commits, 42);
    }

    #[test]
    fn parse_velocity_empty() {
        let entries = parse_velocity("");
        assert!(entries.is_empty());
    }

    #[test]
    fn parse_numstat_basic() {
        let input = "10\t5\tsrc/main.rs\n3\t1\tsrc/lib.rs\n";
        let stats = parse_numstat(input);
        assert_eq!(stats.total_commits, 2);
        assert!((stats.avg_additions - 6.5).abs() < 0.01);
        assert!((stats.avg_deletions - 3.0).abs() < 0.01);
    }

    #[test]
    fn parse_numstat_empty() {
        let stats = parse_numstat("");
        assert_eq!(stats.total_commits, 0);
        assert!((stats.avg_additions - 0.0).abs() < 0.01);
    }

    #[test]
    fn parse_numstat_binary_files_skipped() {
        let input = "0\t0\tbinary_file.bin\n-\t-\timg.png\n10\t5\tcode.rs\n";
        let stats = parse_numstat(input);
        assert_eq!(stats.total_commits, 1);
    }

    #[test]
    fn health_score_mixed() {
        let contributors = vec![
            Contributor {
                commits: 40,
                name: "A".into(),
            },
            Contributor {
                commits: 10,
                name: "B".into(),
            },
        ];
        let churn = vec![
            ChurnEntry {
                count: 60,
                file: "big.rs".into(),
            },
            ChurnEntry {
                count: 40,
                file: "small.rs".into(),
            },
        ];
        let bugs = vec![ChurnEntry {
            count: 10,
            file: "unrelated.rs".into(),
        }];
        let velocity = vec![
            VelocityEntry {
                month: "2025-01".into(),
                commits: 30,
            },
            VelocityEntry {
                month: "2025-02".into(),
                commits: 25,
            },
        ];
        let commit_size = CommitSizeStats {
            avg_additions: 100.0,
            avg_deletions: 50.0,
            total_commits: 10,
            avg_total_lines: 150.0,
        };
        let test_signal = TestSignalStats {
            test_commits: 7,
            total_commits: 100,
            percentage: 7.0,
        };

        let score = compute_health_score_from_signals(
            &contributors,
            &churn,
            &bugs,
            &velocity,
            2,
            Some(&commit_size),
            &test_signal,
        );
        assert!(score.overall > 0 && score.overall < 100);
    }

    #[test]
    fn health_score_no_data() {
        let score = compute_health_score_from_signals(
            &[],
            &[],
            &[],
            &[],
            0,
            None,
            &TestSignalStats {
                test_commits: 0,
                total_commits: 0,
                percentage: 0.0,
            },
        );
        assert!(score.overall < 100);
    }

    #[test]
    fn health_score_bus_factor_single_contributor() {
        let score = compute_health_score_from_signals(
            &[Contributor {
                commits: 10,
                name: "Solo".into(),
            }],
            &[],
            &[],
            &[],
            0,
            None,
            &TestSignalStats {
                test_commits: 0,
                total_commits: 10,
                percentage: 0.0,
            },
        );
        assert_eq!(score.bus_factor.value, 40.0);
        assert_eq!(score.bus_factor.rating, "Warning");
    }

    #[test]
    fn health_score_churn_concentration() {
        let churn = vec![
            ChurnEntry {
                count: 50,
                file: "a.rs".into(),
            },
            ChurnEntry {
                count: 50,
                file: "b.rs".into(),
            },
        ];
        let score = compute_health_score_from_signals(
            &[],
            &churn,
            &[],
            &[],
            0,
            None,
            &TestSignalStats {
                test_commits: 0,
                total_commits: 10,
                percentage: 0.0,
            },
        );
        assert_eq!(score.churn_concentration.value, 0.0);
        assert_eq!(score.churn_concentration.rating, "Critical");
    }

    #[test]
    fn health_score_bug_churn_overlap() {
        let churn = vec![ChurnEntry {
            count: 10,
            file: "shared.rs".into(),
        }];
        let bugs = vec![ChurnEntry {
            count: 5,
            file: "shared.rs".into(),
        }];
        let score = compute_health_score_from_signals(
            &[],
            &churn,
            &bugs,
            &[],
            0,
            None,
            &TestSignalStats {
                test_commits: 0,
                total_commits: 10,
                percentage: 0.0,
            },
        );
        assert_eq!(score.bug_churn_overlap.value, 50.0);
    }

    #[test]
    fn health_score_firefighting_high() {
        let score = compute_health_score_from_signals(
            &[],
            &[],
            &[],
            &[],
            3,
            None,
            &TestSignalStats {
                test_commits: 0,
                total_commits: 100,
                percentage: 0.0,
            },
        );
        assert_eq!(score.firefighting.value, 50.0);
        assert_eq!(score.firefighting.rating, "Warning");
    }

    #[test]
    fn health_score_commit_size_warning() {
        let cs = CommitSizeStats {
            avg_additions: 150.0,
            avg_deletions: 100.0,
            total_commits: 5,
            avg_total_lines: 250.0,
        };
        let score = compute_health_score_from_signals(
            &[],
            &[],
            &[],
            &[],
            0,
            Some(&cs),
            &TestSignalStats {
                test_commits: 0,
                total_commits: 10,
                percentage: 0.0,
            },
        );
        assert_eq!(score.commit_size.value, 50.0);
        assert_eq!(score.commit_size.rating, "Warning");
    }

    #[test]
    fn health_score_test_signal_low() {
        let score = compute_health_score_from_signals(
            &[],
            &[],
            &[],
            &[],
            0,
            None,
            &TestSignalStats {
                test_commits: 3,
                total_commits: 100,
                percentage: 3.0,
            },
        );
        assert_eq!(score.test_signal.value, 0.0);
        assert_eq!(score.test_signal.rating, "Critical");
    }

    #[test]
    fn structured_feedback_health_score() {
        let score = HealthScore {
            overall: 85,
            bus_factor: SignalScore {
                value: 100.0,
                weight: 20.0,
                rating: "Healthy".into(),
                label: "3 contributors".into(),
            },
            churn_concentration: SignalScore {
                value: 100.0,
                weight: 15.0,
                rating: "Healthy".into(),
                label: "".into(),
            },
            bug_churn_overlap: SignalScore {
                value: 100.0,
                weight: 15.0,
                rating: "Healthy".into(),
                label: "".into(),
            },
            velocity_trend: SignalScore {
                value: 100.0,
                weight: 15.0,
                rating: "Healthy".into(),
                label: "".into(),
            },
            firefighting: SignalScore {
                value: 100.0,
                weight: 15.0,
                rating: "Healthy".into(),
                label: "".into(),
            },
            commit_size: SignalScore {
                value: 100.0,
                weight: 10.0,
                rating: "Healthy".into(),
                label: "".into(),
            },
            test_signal: SignalScore {
                value: 50.0,
                weight: 10.0,
                rating: "Warning".into(),
                label: "".into(),
            },
        };
        assert_eq!(score.status_code(), FeedbackStatus::Success);
        assert!(score.summary().contains("85"));
        let json = score.to_json();
        assert_eq!(json["overall"], 85);
    }

    #[test]
    fn structured_feedback_health_score_critical() {
        let score = HealthScore {
            overall: 30,
            bus_factor: SignalScore {
                value: 0.0,
                weight: 20.0,
                rating: "Critical".into(),
                label: "".into(),
            },
            churn_concentration: SignalScore {
                value: 0.0,
                weight: 15.0,
                rating: "Critical".into(),
                label: "".into(),
            },
            bug_churn_overlap: SignalScore {
                value: 0.0,
                weight: 15.0,
                rating: "Critical".into(),
                label: "".into(),
            },
            velocity_trend: SignalScore {
                value: 0.0,
                weight: 15.0,
                rating: "Critical".into(),
                label: "".into(),
            },
            firefighting: SignalScore {
                value: 0.0,
                weight: 15.0,
                rating: "Critical".into(),
                label: "".into(),
            },
            commit_size: SignalScore {
                value: 0.0,
                weight: 10.0,
                rating: "Critical".into(),
                label: "".into(),
            },
            test_signal: SignalScore {
                value: 0.0,
                weight: 10.0,
                rating: "Critical".into(),
                label: "".into(),
            },
        };
        assert_eq!(score.status_code(), FeedbackStatus::Failed);
    }

    #[test]
    fn structured_feedback_health_audit_report() {
        let report = HealthAuditReport {
            churn: vec![],
            contributors_all_time: vec![],
            contributors_recent: vec![],
            bug_clusters: vec![],
            velocity: vec![],
            firefighting: vec![],
            commit_size: None,
            test_signal: TestSignalStats {
                test_commits: 0,
                total_commits: 0,
                percentage: 0.0,
            },
            score: HealthScore {
                overall: 50,
                bus_factor: SignalScore {
                    value: 0.0,
                    weight: 20.0,
                    rating: "".into(),
                    label: "".into(),
                },
                churn_concentration: SignalScore {
                    value: 0.0,
                    weight: 15.0,
                    rating: "".into(),
                    label: "".into(),
                },
                bug_churn_overlap: SignalScore {
                    value: 0.0,
                    weight: 15.0,
                    rating: "".into(),
                    label: "".into(),
                },
                velocity_trend: SignalScore {
                    value: 0.0,
                    weight: 15.0,
                    rating: "".into(),
                    label: "".into(),
                },
                firefighting: SignalScore {
                    value: 0.0,
                    weight: 15.0,
                    rating: "".into(),
                    label: "".into(),
                },
                commit_size: SignalScore {
                    value: 0.0,
                    weight: 10.0,
                    rating: "".into(),
                    label: "".into(),
                },
                test_signal: SignalScore {
                    value: 0.0,
                    weight: 10.0,
                    rating: "".into(),
                    label: "".into(),
                },
            },
        };
        assert_eq!(report.status_code(), FeedbackStatus::Failed);
        assert!(report.summary().contains("50"));
    }

    #[test]
    fn compute_test_signal_from_lines() {
        let output = "abc1234 feat: add auth\nabc1235 fix: login bug\nabc1236 test: add user tests\nabc1237 spec: coverage\nabc1238 refactor: cleanup";
        let total = output.lines().filter(|l| !l.trim().is_empty()).count();
        let test_count = output
            .lines()
            .filter(|l| {
                let l = l.to_lowercase();
                l.contains("test") || l.contains("spec") || l.contains("coverage")
            })
            .count();
        assert_eq!(total, 5);
        assert_eq!(test_count, 2);
    }

    use crate::entities::{
        Context, ContextRelevance, Entity, EntityRelationType, EntityRelationship, Task,
        TaskPriority, TaskStatus,
    };
    use crate::storage::MemoryStorage;

    fn make_task(id: &str, agent: &str) -> GenericEntity {
        Task {
            id: id.to_string(),
            title: "Test".to_string(),
            description: String::new(),
            status: TaskStatus::Todo,
            priority: TaskPriority::Medium,
            agent: agent.to_string(),
            start_time: chrono::Utc::now(),
            end_time: None,
            parent: None,
            children: Vec::new(),
            tags: Vec::new(),
            context_ids: Vec::new(),
            knowledge: Vec::new(),
            files: Vec::new(),
            outcome: None,
            block_reason: None,
            workflow_id: None,
            workflow_state: None,
            metadata: std::collections::HashMap::new(),
        }
        .to_generic()
    }

    fn make_context(id: &str, agent: &str) -> GenericEntity {
        Context {
            id: id.to_string(),
            title: "Test Context".to_string(),
            content: "content".to_string(),
            source: "test".to_string(),
            source_id: None,
            relevance: ContextRelevance::Medium,
            agent: agent.to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            tags: Vec::new(),
            related_entities: Vec::new(),
            metadata: std::collections::HashMap::new(),
        }
        .to_generic()
    }

    fn make_rel(id: &str, src: &str, tgt: &str) -> EntityRelationship {
        EntityRelationship::new(
            id.to_string(),
            "test-agent".to_string(),
            src.to_string(),
            "task".to_string(),
            tgt.to_string(),
            "context".to_string(),
            EntityRelationType::References,
        )
    }

    #[test]
    fn orphan_detector_no_orphans() {
        let mut storage = MemoryStorage::new("test-agent");
        storage.store(&make_task("t1", "a")).unwrap();
        storage.store(&make_context("c1", "a")).unwrap();
        storage
            .store_relationship(&make_rel("r1", "t1", "c1"))
            .unwrap();

        let report = OrphanDetector::detect(&storage).unwrap();
        assert_eq!(report.total_entities, 2);
        assert_eq!(report.total_relationships, 1);
        assert_eq!(report.orphaned_count, 0);
        assert_eq!(report.status_code(), FeedbackStatus::Success);
        assert!(report.summary().contains("clean"));
    }

    #[test]
    fn orphan_detector_finds_orphans() {
        let mut storage = MemoryStorage::new("test-agent");
        storage.store(&make_task("t1", "a")).unwrap();
        storage.store(&make_task("t2", "a")).unwrap();
        storage.store(&make_context("c1", "a")).unwrap();
        storage
            .store_relationship(&make_rel("r1", "t1", "c1"))
            .unwrap();

        let report = OrphanDetector::detect(&storage).unwrap();
        assert_eq!(report.total_entities, 3);
        assert_eq!(report.orphaned_count, 1);
        assert_eq!(report.orphans_by_type.get("task"), Some(&1));
        assert_eq!(report.orphaned_entities[0].id, "t2");
        assert_eq!(report.status_code(), FeedbackStatus::Warning);
        assert!(report.summary().contains("1"));
    }

    #[test]
    fn orphan_detector_empty_storage() {
        let storage = MemoryStorage::new("test-agent");
        let report = OrphanDetector::detect(&storage).unwrap();
        assert_eq!(report.total_entities, 0);
        assert_eq!(report.total_relationships, 0);
        assert_eq!(report.orphaned_count, 0);
        assert_eq!(report.status_code(), FeedbackStatus::Success);
    }

    #[test]
    fn orphan_detector_excluded_types_ignored() {
        let mut storage = MemoryStorage::new("test-agent");
        let session = GenericEntity {
            id: "sess-1".to_string(),
            entity_type: "session".to_string(),
            agent: "a".to_string(),
            timestamp: chrono::Utc::now(),
            data: serde_json::json!({"title": "test"}),
        };
        storage.store(&session).unwrap();
        storage.store(&make_task("t1", "a")).unwrap();

        let report = OrphanDetector::detect(&storage).unwrap();
        assert_eq!(report.total_entities, 1);
        assert_eq!(report.orphaned_count, 1);
        assert!(report.excluded_types.contains(&"session".to_string()));
    }

    #[test]
    fn orphan_detector_bidirectional_relationship() {
        let mut storage = MemoryStorage::new("test-agent");
        storage.store(&make_task("t1", "a")).unwrap();
        storage.store(&make_context("c1", "a")).unwrap();

        let mut rel = make_rel("r1", "t1", "c1");
        rel.direction = crate::entities::RelationshipDirection::Bidirectional;
        storage.store_relationship(&rel).unwrap();

        let report = OrphanDetector::detect(&storage).unwrap();
        assert_eq!(report.orphaned_count, 0);
    }

    #[test]
    fn orphan_report_structured_feedback() {
        let report = OrphanReport {
            total_entities: 10,
            total_relationships: 8,
            orphaned_count: 3,
            orphans_by_type: HashMap::from([("task".to_string(), 2), ("knowledge".to_string(), 1)]),
            orphaned_entities: vec![
                OrphanedEntity {
                    id: "t-orphan-1".to_string(),
                    entity_type: "task".to_string(),
                    agent: "a".to_string(),
                    created_at: chrono::Utc::now(),
                },
                OrphanedEntity {
                    id: "t-orphan-2".to_string(),
                    entity_type: "task".to_string(),
                    agent: "b".to_string(),
                    created_at: chrono::Utc::now(),
                },
                OrphanedEntity {
                    id: "k-orphan-1".to_string(),
                    entity_type: "knowledge".to_string(),
                    agent: "a".to_string(),
                    created_at: chrono::Utc::now(),
                },
            ],
            excluded_types: vec!["session".to_string()],
        };

        assert_eq!(report.status_code(), FeedbackStatus::Warning);
        let summary = report.summary();
        assert!(summary.contains("3"));
        assert!(summary.contains("task"));
        assert!(summary.contains("knowledge"));

        let json = report.to_json();
        assert_eq!(json["orphaned_count"], 3);
        assert_eq!(json["total_entities"], 10);
    }

    #[test]
    fn orphan_report_clean_feedback() {
        let report = OrphanReport {
            total_entities: 5,
            total_relationships: 4,
            orphaned_count: 0,
            orphans_by_type: HashMap::new(),
            orphaned_entities: vec![],
            excluded_types: vec!["session".to_string()],
        };

        assert_eq!(report.status_code(), FeedbackStatus::Success);
        assert!(report.summary().contains("clean"));
    }
}
