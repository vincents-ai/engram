use crate::cli::utils::{create_table, truncate};
use crate::entities::bottleneck_report::{BottleneckEntry, BottleneckReport};
use crate::entities::dora_metrics_report::{DoraMetricsCalculator, DoraMetricsReport};
use crate::entities::task_duration_report::TaskDurationReport;
use crate::entities::{Entity, Task, TaskStatus};
use crate::error::EngramError;
use crate::storage::Storage;
use chrono::{DateTime, Utc};
use clap::Subcommand;
use prettytable::row;
use serde_json::{json, Value};
use std::collections::BTreeMap;

/// A value that could not be rated because the report had no input for it.
const INSUFFICIENT_DATA: &str = "Insufficient data";

/// Completed tasks older than this are counted separately instead of being
/// folded into a per-task average.
const LONG_LIVED_THRESHOLD_HOURS: f64 = 30.0 * 24.0;

/// Honest description of what the task "duration" column measures.
const ELAPSED_SEMANTICS: &str = "elapsed since creation, includes time blocked and deferred";

#[derive(Subcommand)]
pub enum AnalyticsCommands {
    /// Compute and display DORA metrics
    Dora {
        /// Time window in days (default: 30)
        #[arg(long, default_value = "30")]
        window_days: i64,
    },
    /// Generate a task duration report
    Report {},
    /// Identify slowest tasks and bottlenecks
    Bottleneck {
        /// Number of slowest tasks to show (default: 10)
        #[arg(long, default_value = "10")]
        top: usize,
    },
}

pub fn handle_analytics_command<S: Storage>(
    storage: &mut S,
    command: AnalyticsCommands,
) -> Result<(), EngramError> {
    match command {
        AnalyticsCommands::Dora { window_days } => run_dora(storage, window_days),
        AnalyticsCommands::Report {} => run_duration_report(storage),
        AnalyticsCommands::Bottleneck { top } => run_bottleneck(storage, top),
    }
}

// ---------------------------------------------------------------------------
// DORA metrics
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DoraMetric {
    DeploymentFrequency,
    LeadTime,
    ChangeFailureRate,
    MeanTimeToRecovery,
}

/// Rate a DORA metric only when it has inputs. A metric whose input set is
/// empty is reported as `Insufficient data`, never as a rating — in particular
/// never `Elite` for a zero-input value.
fn dora_rating_for(metric: DoraMetric, value: f64, inputs_analyzed: u64) -> &'static str {
    if inputs_analyzed == 0 {
        return INSUFFICIENT_DATA;
    }
    match metric {
        DoraMetric::DeploymentFrequency => dora_rating_deployment_freq(value),
        DoraMetric::LeadTime => dora_rating_lead_time(value),
        DoraMetric::ChangeFailureRate => dora_rating_cfr(value),
        DoraMetric::MeanTimeToRecovery => dora_rating_mttr(value),
    }
}

fn render_dora_report(report: &DoraMetricsReport) -> String {
    let mut out = String::new();
    out.push_str("DORA Metrics Report\n");
    out.push_str("===================\n");
    out.push_str(&format!("  Project:    {}\n", report.project_path));
    out.push_str(&format!(
        "  Window:     {} to {}\n",
        report.window_start.format("%Y-%m-%d"),
        report.window_end.format("%Y-%m-%d")
    ));
    out.push_str(&format!("  Computed:   {}\n", report.computed_at.format("%Y-%m-%d %H:%M UTC")));
    out.push('\n');

    let mut table = create_table();
    table.set_titles(row!["Metric", "Value", "Rating"]);

    let rows: [(&str, DoraMetric, f64, u64, String); 4] = [
        (
            "Deployment Frequency",
            DoraMetric::DeploymentFrequency,
            report.deployment_frequency,
            report.commits_analyzed,
            format!("{:.1}/week", report.deployment_frequency),
        ),
        (
            "Lead Time for Changes",
            DoraMetric::LeadTime,
            report.lead_time_for_changes,
            report.commits_analyzed,
            format!("{:.1} days", report.lead_time_for_changes),
        ),
        (
            "Change Failure Rate",
            DoraMetric::ChangeFailureRate,
            report.change_failure_rate,
            report.executions_analyzed,
            format!("{:.1}%", report.change_failure_rate * 100.0),
        ),
        (
            "Mean Time to Recovery",
            DoraMetric::MeanTimeToRecovery,
            report.mean_time_to_recovery,
            report.escalations_analyzed,
            format!("{:.1} hours", report.mean_time_to_recovery),
        ),
    ];

    for (label, metric, value, inputs, formatted_value) in rows {
        let rating = dora_rating_for(metric, value, inputs);
        let value_text =
            if rating == INSUFFICIENT_DATA { "n/a".to_string() } else { formatted_value };
        table.add_row(row![label, value_text, rating]);
    }

    out.push_str(&table.to_string());
    out.push('\n');
    out.push_str(&format!("  Commits analyzed:      {}\n", report.commits_analyzed));
    out.push_str(&format!("  Executions analyzed:   {}\n", report.executions_analyzed));
    out.push_str(&format!("  Escalations analyzed:  {}\n", report.escalations_analyzed));
    out.push('\n');
    out.push_str(
        "  Rating needs data: deployment frequency and lead time need commits in the window; \
change failure rate needs execution results; mean time to recovery needs resolved escalations.\n",
    );
    out.push_str(&format!("  Report ID: {}\n", report.id));
    out
}

fn run_dora<S: Storage>(storage: &mut S, window_days: i64) -> Result<(), EngramError> {
    let repo_path = std::path::Path::new(".");
    let agent = "default";

    let report = DoraMetricsCalculator::compute(storage, repo_path, agent, window_days)?;

    print!("{}", render_dora_report(&report));

    let generic = report.to_generic();
    storage.store(&generic)?;

    Ok(())
}

fn dora_rating_deployment_freq(freq: f64) -> &'static str {
    if freq >= 1.0 {
        "Elite"
    } else if freq >= 0.5 {
        "High"
    } else if freq >= 0.1 {
        "Medium"
    } else {
        "Low"
    }
}

fn dora_rating_lead_time(days: f64) -> &'static str {
    if days <= 1.0 {
        "Elite"
    } else if days <= 7.0 {
        "High"
    } else if days <= 30.0 {
        "Medium"
    } else {
        "Low"
    }
}

fn dora_rating_cfr(rate: f64) -> &'static str {
    if rate <= 0.05 {
        "Elite"
    } else if rate <= 0.10 {
        "High"
    } else if rate <= 0.15 {
        "Medium"
    } else {
        "Low"
    }
}

fn dora_rating_mttr(hours: f64) -> &'static str {
    if hours <= 1.0 {
        "Elite"
    } else if hours <= 24.0 {
        "High"
    } else if hours <= 168.0 {
        "Medium"
    } else {
        "Low"
    }
}

