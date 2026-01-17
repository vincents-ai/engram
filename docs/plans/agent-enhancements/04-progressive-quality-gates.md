# Progressive Quality Gates for Engram

**Date**: 2026-01-17
**Priority**: High
**Phase**: 1 - Core LLM Agent Features

## Overview

Implement adaptive quality gate system that scales testing requirements based on change complexity, risk level, and impact scope. This ensures efficient use of computational resources while maintaining appropriate quality standards.

## Architecture

### Core Components

1. **Change Complexity Analyzer** - Evaluates scope and complexity of changes
2. **Progressive Gate Selector** - Chooses appropriate gate levels
3. **Resource Optimizer** - Balances testing coverage with execution time
4. **Escalation Engine** - Automatically escalates to higher gate levels on failures
5. **Performance Tracker** - Monitors gate execution efficiency

### Gate Level System

```rust
// src/quality_gates/progressive.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressiveGateConfig {
    pub gate_levels: Vec<GateLevel>,
    pub escalation_rules: Vec<EscalationRule>,
    pub optimization_settings: OptimizationSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GateLevel {
    pub name: String,
    pub threshold: ChangeThreshold,
    pub required_gates: Vec<QualityGate>,
    pub optional_gates: Vec<QualityGate>,
    pub max_execution_time: Duration,
    pub parallelization: ParallelizationStrategy,
    pub failure_handling: FailureHandling,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeThreshold {
    pub max_lines_changed: u32,
    pub max_files_affected: u32,
    pub max_complexity_delta: f32,
    pub allowed_change_types: Vec<ChangeType>,
    pub risk_level_limit: RiskLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParallelizationStrategy {
    Sequential,
    Parallel { max_concurrent: u32 },
    Adaptive { based_on: Vec<AdaptiveFactor> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationRule {
    pub trigger: EscalationTrigger,
    pub action: EscalationAction,
    pub conditions: Vec<EscalationCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EscalationTrigger {
    GateFailure { gate_name: String, failure_count: u32 },
    TimeoutExceeded { max_duration: Duration },
    ResourceThresholdReached { threshold: ResourceThreshold },
    RiskLevelIncrease { from: RiskLevel, to: RiskLevel },
}
```

### Default Gate Level Configuration

```yaml
# .engram/progressive-gates.yaml
gate_levels:
  minimal:
    name: "Minimal Validation"
    threshold:
      max_lines_changed: 10
      max_files_affected: 2
      max_complexity_delta: 5.0
      allowed_change_types: ["documentation", "comments", "formatting"]
      risk_level_limit: "low"
    required_gates:
      - name: "syntax_check"
        command: "cargo check"
        timeout: "30s"
      - name: "format_check"  
        command: "cargo fmt --check"
        timeout: "15s"
    optional_gates: []
    max_execution_time: "1m"
    parallelization: "parallel"

  light:
    name: "Light Testing"
    threshold:
      max_lines_changed: 50
      max_files_affected: 5
      max_complexity_delta: 15.0
      allowed_change_types: ["bug_fix", "refactoring", "feature_minor"]
      risk_level_limit: "medium"
    required_gates:
      - name: "syntax_check"
        command: "cargo check"
      - name: "unit_tests"
        command: "cargo test --lib"
        timeout: "5m"
      - name: "lint_check"
        command: "cargo clippy"
    max_execution_time: "7m"
    parallelization: 
      parallel:
        max_concurrent: 3

  standard:
    name: "Standard Validation"
    threshold:
      max_lines_changed: 200
      max_files_affected: 15
      max_complexity_delta: 50.0
      allowed_change_types: ["feature", "enhancement", "bug_fix"]
      risk_level_limit: "high"
    required_gates:
      - name: "full_test_suite"
        command: "cargo test"
        timeout: "15m"
      - name: "lint_strict"
        command: "cargo clippy -- -D warnings"
      - name: "integration_tests"
        command: "cargo test --test integration"
    optional_gates:
      - name: "benchmark_tests"
        command: "cargo bench --no-run"
        condition: "performance_sensitive_files_changed"
    max_execution_time: "20m"

  comprehensive:
    name: "Comprehensive Validation"  
    threshold:
      max_lines_changed: 999999  # No limit
      max_files_affected: 999999
      max_complexity_delta: 999999.0
      risk_level_limit: "critical"
    required_gates:
      - name: "full_test_suite_all_features"
        command: "cargo test --all-features"
      - name: "build_verification"
        command: "nix build"
      - name: "security_audit"
        command: "cargo audit"
      - name: "dependency_check"
        command: "cargo outdated"
    max_execution_time: "45m"
    parallelization:
      adaptive:
        based_on: ["system_load", "change_complexity", "historical_duration"]

escalation_rules:
  - trigger:
      gate_failure:
        gate_name: "unit_tests"
        failure_count: 1
    action:
      escalate_to_level: "standard"
    conditions:
      - not_in_bdd_red_phase
  
  - trigger:
      timeout_exceeded:
        max_duration: "10m"
    action:
      terminate_and_escalate: "comprehensive"
```

