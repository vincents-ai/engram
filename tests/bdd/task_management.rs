//! BDD tests for task management

use cucumber::{given, then, when, World, cucumber};
use crate::bdd::task_management_steps;
use crate::bdd::EngramWorld;

#[tokio::test]
async fn test_task_management() {
    let runner = cucumber::Cucumber::<EngramWorld>::new()
        .with_cli(args: &[cucumber::cli::App::new()]))
        .steps(task_management_steps());
    
    runner.run_and_exit("./tests/bdd/task_management.feature").await;
}