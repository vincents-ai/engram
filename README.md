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

# Synchronization
engram sync --agents "alice,bob" --strategy intelligent_merge
```

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