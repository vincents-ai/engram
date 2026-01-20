//! BDD test runner for all Engram features

mod bdd;
use cucumber::World;

#[tokio::main]
async fn main() {
    let cwd = std::env::current_dir().unwrap();
    println!("CWD: {:?}", cwd);

    if !std::path::Path::new("tests/features").exists() {
        panic!("tests/features directory not found");
    }

    // Explicitly reference the steps module to ensure it's linked
    bdd::steps::register();

    bdd::EngramWorld::cucumber()
        .run_and_exit("tests/features")
        .await;
}