### Progressive Gate Engine

```rust
// src/quality_gates/progressive_engine.rs
pub struct ProgressiveGateEngine {
    config: ProgressiveGateConfig,
    change_analyzer: ChangeComplexityAnalyzer,
    resource_monitor: ResourceMonitor,
    execution_history: ExecutionHistory,
}

impl ProgressiveGateEngine {
    pub async fn select_appropriate_gates(&self, 
        changes: &[FileChange],
        task_context: &TaskContext
    ) -> Result<GateLevel> {
        // 1. Analyze change complexity
        let complexity_metrics = self.change_analyzer
            .analyze_complexity(changes).await?;
        
        // 2. Check current system resources
        let resource_status = self.resource_monitor.get_current_status().await?;
        
        // 3. Consider execution history for similar changes
        let historical_data = self.execution_history
            .find_similar_executions(changes, 30).await?;
        
        // 4. Select appropriate gate level
        let base_level = self.select_base_gate_level(&complexity_metrics)?;
        
        // 5. Apply resource-based adjustments
        let adjusted_level = self.adjust_for_resources(base_level, &resource_status)?;
        
        // 6. Apply historical optimizations
        let optimized_level = self.apply_historical_optimizations(
            adjusted_level, 
            &historical_data
        )?;
        
        Ok(optimized_level)
    }
    
    fn select_base_gate_level(&self, metrics: &ComplexityMetrics) -> Result<GateLevel> {
        for level in &self.config.gate_levels {
            if self.meets_threshold(&level.threshold, metrics) {
                return Ok(level.clone());
            }
        }
        
        // Default to comprehensive for changes that exceed all thresholds
        Ok(self.config.gate_levels.last().unwrap().clone())
    }
    
    fn meets_threshold(&self, threshold: &ChangeThreshold, metrics: &ComplexityMetrics) -> bool {
        metrics.lines_changed <= threshold.max_lines_changed &&
        metrics.files_affected <= threshold.max_files_affected &&
        metrics.complexity_delta <= threshold.max_complexity_delta &&
        threshold.allowed_change_types.contains(&metrics.primary_change_type) &&
        metrics.risk_level <= threshold.risk_level_limit
    }
    
    pub async fn execute_progressive_gates(&self,
        gate_level: &GateLevel,
        context: &ExecutionContext
    ) -> Result<ProgressiveExecutionResult> {
        let mut results = Vec::new();
        let start_time = Instant::now();
        
        // Execute required gates
        for gate in &gate_level.required_gates {
            let result = self.execute_single_gate(gate, context).await;
            results.push(result.clone());
            
            // Check for escalation triggers
            if let Err(ref error) = result {
                if let Some(escalation) = self.check_escalation_needed(error, gate_level)? {
                    return self.handle_escalation(escalation, context).await;
                }
            }
            
            // Early termination on critical failures
            if self.should_terminate_early(&result, gate_level)? {
                break;
            }
        }
        
        // Execute optional gates based on conditions and remaining time
        let remaining_time = gate_level.max_execution_time - start_time.elapsed();
        if remaining_time > Duration::from_secs(60) {
            for gate in &gate_level.optional_gates {
                if self.should_execute_optional_gate(gate, context, remaining_time).await? {
                    let result = self.execute_single_gate(gate, context).await;
                    results.push(result);
                }
            }
        }
        
        Ok(ProgressiveExecutionResult {
            gate_level: gate_level.name.clone(),
            execution_results: results,
            total_duration: start_time.elapsed(),
            escalation_occurred: false,
        })
    }
}

impl ChangeComplexityAnalyzer {
    pub async fn analyze_complexity(&self, changes: &[FileChange]) -> Result<ComplexityMetrics> {
        let mut total_lines = 0;
        let mut total_complexity_delta = 0.0;
        let mut change_types = Vec::new();
        let mut risk_factors = Vec::new();
        
        for change in changes {
            total_lines += change.lines_added + change.lines_removed;
            total_complexity_delta += change.complexity_delta as f32;
            
            // Analyze change patterns
            change_types.push(self.classify_change_type(change)?);
            
            // Identify risk factors
            risk_factors.extend(self.identify_risk_factors(change)?);
        }
        
        let primary_change_type = self.determine_primary_change_type(&change_types);
        let risk_level = self.assess_overall_risk(&risk_factors);
        
        Ok(ComplexityMetrics {
            lines_changed: total_lines,
            files_affected: changes.len() as u32,
            complexity_delta: total_complexity_delta,
            primary_change_type,
            risk_level,
            change_distribution: self.analyze_change_distribution(changes)?,
        })
    }
}
```

