# Code Change Impact Analysis for Engram

**Date**: 2026-01-17
**Priority**: High
**Phase**: 1 - Core LLM Agent Features

## Overview

Implement an intelligent code change impact analysis system that automatically determines what needs testing, which components are affected, and what risks are introduced by code changes.

## Architecture

### Core Components

1. **Dependency Graph Builder** - Analyzes code structure and dependencies
2. **Change Impact Analyzer** - Determines scope of changes
3. **Risk Assessment Engine** - Evaluates change complexity and risk
4. **Test Recommendation System** - Suggests appropriate testing strategies
5. **Quality Gate Selector** - Chooses relevant quality gates based on impact

### Entity Design

```rust
// src/entities/impact_analysis.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAnalysis {
    pub id: String,
    pub commit_hash: String,
    pub task_id: String,
    pub agent_id: String,
    pub analyzed_at: DateTime<Utc>,
    pub changed_files: Vec<FileChange>,
    pub affected_components: Vec<ComponentImpact>,
    pub risk_assessment: RiskAssessment,
    pub recommended_tests: Vec<TestRecommendation>,
    pub suggested_quality_gates: Vec<String>,
    pub dependency_chain: DependencyChain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChange {
    pub file_path: String,
    pub change_type: ChangeType,
    pub lines_added: u32,
    pub lines_removed: u32,
    pub complexity_delta: i32,
    pub file_category: FileCategory,
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    Created,
    Modified { sections: Vec<ModificationSection> },
    Deleted,
    Renamed { old_path: String },
    Moved { old_path: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentImpact {
    pub component_name: String,
    pub component_type: ComponentType,
    pub impact_level: ImpactLevel,
    pub affected_interfaces: Vec<InterfaceChange>,
    pub downstream_dependencies: Vec<String>,
    pub upstream_dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComponentType {
    Module,
    Function,
    Struct,
    Trait,
    Database { table: String },
    API { endpoint: String },
    Configuration,
    Documentation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub overall_risk_level: RiskLevel,
    pub risk_factors: Vec<RiskFactor>,
    pub complexity_score: f32,
    pub change_magnitude: ChangeMagnitude,
    pub blast_radius: BlastRadius,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,      // Simple changes, well-tested components
    Medium,   // Moderate changes, some integration points
    High,     // Complex changes, critical components
    Critical, // Architecture changes, security-sensitive
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestRecommendation {
    pub test_type: TestType,
    pub priority: TestPriority,
    pub target_components: Vec<String>,
    pub estimated_duration: Duration,
    pub command_suggestion: String,
    pub rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestType {
    Unit { scope: Vec<String> },
    Integration { services: Vec<String> },
    EndToEnd { scenarios: Vec<String> },
    Performance { benchmarks: Vec<String> },
    Security { checks: Vec<String> },
    Compatibility { versions: Vec<String> },
}
```

### Impact Analysis Engine