// ---------------------------------------------------------------------------
// Task duration report
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Default)]
struct DurationStats {
    count: usize,
    p50: f64,
    p90: f64,
    mean: f64,
    min: f64,
    max: f64,
    older_than_30_days: u64,
}

impl DurationStats {
    fn from_values(mut values: Vec<f64>) -> Self {
        values.retain(|value| value.is_finite());
        if values.is_empty() {
            return Self::default();
        }
        values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let count = values.len();
        let mean = values.iter().sum::<f64>() / count as f64;
        Self {
            count,
            p50: median(&values),
            p90: percentile(&values, 0.90),
            mean,
            min: values[0],
            max: values[count - 1],
            older_than_30_days: values
                .iter()
                .filter(|value| **value > LONG_LIVED_THRESHOLD_HOURS)
                .count() as u64,
        }
    }
}

/// Median of an ascending slice.
fn median(sorted: &[f64]) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        (sorted[mid - 1] + sorted[mid]) / 2.0
    } else {
        sorted[mid]
    }
}

/// Nearest-rank percentile over an ascending slice.
fn percentile(sorted: &[f64], fraction: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let rank = ((fraction * sorted.len() as f64).ceil() as usize).saturating_sub(1);
    sorted[rank.min(sorted.len() - 1)]
}

fn status_label(status: &TaskStatus) -> String {
    format!("{:?}", status).to_lowercase()
}

fn elapsed_hours(task: &Task, now: DateTime<Utc>) -> f64 {
    let end = task.end_time.unwrap_or(now);
    (end.signed_duration_since(task.start_time).num_seconds() as f64 / 3600.0).max(0.0)
}

fn parse_timestamp(value: &Value) -> Option<DateTime<Utc>> {
    value
        .as_str()
        .and_then(|raw| DateTime::parse_from_rfc3339(raw).ok())
        .map(|dt| dt.with_timezone(&Utc))
}

/// Read block history from the task payload if it records any.
///
/// Supported shapes, in order:
/// - `metadata.blocked_intervals`: `[{ "blocked_at": ..., "unblocked_at": ... }]`
/// - `metadata.blocked_at` (and optional `metadata.unblocked_at`)
fn blocked_intervals(task: &Task) -> Option<Vec<(DateTime<Utc>, Option<DateTime<Utc>>)>> {
    let metadata = &task.metadata;

    if let Some(intervals) = metadata.get("blocked_intervals").and_then(|value| value.as_array()) {
        let parsed: Vec<_> = intervals
            .iter()
            .filter_map(|item| {
                let blocked_at = parse_timestamp(item.get("blocked_at")?)?;
                let unblocked_at = item.get("unblocked_at").and_then(parse_timestamp);
                Some((blocked_at, unblocked_at))
            })
            .collect();
        if !parsed.is_empty() {
            return Some(parsed);
        }
    }

    let blocked_at = metadata.get("blocked_at").and_then(parse_timestamp)?;
    let unblocked_at = metadata.get("unblocked_at").and_then(parse_timestamp);
    Some(vec![(blocked_at, unblocked_at)])
}

/// Active duration: elapsed minus the parts of the task's life recorded as
/// blocked. `None` when the payload carries no block history.
fn active_hours(task: &Task, elapsed: f64, now: DateTime<Utc>) -> Option<f64> {
    let intervals = blocked_intervals(task)?;
    let end = task.end_time.unwrap_or(now);
    let mut blocked_seconds = 0.0;
    for (blocked_at, unblocked_at) in intervals {
        let from = blocked_at.max(task.start_time);
        let to = unblocked_at.unwrap_or(end).min(end);
        let overlap = to.signed_duration_since(from).num_seconds();
        if overlap > 0 {
            blocked_seconds += overlap as f64;
        }
    }
    Some((elapsed - blocked_seconds / 3600.0).max(0.0))
}

fn load_tasks<S: Storage>(storage: &S) -> Result<Vec<Task>, EngramError> {
    let generics = storage.get_all("task")?;
    Ok(generics.iter().filter_map(|generic| Task::from_generic(generic.clone()).ok()).collect())
}

fn status_counts(tasks: &[Task]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for task in tasks {
        *counts.entry(status_label(&task.status)).or_insert(0) += 1;
    }
    counts
}

fn status_counts_json(counts: &BTreeMap<String, usize>) -> Value {
    Value::Object(counts.iter().map(|(status, count)| (status.clone(), json!(count))).collect())
}

fn render_duration_report(
    report: &TaskDurationReport,
    tasks: &[Task],
    now: DateTime<Utc>,
) -> String {
    let completed: Vec<&Task> =
        tasks.iter().filter(|task| task.status == TaskStatus::Done).collect();
    let elapsed =
        DurationStats::from_values(completed.iter().map(|task| elapsed_hours(task, now)).collect());

    let active_values: Vec<f64> = completed
        .iter()
        .filter_map(|task| active_hours(task, elapsed_hours(task, now), now))
        .collect();
    let active_available = !active_values.is_empty();
    let active = DurationStats::from_values(active_values);

    let long_lived_all =
        tasks.iter().filter(|task| elapsed_hours(task, now) > LONG_LIVED_THRESHOLD_HOURS).count();
    let counts = status_counts(tasks);

    let mut out = String::new();
    out.push_str("Task Duration Report\n");
    out.push_str("====================\n");
    out.push_str(&format!("  Computed: {}\n", report.computed_at.format("%Y-%m-%d %H:%M UTC")));
    out.push_str(&format!("  Duration semantics: {}.\n", ELAPSED_SEMANTICS));
    if active_available {
        out.push_str(&format!(
            "  Active duration: excludes recorded blocked intervals ({} of {} completed tasks carry block history).\n",
            active.count, elapsed.count
        ));
    } else {
        out.push_str(
            "  Active duration: not available — the task payload records no blocked_at/unblocked_at history.\n",
        );
    }
    out.push_str(&format!(
        "  Tasks analyzed: {} completed of {} total. Statistics cover completed tasks only; the table lists every status.\n",
        report.completed_tasks, report.total_tasks_analyzed
    ));
    let counts_text: Vec<String> =
        counts.iter().map(|(status, count)| format!("{}={}", status, count)).collect();
    out.push_str(&format!("  Statuses in table: {}\n", counts_text.join(", ")));
    out.push('\n');

    out.push_str("  Completed task elapsed statistics (hours):\n");
    out.push_str(&format!("    p50 (median):  {:.2}\n", elapsed.p50));
    out.push_str(&format!("    p90:           {:.2}\n", elapsed.p90));
    out.push_str(&format!("    Mean:          {:.2}\n", elapsed.mean));
    out.push_str(&format!("    Min:           {:.2}\n", elapsed.min));
    out.push_str(&format!("    Max:           {:.2}\n", elapsed.max));
    out.push_str(&format!(
        "  Completed tasks older than 30 days: {} (reported separately, not as a per-task average)\n",
        elapsed.older_than_30_days
    ));
    out.push_str(&format!("  Tasks older than 30 days across all statuses: {}\n", long_lived_all));
    out.push('\n');

    if active_available {
        out.push_str("  Completed task active statistics (hours, blocked intervals excluded):\n");
        out.push_str(&format!("    p50:  {:.2}\n", active.p50));
        out.push_str(&format!("    p90:  {:.2}\n", active.p90));
        out.push_str(&format!("    Mean: {:.2}\n", active.mean));
        out.push_str(&format!("    Min:  {:.2}\n", active.min));
        out.push_str(&format!("    Max:  {:.2}\n", active.max));
    } else {
        out.push_str(
            "  Completed task active statistics: not computed — no block history in the task payload.\n",
        );
    }
    out.push('\n');

    if report.task_durations.is_empty() {
        out.push_str("  No tasks found.\n");
    } else {
        let display_count = report.task_durations.len().min(20);
        let mut table = create_table();
        table.set_titles(row!["ID", "Status", "Elapsed (h)", "Title", "Agent"]);

        for entry in &report.task_durations[..display_count] {
            table.add_row(row![
                &entry.task_id[..entry.task_id.len().min(8)],
                &entry.status,
                format!("{:.2}", entry.duration_hours),
                truncate(&entry.title, 40),
                truncate(&entry.agent, 10),
            ]);
        }
        out.push_str(&table.to_string());

        if report.task_durations.len() > display_count {
            out.push_str(&format!(
                "  (showing {} of {} tasks)\n",
                display_count,
                report.task_durations.len()
            ));
        }
    }

    out.push('\n');
    out.push_str(&format!("  Report ID: {}\n", report.id));
    out
}

