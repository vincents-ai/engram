# Engram Codebase Review and Improvement Recommendations

**Date:** 2026-01-22
**Reviewer:** Sisyphus AI Agent
**Scope:** Full codebase analysis with focus on test failures, skills integration, and architecture quality

---

## Executive Summary

Engram is a well-architected distributed memory system for AI agents with a clean Rust implementation. The codebase demonstrates strong patterns for entity management, Git-based storage, and extensible architecture. However, several issues were identified that impact reliability and integration quality.

**Key Findings:**
- 5 failing integration tests (sync operations)
- 12 unused functions generating warnings
- Skills/prompts integration incomplete (compliance prompts missing)
- No natural language query capability for skills discovery

---

## 1. Test Failures Analysis

### Failing Tests (5 total)

#### 1.1 `test_branch_creation_and_switching`
**File:** `tests/sync_integration_tests.rs:83`
**Issue:** Expects error on duplicate branch creation, but behavior is inconsistent

```rust
create_branch("test-branch", Some("test-agent"), None)?;
let result = create_branch("test-branch", Some("test-agent"), None);
assert!(result.is_err());  // Line 83 - FAILS
```

**Root Cause:** The function `create_branch` at `src/cli/sync.rs:1135` does return an error for existing branches (lines 1175-1179), but the test might be running in a context where the error is not propagated correctly.

**Recommendation:** Verify error propagation in test environment. The code looks correct; test setup may have issues.

#### 1.2 `test_branch_listing`
**File:** `tests/sync_integration_tests.rs`
**Issue:** Repository not found errors

```
Error: Git("Failed to open repository: could not find repository at '/tmp/nix-shell.APRQeI/.tmpzN2tGN/.engram'; class=Repository (6), code=NotFound (-3)")
```

**Root Cause:** Temp directory cleanup timing issues. The `SyncTestFixture` creates a temp directory but it's being cleaned up before tests complete.

**Recommendation:**
```rust
// Fix: Keep temp_dir alive longer
impl SyncTestFixture {
    fn new() -> Result<Self, EngramError> {
        let temp_dir = TempDir::new().map_err(|e| EngramError::Io(e))?;
        // ... existing code ...
        
        // FIX: Keep temp_dir in scope longer by returning it
        Ok(SyncTestFixture {
            temp_dir,  // Ensure this is used, not dropped
            storage,
            repo_path: repo_path.to_string_lossy().to_string(),
        })
    }
}
```

#### 1.3 `test_authentication_credentials`
**File:** `tests/sync_integration_tests.rs:159`
**Issue:** Invalid auth type returns Ok(None) instead of error

```rust
let invalid_auth = RemoteAuth {
    auth_type: "invalid".to_string(),
    // ...
};
let result = create_credentials(&invalid_auth);
assert!(result.is_err());  // FAILS - returns Ok(None)
```

**Root Cause:** `src/cli/sync.rs:958` has catch-all pattern:
```rust
"none" | _ => Ok(None),  // Invalid types fall through to Ok(None)
```

**Recommendation:**
```rust
// Fix: Return error for unknown auth types
"none" => Ok(None),
_ => Err(EngramError::Validation(format!(
    "Unknown authentication type: {}. Valid types: ssh, http, https, none",
    auth.auth_type
))),
```

#### 1.4 `test_concurrent_branch_operations`
**File:** `tests/sync_integration_tests.rs`
**Issue:** Repository not found in concurrent operations

**Root Cause:** Race condition in temp directory handling with parallel test execution.

**Recommendation:**
```rust
// Fix: Use separate temp directories per test or run serially
#[tokio::test]
async fn test_concurrent_branch_operations() -> Result<(), EngramError> {
    // Each test gets isolated storage
    let temp_dir = TempDir::new()?;
    let storage = create_isolated_storage(&temp_dir)?;
    // ...
}
```

#### 1.5 `test_multi_agent_branch_isolation`
**File:** `tests/sync_integration_tests.rs`
**Issue:** Branch not found when switching between agents

**Root Cause:** Branch isolation not working correctly when switching branches sequentially.

---

## 2. Code Quality Issues

### 2.1 Unused Functions (12 warnings)

**File:** `src/locus_cli/`
**Issue:** Multiple functions defined but never called

