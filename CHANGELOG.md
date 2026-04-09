# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **PersonaArchitect SEP Integration**: Persona and Lesson entities with full CRUD CLI
  - Persona entity: name, role, domain, expertise, CoV/FAP/OV structured sections
  - Lesson entity: title, domain, category, content, difficulty, source_persona
  - CLI: `engram persona create|list|show|update|delete`, `engram lesson create|list|show|update|delete`
  - Two-tier storage: embedded defaults + filesystem overrides
  - Migrated 172 persona YAMLs with domain-adapted CoV/FAP/OV sections
- **Health CLI**: Git-based workspace diagnostics
  - `engram health check` — consistency validation and orphan detection
  - Health skill for structured project health assessment
- **Doc Subcommand**: `engram doc` with entity documentation, CLI reference, mdBook build, NLQ integration
- **Task Archiving**: Archive completed tasks to reduce noise in active queries
- **NLQ Deep Graph Walk**: Extended natural language queries to traverse entity relationships
- **Workflow Command Guards**: Enforce task-workflow binding before state transitions
- **Escalation CLI**: `engram escalation create|list|approve|deny|cancel` for permission management
- **Analytics & DORA Metrics**: Deployment frequency, lead time, change failure rate, MTTR reporting
  - `engram analytics` CLI with DORA dashboard
- **StructuredFeedback Trait**: Unified feedback interface across validation, testing, and review
- **Transition Side-Effects**: Automatic actions on workflow state changes
- **Quality Gates Consolidation**: Single quality gate pipeline replacing scattered checks
- **Session Enhancements**: Session summaries, zombie session detection, stale task identification, session filters, `--scope` flags on `engram next`
- **Locus TUI Screens**: Contexts, ADRs, and Theories views with auto-refresh
- **Workflow Engine**: Commit policy enforcement, auto-transition fixes
- **Knowledge Types**: Typed knowledge entities for better categorization
- **Skills**: engram-commit-convention, ask-first refinement, engram-first research protocols

### Changed
- Renamed GitStorage → GitRefsStorage throughout codebase
- Dead code cleanup, compliance fixes, sysinfo ResourceMonitor

## [0.6.3] - 2026-04-08

### Added
- `--all` and `--offset` pagination flags on all list commands

## [0.6.2] - 2026-04-08

### Fixed
- UTF-8 safe string truncation in `truncate()` and skills preview

## [0.6.1] - 2026-04-07

### Added
- **TUI Sync View**: Remote listing, sync status table, pull/push actions in Locus TUI
- **Session Skills**: session-start and session-end protocols with sync integration

### Fixed
- PATH-first binary resolution in commit-msg hook template

## [0.6.0] - 2026-04-07

### Added
- **Sync System** (8 phases): Multi-agent coordination via git refs
  - Phase 1: Project ID derivation from SHA-512 of root commit
  - Phase 2: Versioned sidecar refs written on every entity write
  - Phase 3: RemoteConfig with project_id and `import-git-remotes` subcommand
  - Phase 4+5: Version-aware pull/push with `refs/engram/*` refspec
  - Phase 6: `engram sync both` (pull-then-push)
  - Phase 7: Conflict resolution CLI with stateless re-derivation
  - Phase 8: `engram sync status` with per-type table and `--json` output
- **Locus TUI**: Terminal UI for browsing entities, views, events with full test suite
- **Auto-discover Skills**: Runtime skill loading from `./skills/` instead of compile-time embedding
- **test-harness-review Skill**: Audit test coverage across behavioral, integration, property, scenario, and unit tests
- mdBook build output with theme assets

### Changed
- Skill path references updated from opencode to engram
- Prompts stripped of OpenCode-specific content

## [0.5.0] - 2026-04-04

### Added
- **Screenplay Skills** (11): session-start, beat-sheet-builder, outliner, logline-writer, theme-developer, world-builder, character-developer, scene-writer, dialogue-refiner, plot-hole-finder, rewriter
- Extended `engram ask query` to search across all entity types

## [0.4.0] - 2026-04-03

### Added
- **Go-to-Market Skills** (3): market-validation, gtm-strategy, launch-execution
  - Corresponding pipelines: 101-market-validation, 102-gtm-strategy, 103-launch-execution
- **Tmux Skill**: Named session management for parallel agent isolation
- **NLQ Fulltext Search**: Natural language query intent mapping to fulltext search
- **Meta Skills**: Extended setup with `--force`, `--dir`, `--tool` flags
- mdBook branding with logo, favicon, and Engram color theme
- Install script (`install.sh`) as release artifact

### Fixed
- Corrected all 14 skill CLI commands
- Enforced `--task-id` on reasoning create

## [0.3.0] - 2026-03-10

### Added
- **Theory Building Entity**: Formal mental model capture based on Peter Naur's "Programming as Theory Building" (1985)
  - `domain_name`: High-level domain identifier
  - `conceptual_model`: HashMap of concept → definition
  - `system_mapping`: HashMap of concept → code location
  - `design_rationale`: HashMap of decision → reason
  - `invariants`: Vec of must-be-true statements
  - `iteration_count`: Tracks theory evolution
  - `apply_reflection_updates()`: Evolve theory from reflections

- **StateReflection Entity**: Cognitive dissonance detection for AI agents
  - `theory_id`: Theory being evaluated
  - `observed_state`: Raw error/observation
  - `cognitive_dissonance`: Vec of conflicts detected
  - `proposed_theory_updates`: How to resolve conflicts
  - `dissonance_score`: 0.0-1.0 (≥0.7 requires theory mutation)
  - `trigger_type`: test_failure, runtime_error, unexpected_output, etc.
  - `severity`: none, low, medium, high, critical

- **Session Enhancements**
  - New `Reflecting` status - blocks code execution until theory updated
  - `bind_theory()` - injects theory invariants into session metadata
  - `active_theory_id`, `theory_ids`, `reflection_ids` fields

- **CLI Commands**
  - `engram theory create|list|show|update|delete`
  - `engram reflect create|list|show|record-dissonance|propose-update|resolve|requires-mutation`
  - `engram session bind-theory|trigger-reflection|resolve-reflection`

- **mdBook Documentation**: Full documentation with Theory Building and State Reflection guides

- **Pipeline**: `prompts/ai/pipelines/00-theory-building.yaml` for theory extraction workflows

- **Agent Persona**: `prompts/agents/168-the-theorist.yaml` for reverse-engineering mental models

### Fixed
- Various clippy and formatting issues
- Syntax errors in theory.rs and session.rs

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