fn duration_report_metadata(tasks: &[Task], now: DateTime<Utc>) -> Vec<(&'static str, Value)> {
    let completed: Vec<&Task> =
        tasks.iter().filter(|task| task.status == TaskStatus::Done).collect();
    let elapsed =
        DurationStats::from_values(completed.iter().map(|task| elapsed_hours(task, now)).collect());
    let active_values: Vec<f64> = completed
        .iter()
        .filter_map(|task| active_hours(task, elapsed_hours(task, now), now))
        .collect();
    let active = DurationStats::from_values(active_values);
    let counts = status_counts(tasks);

    let mut fields = vec![
        ("duration_semantics", json!(ELAPSED_SEMANTICS)),
        ("p90_duration_hours", json!(elapsed.p90)),
        ("long_lived_threshold_days", json!(LONG_LIVED_THRESHOLD_HOURS / 24.0)),
        ("long_lived_completed_tasks", json!(elapsed.older_than_30_days)),
        (
            "long_lived_all_tasks",
            json!(tasks
                .iter()
                .filter(|task| elapsed_hours(task, now) > LONG_LIVED_THRESHOLD_HOURS)
                .count()),
        ),
        ("status_counts", status_counts_json(&counts)),
        ("active_duration_available", json!(active.count > 0)),
    ];

    if active.count > 0 {
        fields.push(("active_p50_duration_hours", json!(active.p50)));
        fields.push(("active_p90_duration_hours", json!(active.p90)));
        fields.push(("active_mean_duration_hours", json!(active.mean)));
        fields.push(("active_min_duration_hours", json!(active.min)));
        fields.push(("active_max_duration_hours", json!(active.max)));
        fields.push(("active_tasks_with_history", json!(active.count)));
    }

    fields
}

fn run_duration_report<S: Storage>(storage: &mut S) -> Result<(), EngramError> {
    let repo_path = std::path::Path::new(".");
    let agent = "default";

    let report = TaskDurationReport::compute(storage, repo_path, agent)?;
    let tasks = load_tasks(storage)?;
    let now = Utc::now();

    print!("{}", render_duration_report(&report, &tasks, now));

    let mut generic = report.to_generic();
    if let Some(object) = generic.data.as_object_mut() {
        for (key, value) in duration_report_metadata(&tasks, now) {
            object.insert(key.to_string(), value);
        }
    }
    storage.store(&generic)?;

    Ok(())
}

// ---------------------------------------------------------------------------
// Bottleneck report
// ---------------------------------------------------------------------------

const CATEGORY_THIRD_PARTY: &str = "waiting on a third party";
const CATEGORY_PERSON: &str = "waiting on a person";
const CATEGORY_DEFERRED: &str = "deferred by the owner";
const CATEGORY_UNKNOWN: &str = "unknown";

/// Normalise a verbatim block reason into a stable category.
fn categorise_block_reason(reason: Option<&str>) -> &'static str {
    let Some(reason) = reason else {
        return CATEGORY_UNKNOWN;
    };
    let reason = reason.to_lowercase();

    const DEFERRED: &[&str] = &[
        "deferred",
        "postponed",
        "depriorit",
        "on hold",
        "backlog",
        "not now",
        "intentional",
        "by design",
    ];
    const THIRD_PARTY: &[&str] = &[
        "third party",
        "third-party",
        "external",
        "vendor",
        "upstream",
        "dependency",
        "depend",
        "crate",
        "library",
        "api",
        "service",
        "outage",
        "quota",
        "network",
        "ci ",
        "build",
        "registry",
        "infrastructure",
        "infra",
    ];
    const PERSON: &[&str] = &[
        "review",
        "approv",
        "feedback",
        "response",
        "reply",
        "answer",
        "sign-off",
        "signoff",
        "maintainer",
        "human",
        "person",
        "assignee",
        "owner",
        "decision",
    ];

    if DEFERRED.iter().any(|keyword| reason.contains(keyword)) {
        CATEGORY_DEFERRED
    } else if THIRD_PARTY.iter().any(|keyword| reason.contains(keyword)) {
        CATEGORY_THIRD_PARTY
    } else if PERSON.iter().any(|keyword| reason.contains(keyword)) {
        CATEGORY_PERSON
    } else {
        CATEGORY_UNKNOWN
    }
}

#[derive(Debug, Clone)]
struct BlockedReasonGroup {
    category: &'static str,
    count: usize,
    waiting_hours_total: f64,
    reasons: Vec<String>,
}

fn group_blocked_by_reason(blocked: &[BottleneckEntry]) -> Vec<BlockedReasonGroup> {
    let mut groups: BTreeMap<&'static str, BlockedReasonGroup> = BTreeMap::new();
    for entry in blocked {
        let category = categorise_block_reason(entry.block_reason.as_deref());
        let group = groups.entry(category).or_insert_with(|| BlockedReasonGroup {
            category,
            count: 0,
            waiting_hours_total: 0.0,
            reasons: Vec::new(),
        });
        group.count += 1;
        group.waiting_hours_total += entry.duration_hours;
        group
            .reasons
            .push(entry.block_reason.clone().unwrap_or_else(|| "(no reason recorded)".to_string()));
    }
    groups.into_values().collect()
}

