# Import Feature Design Document

## Overview

The `engram import` command will parse structured markdown files and auto-create engram entities with relationships based on pattern matching.

## Goals

1. **Batch Entity Creation**: Import multiple entities from a single markdown file
2. **Auto-Linking**: Automatically create relationships based on patterns
3. **Version Control Friendly**: Markdown files that can be reviewed, diffed, and merged
4. **Single Source of Truth**: Documentation and entity state stay in sync

## CLI Interface

```bash
# Basic import
engram import --file review.md

# Verbose output
engram import --file review.md --verbose

# Dry run (preview without creating)
engram import --file review.md --dry-run

# Overwrite existing entities
engram import --file review.md --force

# JSON output
engram import --file review.md --json
```

## Input Format: Engram Markdown (EMD)

### Frontmatter

All documents must start with YAML frontmatter:

```yaml
---
title: Codebase Review: Engram v0.1.3
type: review | context | task | reasoning | findings
date: 2026-01-22
author: sisyphus
tags: [codebase, review, engram]
parent_id: (optional UUID for containment)
---
```

### Supported Section Types

#### 1. Findings Section

```markdown
## Findings

### Finding: 5 Failing Tests
[Finding: 3ecbee2c-aedc-4f74-ab7e-49491dda201e]

Description of the finding...

### Finding: 12 Unused Functions
[Finding: e4d56a1c-ced7-400b-928d-d568050]
```

**5d3cfBehavior:** Creates Context entities for each finding

#### 2. Tasks Section

```markdown
## Tasks

- [P0: da23ec54-04c8-4679-81e4-d90c09642d4c] Fix 5 failing tests
- [P1: 58339da6-71e3-41df-a6af-93d6b4f861eb] Add --file to reasoning
```

**Behavior:** Links to existing tasks (does NOT create new tasks)

#### 3. Entity Links Section

```markdown
## Entity Graph

- Main Context [contains] → Finding: Tests [3ecbee2c-...]
- Task [references] → Finding: Tests [3ecbee2c-...]
```

**Behavior:** Creates relationships between entities

#### 4. Reasoning Section

```markdown
## Reasoning

### Analysis: Test Reliability
[Reasoning: 85d49be1-f62c-4bf3-847f-4e4f65ff1052]
[Task: da23ec54-04c8-4679-81e4-d90c09642d4c]

Detailed reasoning content...
```

**Behavior:** Creates Reasoning entities linked to tasks

## Pattern Reference

### Auto-Linking Patterns

| Pattern | Meaning | Creates |
|---------|---------|---------|
| `[UUID]` | UUID reference | Relationship to entity |
| `[Finding: UUID]` | Finding reference | Context entity + link |
| `[Task: UUID]` | Task reference | Task relationship |
| `[Reasoning: UUID]` | Reasoning reference | Reasoning entity + link |
| `[P0: UUID]` | Priority task | Task relationship |
| `[[Title]]` | Title search | Link to entity by title |
| `[contains]` | Relationship type | Relationship |
| `[references]` | Relationship type | Relationship |

### Relationship Shortcuts

```markdown
# Contains relationship
[Finding: 3ecbee2c-...] → child of main context

# References relationship  
[Task: xxx] → references Finding: yyy

# Documents relationship
[Reasoning: zzz] → documents Task: xxx
```

## Architecture

### Module Structure

```
src/cli/
├── import.rs          # Main import command
├── mod.rs             # Add 'pub mod import;'
└── main.rs            # Add Import command handler

src/import/
├── parser.rs          # YAML frontmatter parser
├── patterns.rs        # UUID pattern matcher
├── entities.rs        # Entity builders
├── relationships.rs   # Relationship builder
└── mod.rs             # Module organization
```

### Key Functions

