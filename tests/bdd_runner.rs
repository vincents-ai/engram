//! BDD test runner for all Engram features

mod bdd;

#[tokio::main]
async fn main() {
    bdd::EngramWorld::cucumber()
        .run_and_exit("rust/tests/features")
        .await;
}
