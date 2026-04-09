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
                &entry.task_id[..entry.task_id.len().min(8)],
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
                &entry.task_id[..entry.task_id.len().min(8)],
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
                &entry.task_id[..entry.task_id.len().min(8)],
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
            make_task(
                "t1",
                "Done",
                TaskStatus::Done,
                now - Duration::hours(2),
                Some(now),
                None,
            ),
            make_task(
                "t2",
                "Todo",
                TaskStatus::Todo,
                now - Duration::hours(1),
                None,
                None,
            ),
            make_task(
                "t3",
                "In progress",
                TaskStatus::InProgress,
                now - Duration::hours(3),
                None,
                None,
            ),
            make_task(
                "t4",
                "Done2",
                TaskStatus::Done,
                now - Duration::hours(5),
                Some(now),
                None,
            ),
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
        let t1 = make_task(
            "t1",
            "Short",
            TaskStatus::Done,
            now - Duration::hours(1),
            Some(now),
            None,
        );
        let t2 = make_task(
            "t2",
            "Long",
            TaskStatus::Done,
            now - Duration::hours(10),
            Some(now),
            None,
        );
        let t3 = make_task(
            "t3",
            "Medium",
            TaskStatus::Done,
            now - Duration::hours(5),
            Some(now),
            None,
        );
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
        let t1 = make_task(
            "t1",
            "Short",
            TaskStatus::Done,
            now - Duration::hours(1),
            Some(now),
            None,
        );
        let t2 = make_task(
            "t2",
            "Long",
            TaskStatus::Done,
            now - Duration::hours(10),
            Some(now),
            None,
        );
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
        let t = make_task(
            "t1",
            "Task",
            TaskStatus::Done,
            now - Duration::hours(1),
            Some(now),
            None,
        );
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
        let t = make_task(
            "t1",
            "Only",
            TaskStatus::Done,
            now - Duration::hours(1),
            Some(now),
            None,
        );
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
            make_task(
                "t1",
                "Done",
                TaskStatus::Done,
                now - Duration::hours(100),
                Some(now),
                None,
            ),
            make_task(
                "t2",
                "Blocked",
                TaskStatus::Blocked,
                now - Duration::hours(50),
                None,
                Some("dep".to_string()),
            ),
            make_task(
                "t3",
                "Todo",
                TaskStatus::Todo,
                now - Duration::hours(1),
                None,
                None,
            ),
            make_task(
                "t4",
                "Done2",
                TaskStatus::Done,
                now - Duration::hours(5),
                Some(now),
                None,
            ),
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
}