```rust
// Main entry point
pub fn handle_import_command(
    file: PathBuf,
    verbose: bool,
    dry_run: bool,
    force: bool,
) -> Result<ImportResult, EngramError>

// Parse document structure
pub fn parse_emd_document(content: &str) -> Result<EmdDocument, ImportError>

// Extract frontmatter
pub fn parse_frontmatter(content: &str) -> Result<Frontmatter, ImportError>

// Find all sections
pub fn extract_sections(content: &str) -> Vec<Section>

// Match UUID patterns
pub fn find_uuids(text: &str) -> Vec<UuidMatch>

// Create entities from findings
pub fn create_finding_entities(
    sections: &[Section],
    storage: &mut dyn Storage,
) -> Result<Vec<EntityId>, EngramError>

// Create relationships
pub fn create_relationships(
    patterns: &[RelationshipPattern],
    storage: &mut dyn Storage,
) -> Result<Vec<RelationshipId>, EngramError>
```

### Data Structures

```rust
pub struct Frontmatter {
    pub title: String,
    pub doc_type: DocType,  // review, context, task, reasoning, findings
    pub date: Option<DateTime<Utc>>,
    pub author: Option<String>,
    pub tags: Vec<String>,
    pub parent_id: Option<Uuid>,
}

pub struct EmdDocument {
    pub frontmatter: Frontmatter,
    pub sections: Vec<Section>,
    pub findings: Vec<Finding>,
    pub tasks: Vec<TaskRef>,
    pub relationships: Vec<RelationshipPattern>,
    pub reasoning: Vec<ReasoningSection>,
}

pub struct Section {
    pub level: usize,  // 1 = ##, 2 = ###, etc.
    pub title: String,
    pub content: String,
}

pub struct UuidMatch {
    pub uuid: Uuid,
    pub pattern: String,  // "[UUID]" or "[Finding: UUID]"
    pub context: String,  // Surrounding text
}

pub struct RelationshipPattern {
    pub source: Uuid,
    pub target: Uuid,
    pub rel_type: RelationshipType,
    pub context: String,
}
```

## Implementation Phases

### Phase 1: Core Import

1. Create `src/cli/import.rs` with basic CLI structure
2. Add `pub mod import;` to `cli/mod.rs`
3. Add `Import` variant to `Commands` enum
4. Implement YAML frontmatter parser (reuse `serde_yaml`)
5. Implement basic markdown section extraction

### Phase 2: Entity Creation

1. Implement `create_context_from_finding()` 
2. Implement `create_reasoning_from_section()`
3. Handle entity ID assignment (UUIDs vs auto-generate)
4. Store entities via Storage trait

### Phase 3: Auto-Linking

1. Implement UUID pattern matcher (regex)
2. Parse relationship shortcuts (`[contains]`, `[references]`)
3. Create relationships via `storage.store()`
4. Handle entity not found errors gracefully

### Phase 4: Polish

1. Add `--dry-run` for preview mode
2. Add `--verbose` for detailed output
3. Add `--force` for overwriting
4. Add `--json` for programmatic output
5. Unit tests and error handling

## Error Handling

### Recoverable Errors (continue, log warning)

- Entity not found when creating relationship
- Invalid UUID format in pattern
- Duplicate relationship (skip)

### Fatal Errors (abort import)

- Missing frontmatter
- Invalid YAML in frontmatter
- Missing required section
- Storage write failure

### Error Codes

```rust
enum ImportError {
    InvalidFrontmatter(String),
    MissingSection(String),
    EntityNotFound(Uuid),
    DuplicateEntity(Uuid),
    StorageError(String),
    PatternParseError(String),
}
```

## Example Usage

### Import CODEBASE_REVIEW.md

```bash
$ engram import --file ./engram/CODEBASE_REVIEW.md --verbose
[INFO] Parsing frontmatter...
[INFO] Title: Codebase Review: Engram v0.1.3
[INFO] Type: review
[INFO] Found 3 findings
[INFO] Found 5 tasks
[INFO] Found 2 relationship patterns
[INFO] Creating entities...
[INFO] Created context: d453d97e-0f2d-416c-9199-fc70834cb546
[INFO] Created context: 3ecbee2c-aedc-4f74-ab7e-49491dda201e (Finding)
[INFO] Created context: e4d56a1c-ced7-400b-928d-d5685d3cf050 (Finding)
[INFO] Created context: f86f49f5-c864-4a9f-8da8-29affc64aa72 (Finding)
[INFO] Created reasoning: 85d49be1-f62c-4bf3-847f-4e4f65ff1052
[INFO] Created 299 relationships
[SUCCESS] Import complete: 7 entities, 299 relationships
```