```rust
// src/analysis/mod.rs
pub struct ImpactAnalysisEngine {
    dependency_analyzer: DependencyAnalyzer,
    risk_evaluator: RiskEvaluator,
    test_recommender: TestRecommender,
    quality_gate_selector: QualityGateSelector,
    project_metadata: ProjectMetadata,
}

impl ImpactAnalysisEngine {
    pub async fn analyze_changes(&self, 
        changed_files: Vec<String>,
        commit_hash: String
    ) -> Result<ImpactAnalysis> {
        // 1. Analyze file changes in detail
        let file_changes = self.analyze_file_changes(&changed_files).await?;
        
        // 2. Build dependency graph for affected components
        let dependency_chain = self.dependency_analyzer
            .build_impact_chain(&file_changes).await?;
        
        // 3. Identify affected components
        let affected_components = self.identify_affected_components(
            &file_changes, 
            &dependency_chain
        ).await?;
        
        // 4. Assess risk levels
        let risk_assessment = self.risk_evaluator
            .assess_risk(&file_changes, &affected_components).await?;
        
        // 5. Recommend tests
        let test_recommendations = self.test_recommender
            .recommend_tests(&affected_components, &risk_assessment).await?;
        
        // 6. Select quality gates
        let suggested_gates = self.quality_gate_selector
            .select_gates(&risk_assessment, &test_recommendations).await?;
        
        Ok(ImpactAnalysis {
            commit_hash,
            changed_files: file_changes,
            affected_components,
            risk_assessment,
            recommended_tests: test_recommendations,
            suggested_quality_gates: suggested_gates,
            dependency_chain,
            // ... other fields
        })
    }
    
    async fn analyze_file_changes(&self, files: &[String]) -> Result<Vec<FileChange>> {
        let mut changes = Vec::new();
        
        for file_path in files {
            // Get git diff information
            let diff_info = self.get_diff_info(file_path).await?;
            
            // Analyze code complexity changes
            let complexity_delta = self.calculate_complexity_change(file_path).await?;
            
            // Categorize file type
            let file_category = self.categorize_file(file_path);
            
            // Detect language
            let language = self.detect_language(file_path);
            
            changes.push(FileChange {
                file_path: file_path.clone(),
                change_type: diff_info.change_type,
                lines_added: diff_info.lines_added,
                lines_removed: diff_info.lines_removed,
                complexity_delta,
                file_category,
                language,
            });
        }
        
        Ok(changes)
    }
}

// Dependency analysis for Rust projects
impl DependencyAnalyzer {
    async fn build_impact_chain(&self, changes: &[FileChange]) -> Result<DependencyChain> {
        let mut chain = DependencyChain::new();
        
        for change in changes {
            // Parse Rust AST to find dependencies
            if change.language == "rust" {
                let dependencies = self.analyze_rust_dependencies(&change.file_path).await?;
                chain.add_dependencies(change.file_path.clone(), dependencies);
            }
            
            // Analyze other file types (YAML, TOML, etc.)
            let config_deps = self.analyze_config_dependencies(&change.file_path).await?;
            chain.add_config_dependencies(change.file_path.clone(), config_deps);
        }
        
        // Build reverse dependency map
        chain.build_reverse_dependencies();
        
        Ok(chain)
    }
    
    async fn analyze_rust_dependencies(&self, file_path: &str) -> Result<Vec<Dependency>> {
        // Use syn crate to parse Rust AST
        let content = std::fs::read_to_string(file_path)?;
        let syntax_tree = syn::parse_file(&content)?;
        
        let mut dependencies = Vec::new();
        
        // Analyze imports
        for item in &syntax_tree.items {
            match item {
                syn::Item::Use(use_item) => {
                    dependencies.extend(self.extract_use_dependencies(use_item));
                },
                syn::Item::Fn(func) => {
                    dependencies.extend(self.analyze_function_dependencies(func));
                },
                syn::Item::Struct(struct_item) => {
                    dependencies.extend(self.analyze_struct_dependencies(struct_item));
                },
                // ... other item types
                _ => {}
            }
        }
        
        Ok(dependencies)
    }
}
```

### Integration with Quality Gates

```rust
// Enhanced quality gate selection based on impact analysis
impl QualityGateSelector {
    pub async fn select_gates(&self, 
        risk: &RiskAssessment, 
        tests: &[TestRecommendation]
    ) -> Result<Vec<String>> {
        let mut gates = Vec::new();
        
        // Always run basic checks
        gates.push("cargo check".to_string());
        
        // Risk-based gate selection
        match risk.overall_risk_level {
            RiskLevel::Low => {
                gates.push("cargo test -- --lib".to_string());
            },
            RiskLevel::Medium => {
                gates.push("cargo test".to_string());
                gates.push("cargo clippy".to_string());
            },
            RiskLevel::High => {
                gates.push("cargo test --all-features".to_string());
                gates.push("cargo clippy -- -D warnings".to_string());
                gates.push("nix build".to_string());
            },
            RiskLevel::Critical => {
                gates.push("cargo test --all-features".to_string());
                gates.push("cargo clippy -- -D warnings".to_string());
                gates.push("nix build".to_string());
                gates.push("cargo audit".to_string());
                gates.push("engram security-scan".to_string());
            }
        }
        
        // Component-specific gates
        for factor in &risk.risk_factors {
            match factor {
                RiskFactor::DatabaseChanges => {
                    gates.push("engram test-migrations".to_string());
                },
                RiskFactor::ApiChanges => {
                    gates.push("engram api-compatibility-check".to_string());
                },
                RiskFactor::SecuritySensitive => {
                    gates.push("engram security-audit".to_string());
                },
                // ... other risk factors
            }
        }
        
        // Test-specific gates
        for test_rec in tests {
            if test_rec.priority == TestPriority::Critical {
                gates.push(test_rec.command_suggestion.clone());
            }
        }
        
        // Remove duplicates and sort by priority
        gates.sort();
        gates.dedup();
        
        Ok(gates)
    }
}
```

## Workflow Integration

```yaml
# Enhanced workflow with impact analysis
workflow_stages:
  - name: "impact_analysis"
    description: "Analyze change impact and select appropriate quality gates"
    commit_policy: "analysis_required"
    quality_gates:
      - command: "engram analyze-impact --files {changed_files}"
        required: true
        provides: ["impact_analysis", "risk_level", "test_recommendations"]
      
  - name: "risk_based_testing"
    description: "Execute tests based on impact analysis"
    commit_policy: "dynamic_gates"
    quality_gates:
      - command: "{recommended_gates}"  # Populated from impact analysis
        required: true
        dynamic: true
```

## CLI Integration

