# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.2] - 2026-01-20

### Added
- **Agent Sandboxing System**: Complete permission escalation workflow
  - EscalationHandler with create/approve/deny/cancel operations
  - Risk assessment and alternative suggestions for blocked operations
  - Integration with SandboxEngine validation pipeline
  - Automatic escalation creation on permission violations
  - Priority inference based on sandbox level and operation type
- **engram next command**: AI prompt generation for task-driven development
  - Automatic task prioritization and selection
  - Workflow state-aware prompt interpolation
  - Dynamic prompt templates in workflow states
  - Task relationship and dependency analysis
- **Workflow Validator**: State transition validation logic
  - Workflow state progression validation
  - Transition rule enforcement
  - State-based commit policy validation
- **Perkeep Integration**: Backup and restore for all entity types
  - Content-addressable blob storage using Perkeep server
  - Backup/restore commands: `engram perkeep backup`, `restore`, `list`, `health`
  - Configuration via PERKEEP_SERVER and PERKEEP_AUTH_TOKEN environment variables
- **engram info command**: Workspace and storage visibility
  - Shows storage backend path and type
  - Displays entity counts by type
  - Shows current agent and workspace path
- **Analytics System**: Task duration and workflow reporting
  - TaskDurationReport: Tracks time to complete tasks
  - WorkflowStageReport: Analyzes workflow state progression
  - BottleneckReport: Identifies workflow inefficiencies
- **Vector Search**: Optional semantic similarity search
  - SQLite-based vector storage with sqlite-vec
  - FastEmbed provider for embeddings
  - SearchQuery and SearchResult types for similarity search

### Fixed
- Build warnings and compilation errors
- Feature-gated sandbox module to prevent unused code warnings
- Test compilation errors in BDD framework
- Agent sandbox entity serialization in generic conversion
- Workflow instance persistence to storage (fixed in previous commit)

### Changed
- Improved workflow engine with prompt template support
- Enhanced workflow state machine with validation rules

## [0.1.1] - 2026-01-19

### Added
- Initial engram implementation with core entities
- Git-backed storage system
- Basic CLI commands for task, context, and relationship management
- Workflow engine foundation
- Commit validation system

### Fixed
- Various stability and reliability improvements

## [0.1.0] - 2026-01-17

### Added
- Initial release
- Core entity types (Task, Context, Reasoning, Relationship)
- Basic CLI interface
- Git storage backend
