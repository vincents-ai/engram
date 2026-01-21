# Engram - Rust Implementation

A distributed memory system for AI agents, implemented in Rust with extensible architecture and BDD testing.

## Features

### Core Architecture
- **Extensible Entity System**: Trait-based entity architecture supporting dynamic model loading
- **Git-based Storage**: Content-addressable storage with multi-agent synchronization
- **Modular CLI**: Command-line interface with subcommands for all operations
- **BDD Testing**: Gherkin-style behavior-driven development testing
- **Configuration System**: Extensible configuration with validation
- **Plugin Support**: Dynamic plugin loading for custom extensions

### Entity Types
- **Tasks**: Work items with status tracking, priority, and hierarchy
- **Context**: Background information with relevance scoring and source tracking
- **Reasoning**: Decision chains with confidence levels and evidence tracking
- **Knowledge**: Information storage with usage metrics and categorization
- **Sessions**: Agent sessions with SPACE framework and DORA metrics
- **Compliance**: Requirements tracking with violations and remediation
- **Rules**: System rules with execution history and conditions
- **Standards**: Team standards with versioning and relationships
- **ADRs**: Architectural Decision Records with alternatives and outcomes
- **Workflows**: State machines with transitions and permission schemes
- **Relationships**: Entity relationships with graph operations and path finding
- **Commit Validation**: Pre-commit hook system with task relationship requirements

### Relationship Management
The relationship system provides powerful graph-based operations for modeling connections between entities:

#### Relationship Types
- **DependsOn**: Entity dependencies and prerequisites
- **Contains**: Hierarchical containment relationships
- **References**: Cross-references and citations
- **Fulfills**: Implementation and completion relationships
- **Implements**: Technical implementation relationships
- **Supersedes**: Version and replacement relationships
- **AssociatedWith**: General associations
- **Influences**: Impact and influence relationships
- **Custom**: User-defined relationship types

#### Relationship Features
- **Bidirectional/Unidirectional**: Configurable relationship directions
- **Weighted Connections**: Relationship strength (Weak, Medium, Strong, Critical)
- **Graph Traversal**: BFS, DFS, and Dijkstra pathfinding algorithms
- **Relationship Constraints**: Validation rules and cardinality limits
- **Analytics**: Connection statistics and graph metrics

### Storage Features
- **Content-addressable**: SHA-256 hashing for integrity verification
- **Multi-agent**: Collaboration with conflict resolution strategies
- **Git Integration**: Full Git history, branching, and merging
- **Performance**: Efficient querying and indexing

### CLI Commands
```bash
# Setup
engram setup workspace
engram setup agent --name alice --type coder

# Task Management
engram task create --title "Implement auth" --priority high
engram task list --agent alice
engram task update <id> --status done

# Context Management
engram context create --title "API docs" --source "documentation"
engram context list

# Reasoning Chains
engram reasoning create --title "Authentication approach" --task-id <id>

# Knowledge Management
engram knowledge create --title "OAuth2 flows" --type pattern

# Session Management
engram session start --agent alice --auto-detect
engram session status --id <session-id> --metrics

# Compliance & Standards
engram compliance create --title "Security requirements" --category security
engram standard create --title "Coding standards" --category coding

# ADRs
engram adr create --title "Database choice" --number 001

# Workflows
engram workflow create --title "Development pipeline"

# Relationship Management
engram relationship create --source-id task1 --source-type task --target-id task2 --target-type task --relationship-type depends-on --agent alice
engram relationship list --agent alice
engram relationship get <relationship-id>
engram relationship find-path --source-id task1 --target-id task3 --algorithm dijkstra
engram relationship connected --entity-id task1 --relationship-type depends-on
engram relationship stats --agent alice
engram relationship delete <relationship-id>

# Commit Validation and Hooks
engram validation commit --message "feat: implement user authentication [TASK-123]"
engram validation commit --message "test commit" --dry-run
engram validation hook install
engram validation hook uninstall
engram validation hook status
engram validation check

# Synchronization
engram sync --agents "alice,bob" --strategy intelligent_merge

# Perkeep Backup and Restore
engram perkeep backup --description "Full backup"
engram perkeep backup --entity-type task --include-relationships
engram perkeep list --detailed
engram perkeep restore --blobref "sha256-..."
engram perkeep health
engram perkeep config --server "http://localhost:3179"
```

## Perkeep Integration

