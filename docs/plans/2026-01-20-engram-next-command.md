# Engram Next Command & Prompt Generation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement `engram next` command to auto-select tasks and generate context-aware prompts based on workflow state.

**Architecture:** 
Extend `WorkflowState` to include prompt templates. Implement `engram next` CLI command to find the highest priority task, load its workflow state, interpolate variables into the template, and output the result.

**Tech Stack:** Rust, Clap (CLI), Serde (JSON/Serialization), Regex (Interpolation)

### Task 1: Extend Workflow Entity

**Files:**
- Modify: `src/entities/workflow.rs`

**Step 1: Write the failing test**

Create a new test file `tests/unit/workflow_prompt_test.rs` (or add to existing workflow tests if preferred, but separate is cleaner for TDD).

```rust
#[test]
fn test_workflow_state_with_prompts() {
    let json = r#"{
        "id": "state-1",
        "name": "Review",
        "state_type": "review",
        "description": "Code review",
        "is_final": false,
        "prompts": {
            "system": "You are a reviewer",
            "user": "Review task {{TASK_ID}}"
        }
    }"#;
    
    let state: WorkflowState = serde_json::from_str(json).unwrap();
    assert_eq!(state.prompts.unwrap().system.unwrap(), "You are a reviewer");
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test --test unit` (or specific test path)
Expected: FAIL (field `prompts` does not exist)

**Step 3: Modify `WorkflowState` struct**

In `src/entities/workflow.rs`:

```rust
// Add PromptTemplate struct
#[derive(Debug, Clone, Serialize, Deserialize, Validate, PartialEq)]
pub struct PromptTemplate {
    #[serde(rename = "system", skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

// Update WorkflowState
pub struct WorkflowState {
    // ... existing fields
    #[serde(rename = "prompts", skip_serializing_if = "Option::is_none")]
    pub prompts: Option<PromptTemplate>,
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test`
Expected: PASS

**Step 5: Commit**

```bash
git add src/entities/workflow.rs tests/unit/workflow_prompt_test.rs
git commit -m "feat: add prompt templates to workflow state [TASK_ID]"
```

### Task 2: Implement Prompt Interpolation Logic

**Files:**
- Create: `src/cli/next.rs` (Module structure)
- Modify: `src/cli/mod.rs` (Register module)

**Step 1: Write the failing test**

In `src/cli/next.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpolation() {
        let template = "Hello {{AGENT_NAME}}, working on {{TASK_ID}}";
        let mut context = HashMap::new();
        context.insert("AGENT_NAME".to_string(), "Alice".to_string());
        context.insert("TASK_ID".to_string(), "123".to_string());
        
        let result = interpolate(template, &context);
        assert_eq!(result, "Hello Alice, working on 123");
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test`
Expected: FAIL (function not found)

**Step 3: Implement `interpolate` function**

In `src/cli/next.rs`:

```rust
pub fn interpolate(template: &str, context: &HashMap<String, String>) -> String {
    let mut result = template.to_string();
    for (key, value) in context {
        let placeholder = format!("{{{{{}}}}}", key); // {{KEY}}
        result = result.replace(&placeholder, value);
    }
    result
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test`
Expected: PASS

**Step 5: Commit**

```bash
git add src/cli/next.rs
git commit -m "feat: implement prompt interpolation logic [TASK_ID]"
```

### Task 3: Implement Task Selection Logic

**Files:**
- Modify: `src/cli/next.rs`

**Step 1: Write the failing test**

In `src/cli/next.rs`:

```rust
#[test]
fn test_select_next_task() {
    // Mock storage with 3 tasks: 
    // 1. High Priority, Todo
    // 2. Medium Priority, InProgress (Should be selected first)
    // 3. Low Priority, Done
    
    // logic should prefer InProgress > Todo (High > Medium > Low)
}
```

**Step 2: Run test to verify it fails**

Expected: FAIL

**Step 3: Implement `find_next_task`**

In `src/cli/next.rs`:

```rust
pub fn find_next_task<S: Storage>(storage: &S, agent: &str) -> Result<Option<Task>, EngramError> {
    let tasks = storage.query_by_agent(agent, Some("task"))?;
    // Filter and sort logic:
    // 1. Filter out Done/Cancelled
    // 2. Sort by: Status (InProgress > Todo), then Priority (Critical > ... > Low)
    // 3. Return first
}
```

**Step 4: Run test to verify it passes**

Expected: PASS

**Step 5: Commit**

```bash
git add src/cli/next.rs
git commit -m "feat: implement task selection logic [TASK_ID]"
```

### Task 4: Implement `engram next` Command

**Files:**
- Modify: `src/cli/mod.rs` (Add command variant)
- Modify: `src/cli/next.rs` (Implement command handler)
- Modify: `src/main.rs` (if necessary for dispatch)

**Step 1: Update CLI Enum**

In `src/cli/mod.rs`:

```rust
pub enum Commands {
    // ... existing
    /// Get next task and generate prompt
    Next {
        /// Optional specific task ID
        #[arg(long, short)]
        id: Option<String>,
        
        /// Output format (markdown, json)
        #[arg(long, default_value = "markdown")]
        format: String,
    },
}
```

**Step 2: Implement Command Handler**

In `src/cli/next.rs`:

```rust
pub fn handle_next_command<S: Storage>(storage: &mut S, id: Option<String>, format: String) -> Result<(), EngramError> {
    // 1. Identify Task (ID or find_next_task)
    // 2. Load Task Entity
    // 3. Load associated Workflow & State (if any)
    // 4. Build Context HashMap
    // 5. Interpolate Prompts (use Workflow prompts or Default fallback)
    // 6. Output formatted string
}
```

**Step 3: Commit**

```bash
git add src/cli/mod.rs src/cli/next.rs
git commit -m "feat: implement engram next command [TASK_ID]"
```
