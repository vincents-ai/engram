# Workflow Integration System Design

**Date**: 2026-01-17  
**Status**: Validated  
**Task**: 96807023-e396-49d6-b614-3e99d1e4e4a0

## Overview

Design for comprehensive workflow integration system that adds quality gates, development stage management, and agent collaboration to Engram's existing task management capabilities.

## Architecture

### Core Components

The system extends Engram's entity-relationship model with three new components:

1. **Workflow Definitions**: YAML-based DSL stored as engram entities
2. **Execution Results**: Quality gate output stored as engram entities with relationships
3. **Workflow Engine**: Orchestrates transitions and integrates with commit validation

### Workflow Definition Format

```yaml
name: "Feature Development"
description: "Complete BDD workflow for new features"
stages:
  - name: "requirements"
    description: "Requirements gathering and brainstorming"
    commit_policy: "engram_only"
    quality_gates:
      - command: "engram validate requirements-complete"
        required: true
        
  - name: "planning" 
    description: "Technical planning and architecture design"
    commit_policy: "engram_only"
    quality_gates:
      - command: "engram validate design-documented"
        required: true
    
  - name: "research"
    description: "Technical research and proof of concepts"
    commit_policy: "research_artifacts"
    quality_gates:
      - command: "engram validate research-documented"
        required: true
        
  - name: "bdd"
    description: "Write failing tests that specify desired behavior"
    commit_policy: "tests_only"
    quality_gates:
      - command: "cargo test"
        required: true
        expected_result: "failure"  # RED phase - tests MUST fail initially
        failure_message: "Tests should fail in BDD phase - this proves they're testing something real"
        
  - name: "development"
    description: "Implementation to make tests pass (GREEN phase)"
    commit_policy: "code_with_tests"
    quality_gates:
      - command: "cargo test"
        required: true
        expected_result: "success"  # GREEN phase - now tests MUST pass
      - command: "cargo clippy"
        required: false
        
  - name: "integration"
    description: "Full system testing and validation"
    commit_policy: "full_validation"
    quality_gates:
      - command: "nix build"
        required: true
      - command: "cargo test --all-features"
        required: true

transitions:
  - from: "planning"
    to: "research"
    trigger: "manual"
  - from: "development"
    to: "integration"
    trigger: "auto"    # Triggered by quality gate success
```

### Execution Results Entity

```rust
struct ExecutionResult {
    id: Uuid,
    task_id: Uuid,
    workflow_stage: String,
    command: String,
    exit_code: i32,
    stdout: String,
    stderr: String,
    timestamp: DateTime<Utc>,
    duration_ms: u64,
    environment: HashMap<String, String>,
    file_changes: Vec<String>,
    expected_result: Option<String>, // "success", "failure", "any"
    validation_status: ValidationStatus,
}

enum ValidationStatus {
    Passed,
    Failed { reason: String },
    Skipped { reason: String },
}
```

### Workflow Engine

Core orchestration component that:
- Evaluates stage transition conditions
- Executes quality gates and stores results
- Enforces commit policies per workflow stage
- Integrates with existing commit validation system

### CLI Integration

New commands following existing patterns:
- `engram workflow create` - Create workflow definitions
- `engram workflow assign --task-id [uuid] --workflow [name]`
- `engram task advance [uuid]` - Manual stage progression
- `engram task status [uuid]` - Show stage and quality gate results
- `engram workflow validate [uuid]` - Check gates without advancing

### Commit Policy Enforcement

Extended commit validation checks:
1. Task workflow stage and allowed commit types
2. Automatic quality gate execution on commits
3. Automatic stage transitions when conditions met
4. Rich error messages for policy violations

## Implementation Strategy

### File Structure
```
src/
├── entities/
│   ├── workflow.rs          # Workflow definition entity
│   └── execution_result.rs  # Quality gate execution results
├── workflow/
│   ├── mod.rs              # Workflow engine module
│   ├── engine.rs           # Core orchestration
│   ├── parser.rs           # YAML parsing
│   └── transitions.rs      # Stage transition logic
├── cli/
│   └── workflow.rs         # Workflow management commands
└── validation/
    └── workflow_validator.rs # Workflow-aware validation
```

### Integration Points

1. **Storage Layer**: Uses existing engram entity patterns
2. **Relationship System**: Standard engram relationships
3. **Commit Validation**: Extends existing hook system
4. **CLI Pattern**: Follows `engram [entity] [action]` convention

### Migration Strategy

- Backward compatible: existing tasks continue unchanged
- Opt-in workflow assignment via CLI
- Default "simple" workflow for compatibility
- Incremental adoption for complex tasks

## Future Evolution

### Phase 1: Configurable Gates (This Design)
- YAML workflow definitions
- Manual and automatic transitions
- Basic execution result storage

### Phase 2: Smart Gates
- Automatic detection of required testing
- Progressive requirements based on changes
- Context-aware policies for different change types

### Phase 3: Advanced DSL
- Rust plugins for complex transition logic
- Custom quality gate implementations
- Workflow templates and inheritance

## Quality Gates Framework

### Built-in Validators
- `cargo test` - Unit and integration tests
- `cargo clippy` - Linting
- `nix build` - Full system build
- `engram validate [custom]` - Project-specific checks

### Custom Commands
- Full shell command support
- Environment variable access
- Working directory control
- Timeout and resource limits

### Execution Context
- Results linked to specific commits
- File change tracking for targeted validation
- Environment capture for reproducibility
- Agent-accessible execution history

## Agent Collaboration Features

### Context Sharing
- Previous execution results available to all agents
- Searchable execution history via standard engram queries
- Cross-agent learning from failure patterns
- Execution result relationships enable discovery

### Decision Support
- "What was the last test failure for this task?"
- "Which commit broke the build?"
- "Has this error pattern been seen before?"
- Historical success/failure analysis

## Success Criteria

1. **Quality Enforcement**: No commits bypass appropriate validation
2. **Development Flow**: BDD Red-Green-Refactor cycle properly enforced
3. **Agent Collaboration**: Execution context seamlessly shared
4. **Flexibility**: Custom workflows definable via YAML
5. **Integration**: Seamless with existing engram functionality

## Dependencies

This design depends on:
- Existing engram entity storage system
- Current commit validation framework
- CLI command patterns
- Relationship traversal capabilities

No breaking changes to existing functionality required.