Engram supports backing up and restoring entities using [Perkeep](https://perkeep.org), a personal data store for content-addressable storage.

### Configuration

Set environment variables to configure Perkeep:
```bash
export PERKEEP_SERVER="http://localhost:3179"  # Default: http://localhost:3179
export PERKEEP_AUTH_TOKEN="your-token"         # Optional: for authenticated servers
```

### Backup Commands

```bash
# Backup all entity types
engram perkeep backup

# Backup specific entity type
engram perkeep backup --entity-type task

# Backup with description
engram perkeep backup --description "Weekly backup"

# Include relationships in backup (default: true)
engram perkeep backup --include-relationships
```

### Restore Commands

```bash
# Restore from most recent backup
engram perkeep restore

# Restore from specific backup blobref
engram perkeep restore --blobref "sha256-abc123..."

# Dry run (preview what would be restored)
engram perkeep restore --dry-run

# Restore with agent override
engram perkeep restore --agent default
```

### Management Commands

```bash
# List available backups
engram perkeep list
engram perkeep list --detailed

# Check server health
engram perkeep health

# Configure settings
engram perkeep config --server "http://localhost:3179"
```

### How It Works

1. **Content-Addressable Storage**: Entities are serialized to JSON and uploaded as blobs to Perkeep
2. **Backup Metadata**: A schema object tracks all entity blobrefs, timestamps, and counts
3. **Entity Types**: task, context, reasoning, knowledge, session, and relationship entities
4. **Restore Process**: Fetches backup metadata, retrieves all entity blobs, and stores them

## Development

### Building
```bash
cargo build --release
```

### Testing
```bash
# Run BDD tests
cargo test --test bdd

# Run unit tests
cargo test

# Run with specific feature
cargo test --features plugins
```

### Examples

See the `examples/` directory for usage examples:

```bash
cargo run --example basic_usage
```

## Configuration

Engram supports YAML/TOML configuration with the following structure:

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

## Architecture

### Extensible Models
The Rust implementation uses trait-based architecture for extensibility:

```rust
pub trait Entity: Serialize + for<'de> Deserialize<'de> + Send + Sync {
    fn entity_type() -> &'static str;
    fn id(&self) -> &str;
    fn validate(&self) -> Result<(), String>;
    // ... other methods
}
```

### Plugin System
Custom entity types can be added dynamically:

```rust
// Register new entity type
registry.register::<CustomEntity>();

// Create from generic representation
let entity = registry.create(generic_entity)?;
```

### Commit Validation and Pre-commit Hooks

The validation system enforces disciplined development practices by ensuring all commits follow established patterns and reference proper tasks with required relationships.

#### Key Features:
- **Task Reference Validation**: Commits must reference existing tasks in multiple formats
- **Relationship Requirements**: Tasks must have reasoning and context relationships
- **File Scope Validation**: Changed files must match planned task scope
- **Exemption Support**: Configurable exemptions for chore, docs, fixup commits
- **Performance Optimized**: Caching and parallel validation support

#### Supported Task ID Formats:
- `[TASK-123]` - Brackets format
- `[task:auth-impl-001]` - Colon format  
- `Refs: #456` - Reference format

#### Validation Rules:
```bash
# Validate a commit
engram validation commit --message "feat: implement authentication [TASK-123]"

# Check hook status
engram validation hook status

# Install validation hook
engram validation hook install
```

#### Configuration:
Configuration via `.engram/validation.yaml`:

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

### Plugin System

### Storage Layer
The storage layer supports multiple backends through the `Storage` trait:

```rust
pub trait Storage: Send + Sync {
    fn store(&mut self, entity: &dyn Entity) -> Result<(), EngramError>;
    fn get(&self, id: &str, entity_type: &str) -> Result<Option<Box<dyn Entity>>, EngramError>;
    // ... other methods
}
```

## BDD Testing

Behavior-driven development testing with Gherkin syntax:

```gherkin
Feature: Task Management
  Scenario: Create a new task
    Given I have a workspace
    And I am logged in as agent "test-agent"
    When I create a new task "Implement login feature"
    Then the task should be created successfully
```

## Performance

- **Memory Efficient**: Content-addressable storage prevents duplication
- **Fast Queries**: Indexed storage with agent-based filtering
- **Scalable**: Git handles large repositories efficiently
- **Concurrent Safe**: Thread-safe storage operations

## Enterprise Features

- **Multi-tenant**: Isolated workspaces with shared memory
- **Audit Trails**: Complete Git history for compliance
- **Conflict Resolution**: Multiple strategies for team collaboration
- **Analytics**: SPACE framework and DORA metrics
- **Security**: Content integrity verification with SHA-256

## Migration from Go

The Rust implementation is designed as a drop-in replacement for the Go version:

1. Same CLI commands and arguments
2. Compatible data formats (JSON/YAML)
3. Same Git storage structure
4. Enhanced performance and features

## Contributing

The Rust implementation welcomes contributions for:

- New entity types
- Storage backends
- CLI commands
- BDD test scenarios
- Plugin examples

## License

AGPL-3.0-or-later OR Commercial - dual-licensed for open source and commercial use