### Dry Run

```bash
$ engram import --file ./engram/CODEBASE_REVIEW.md --dry-run
[DRY RUN] Would create 7 entities
[DRY RUN] Would create 299 relationships
[DRY RUN] Entities:
  - d453d97e-0f2d-416c-9199-fc70834cb546 (Context: Codebase Review)
  - 3ecbee2c-aedc-4f74-ab7e-49491dda201e (Context: Finding: 5 Failing Tests)
  ...
```

## Edge Cases

### 1. Entity Already Exists

**Scenario:** UUID in file already exists in storage

**Options:**
- `--force`: Overwrite existing entity
- Default: Skip and warn

### 2. UUID in Pattern Not Found

**Scenario:** `[contains]→ [UUID]` but UUID doesn't exist

**Options:**
- Default: Skip and warn (but continue import)
- `--strict`: Fail on missing entity

### 3. Malformed UUID

**Scenario:** `[Finding: not-a-uuid]`

**Result:** Error, abort import

### 4. Multiple Patterns in One Line

**Scenario:** `Task [references] → Finding [UUID] and [UUID2]`

**Result:** Create multiple relationships

### 5. Self-Referential Relationship

**Scenario:** `[UUID] [contains] → [same UUID]`

**Result:** Skip and warn

## Testing

### Unit Tests

```rust
#[test]
fn test_frontmatter_parsing() { ... }

#[test]
fn test_uuid_pattern_matching() { ... }

#[test]
fn test_section_extraction() { ... }

#[test]
fn test_relationship_parsing() { ... }
```

### Integration Tests

```bash
# Test import of CODEBASE_REVIEW.md
cargo test import_codebase_review

# Test dry run mode
cargo test import_dry_run

# Test error handling
cargo test import_errors
```

## Future Enhancements (Post-MVP)

### 1. Two-Way Sync

```bash
# Export entities to markdown
engram export --task-id xxx --file review.md

# Detect changes and update
engram import --file review.md --update
```

### 2. Template System

```bash
# Create from template
engram import --template review --file new_review.md
```

### 3. Bulk Import Directory

```bash
# Import all .emd files in directory
engram import --dir ./reviews/
```

### 4. Git Integration

```bash
# Import on git checkout
engram import --on-checkout review.md
```

## Files to Create/Modify

### New Files

- `src/cli/import.rs` - Main import command
- `src/import/mod.rs` - Import module
- `src/import/parser.rs` - YAML/markdown parsing
- `src/import/patterns.rs` - Pattern matching
- `tests/import_tests.rs` - Integration tests

### Modified Files

- `src/cli/mod.rs` - Add import module and command
- `src/main.rs` - Add import handler
- `Cargo.toml` - May need `serde_yaml` dependency (check if exists)

## Dependencies

**Already available:**
- `clap` - CLI parsing
- `serde` / `serde_json` - Serialization
- `serde_yaml` - Check if available
- `regex` - Pattern matching
- `tokio` - Async runtime
- `chrono` - Date/time

**May need:**
- `pulldown-cmark` - Better markdown parsing (optional, regex may suffice)

## Success Criteria

1. `engram import --file CODEBASE_REVIEW.md` imports all entities
2. Relationships created correctly (test with `engram relationship list`)
3. Dry run shows accurate preview
4. Error handling works (invalid UUID, missing entity)
5. Verbose mode shows progress
6. Tests pass (unit + integration)

## Timeline

- **Design:** Done
- **Phase 1 (Core):** 2-3 hours
- **Phase 2 (Entities):** 2-3 hours  
- **Phase 3 (Linking):** 2-3 hours
- **Phase 4 (Polish):** 1-2 hours
- **Testing:** 2 hours

**Total Estimate:** 10-14 hours
