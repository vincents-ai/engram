# Engram — Development Guide

Contributing to engram? This doc covers building, testing, and architecture.

## Build & Run

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# BDD tests (Gherkin)
cargo test --test bdd

# Run with specific feature
cargo test --features plugins
```

## Architecture Overview

Engram is a CLI tool with git-based storage. Everything lives in `.git/refs/engram/` — no files in your working tree.

### Entity System

All domain objects (tasks, context, reasoning, etc.) implement a shared `Entity` trait:

```rust
pub trait Entity: Serialize + for<'de> Deserialize<'de> + Send + Sync {
    fn entity_type() -> &'static str;
    fn id(&self) -> &str;
    fn validate(&self) -> Result<(), String>;
}
```

Entity types can be registered dynamically via the plugin system.

### Entity Types

| Entity | Purpose |
|--------|---------|
| **Tasks** | Work items with status, priority, hierarchy |
| **Context** | Background info with source tracking |
| **Reasoning** | Decision chains with confidence levels and evidence |
| **Knowledge** | Reusable patterns with usage metrics |
| **Sessions** | Agent work periods with SPACE/DORA metrics |
| **Compliance** | Requirements with violations and remediation |
| **Rules** | System rules with execution history |
| **Standards** | Team standards with versioning |
| **ADRs** | Architecture Decision Records |
| **Workflows** | State machines with transitions and guards |
| **Relationships** | Typed graph edges between entities |

### Storage Layer

Multiple backends through the `Storage` trait:

```rust
pub trait Storage: Send + Sync {
    fn store(&mut self, entity: &dyn Entity) -> Result<(), EngramError>;
    fn get(&self, id: &str, entity_type: &str) -> Result<Option<Box<dyn Entity>>, EngramError>;
}
```

Storage is content-addressable (SHA-256), multi-agent aware, and uses git refs directly.

### Plugin System

Custom entity types can be loaded dynamically:

```rust
registry.register::<CustomEntity>();
let entity = registry.create(generic_entity)?;
```

### Relationship Graph

Typed edges connect any two entities:

| Type | Meaning |
|------|---------|
| `DependsOn` | Dependency / prerequisite |
| `Contains` | Hierarchical containment |
| `References` | Cross-reference / citation |
| `Fulfills` | Implementation / completion |
| `Implements` | Technical implementation |
| `Supersedes` | Version / replacement |
| `AssociatedWith` | General association |
| `Influences` | Impact / influence |
| `Custom` | User-defined |

Features: bidirectional/unidirectional, weighted connections (Weak → Critical), BFS/DFS/Dijkstra traversal, constraint validation, graph analytics.

## CLI Commands

### Setup

```bash
engram setup workspace
engram setup agent --name alice --type coder
engram skills setup
engram setup skills          # all 44 skills
engram setup prompts         # prompt library
engram validate hook install
```

### Entity CRUD

```bash
# Tasks
engram task create --title "Implement auth" --priority high
engram task list --agent alice
engram task update <id> --status done

# Context
engram context create --title "API docs" --source "documentation"
engram context list

# Reasoning
engram reasoning create --title "Authentication approach" --task-id <id>

# Knowledge
engram knowledge create --title "OAuth2 flows" --type pattern

# Sessions
engram session start --agent alice --auto-detect
engram session status --id <session-id> --metrics

# Compliance & Standards
engram compliance create --title "Security requirements" --category security
engram standard create --title "Coding standards" --category coding

# ADRs
engram adr create --title "Database choice" --number 001

# Workflows
engram workflow create --title "Development pipeline"
```

### Relationships

```bash
engram relationship create --source-id task1 --source-type task \
  --target-id task2 --target-type task --relationship-type depends-on --agent alice
engram relationship list --agent alice
engram relationship get <id>
engram relationship find-path --source-id task1 --target-id task3 --algorithm dijkstra
engram relationship connected --entity-id task1 --relationship-type depends-on
engram relationship stats --agent alice
engram relationship delete <id>
```

### Validation & Hooks

```bash
engram validate commit --message "feat: implement auth [TASK-123]"
engram validate commit --message "test commit" --dry-run
engram validate hook install
engram validate hook uninstall
engram validate hook status
engram validate check
```

Supported task ID formats: `[TASK-123]`, `[task:auth-impl-001]`, `Refs: #456`.

Validation config in `.engram/validation.yaml`:

```yaml
enabled: true
require_task_reference: true
require_reasoning_relationship: true
require_context_relationship: true
task_id_patterns:
  - pattern: '\[([A-Z]+-\d+)\]'
    name: "Brackets format"
    example: "[TASK-123]"
exemptions:
  - message_pattern: '^(chore|docs):'
    skip_specific: ["require_task_reference"]
performance:
  cache_ttl_seconds: 300
  enable_parallel_validation: true
```

### Sync

```bash
engram sync --agents "alice,bob" --strategy intelligent_merge
```

### Perkeep Backup/Restore

```bash
# Config
export PERKEEP_SERVER="http://localhost:3179"
export PERKEEP_AUTH_TOKEN="your-token"

# Backup
engram perkeep backup
engram perkeep backup --entity-type task
engram perkeep backup --description "Weekly backup" --include-relationships

# Restore
engram perkeep restore
engram perkeep restore --blobref "sha256-abc123..." --dry-run

# Management
engram perkeep list --detailed
engram perkeep health
engram perkeep config --server "http://localhost:3179"
```

## Skills & Prompts

### Skills

```bash
# Core memory skill
cat ./engram/skills/meta/use-engram-memory.md

# Agent delegation
cat ./engram/skills/meta/delegate-to-agents.md

# Feature planning
cat ./engram/skills/workflow/plan-feature.md

# Compliance checking
cat ./engram/skills/compliance/check-compliance.md
```

### Prompt Library

```bash
# Agent prompts (170+)
ls ./engram/prompts/agents/

# Pipeline templates (100+)
ls ./engram/prompts/ai/pipelines/

# Compliance prompts (250+)
ls ./engram/prompts/compliance_and_certification/prompts/audit_checkpoints/
```

### Engram-Adapted Prompts

Core agents and pipelines adapted for engram integration:

| Prompt | What it does with engram |
|--------|-------------------------|
| `01-the-one.yaml` | Orchestrator with engram task creation |
| `03-the-architect.yaml` | Architecture with engram context storage |
| `05-the-deconstructor.yaml` | Task breakdown with engram subtasks |
| `01-greenfield-feature-launch.yaml` | Engram workflow orchestration |

All include: `task_id` parameter, `engram reasoning create`, `engram context create`, `engram relationship create`, JSON response with entity IDs.

## Configuration

`.engram/config.yaml`:

```yaml
app:
  log_level: info
  default_agent: default
  git:
    author_name: Your Name
    author_email: your.email@example.com

workspace:
  agents:
    coder:
      type: implementation
      description: "Handles code changes"

storage:
  storage_type: git
  base_path: .engram
  sync_strategy: intelligent_merge

features:
  plugins: true
  analytics: true
  experimental: false
```

## BDD Testing

Gherkin-style behavior tests:

```gherkin
Feature: Task Management
  Scenario: Create a new task
    Given I have a workspace
    And I am logged in as agent "test-agent"
    When I create a new task "Implement login feature"
    Then the task should be created successfully
```

## Contributing

Contributions welcome for:

- New entity types
- Storage backends
- CLI commands
- BDD test scenarios
- Plugin examples

## License

AGPL-3.0-or-later OR Commercial — dual-licensed.