## CLI Integration

```bash
# Progressive gate management
engram gates analyze --changes src/auth.rs      # Analyze required gate level
engram gates select --auto                      # Auto-select appropriate gates
engram gates execute --level standard           # Execute specific gate level
engram gates escalate --from light --to standard  # Manual escalation

# Gate level configuration
engram gates config list                        # Show current gate levels
engram gates config edit minimal               # Edit specific gate level
engram gates config validate                   # Validate gate configuration
engram gates config optimize --based-on-history # Auto-optimize based on history

# Execution monitoring
engram gates status                             # Current gate execution status  
engram gates history --last 10                 # Recent gate executions
engram gates performance --gate-level standard # Performance metrics by level
engram gates resource-usage                    # Resource consumption analysis
```

## Integration with Existing Systems

### Workflow Integration
```yaml
# Enhanced workflow with progressive gates
workflow_stages:
  - name: "development"
    description: "Development stage with adaptive quality gates"
    commit_policy: "progressive_validation"
    quality_gates:
      - command: "engram gates execute --auto"
        required: true
        adaptive: true
        escalation_enabled: true
```

### Task Context Integration
```rust
impl TaskExecutor {
    async fn execute_with_progressive_gates(&self, task: &Task) -> Result<TaskResult> {
        // 1. Get current changes
        let changes = self.get_staged_changes().await?;
        
        // 2. Select appropriate gate level
        let gate_level = self.progressive_engine
            .select_appropriate_gates(&changes, &task.context).await?;
        
        // 3. Execute gates with monitoring
        let gate_results = self.progressive_engine
            .execute_progressive_gates(&gate_level, &task.execution_context).await?;
        
        // 4. Update task with gate execution results
        self.update_task_with_gate_results(task, &gate_results).await?;
        
        Ok(TaskResult::from_gate_results(gate_results))
    }
}
```

## Implementation Phases

### Phase 1: Basic Level Selection (2 weeks)
- Implement gate level configuration system
- Basic change analysis for level selection
- Simple gate execution with level-based selection
- Integration with existing quality gate system

