use crate::cli::utils::{create_table, truncate};
use crate::entities::bottleneck_report::BottleneckReport;
use crate::entities::dora_metrics_report::DoraMetricsCalculator;
use crate::entities::task_duration_report::TaskDurationReport;
use crate::entities::Entity;
use crate::error::EngramError;
use crate::storage::Storage;
use clap::Subcommand;
use prettytable::row;

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

fn run_dora<S: Storage>(storage: &mut S, window_days: i64) -> Result<(), EngramError> {
    let repo_path = std::path::Path::new(".");
    let agent = "default";

    let report = DoraMetricsCalculator::compute(storage, repo_path, agent, window_days)?;

    println!("DORA Metrics Report");
    println!("===================");
    println!("  Project:    {}", report.project_path);
    println!(
        "  Window:     {} to {}",
        report.window_start.format("%Y-%m-%d"),
        report.window_end.format("%Y-%m-%d")
    );
    println!(
        "  Computed:   {}",
        report.computed_at.format("%Y-%m-%d %H:%M UTC")
    );
    println!();

    let mut table = create_table();
    table.set_titles(row!["Metric", "Value", "Rating"]);

    let dep_rating = dora_rating_deployment_freq(report.deployment_frequency);
    let lt_rating = dora_rating_lead_time(report.lead_time_for_changes);
    let cfr_rating = dora_rating_cfr(report.change_failure_rate);
    let mttr_rating = dora_rating_mttr(report.mean_time_to_recovery);

    table.add_row(row![
        "Deployment Frequency",
        format!("{:.1}/week", report.deployment_frequency),
        dep_rating
    ]);
    table.add_row(row![
        "Lead Time for Changes",
        format!("{:.1} days", report.lead_time_for_changes),
        lt_rating
    ]);
    table.add_row(row![
        "Change Failure Rate",
        format!("{:.1}%", report.change_failure_rate * 100.0),
        cfr_rating
    ]);
    table.add_row(row![
        "Mean Time to Recovery",
        format!("{:.1} hours", report.mean_time_to_recovery),
        mttr_rating
    ]);
    table.printstd();

    println!();
    println!("  Commits analyzed:      {}", report.commits_analyzed);
    println!("  Executions analyzed:   {}", report.executions_analyzed);
    println!("  Escalations analyzed:  {}", report.escalations_analyzed);
    println!("  Report ID: {}", report.id);

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

fn run_duration_report<S: Storage>(storage: &mut S) -> Result<(), EngramError> {
    let repo_path = std::path::Path::new(".");
    let agent = "default";

    let report = TaskDurationReport::compute(storage, repo_path, agent)?;

    println!("Task Duration Report");
    println!("====================");
    println!(
        "  Computed: {}",
        report.computed_at.format("%Y-%m-%d %H:%M UTC")
    );
    println!(
        "  Tasks analyzed: {}/{} completed",
        report.completed_tasks, report.total_tasks_analyzed
    );
    println!();

    println!("  Summary Statistics:");
    println!("    Median:  {:.2} hours", report.median_duration_hours);
    println!("    Mean:    {:.2} hours", report.mean_duration_hours);
    println!("    Min:     {:.2} hours", report.min_duration_hours);
    println!("    Max:     {:.2} hours", report.max_duration_hours);
    println!();

    if report.task_durations.is_empty() {
        println!("  No tasks found.");
    } else {
        let display_count = report.task_durations.len().min(20);
        let mut table = create_table();
        table.set_titles(row!["ID", "Status", "Duration (h)", "Title", "Agent"]);

        for entry in &report.task_durations[..display_count] {
            table.add_row(row![
                &entry.task_id[..8],
                &entry.status,
                format!("{:.2}", entry.duration_hours),
                truncate(&entry.title, 40),
                truncate(&entry.agent, 10),
            ]);
        }
        table.printstd();

        if report.task_durations.len() > display_count {
            println!(
                "  (showing {} of {} tasks)",
                display_count,
                report.task_durations.len()
            );
        }
    }

    println!();
    println!("  Report ID: {}", report.id);

    let generic = report.to_generic();
    storage.store(&generic)?;

    Ok(())
}

fn run_bottleneck<S: Storage>(storage: &mut S, top: usize) -> Result<(), EngramError> {
    let repo_path = std::path::Path::new(".");
    let agent = "default";

    let report = BottleneckReport::compute(storage, repo_path, agent, top)?;

    println!("Bottleneck Report");
    println!("=================");
    println!(
        "  Computed: {}",
        report.computed_at.format("%Y-%m-%d %H:%M UTC")
    );
    println!("  Total tasks: {}", report.total_analyzed);
    println!("  Blocked:     {}", report.blocked_count);
    println!();

    if report.slowest_tasks.is_empty() {
        println!("  No tasks found.");
    } else {
        println!("  Slowest Tasks (top {}):", report.slowest_tasks.len());
        let mut table = create_table();
        table.set_titles(row!["ID", "Status", "Duration (h)", "Title", "Agent"]);

        for entry in &report.slowest_tasks {
            table.add_row(row![
                &entry.task_id[..8],
                &entry.status,
                format!("{:.2}", entry.duration_hours),
                truncate(&entry.title, 40),
                truncate(&entry.agent, 10),
            ]);
        }
        table.printstd();
    }

    if !report.blocked_tasks.is_empty() {
        println!();
        println!("  Currently Blocked:");
        let mut table = create_table();
        table.set_titles(row!["ID", "Duration (h)", "Block Reason", "Title"]);

        for entry in &report.blocked_tasks {
            table.add_row(row![
                &entry.task_id[..8],
                format!("{:.2}", entry.duration_hours),
                truncate(entry.block_reason.as_deref().unwrap_or("—"), 30),
                truncate(&entry.title, 40),
            ]);
        }
        table.printstd();
    }

    println!();
    println!("  Report ID: {}", report.id);

    let generic = report.to_generic();
    storage.store(&generic)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::MemoryStorage;

    fn make_storage() -> MemoryStorage {
        MemoryStorage::new("default")
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
    fn test_run_duration_report_empty() {
        let mut storage = make_storage();
        let result = run_duration_report(&mut storage);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_bottleneck_empty() {
        let mut storage = make_storage();
        let result = run_bottleneck(&mut storage, 5);
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
}