```
handle_govern_command - never used
handle_policy_command - never used
handle_quality_gate_command - never used
handle_compliance_command - never used
handle_override_command - never used
handle_emergency_command - never used
handle_template_command - never used
handle_visualize_command - never used
handle_workflow_command - never used
generate_workflow_template - never used
```

**Recommendation:** Either implement these commands or remove the dead code.

### 2.2 Unused Imports

**File:** `src/locus_main.rs`
```rust
use engram::Config;  // unused
use std::io;          // unused
```

**Recommendation:** Remove unused imports to clean up compilation.

---

## 3. Skills and Prompts Integration Gaps

### 3.1 Current State
- ✅ 5 skills integrated in `./engram/skills/`
- ✅ 170 agent prompts in `./engram/prompts/agents/`
- ✅ 103 pipeline templates in `./engram/prompts/ai/pipelines/`
- ❌ 164 compliance prompts NOT integrated in `./engram/prompts/compliance/`
- ❌ No engram entities created for compliance prompts

### 3.2 Missing Features

#### 3.2.1 Natural Language Skills Discovery
**Current:** No way to query "What skills are available for planning?"
**Desired:** `engram ask "What skills help with feature planning?"`

**Recommendation:** Implement NLQ integration for skills:

```rust
// src/nlq/skills_handler.rs
pub async fn handle_skill_query(query: &str) -> Result<NLQResponse, EngramError> {
    // Parse natural language for skill requests
    // Query engram context for matching skills
    // Return formatted skill recommendations
}
```

#### 3.2.2 Cross-Reference Relationships
**Current:** Skills exist in isolation
**Desired:** 
- `plan-feature` → contains → `delegate-to-agents`
- `plan-feature` → contains → `audit-trail`
- `delegate-to-agents` → references → [relevant agents]

**Recommendation:** Add relationship creation when skills are used:

```rust
// When using plan-feature skill, automatically create relationships
engram relationship create \
    --source-id plan-feature-task \
    --target-id delegate-to-agents-skill \
    --relationship-type contains
```

#### 3.2.3 Skill Template for Auto-Registration
**Current:** Skills are markdown files, not engram entities
**Desired:** Skills auto-register themselves when loaded

**Recommendation:** Create skill registration format:

```yaml
# skill.yaml
name: plan-feature
description: Plan feature implementation using pipeline templates
engram_entities:
  - type: context
    title: "Skill: Plan Feature"
    tags: [workflow, planning]
  - type: task
    title: "Use Plan Feature Skill"
```

---

## 4. Architecture Improvements

### 4.1 Entity System Enhancements

#### 4.1.1 Skill Entity Type
**Current:** Skills are markdown files, not first-class entities
**Proposed:** Add dedicated Skill entity type

```rust
// src/entities/skill.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: SkillCategory,
    pub content: String,
    pub tags: Vec<String>,
    pub engram_patterns: Vec<EngramPattern>,
    pub examples: Vec<SkillExample>,
}

pub enum SkillCategory {
    Meta,
    Workflow,
    Compliance,
    Domain,
}
```

#### 4.1.2 Prompt Entity Type
**Current:** Prompts are YAML files, not queryable
**Proposed:** Add Prompt entity for discovery

```rust
// src/entities/prompt.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prompt {
    pub id: String,
    pub name: String,
    pub agent_type: String,
    pub template: String,
    pub engram_parameters: Vec<EngramParameter>,
    pub usage_examples: Vec<String>,
}
```

### 4.2 CLI Improvements

#### 4.2.1 Skills Commands
```bash
# List all skills
engram skill list

# Show skill details
engram skill show plan-feature

# Search skills
engram skill search "planning"

# Use skill (creates engram entities)
engram skill use plan-feature --task-id xxx
```

#### 4.2.2 Prompts Commands
```bash
# List all prompts
engram prompt list

# Search prompts
engram prompt search "authentication"

# Show prompt
engram prompt show 01-the-one
```

### 4.3 Storage Enhancements

#### 4.3.1 Skills Index
**Current:** Skills stored as files, no indexing
**Proposed:** Create skills index in engram storage