```bash
engram gates execute --level light
# → Selected: Light Testing (3 gates, ~7min estimated)
# → Running: cargo check ✓ (30s)
# → Running: cargo test --lib ✓ (4m 15s) 
# → Running: cargo clippy ✓ (45s)
# → Completed: All gates passed (5m 30s total)
```

### Phase 2: Adaptive Selection (2 weeks)
- Change complexity analysis
- Risk-based gate level selection
- Resource-aware optimization
- Basic escalation rules

### Phase 3: Advanced Optimization (3 weeks)
- Historical performance analysis
- Intelligent gate parallelization
- Dynamic timeout adjustment
- Smart optional gate selection

### Phase 4: Machine Learning Enhancement (3 weeks)
- Predictive gate selection based on patterns
- Automated threshold optimization
- Performance regression detection
- Continuous improvement feedback loops

## File Structure

```
src/
├── quality_gates/
│   ├── progressive_engine.rs   # Main progressive gate engine
│   ├── level_selector.rs       # Gate level selection logic
│   ├── escalation_handler.rs   # Escalation rule processing
│   ├── resource_optimizer.rs   # Resource-aware optimization
│   └── complexity_analyzer.rs  # Change complexity analysis
├── entities/
│   ├── progressive_config.rs   # Gate level configuration entities
│   └── execution_metrics.rs    # Performance tracking entities
├── analysis/
│   └── change_complexity.rs    # Change analysis integration
└── cli/
    └── progressive_gates.rs     # CLI commands for gate management
```

## Example Execution Scenarios

### Scenario 1: Small Bug Fix
```bash
$ engram gates analyze --changes src/utils.rs
→ Change Analysis:
  • 5 lines changed in 1 file
  • Complexity delta: +2.1
  • Change type: bug_fix  
  • Risk level: low

→ Selected Gate Level: MINIMAL
  • Estimated time: <1 minute
  • Gates: syntax_check, format_check
  • Resource usage: Low

$ engram gates execute --auto
✓ cargo check (18s)
✓ cargo fmt --check (3s)
→ All gates passed (21s total)
```

### Scenario 2: Feature Development with Escalation
```bash  
$ engram gates execute --level standard
→ Running Standard Validation (estimated 20m)...

✓ cargo test (12m)
✗ cargo clippy -- -D warnings (2m) 
  Error: 3 clippy warnings found
  
→ Escalation triggered: clippy failure
→ Escalating to COMPREHENSIVE level...

✓ cargo test --all-features (15m)
✓ nix build (8m)  
✓ cargo audit (30s)
→ Escalated validation completed (25m 30s total)
```

### Scenario 3: Critical System Change
```bash
$ engram gates analyze --changes src/security/ src/database/
→ Change Analysis:
  • 145 lines changed in 8 files
  • Complexity delta: +67.3
  • Change type: security_critical
  • Risk level: critical
  • Affected components: authentication, user_data

→ Selected Gate Level: COMPREHENSIVE
  • Estimated time: 45 minutes
  • Additional checks: security_audit, dependency_review
  • Parallel execution: 4 concurrent gates

$ engram gates execute --auto
Running Comprehensive Validation (4 gates in parallel)...
[████████████████████████████████] 100% (42m 15s)
All gates passed with comprehensive validation
```

## Success Metrics

1. **Efficiency Gain**: 40% reduction in average gate execution time
2. **Resource Optimization**: 50% better resource utilization
3. **Quality Maintenance**: No decrease in defect detection rate
4. **Developer Satisfaction**: 60% faster feedback for small changes
5. **Escalation Accuracy**: 85% of escalations correctly identify issues

## Integration Points

- Builds on existing QualityGate system
- Integrates with change impact analysis
- Uses existing workflow engine
- Compatible with current CLI patterns
- Leverages existing storage and entity systems

## Future Enhancements

- Machine learning models for optimal gate selection
- Cross-project learning for gate optimization
- Integration with CI/CD pipeline optimization
- Real-time resource monitoring and adjustment
- Predictive failure analysis for proactive escalation
- Custom gate level creation based on team preferences
