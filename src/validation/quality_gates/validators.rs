use super::{ExpectedResult, QualityGate};
use std::collections::HashMap;

pub struct BuiltinValidators;

impl BuiltinValidators {
    /// Cargo test runner
    pub fn cargo_test() -> QualityGate {
        QualityGate::new("cargo-test".to_string(), "cargo test".to_string()).with_timeout(600)
    }

    /// Cargo test runner expecting failure (BDD RED phase)
    pub fn cargo_test_red_phase() -> QualityGate {
        QualityGate::new("cargo-test-red".to_string(), "cargo test".to_string())
            .with_expected_result(ExpectedResult::Failure)
            .with_timeout(600)
            .with_failure_message(
                "Tests should fail in BDD RED phase - this proves they're testing something real"
                    .to_string(),
            )
    }

    /// Cargo clippy linter
    pub fn cargo_clippy() -> QualityGate {
        QualityGate::new(
            "cargo-clippy".to_string(),
            "cargo clippy -- -D warnings".to_string(),
        )
        .with_timeout(300)
    }

    /// Cargo clippy linter (optional)
    pub fn cargo_clippy_optional() -> QualityGate {
        Self::cargo_clippy().optional()
    }

    /// Nix build
    pub fn nix_build() -> QualityGate {
        QualityGate::new("nix-build".to_string(), "nix build".to_string()).with_timeout(1200)
    }

    /// Nix checks
    pub fn nix_checks() -> QualityGate {
        QualityGate::new("nix-checks".to_string(), "nix flake check".to_string()).with_timeout(900)
    }

    /// Custom engram validation
    pub fn engram_validate(validation_type: &str) -> QualityGate {
        QualityGate::new(
            format!("engram-validate-{}", validation_type),
            format!("engram validate {}", validation_type),
        )
        .with_timeout(120)
    }

    /// Git status check (no uncommitted changes)
    pub fn git_status_clean() -> QualityGate {
        QualityGate::new(
            "git-status-clean".to_string(),
            "sh -c 'git diff --quiet && git diff --cached --quiet'".to_string(),
        )
        .with_timeout(30)
    }

    /// Full test suite with all features
    pub fn full_test_suite() -> QualityGate {
        QualityGate::new(
            "full-test-suite".to_string(),
            "cargo test --all-features".to_string(),
        )
        .with_timeout(1200)
    }

    /// Create quality gates for a specific workflow stage
    pub fn for_stage(stage: &str) -> Vec<QualityGate> {
        match stage {
            "requirements" => vec![Self::engram_validate("requirements-complete")],
            "planning" => vec![Self::engram_validate("design-documented")],
            "research" => vec![Self::engram_validate("research-documented")],
            "bdd" => vec![Self::cargo_test_red_phase()],
            "development" => vec![Self::cargo_test(), Self::cargo_clippy_optional()],
            "integration" => vec![
                Self::nix_build(),
                Self::full_test_suite(),
                Self::git_status_clean().optional(),
            ],
            _ => vec![],
        }
    }

    /// Create development environment setup gates
    pub fn development_setup() -> Vec<QualityGate> {
        vec![
            QualityGate::new("rust-version".to_string(), "rustc --version".to_string())
                .with_timeout(30),
            QualityGate::new("cargo-version".to_string(), "cargo --version".to_string())
                .with_timeout(30),
            QualityGate::new("nix-version".to_string(), "nix --version".to_string())
                .with_timeout(30)
                .optional(),
        ]
    }

    /// Create custom gate with environment variables
    pub fn with_env(name: String, command: String, env: HashMap<String, String>) -> QualityGate {
        QualityGate::new(name, command).with_environment(env)
    }

    /// Create performance benchmark gate
    pub fn performance_benchmark(benchmark_name: &str) -> QualityGate {
        QualityGate::new(
            format!("benchmark-{}", benchmark_name),
            format!("cargo bench --bench {}", benchmark_name),
        )
        .with_timeout(600)
        .optional()
    }

    /// Create security audit gate
    pub fn security_audit() -> QualityGate {
        QualityGate::new("security-audit".to_string(), "cargo audit".to_string()).with_timeout(180)
    }

    /// Create documentation generation gate
    pub fn docs_generation() -> QualityGate {
        QualityGate::new(
            "docs-generation".to_string(),
            "cargo doc --no-deps".to_string(),
        )
        .with_timeout(300)
        .optional()
    }

    /// Create format check gate
    pub fn format_check() -> QualityGate {
        QualityGate::new(
            "format-check".to_string(),
            "cargo fmt -- --check".to_string(),
        )
        .with_timeout(60)
    }
}
