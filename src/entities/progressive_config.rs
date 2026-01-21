use crate::entities::{Entity, GenericEntity};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ProgressiveGateConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub gate_levels: Vec<GateLevel>,
    pub escalation_rules: Vec<EscalationRule>,
    pub optimization_settings: OptimizationSettings,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub agent: String,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct GateLevel {
    #[validate(length(min = 1))]
    pub name: String,
    pub threshold: ChangeThreshold,
    pub required_gates: Vec<QualityGate>,
    pub optional_gates: Vec<QualityGate>,
    #[serde(with = "duration_serde")]
    pub max_execution_time: Duration,
    pub parallelization: ParallelizationStrategy,
    pub failure_handling: FailureHandling,
    pub enabled: bool,
    pub priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeThreshold {
    pub max_lines_changed: u32,
    pub max_files_affected: u32,
    pub max_complexity_delta: f32,
    pub allowed_change_types: Vec<ChangeType>,
    pub risk_level_limit: ProgressiveRiskLevel,
    pub file_patterns: Vec<FilePattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityGate {
    pub name: String,
    pub command: String,
    #[serde(with = "duration_serde")]
    pub timeout: Duration,
    pub required: bool,
    pub condition: Option<GateCondition>,
    pub environment: HashMap<String, String>,
    pub retry_policy: RetryPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParallelizationStrategy {
    Sequential,
    Parallel { max_concurrent: u32 },
    Adaptive { based_on: Vec<AdaptiveFactor> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdaptiveFactor {
    SystemLoad,
    ChangeComplexity,
    HistoricalDuration,
    ResourceAvailability,
    TimeOfDay,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailureHandling {
    pub continue_on_optional_failure: bool,
    pub fail_fast: bool,
    pub escalate_on_failure: bool,
    pub retry_failed_gates: bool,
    pub notification_channels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationRule {
    pub name: String,
    pub trigger: EscalationTrigger,
    pub action: EscalationAction,
    pub conditions: Vec<EscalationCondition>,
    pub enabled: bool,
    pub cooldown_period: Option<Duration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EscalationTrigger {
    GateFailure {
        gate_name: String,
        failure_count: u32,
    },
    TimeoutExceeded {
        max_duration: Duration,
    },
    ResourceThresholdReached {
        threshold: ResourceThreshold,
    },
    RiskLevelIncrease {
        from: ProgressiveRiskLevel,
        to: ProgressiveRiskLevel,
    },
    ConsecutiveFailures {
        count: u32,
        window: Duration,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EscalationAction {
    EscalateTo { level_name: String },
    TerminateAndEscalate { level_name: String },
    NotifyOnly { channels: Vec<String> },
    ManualApproval { approvers: Vec<String> },
    SkipRemaining,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EscalationCondition {
    NotInBddRedPhase,
    WorkingHours,
    CriticalPathChange,
    MinimumExecutionTime { duration: Duration },
    MaxEscalationsPerDay { limit: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationSettings {
    pub enable_historical_learning: bool,
    pub performance_tracking: bool,
    pub resource_optimization: bool,
    pub cache_results: bool,
    pub max_cache_age: Duration,
    pub adaptive_timeouts: bool,
    pub prediction_model: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum ChangeType {
    Documentation,
    Comments,
    Formatting,
    BugFix,
    Refactoring,
    FeatureMinor,
    Feature,
    Enhancement,
    BreakingChange,
    SecurityCritical,
    Performance,
    Configuration,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum ProgressiveRiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilePattern {
    pub pattern: String,
    pub pattern_type: PatternType,
    pub weight: f32,
    pub risk_multiplier: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternType {
    Glob,
    Regex,
    Extension,
    Directory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GateCondition {
    FilePatternMatch { patterns: Vec<String> },
    ChangeTypeMatch { types: Vec<ChangeType> },
    RiskLevelAbove { level: ProgressiveRiskLevel },
    LinesChangedAbove { count: u32 },
    ComplexityDeltaAbove { threshold: f32 },
    TimeWindowActive { start_hour: u8, end_hour: u8 },
    EnvironmentVariable { var: String, value: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub backoff_strategy: BackoffStrategy,
    pub retry_conditions: Vec<RetryCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackoffStrategy {
    Linear { increment: Duration },
    Exponential { base: f32, max: Duration },
    Fixed { delay: Duration },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RetryCondition {
    ExitCodeEquals { code: i32 },
    TimeoutOccurred,
    ResourceUnavailable,
    NetworkError,
    TemporaryFailure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceThreshold {
    pub memory_mb: Option<u64>,
    pub cpu_percentage: Option<f32>,
    pub disk_space_mb: Option<u64>,
    pub network_bandwidth_mbps: Option<f32>,
}

impl ProgressiveGateConfig {
    pub fn new(name: String, agent: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            description: String::new(),
            gate_levels: Vec::new(),
            escalation_rules: Vec::new(),
            optimization_settings: OptimizationSettings::default(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            agent,
            active: true,
        }
    }

    pub fn add_gate_level(&mut self, level: GateLevel) {
        self.gate_levels.push(level);
        self.updated_at = Utc::now();
    }

    pub fn find_appropriate_level(&self, metrics: &ComplexityMetrics) -> Option<&GateLevel> {
        for level in &self.gate_levels {
            if self.meets_threshold(&level.threshold, metrics) {
                return Some(level);
            }
        }
        self.gate_levels.last()
    }

    fn meets_threshold(&self, threshold: &ChangeThreshold, metrics: &ComplexityMetrics) -> bool {
        metrics.lines_changed <= threshold.max_lines_changed
            && metrics.files_affected <= threshold.max_files_affected
            && metrics.complexity_delta <= threshold.max_complexity_delta
            && threshold
                .allowed_change_types
                .contains(&metrics.primary_change_type)
            && metrics.risk_level <= threshold.risk_level_limit
    }

    pub fn get_enabled_levels(&self) -> Vec<&GateLevel> {
        self.gate_levels.iter().filter(|l| l.enabled).collect()
    }
}

impl Default for OptimizationSettings {
    fn default() -> Self {
        Self {
            enable_historical_learning: true,
            performance_tracking: true,
            resource_optimization: true,
            cache_results: true,
            max_cache_age: Duration::from_secs(3600),
            adaptive_timeouts: true,
            prediction_model: None,
        }
    }
}

impl Default for FailureHandling {
    fn default() -> Self {
        Self {
            continue_on_optional_failure: true,
            fail_fast: false,
            escalate_on_failure: true,
            retry_failed_gates: true,
            notification_channels: Vec::new(),
        }
    }
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            backoff_strategy: BackoffStrategy::Exponential {
                base: 2.0,
                max: Duration::from_secs(300),
            },
            retry_conditions: vec![RetryCondition::TemporaryFailure],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityMetrics {
    pub lines_changed: u32,
    pub files_affected: u32,
    pub complexity_delta: f32,
    pub primary_change_type: ChangeType,
    pub risk_level: ProgressiveRiskLevel,
    pub change_distribution: ChangeDistribution,
    pub risk_factors: Vec<String>,
    pub performance_impact: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeDistribution {
    pub source_files: u32,
    pub test_files: u32,
    pub config_files: u32,
    pub documentation: u32,
    pub build_files: u32,
}

impl Entity for ProgressiveGateConfig {
    fn entity_type() -> &'static str {
        "progressive_gate_config"
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn agent(&self) -> &str {
        &self.agent
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.created_at
    }

    fn validate_entity(&self) -> super::EntityResult<()> {
        if let Err(errors) = <ProgressiveGateConfig as validator::Validate>::validate(self) {
            let error_messages: Vec<String> = errors
                .field_errors()
                .values()
                .flat_map(|field_errors| field_errors.iter())
                .map(|error| {
                    error
                        .message
                        .clone()
                        .map(|s| s.to_string())
                        .unwrap_or_default()
                })
                .collect();
            return Err(error_messages.join(", "));
        }

        if self.name.is_empty() {
            return Err("Config name cannot be empty".to_string());
        }

        if self.gate_levels.is_empty() {
            return Err("At least one gate level must be configured".to_string());
        }

        Ok(())
    }

    fn to_generic(&self) -> GenericEntity {
        GenericEntity {
            id: self.id.clone(),
            entity_type: Self::entity_type().to_string(),
            agent: self.agent.clone(),
            timestamp: self.created_at,
            data: serde_json::to_value(self).unwrap_or_default(),
        }
    }

    fn from_generic(generic: GenericEntity) -> super::EntityResult<Self>
    where
        Self: Sized,
    {
        serde_json::from_value(generic.data)
            .map_err(|e| format!("Failed to deserialize ProgressiveGateConfig: {}", e))
    }

    fn as_any(&self) -> &dyn std::any::Any
    where
        Self: Sized,
    {
        self
    }
}

mod duration_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        duration.as_secs().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = u64::deserialize(deserializer)?;
        Ok(Duration::from_secs(secs))
    }
}