```rust
// Automatically index skills on startup
pub fn index_skills(storage: &mut GitRefsStorage) -> Result<(), EngramError> {
    let skills_dir = Path::new("./engram/skills");
    for entry in walkdir::WalkDir::new(skills_dir) {
        if entry?.file_type().is_file() {
            let skill = parse_skill_file(entry.path())?;
            storage.store(&skill.to_entity())?;
        }
    }
    Ok(())
}
```

---

## 5. Prioritized Improvements

### P0 - Critical (Fix Immediately)

1. **Fix Test Failures**
   - [ ] Correct `create_credentials` to reject unknown auth types
   - [ ] Fix temp directory handling in sync tests
   - [ ] Verify branch creation error handling

2. **Complete Compliance Integration**
   - [ ] Create engram entities for 164 compliance prompts
   - [ ] Link compliance entities to tracking task

### P1 - High Priority (This Sprint)

3. **Add Skills Discovery**
   - [ ] Implement `engram ask "What skills...?"` for skills
   - [ ] Create skill search functionality

4. **Remove Dead Code**
   - [ ] Remove 12 unused functions from locus_cli
   - [ ] Remove unused imports from locus_main.rs

### P2 - Medium Priority (Next Sprint)

5. **Skill Auto-Registration**
   - [ ] Create skill.yaml format for auto-registration
   - [ ] Implement skills index on startup

6. **Cross-References**
   - [ ] Link related skills automatically
   - [ ] Add relationship suggestions when using skills

### P3 - Lower Priority (Backlog)

7. **Dedicated Entity Types**
   - [ ] Create Skill entity type
   - [ ] Create Prompt entity type
   - [ ] Add skill/prompt CLI commands

---

## 6. Files Requiring Changes

### Immediate Fixes

| File | Change |
|------|--------|
| `src/cli/sync.rs:958` | Return error for unknown auth types |
| `tests/sync_integration_tests.rs` | Fix temp directory handling |
| `./engram/prompts/compliance/` | Create entities for compliance prompts |

### New Files to Create

| File | Purpose |
|------|---------|
| `src/cli/skill.rs` | Skills CLI commands |
| `src/cli/prompt.rs` | Prompts CLI commands |
| `src/nlq/skills_handler.rs` | NLQ handler for skills |
| `src/entities/skill.rs` | Skill entity type |
| `src/entities/prompt.rs` | Prompt entity type |

### Files to Clean Up

| File | Change |
|------|--------|
| `src/locus_cli/govern.rs` | Remove unused functions or implement |
| `src/locus_cli/override_cmd.rs` | Remove unused functions or implement |
| `src/locus_cli/template.rs` | Remove unused functions or implement |
| `src/locus_cli/visualize.rs` | Remove unused functions or implement |
| `src/locus_cli/workflow.rs` | Remove unused functions or implement |
| `src/locus_main.rs` | Remove unused imports |

---

## 7. Testing Recommendations

### 7.1 Add Integration Tests for Skills

```rust
#[tokio::test]
async fn test_skill_discovery() {
    // Test engram ask "What skills are available?"
    let response = engram_ask("What skills are available for planning?").await;
    assert!(response.contains("plan-feature"));
}

#[tokio::test]
async fn test_skill_usage_creates_entities() {
    // Test that using a skill creates engram entities
    let task_id = create_task("Test task").await;
    use_skill("plan-feature", task_id).await;
    
    let relationships = get_relationships(task_id);
    assert!(relationships.contains_skill("plan-feature"));
}
```

### 7.2 Add Compliance Prompt Tests

```rust
#[tokio::test]
async fn test_compliance_prompts_indexed() {
    let prompts = list_compliance_prompts().await;
    assert!(prompts.len() >= 164);  // All compliance prompts indexed
}
```

---

## 8. Conclusion

Engram demonstrates solid architecture with clean Rust code patterns. The main areas needing attention are:

1. **Test Reliability** - 5 failing tests need fixes
2. **Dead Code** - 12 unused functions clutter the codebase
3. **Skills Integration** - Compliance prompts missing, no skills discovery
4. **Architecture Evolution** - Consider dedicated Skill/Prompt entity types

The recommendations above provide a clear path to improve code quality, complete the skills integration, and establish a foundation for future enhancements.

---

**Reviewed by:** Sisyphus
**Date:** 2026-01-22
**Next Review:** After P0 fixes are applied