```bash
# Impact analysis commands
engram analyze-impact                           # Analyze current working directory changes
engram analyze-impact --commit abc123           # Analyze specific commit
engram analyze-impact --files src/auth.rs src/db.rs  # Analyze specific files
engram analyze-impact --compare main..feature-branch  # Compare branches

# Results and recommendations  
engram impact show <analysis-id>               # Show impact analysis details
engram impact recommendations <analysis-id>    # Show test recommendations
engram impact risks <analysis-id>              # Show risk assessment
engram impact dependencies <analysis-id>       # Show dependency chain

# Historical analysis
engram impact history --task auth-123          # Impact history for task
engram impact trends --component database      # Impact trends for component
engram impact compare <analysis-1> <analysis-2>  # Compare analyses

# Integration with existing commands
engram task create --with-impact-analysis "Refactor authentication"
engram workflow suggest --based-on-impact     # Suggest workflow based on risk
```

## Implementation Phases

### Phase 1: Basic File Analysis (2 weeks)
- File change detection and categorization
- Simple dependency analysis for common patterns
- Basic risk assessment based on file types and change size
- Integration with existing quality gate system

```bash
engram analyze-impact --files src/auth.rs
# → Risk Level: Medium
# → Affected Components: auth module, user sessions
# → Recommended Gates: cargo test, cargo clippy
```

### Phase 2: Rust AST Analysis (3 weeks)
- Full Rust syntax tree parsing
- Function-level dependency tracking
- Struct and trait impact analysis
- Interface change detection

### Phase 3: Cross-Component Analysis (3 weeks)
- Module dependency graph building
- Downstream impact propagation
- API compatibility checking
- Database schema change detection

### Phase 4: Advanced Risk Assessment (2 weeks)
- Machine learning-based risk scoring
- Historical failure correlation
- Complexity metrics integration
- Performance impact prediction

## File Structure

```
src/
├── entities/
│   ├── impact_analysis.rs      # Core impact entities
│   ├── dependency_graph.rs     # Dependency tracking entities
│   └── risk_assessment.rs      # Risk evaluation entities
├── analysis/
│   ├── mod.rs                  # Main analysis engine
│   ├── dependency_analyzer.rs  # Code dependency analysis
│   ├── risk_evaluator.rs      # Risk assessment logic
│   ├── test_recommender.rs     # Test recommendation engine
│   └── parsers/
│       ├── rust_parser.rs      # Rust AST analysis
│       ├── yaml_parser.rs      # Configuration analysis
│       └── sql_parser.rs       # Database schema analysis
├── integration/
│   └── quality_gate_selector.rs # Dynamic gate selection
└── cli/
    └── impact.rs               # Impact analysis commands
```

## Example Analysis Output

```bash
$ engram analyze-impact --files src/auth/mod.rs src/database/users.rs

📊 Impact Analysis Results
=========================

🔍 Changed Files:
  • src/auth/mod.rs: Modified (15 lines added, 3 removed)
    - Function: validate_token() - signature changed
    - Struct: UserSession - new field added
  • src/database/users.rs: Modified (8 lines added, 0 removed)
    - Function: create_user() - logic enhanced

⚠️  Risk Assessment: MEDIUM
  • API signature change detected (validate_token)
  • Database interaction modified
  • Moderate complexity increase (+12 points)
  • 3 downstream components affected

🔗 Affected Components:
  • auth module (direct) - API changes
  • user_service (indirect) - uses validate_token
  • admin_panel (indirect) - displays user sessions
  
🧪 Recommended Tests:
  1. Unit Tests (Priority: High)
     - auth::validate_token() variants
     - database::create_user() edge cases
  2. Integration Tests (Priority: Medium)  
     - user login flow end-to-end
     - admin panel user display

🛡️  Suggested Quality Gates:
  • cargo test auth database
  • cargo clippy
  • engram api-compatibility-check --module auth
  • engram integration-test --scenario user-auth

⏱️  Estimated Testing Time: 15-20 minutes
```

## Success Metrics

1. **Accuracy**: 85% accuracy in identifying affected components
2. **Risk Prediction**: 90% correlation between predicted and actual risk
3. **Test Efficiency**: 40% reduction in unnecessary test execution
4. **Quality Gate Optimization**: 60% improvement in relevant gate selection
5. **Time Savings**: 25% reduction in development cycle time

## Integration Points

- Extends existing quality gate system
- Integrates with Git change detection
- Uses existing Engram entity storage
- Compatible with workflow engine
- Leverages existing CLI patterns

## Dependencies

- Git integration for change detection
- Rust syntax parsing (syn crate)
- YAML/TOML parsing for configuration analysis
- Existing workflow and quality gate infrastructure

## Future Enhancements

- Machine learning models for risk prediction
- Integration with external code analysis tools
- Performance impact prediction
- Security vulnerability assessment
- Cross-repository impact analysis
- Real-time impact monitoring during development
