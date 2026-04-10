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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_for_stage_requirements() {
        let gates = BuiltinValidators::for_stage("requirements");
        assert_eq!(gates.len(), 1);
        assert_eq!(gates[0].name, "engram-validate-requirements-complete");
    }

    #[test]
    fn test_for_stage_planning() {
        let gates = BuiltinValidators::for_stage("planning");
        assert_eq!(gates.len(), 1);
        assert_eq!(gates[0].name, "engram-validate-design-documented");
    }

    #[test]
    fn test_for_stage_research() {
        let gates = BuiltinValidators::for_stage("research");
        assert_eq!(gates.len(), 1);
        assert_eq!(gates[0].name, "engram-validate-research-documented");
    }

    #[test]
    fn test_for_stage_bdd() {
        let gates = BuiltinValidators::for_stage("bdd");
        assert_eq!(gates.len(), 1);
        assert_eq!(gates[0].name, "cargo-test-red");
        assert_eq!(gates[0].expected_result, ExpectedResult::Failure);
        assert!(gates[0].failure_message.is_some());
    }

    #[test]
    fn test_for_stage_development() {
        let gates = BuiltinValidators::for_stage("development");
        assert_eq!(gates.len(), 2);
        assert_eq!(gates[0].name, "cargo-test");
        assert!(gates[0].required);
        assert_eq!(gates[1].name, "cargo-clippy");
        assert!(!gates[1].required);
    }

    #[test]
    fn test_for_stage_integration() {
        let gates = BuiltinValidators::for_stage("integration");
        assert_eq!(gates.len(), 3);
        assert_eq!(gates[0].name, "nix-build");
        assert_eq!(gates[1].name, "full-test-suite");
        assert_eq!(gates[2].name, "git-status-clean");
        assert!(!gates[2].required);
    }

    #[test]
    fn test_for_stage_unknown_returns_empty() {
        let gates = BuiltinValidators::for_stage("nonexistent");
        assert!(gates.is_empty());
    }

    #[test]
    fn test_cargo_test() {
        let gate = BuiltinValidators::cargo_test();
        assert_eq!(gate.name, "cargo-test");
        assert_eq!(gate.command, "cargo test");
        assert_eq!(gate.timeout_seconds, Some(600));
        assert!(gate.required);
    }

    #[test]
    fn test_cargo_clippy() {
        let gate = BuiltinValidators::cargo_clippy();
        assert_eq!(gate.name, "cargo-clippy");
        assert!(gate.command.contains("clippy"));
        assert_eq!(gate.timeout_seconds, Some(300));
        assert!(gate.required);
    }

    #[test]
    fn test_cargo_clippy_optional() {
        let gate = BuiltinValidators::cargo_clippy_optional();
        assert!(!gate.required);
        assert_eq!(gate.name, "cargo-clippy");
    }

    #[test]
    fn test_nix_build() {
        let gate = BuiltinValidators::nix_build();
        assert_eq!(gate.name, "nix-build");
        assert_eq!(gate.command, "nix build");
        assert_eq!(gate.timeout_seconds, Some(1200));
    }

    #[test]
    fn test_nix_checks() {
        let gate = BuiltinValidators::nix_checks();
        assert_eq!(gate.name, "nix-checks");
        assert_eq!(gate.command, "nix flake check");
        assert_eq!(gate.timeout_seconds, Some(900));
    }

    #[test]
    fn test_engram_validate() {
        let gate = BuiltinValidators::engram_validate("requirements-complete");
        assert_eq!(gate.name, "engram-validate-requirements-complete");
        assert!(gate
            .command
            .contains("engram validate requirements-complete"));
        assert_eq!(gate.timeout_seconds, Some(120));
    }

    #[test]
    fn test_git_status_clean() {
        let gate = BuiltinValidators::git_status_clean();
        assert_eq!(gate.name, "git-status-clean");
        assert!(gate.command.contains("git diff"));
        assert_eq!(gate.timeout_seconds, Some(30));
    }

    #[test]
    fn test_full_test_suite() {
        let gate = BuiltinValidators::full_test_suite();
        assert_eq!(gate.name, "full-test-suite");
        assert!(gate.command.contains("--all-features"));
        assert_eq!(gate.timeout_seconds, Some(1200));
    }

    #[test]
    fn test_development_setup() {
        let gates = BuiltinValidators::development_setup();
        assert_eq!(gates.len(), 3);
        assert_eq!(gates[0].name, "rust-version");
        assert_eq!(gates[1].name, "cargo-version");
        assert_eq!(gates[2].name, "nix-version");
        assert!(gates[0].required);
        assert!(gates[1].required);
        assert!(!gates[2].required);
    }

    #[test]
    fn test_with_env() {
        let mut env = HashMap::new();
        env.insert("KEY".to_string(), "value".to_string());
        let gate =
            BuiltinValidators::with_env("env-test".to_string(), "echo $KEY".to_string(), env);
        assert_eq!(gate.name, "env-test");
        assert_eq!(gate.command, "echo $KEY");
        assert_eq!(gate.environment.get("KEY").unwrap(), "value");
    }

    #[test]
    fn test_performance_benchmark() {
        let gate = BuiltinValidators::performance_benchmark("my-bench");
        assert_eq!(gate.name, "benchmark-my-bench");
        assert!(gate.command.contains("cargo bench"));
        assert!(!gate.required);
        assert_eq!(gate.timeout_seconds, Some(600));
    }

    #[test]
    fn test_security_audit() {
        let gate = BuiltinValidators::security_audit();
        assert_eq!(gate.name, "security-audit");
        assert_eq!(gate.command, "cargo audit");
        assert_eq!(gate.timeout_seconds, Some(180));
    }

    #[test]
    fn test_docs_generation() {
        let gate = BuiltinValidators::docs_generation();
        assert_eq!(gate.name, "docs-generation");
        assert!(gate.command.contains("cargo doc"));
        assert!(!gate.required);
        assert_eq!(gate.timeout_seconds, Some(300));
    }

    #[test]
    fn test_format_check() {
        let gate = BuiltinValidators::format_check();
        assert_eq!(gate.name, "format-check");
        assert!(gate.command.contains("cargo fmt"));
        assert_eq!(gate.timeout_seconds, Some(60));
    }
}