fn render_bottleneck_report(
    report: &BottleneckReport,
    cancelled: &[BottleneckEntry],
    groups: &[BlockedReasonGroup],
    counts: &BTreeMap<String, usize>,
) -> String {
    let mut out = String::new();
    out.push_str("Bottleneck Report\n");
    out.push_str("=================\n");
    out.push_str(&format!("  Computed: {}\n", report.computed_at.format("%Y-%m-%d %H:%M UTC")));
    let counts_text: Vec<String> =
        counts.iter().map(|(status, count)| format!("{}={}", status, count)).collect();
    out.push_str(&format!(
        "  Total tasks: {} (statuses: {})\n",
        report.total_analyzed,
        counts_text.join(", ")
    ));
    out.push_str(&format!("  Blocked:     {}\n", report.blocked_count));
    out.push_str(&format!("  Closed without completion (cancelled): {}\n", cancelled.len()));
    out.push('\n');

    if report.slowest_tasks.is_empty() {
        out.push_str("  No open or completed tasks found.\n");
    } else {
        out.push_str(&format!(
            "  Slowest Tasks (top {}, cancelled excluded — elapsed since creation):\n",
            report.slowest_tasks.len()
        ));
        let mut table = create_table();
        table.set_titles(row!["ID", "Status", "Elapsed (h)", "Title", "Agent"]);

        for entry in &report.slowest_tasks {
            table.add_row(row![
                &entry.task_id[..entry.task_id.len().min(8)],
                &entry.status,
                format!("{:.2}", entry.duration_hours),
                truncate(&entry.title, 40),
                truncate(&entry.agent, 10),
            ]);
        }
        out.push_str(&table.to_string());
    }

    if !report.blocked_tasks.is_empty() {
        out.push('\n');
        out.push_str(&format!(
            "  Currently Blocked (grouped by reason; waiting = elapsed since creation): {}\n",
            report.blocked_tasks.len()
        ));
        let category_text: Vec<String> =
            groups.iter().map(|group| format!("{}={}", group.category, group.count)).collect();
        out.push_str(&format!("  By category: {}\n", category_text.join(", ")));

        let mut ordered: Vec<(&'static str, &BottleneckEntry)> = report
            .blocked_tasks
            .iter()
            .map(|entry| (categorise_block_reason(entry.block_reason.as_deref()), entry))
            .collect();
        ordered.sort_by(|a, b| {
            a.0.cmp(b.0).then_with(|| {
                b.1.duration_hours
                    .partial_cmp(&a.1.duration_hours)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
        });

        let mut table = create_table();
        table.set_titles(row!["ID", "Category", "Waiting (h)", "Reason (verbatim)", "Title"]);
        for (category, entry) in ordered {
            table.add_row(row![
                &entry.task_id[..entry.task_id.len().min(8)],
                category,
                format!("{:.2}", entry.duration_hours),
                entry.block_reason.as_deref().unwrap_or("—"),
                truncate(&entry.title, 40),
            ]);
        }
        out.push_str(&table.to_string());
    }

    if !cancelled.is_empty() {
        out.push('\n');
        out.push_str("  Closed Without Completion (cancelled — not bottlenecks):\n");
        let mut table = create_table();
        table.set_titles(row!["ID", "Elapsed (h)", "Title", "Agent"]);
        for entry in cancelled {
            table.add_row(row![
                &entry.task_id[..entry.task_id.len().min(8)],
                format!("{:.2}", entry.duration_hours),
                truncate(&entry.title, 40),
                truncate(&entry.agent, 10),
            ]);
        }
        out.push_str(&table.to_string());
    }

    out.push('\n');
    out.push_str(&format!("  Report ID: {}\n", report.id));
    out
}

fn bottleneck_metadata(
    cancelled: &[BottleneckEntry],
    groups: &[BlockedReasonGroup],
    counts: &BTreeMap<String, usize>,
) -> Vec<(&'static str, Value)> {
    let cancelled_json = Value::Array(
        cancelled
            .iter()
            .map(|entry| {
                json!({
                    "task_id": entry.task_id,
                    "title": entry.title,
                    "status": entry.status,
                    "elapsed_hours": entry.duration_hours,
                    "block_reason": entry.block_reason,
                })
            })
            .collect(),
    );
    let groups_json = Value::Array(
        groups
            .iter()
            .map(|group| {
                json!({
                    "category": group.category,
                    "count": group.count,
                    "waiting_hours_total": group.waiting_hours_total,
                    "reasons": group.reasons,
                })
            })
            .collect(),
    );

    vec![
        ("slowest_excludes_cancelled", json!(true)),
        ("cancelled_count", json!(cancelled.len())),
        ("cancelled_tasks", cancelled_json),
        ("blocked_reason_groups", groups_json),
        ("status_counts", status_counts_json(counts)),
    ]
}

fn run_bottleneck<S: Storage>(storage: &mut S, top: usize) -> Result<(), EngramError> {
    let repo_path = std::path::Path::new(".");
    let agent = "default";

    // Take every entry so cancelling a task does not consume one of the
    // requested "slowest" slots.
    let mut report = BottleneckReport::compute(storage, repo_path, agent, usize::MAX)?;

    let mut all_entries = std::mem::take(&mut report.slowest_tasks);
    let counts = {
        let mut counts = BTreeMap::new();
        for entry in &all_entries {
            *counts.entry(entry.status.clone()).or_insert(0) += 1;
        }
        counts
    };
    let cancelled: Vec<BottleneckEntry> =
        all_entries.iter().filter(|entry| entry.status == "cancelled").cloned().collect();
    all_entries.retain(|entry| entry.status != "cancelled");
    all_entries.truncate(top);
    report.slowest_tasks = all_entries;

    let groups = group_blocked_by_reason(&report.blocked_tasks);

    print!("{}", render_bottleneck_report(&report, &cancelled, &groups, &counts));

    let mut generic = report.to_generic();
    if let Some(object) = generic.data.as_object_mut() {
        for (key, value) in bottleneck_metadata(&cancelled, &groups, &counts) {
            object.insert(key.to_string(), value);
        }
    }
    storage.store(&generic)?;

    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{Task, TaskPriority, TaskStatus};
    use crate::storage::MemoryStorage;
    use chrono::{Duration, Utc};

    fn make_storage() -> MemoryStorage {
        MemoryStorage::new("default")
    }

    fn make_task(
        id: &str,
        title: &str,
        status: TaskStatus,
        start: chrono::DateTime<Utc>,
        end: Option<chrono::DateTime<Utc>>,
        block_reason: Option<String>,
    ) -> Task {
        Task {
            id: id.to_string(),
            title: title.to_string(),
            description: "test".to_string(),
            status,
            priority: TaskPriority::Medium,
            agent: "default".to_string(),
            start_time: start,
            end_time: end,
            parent: None,
            children: Vec::new(),
            tags: Vec::new(),
            context_ids: Vec::new(),
            knowledge: Vec::new(),
            files: Vec::new(),
            outcome: None,
            block_reason,
            workflow_id: None,
            workflow_state: None,
            metadata: std::collections::HashMap::new(),
        }
    }

    #[test]
    fn test_dora_rating_functions() {
        assert_eq!(dora_rating_deployment_freq(2.0), "Elite");
        assert_eq!(dora_rating_deployment_freq(0.7), "High");
        assert_eq!(dora_rating_deployment_freq(0.0), "Low");

        assert_eq!(dora_rating_lead_time(0.5), "Elite");
        assert_eq!(dora_rating_lead_time(5.0), "High");
        assert_eq!(dora_rating_lead_time(15.0), "Medium");

        assert_eq!(dora_rating_cfr(0.02), "Elite");
        assert_eq!(dora_rating_cfr(0.5), "Low");

        assert_eq!(dora_rating_mttr(0.5), "Elite");
        assert_eq!(dora_rating_mttr(200.0), "Low");
    }

    #[test]
    fn test_dora_rating_deployment_freq_all_thresholds() {
        assert_eq!(dora_rating_deployment_freq(1.0), "Elite");
        assert_eq!(dora_rating_deployment_freq(0.99), "High");
        assert_eq!(dora_rating_deployment_freq(0.5), "High");
        assert_eq!(dora_rating_deployment_freq(0.49), "Medium");
        assert_eq!(dora_rating_deployment_freq(0.1), "Medium");
        assert_eq!(dora_rating_deployment_freq(0.09), "Low");
        assert_eq!(dora_rating_deployment_freq(0.0), "Low");
        assert_eq!(dora_rating_deployment_freq(100.0), "Elite");
    }

    #[test]
    fn test_dora_rating_lead_time_all_thresholds() {
        assert_eq!(dora_rating_lead_time(1.0), "Elite");
        assert_eq!(dora_rating_lead_time(0.0), "Elite");
        assert_eq!(dora_rating_lead_time(1.01), "High");
        assert_eq!(dora_rating_lead_time(7.0), "High");
        assert_eq!(dora_rating_lead_time(7.01), "Medium");
        assert_eq!(dora_rating_lead_time(30.0), "Medium");
        assert_eq!(dora_rating_lead_time(30.01), "Low");
        assert_eq!(dora_rating_lead_time(365.0), "Low");
    }

    #[test]
    fn test_dora_rating_cfr_all_thresholds() {
        assert_eq!(dora_rating_cfr(0.0), "Elite");
        assert_eq!(dora_rating_cfr(0.05), "Elite");
        assert_eq!(dora_rating_cfr(0.051), "High");
        assert_eq!(dora_rating_cfr(0.10), "High");
        assert_eq!(dora_rating_cfr(0.101), "Medium");
        assert_eq!(dora_rating_cfr(0.15), "Medium");
        assert_eq!(dora_rating_cfr(0.151), "Low");
        assert_eq!(dora_rating_cfr(1.0), "Low");
    }

    #[test]
    fn test_dora_rating_mttr_all_thresholds() {
        assert_eq!(dora_rating_mttr(0.0), "Elite");
        assert_eq!(dora_rating_mttr(1.0), "Elite");
        assert_eq!(dora_rating_mttr(1.01), "High");
        assert_eq!(dora_rating_mttr(24.0), "High");
        assert_eq!(dora_rating_mttr(24.01), "Medium");
        assert_eq!(dora_rating_mttr(168.0), "Medium");
        assert_eq!(dora_rating_mttr(168.01), "Low");
        assert_eq!(dora_rating_mttr(1000.0), "Low");
    }

    #[test]
    fn test_dora_rating_functions_return_static_str() {
        let s = dora_rating_deployment_freq(1.0);
        let _owned: &'static str = s;

        let s = dora_rating_lead_time(1.0);
        let _owned: &'static str = s;

        let s = dora_rating_cfr(0.0);
        let _owned: &'static str = s;

        let s = dora_rating_mttr(0.0);
        let _owned: &'static str = s;
    }

    #[test]
    fn test_run_dora_empty_storage() {
        let mut storage = make_storage();
        let result = run_dora(&mut storage, 30);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_dora_with_window_days_7() {
        let mut storage = make_storage();
        let result = run_dora(&mut storage, 7);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_dora_with_window_days_1() {
        let mut storage = make_storage();
        let result = run_dora(&mut storage, 1);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_dora_with_window_days_365() {
        let mut storage = make_storage();
        let result = run_dora(&mut storage, 365);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_dora_stores_report() {
        let mut storage = make_storage();
        run_dora(&mut storage, 30).unwrap();
        let reports = storage.get_all("dora_metrics_report").unwrap();
        assert_eq!(reports.len(), 1);
    }

    #[test]
    fn test_run_dora_multiple_runs_store_multiple_reports() {
        let mut storage = make_storage();
        run_dora(&mut storage, 30).unwrap();
        run_dora(&mut storage, 30).unwrap();
        run_dora(&mut storage, 30).unwrap();
        let reports = storage.get_all("dora_metrics_report").unwrap();
        assert_eq!(reports.len(), 3);
    }

    #[test]
    fn test_run_dora_report_fields_valid() {
        let mut storage = make_storage();
        run_dora(&mut storage, 30).unwrap();
        let reports = storage.get_all("dora_metrics_report").unwrap();
        let report = &reports[0];
        assert!(report.data.get("id").is_some());
        assert!(report.data.get("project_path").is_some());
        assert!(report.data.get("deployment_frequency").is_some());
        assert!(report.data.get("lead_time_for_changes").is_some());
        assert!(report.data.get("change_failure_rate").is_some());
        assert!(report.data.get("mean_time_to_recovery").is_some());
        assert!(report.data.get("commits_analyzed").is_some());
        assert!(report.data.get("window_start").is_some());
        assert!(report.data.get("window_end").is_some());
    }

    #[test]
    fn test_run_duration_report_empty() {
        let mut storage = make_storage();
        let result = run_duration_report(&mut storage);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_duration_report_stores_report() {
        let mut storage = make_storage();
        run_duration_report(&mut storage).unwrap();
        let reports = storage.get_all("task_duration_report").unwrap();
        assert_eq!(reports.len(), 1);
    }

    #[test]
    fn test_run_duration_report_empty_has_no_tasks() {
        let mut storage = make_storage();
        run_duration_report(&mut storage).unwrap();
        let reports = storage.get_all("task_duration_report").unwrap();
        let data = &reports[0].data;
        assert_eq!(data["total_tasks_analyzed"], 0);
        assert_eq!(data["completed_tasks"], 0);
        // task_durations is skip_serialized when empty, so it may be Null or []
        let durations = &data["task_durations"];
        assert!(
            durations.is_null() || durations == &serde_json::Value::Array(vec![]),
            "expected null or empty array, got: {:?}",
            durations
        );
    }

    #[test]
    fn test_run_duration_report_with_done_tasks() {
        let mut storage = make_storage();
        let now = Utc::now();
        let t1 = make_task(
            "task-1",
            "Done task",
            TaskStatus::Done,
            now - Duration::hours(2),
            Some(now),
            None,
        );
        let t2 = make_task(
            "task-2",
            "Another done",
            TaskStatus::Done,
            now - Duration::hours(4),
            Some(now),
            None,
        );
        storage.store(&t1.to_generic()).unwrap();
        storage.store(&t2.to_generic()).unwrap();

        run_duration_report(&mut storage).unwrap();
        let reports = storage.get_all("task_duration_report").unwrap();
        let data = &reports[0].data;
        assert_eq!(data["total_tasks_analyzed"], 2);
        assert_eq!(data["completed_tasks"], 2);
    }

    #[test]
    fn test_run_duration_report_with_mixed_statuses() {
        let mut storage = make_storage();
        let now = Utc::now();
        let tasks = vec![
            make_task("t1", "Done", TaskStatus::Done, now - Duration::hours(2), Some(now), None),
            make_task("t2", "Todo", TaskStatus::Todo, now - Duration::hours(1), None, None),
            make_task(
                "t3",
                "In progress",
                TaskStatus::InProgress,
                now - Duration::hours(3),
                None,
                None,
            ),
            make_task("t4", "Done2", TaskStatus::Done, now - Duration::hours(5), Some(now), None),
        ];
        for t in &tasks {
            storage.store(&t.to_generic()).unwrap();
        }

        run_duration_report(&mut storage).unwrap();
        let reports = storage.get_all("task_duration_report").unwrap();
        let data = &reports[0].data;
        assert_eq!(data["total_tasks_analyzed"], 4);
        assert_eq!(data["completed_tasks"], 2);
        let durations = data["task_durations"].as_array().unwrap();
        assert_eq!(durations.len(), 4);
    }

    #[test]
    fn test_run_duration_report_statistics() {
        let mut storage = make_storage();
        let now = Utc::now();
        let t1 =
            make_task("t1", "Short", TaskStatus::Done, now - Duration::hours(1), Some(now), None);
        let t2 =
            make_task("t2", "Long", TaskStatus::Done, now - Duration::hours(10), Some(now), None);
        let t3 =
            make_task("t3", "Medium", TaskStatus::Done, now - Duration::hours(5), Some(now), None);
        storage.store(&t1.to_generic()).unwrap();
        storage.store(&t2.to_generic()).unwrap();
        storage.store(&t3.to_generic()).unwrap();

        run_duration_report(&mut storage).unwrap();
        let reports = storage.get_all("task_duration_report").unwrap();
        let data = &reports[0].data;
        assert_eq!(data["completed_tasks"], 3);
        assert!(data["median_duration_hours"].as_f64().unwrap() > 0.0);
        assert!(data["mean_duration_hours"].as_f64().unwrap() > 0.0);
        assert!(
            data["min_duration_hours"].as_f64().unwrap()
                <= data["max_duration_hours"].as_f64().unwrap()
        );
    }

    #[test]
    fn test_run_duration_report_sorted_by_duration_desc() {
        let mut storage = make_storage();
        let now = Utc::now();
        let t1 =
            make_task("t1", "Short", TaskStatus::Done, now - Duration::hours(1), Some(now), None);
        let t2 =
            make_task("t2", "Long", TaskStatus::Done, now - Duration::hours(10), Some(now), None);
        storage.store(&t1.to_generic()).unwrap();
        storage.store(&t2.to_generic()).unwrap();

        run_duration_report(&mut storage).unwrap();
        let reports = storage.get_all("task_duration_report").unwrap();
        let data = &reports[0].data;
        let durations = data["task_durations"].as_array().unwrap();
        let first = durations[0]["duration_hours"].as_f64().unwrap();
        let second = durations[1]["duration_hours"].as_f64().unwrap();
        assert!(first >= second);
    }

    #[test]
    fn test_run_bottleneck_empty() {
        let mut storage = make_storage();
        let result = run_bottleneck(&mut storage, 5);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_bottleneck_stores_report() {
        let mut storage = make_storage();
        run_bottleneck(&mut storage, 5).unwrap();
        let reports = storage.get_all("bottleneck_report").unwrap();
        assert_eq!(reports.len(), 1);
    }

    #[test]
    fn test_run_bottleneck_empty_report_fields() {
        let mut storage = make_storage();
        run_bottleneck(&mut storage, 10).unwrap();
        let reports = storage.get_all("bottleneck_report").unwrap();
        let data = &reports[0].data;
        assert_eq!(data["total_analyzed"], 0);
        assert_eq!(data["blocked_count"], 0);
        // slowest_tasks is skip_serialized when empty, so it may be Null or []
        let slowest = &data["slowest_tasks"];
        assert!(
            slowest.is_null() || slowest == &serde_json::Value::Array(vec![]),
            "expected null or empty array, got: {:?}",
            slowest
        );
    }

    #[test]
    fn test_run_bottleneck_with_tasks() {
        let mut storage = make_storage();
        let now = Utc::now();
        let t1 = make_task(
            "t1",
            "Slow task",
            TaskStatus::InProgress,
            now - Duration::hours(20),
            None,
            None,
        );
        let t2 = make_task(
            "t2",
            "Faster task",
            TaskStatus::Done,
            now - Duration::hours(2),
            Some(now),
            None,
        );
        storage.store(&t1.to_generic()).unwrap();
        storage.store(&t2.to_generic()).unwrap();

        run_bottleneck(&mut storage, 10).unwrap();
        let reports = storage.get_all("bottleneck_report").unwrap();
        let data = &reports[0].data;
        assert_eq!(data["total_analyzed"], 2);
        let slowest = data["slowest_tasks"].as_array().unwrap();
        assert_eq!(slowest.len(), 2);
        assert!(
            slowest[0]["duration_hours"].as_f64().unwrap()
                >= slowest[1]["duration_hours"].as_f64().unwrap()
        );
    }

    #[test]
    fn test_run_bottleneck_top_n_limits_results() {
        let mut storage = make_storage();
        let now = Utc::now();
        for i in 0..5 {
            let t = make_task(
                &format!("t{}", i),
                &format!("Task {}", i),
                TaskStatus::Done,
                now - Duration::hours(i as i64 + 1),
                Some(now),
                None,
            );
            storage.store(&t.to_generic()).unwrap();
        }

        run_bottleneck(&mut storage, 2).unwrap();
        let reports = storage.get_all("bottleneck_report").unwrap();
        let data = &reports[0].data;
        assert_eq!(data["total_analyzed"], 5);
        let slowest = data["slowest_tasks"].as_array().unwrap();
        assert_eq!(slowest.len(), 2);
    }

    #[test]
    fn test_run_bottleneck_blocked_tasks() {
        let mut storage = make_storage();
        let now = Utc::now();
        let t1 = make_task(
            "t1",
            "Blocked",
            TaskStatus::Blocked,
            now - Duration::hours(48),
            None,
            Some("Waiting for review".to_string()),
        );
        let t2 = make_task(
            "t2",
            "Also blocked",
            TaskStatus::Blocked,
            now - Duration::hours(12),
            None,
            Some("Missing dependency".to_string()),
        );
        let t3 = make_task(
            "t3",
            "Not blocked",
            TaskStatus::Done,
            now - Duration::hours(1),
            Some(now),
            None,
        );
        storage.store(&t1.to_generic()).unwrap();
        storage.store(&t2.to_generic()).unwrap();
        storage.store(&t3.to_generic()).unwrap();

        run_bottleneck(&mut storage, 10).unwrap();
        let reports = storage.get_all("bottleneck_report").unwrap();
        let data = &reports[0].data;
        assert_eq!(data["blocked_count"], 2);
        assert_eq!(data["total_analyzed"], 3);
        let blocked = data["blocked_tasks"].as_array().unwrap();
        assert_eq!(blocked.len(), 2);
    }

    #[test]
    fn test_run_bottleneck_blocked_tasks_sorted_by_duration() {
        let mut storage = make_storage();
        let now = Utc::now();
        let t1 = make_task(
            "t1",
            "Blocked short",
            TaskStatus::Blocked,
            now - Duration::hours(5),
            None,
            Some("reason1".to_string()),
        );
        let t2 = make_task(
            "t2",
            "Blocked long",
            TaskStatus::Blocked,
            now - Duration::hours(50),
            None,
            Some("reason2".to_string()),
        );
        storage.store(&t1.to_generic()).unwrap();
        storage.store(&t2.to_generic()).unwrap();

        run_bottleneck(&mut storage, 10).unwrap();
        let reports = storage.get_all("bottleneck_report").unwrap();
        let data = &reports[0].data;
        let blocked = data["blocked_tasks"].as_array().unwrap();
        assert!(
            blocked[0]["duration_hours"].as_f64().unwrap()
                >= blocked[1]["duration_hours"].as_f64().unwrap()
        );
    }

    #[test]
    fn test_run_bottleneck_top_zero() {
        let mut storage = make_storage();
        let now = Utc::now();
        let t =
            make_task("t1", "Task", TaskStatus::Done, now - Duration::hours(1), Some(now), None);
        storage.store(&t.to_generic()).unwrap();

        run_bottleneck(&mut storage, 0).unwrap();
        let reports = storage.get_all("bottleneck_report").unwrap();
        let data = &reports[0].data;
        assert_eq!(data["total_analyzed"], 1);
        // slowest_tasks is skip_serialized when empty; may be Null or []
        let slowest_val = &data["slowest_tasks"];
        let slowest_len = slowest_val.as_array().map(|a| a.len()).unwrap_or(0);
        assert_eq!(slowest_len, 0);
    }

    #[test]
    fn test_run_bottleneck_top_larger_than_tasks() {
        let mut storage = make_storage();
        let now = Utc::now();
        let t =
            make_task("t1", "Only", TaskStatus::Done, now - Duration::hours(1), Some(now), None);
        storage.store(&t.to_generic()).unwrap();

        run_bottleneck(&mut storage, 100).unwrap();
        let reports = storage.get_all("bottleneck_report").unwrap();
        let data = &reports[0].data;
        let slowest = data["slowest_tasks"].as_array().unwrap();
        assert_eq!(slowest.len(), 1);
    }

    #[test]
    fn test_handle_analytics_command_dora() {
        let mut storage = make_storage();
        let result =
            handle_analytics_command(&mut storage, AnalyticsCommands::Dora { window_days: 30 });
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_analytics_command_report() {
        let mut storage = make_storage();
        let result = handle_analytics_command(&mut storage, AnalyticsCommands::Report {});
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_analytics_command_bottleneck() {
        let mut storage = make_storage();
        let result =
            handle_analytics_command(&mut storage, AnalyticsCommands::Bottleneck { top: 5 });
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_analytics_command_all_commands_produce_reports() {
        let mut storage = make_storage();
        handle_analytics_command(&mut storage, AnalyticsCommands::Dora { window_days: 30 })
            .unwrap();
        handle_analytics_command(&mut storage, AnalyticsCommands::Report {}).unwrap();
        handle_analytics_command(&mut storage, AnalyticsCommands::Bottleneck { top: 5 }).unwrap();

        assert_eq!(storage.get_all("dora_metrics_report").unwrap().len(), 1);
        assert_eq!(storage.get_all("task_duration_report").unwrap().len(), 1);
        assert_eq!(storage.get_all("bottleneck_report").unwrap().len(), 1);
    }

    #[test]
    fn test_run_duration_report_with_cancelled_task() {
        let mut storage = make_storage();
        let now = Utc::now();
        let t = make_task(
            "t1",
            "Cancelled",
            TaskStatus::Cancelled,
            now - Duration::hours(3),
            Some(now),
            None,
        );
        storage.store(&t.to_generic()).unwrap();

        run_duration_report(&mut storage).unwrap();
        let reports = storage.get_all("task_duration_report").unwrap();
        let data = &reports[0].data;
        assert_eq!(data["total_tasks_analyzed"], 1);
        assert_eq!(data["completed_tasks"], 0);
    }

    #[test]
    fn test_run_bottleneck_with_blocked_and_done_mixed() {
        let mut storage = make_storage();
        let now = Utc::now();
        let tasks = vec![
            make_task("t1", "Done", TaskStatus::Done, now - Duration::hours(100), Some(now), None),
            make_task(
                "t2",
                "Blocked",
                TaskStatus::Blocked,
                now - Duration::hours(50),
                None,
                Some("dep".to_string()),
            ),
            make_task("t3", "Todo", TaskStatus::Todo, now - Duration::hours(1), None, None),
            make_task("t4", "Done2", TaskStatus::Done, now - Duration::hours(5), Some(now), None),
        ];
        for t in &tasks {
            storage.store(&t.to_generic()).unwrap();
        }

        run_bottleneck(&mut storage, 2).unwrap();
        let reports = storage.get_all("bottleneck_report").unwrap();
        let data = &reports[0].data;
        assert_eq!(data["total_analyzed"], 4);
        assert_eq!(data["blocked_count"], 1);
        let slowest = data["slowest_tasks"].as_array().unwrap();
        assert_eq!(slowest.len(), 2);
    }

    #[test]
    fn test_run_dora_negative_metrics_are_zero() {
        let mut storage = make_storage();
        run_dora(&mut storage, 30).unwrap();
        let reports = storage.get_all("dora_metrics_report").unwrap();
        let data = &reports[0].data;
        let dep_freq = data["deployment_frequency"].as_f64().unwrap();
        let lt = data["lead_time_for_changes"].as_f64().unwrap();
        let cfr = data["change_failure_rate"].as_f64().unwrap();
        let mttr = data["mean_time_to_recovery"].as_f64().unwrap();
        assert!(dep_freq >= 0.0);
        assert!(lt >= 0.0);
        assert!(cfr >= 0.0 && cfr <= 1.0);
        assert!(mttr >= 0.0);
    }

    #[test]
    fn test_run_duration_report_incomplete_task_has_duration() {
        let mut storage = make_storage();
        let now = Utc::now();
        let t = make_task(
            "t1",
            "In progress",
            TaskStatus::InProgress,
            now - Duration::hours(3),
            None,
            None,
        );
        storage.store(&t.to_generic()).unwrap();

        run_duration_report(&mut storage).unwrap();
        let reports = storage.get_all("task_duration_report").unwrap();
        let data = &reports[0].data;
        assert_eq!(data["total_tasks_analyzed"], 1);
        let durations = data["task_durations"].as_array().unwrap();
        assert_eq!(durations.len(), 1);
        assert!(durations[0]["duration_hours"].as_f64().unwrap() > 0.0);
    }

    #[test]
    fn an_empty_dora_report_reports_insufficient_data_not_elite() {
        let report = DoraMetricsReport::new(".".to_string(), "default".to_string());
        let rendered = render_dora_report(&report);

        assert_eq!(
            rendered.matches(INSUFFICIENT_DATA).count(),
            4,
            "every metric must be gated on its input count:\n{rendered}"
        );
        assert!(
            !rendered.contains("Elite"),
            "an empty report must not rate any metric Elite:\n{rendered}"
        );
        assert!(rendered.contains("Rating needs data:"));

        // The report path uses the same gate: entity storage starts empty, so
        // the execution and escalation inputs are zero.
        let mut storage = make_storage();
        run_dora(&mut storage, 30).unwrap();
        let reports = storage.get_all("dora_metrics_report").unwrap();
        let data = &reports[0].data;
        assert_eq!(data["executions_analyzed"], 0);
        assert_eq!(data["escalations_analyzed"], 0);
    }

    #[test]
    fn a_zero_value_metric_is_not_rated_elite() {
        for metric in [
            DoraMetric::DeploymentFrequency,
            DoraMetric::LeadTime,
            DoraMetric::ChangeFailureRate,
            DoraMetric::MeanTimeToRecovery,
        ] {
            let rating = dora_rating_for(metric, 0.0, 0);
            assert_eq!(rating, INSUFFICIENT_DATA, "{:?}", metric);
            assert_ne!(rating, "Elite", "{:?}", metric);
        }

        // Non-zero inputs keep their threshold ratings.
        assert_eq!(dora_rating_for(DoraMetric::LeadTime, 2.0, 30), "High");
        assert_eq!(dora_rating_for(DoraMetric::DeploymentFrequency, 2.0, 30), "Elite");
        assert_eq!(dora_rating_for(DoraMetric::ChangeFailureRate, 0.5, 4), "Low");
        assert_eq!(dora_rating_for(DoraMetric::MeanTimeToRecovery, 200.0, 1), "Low");
    }

    #[test]
    fn cancelled_tasks_are_not_counted_as_slowest() {
        let mut storage = make_storage();
        let now = Utc::now();
        let cancelled = make_task(
            "cancelled-1",
            "Abandoned",
            TaskStatus::Cancelled,
            now - Duration::hours(1000),
            Some(now),
            None,
        );
        let done = make_task(
            "done-1",
            "Finished",
            TaskStatus::Done,
            now - Duration::hours(2),
            Some(now),
            None,
        );
        storage.store(&cancelled.to_generic()).unwrap();
        storage.store(&done.to_generic()).unwrap();

        run_bottleneck(&mut storage, 10).unwrap();
        let reports = storage.get_all("bottleneck_report").unwrap();
        let data = &reports[0].data;

        let slowest = data["slowest_tasks"].as_array().unwrap();
        assert!(
            slowest.iter().all(|entry| entry["status"] != "cancelled"),
            "cancelled tasks are not bottlenecks: {:?}",
            slowest
        );
        assert_eq!(slowest.len(), 1);
        assert_eq!(slowest[0]["task_id"], "done-1");

        assert_eq!(data["cancelled_count"], 1);
        let cancelled_tasks = data["cancelled_tasks"].as_array().unwrap();
        assert_eq!(cancelled_tasks.len(), 1);
        assert_eq!(cancelled_tasks[0]["task_id"], "cancelled-1");
    }

    #[test]
    fn blocked_tasks_are_grouped_by_reason_with_the_verbatim_text_kept() {
        let mut storage = make_storage();
        let now = Utc::now();
        let reasons = [
            ("b1", "Waiting for external API access"),
            ("b2", "Waiting for Alice to review"),
            ("b3", "Deferred until next quarter"),
            ("b4", "Needs more thinking"),
        ];
        for (id, reason) in reasons {
            let task = make_task(
                id,
                reason,
                TaskStatus::Blocked,
                now - Duration::hours(5),
                None,
                Some(reason.to_string()),
            );
            storage.store(&task.to_generic()).unwrap();
        }

        run_bottleneck(&mut storage, 10).unwrap();
        let reports = storage.get_all("bottleneck_report").unwrap();
        let data = &reports[0].data;

        let groups = data["blocked_reason_groups"].as_array().unwrap();
        assert_eq!(groups.len(), 4, "one group per category: {:?}", groups);
        let group_for = |category: &str| {
            groups
                .iter()
                .find(|group| group["category"] == category)
                .unwrap_or_else(|| panic!("missing category {category}: {:?}", groups))
                .clone()
        };

        let third_party = group_for("waiting on a third party");
        assert_eq!(third_party["count"], 1);
        assert!(third_party["reasons"]
            .as_array()
            .unwrap()
            .iter()
            .any(|reason| reason == "Waiting for external API access"));
        assert_eq!(group_for("waiting on a person")["count"], 1);
        assert_eq!(group_for("deferred by the owner")["count"], 1);
        assert_eq!(group_for("unknown")["count"], 1);

        let blocked = data["blocked_tasks"].as_array().unwrap();
        for (id, reason) in reasons {
            let entry = blocked
                .iter()
                .find(|entry| entry["task_id"] == id)
                .unwrap_or_else(|| panic!("missing blocked task {id}"));
            assert_eq!(entry["block_reason"], reason);
        }
    }

    #[test]
    fn the_duration_report_reports_p90_and_a_long_lived_count() {
        let mut storage = make_storage();
        let now = Utc::now();
        for hours in 1i64..=10 {
            let task = make_task(
                &format!("t{hours}"),
                "Done",
                TaskStatus::Done,
                now - Duration::hours(hours),
                Some(now),
                None,
            );
            storage.store(&task.to_generic()).unwrap();
        }
        let long_lived = make_task(
            "long",
            "Ancient",
            TaskStatus::Done,
            now - Duration::hours(721),
            Some(now),
            None,
        );
        storage.store(&long_lived.to_generic()).unwrap();

        run_duration_report(&mut storage).unwrap();
        let reports = storage.get_all("task_duration_report").unwrap();
        let data = &reports[0].data;

        let p90 = data["p90_duration_hours"].as_f64().unwrap();
        assert!((10.0..721.0).contains(&p90), "p90 was {p90}");
        assert_eq!(data["long_lived_completed_tasks"], 1);
        assert_eq!(data["long_lived_all_tasks"], 1);
        assert_eq!(data["active_duration_available"], false);
        assert_eq!(data["duration_semantics"], ELAPSED_SEMANTICS);
    }
}
