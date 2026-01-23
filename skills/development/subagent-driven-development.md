---
name: subagent-driven-development-engram
description: "Engram-integrated version. Use when executing implementation plans - tracks agent assignment and reviews via engram task.agent field and context entities."
---

# Subagent-Driven Development (Engram-Integrated)

## Overview

Execute plan by dispatching fresh subagent per task, with two-stage review after each: spec compliance review first, then code quality review. Track everything via engram.

## Key Changes from Original

**Original:** Track in conversation, TodoWrite
**Engram-integrated:** Track agent assignment via `engram task --agent`, store reviews as context entities, use `engram relationship` for task relationships.

## The Process with Engram

### Step 1: Read Plan and Create Task Hierarchy

```bash
# Get parent task (from writing-plans-engram)
PARENT_TASK_ID="[PARENT_TASK_ID]"

# Get all subtasks
engram task list --parent $PARENT_TASK_ID --json | jq -r '.[].id' > /tmp/subtasks.txt

# Create TodoWrite with engram task IDs
while read SUBTASK_ID; do
  SUBTASK_TITLE=$(engram task show $SUBTASK_ID --json | jq -r '.data.title')
  echo "Task: $SUBTASK_TITLE (ID: $SUBTASK_ID)"
done < /tmp/subtasks.txt
```

### Step 2: Dispatch Implementer Subagent

**Update task status and assign agent:**

```bash
# Mark task as in progress and assign agent
engram task update [SUBTASK_ID] \
  --status in_progress \
  --agent implementer-subagent

# Create reasoning for dispatch
engram reasoning create \
  --title "[Task] Implementer Subagent Dispatched" \
  --task-id [SUBTASK_ID] \
  --content "**Agent:** implementer-subagent\n**Started:** [timestamp]\n**Subtask:** [task title]\n\n**Context provided:**\n- Full task text from engram task show\n- Related design context from engram relationship connected\n- Previous subtask results (if any)" \
  --confidence 1.0 \
  --tags "subagent,dispatch,implementer"
```

**Get full task context for subagent:**

```bash
# Get task details
engram task show [SUBTASK_ID] --json | jq '.data'

# Get related context (design docs)
engram relationship connected --entity-id [SUBTASK_ID] --relationship-type references

# Get step details (reasoning)
engram reasoning list --task-id [SUBTASK_ID]
```

### Step 3: Two-Stage Review with Engram

#### Spec Compliance Review

```bash
# After implementer completes, dispatch spec reviewer
engram task update [SUBTASK_ID] \
  --status pending_review_spec \
  --agent spec-reviewer

# Create spec review as context entity
engram context create \
  --title "[Task] Spec Compliance Review" \
  --content "**Reviewer:** spec-reviewer\n**Completed:** [timestamp]\n\n**Requirements from plan:**\n- [Requirement 1]\n- [Requirement 2]\n\n**Changes implemented:**\n\`\`\`diff\n[git diff or file changes]\n\`\`\`\n\n**Compliance Check:**\n✅ [Requirement met]\n✅ [Requirement met]\n❌ [Requirement missing]: [detail]\n\n**Review Result:** PASSED / FAILED WITH ISSUES" \
  --source "spec-review" \
  --tags "review,spec-compliance,[task-tag]"
```

**If spec review fails:**

```bash
# Update task back to in_progress
engram task update [SUBTASK_ID] \
  --status in_progress \
  --agent implementer-subagent

# Create reasoning for issues found
engram reasoning create \
  --title "[Task] Spec Review Issues" \
  --task-id [SUBTASK_ID] \
  --content "**Issues found:**\n- [Issue 1]: [detail]\n- [Issue 2]: [detail]\n\n**Required fixes:**\n1. [Fix 1]\n2. [Fix 2]\n\n**Re-review required after fixes" \
  --confidence 0.9 \
  --tags "subagent,spec-review-issues"
```

#### Code Quality Review

```bash
# After spec compliance passes, dispatch code quality reviewer
engram task update [SUBTASK_ID] \
  --status pending_review_quality \
  --agent code-quality-reviewer

# Create code quality review as context entity
engram context create \
  --title "[Task] Code Quality Review" \
  --content "**Reviewer:** code-quality-reviewer\n**Completed:** [timestamp]\n\n**Code Quality Check:**\n✅ Tests pass\n✅ No type errors\n✅ Error handling complete\n✅ No hardcoded secrets\n✅ Code follows project patterns\n\n**Strengths:**\n- [Positive 1]\n- [Positive 2]\n\n**Issues Found:**\n- [Critical]: [detail] - MUST FIX\n- [Major]: [detail] - SHOULD FIX\n- [Minor]: [detail] - NICE TO HAVE\n\n**Review Result:** APPROVED / CHANGES REQUESTED" \
  --source "code-quality-review" \
  --tags "review,code-quality,[task-tag]"
```

**If quality review fails:**

```bash
# Update task back to in_progress
engram task update [SUBTASK_ID] \
  --status in_progress \
  --agent implementer-subagent

# Create reasoning for quality issues
engram reasoning create \
  --title "[Task] Code Quality Issues" \
  --task-id [SUBTASK_ID] \
  --content "**Quality issues found:**\n- [Critical]: [detail] - MUST FIX\n- [Major]: [detail] - SHOULD FIX\n\n**Required fixes:**\n1. [Fix 1]\n2. [Fix 2]\n\n**Re-review required after fixes" \
  --confidence 0.9 \
  --tags "subagent,quality-review-issues"
```

### Step 4: Mark Task Complete

```bash
# After both reviews pass
engram task update [SUBTASK_ID] \
  --status done \
  --outcome "Passed spec compliance and code quality review"

# Create reasoning for completion
engram reasoning create \
  --title "[Task] Subagent Completion Report" \
  --task-id [SUBTASK_ID] \
  --content "**Completed by:** implementer-subagent\n**Duration:** [start to end time]\n\n**Changes committed:**\n\`\`\`\n[git log --oneline]\n\`\`\`\n\n**Spec review:** ✅ PASSED\n**Code quality review:** ✅ PASSED\n**Tests:** ✅ All pass\n\n**Files created/modified:**\n- \`[file]\`: [change]\n- \`[file]\`: [change]" \
  --confidence 1.0 \
  --tags "subagent,completion,[task-tag]"
```

### Step 5: Check for More Tasks

```bash
# Check if more subtasks remain
REMAINING=$(engram task list --parent $PARENT_TASK_ID --status pending --json | jq 'length')

if [ $REMAINING -gt 0 ]; then
  # Get next pending task
  NEXT_TASK=$(engram task list --parent $PARENT_TASK_ID --status pending --json | jq -r '.[0].id')
  
  # Dispatch next implementer
  engram task update $NEXT_TASK --status in_progress --agent implementer-subagent
else
  # All tasks complete - final review
  echo "All subtasks complete. Dispatching final code reviewer."
fi
```

### Step 6: Final Review (After All Tasks)

```bash
# Create final review context
engram context create \
  --title "[Feature] Final Code Review" \
  --content "**Reviewer:** final-code-reviewer\n**Completed:** [timestamp]\n\n**All subtasks completed:**\n- [x] Task 1: [title] - ✅\n- [x] Task 2: [title] - ✅\n- [x] Task 3: [title] - ✅\n\n**Overall assessment:**\n✅ All requirements met\n✅ Code quality consistent\n✅ Tests pass\n✅ No regressions\n\n**Ready for:** superpowers:finishing-a-development-branch" \
  --source "final-review" \
  --tags "review,final,[feature-tag]"
```

## Engram Integration Summary

| Original Tracking | Engram Integration |
|-------------------|-------------------|
| TodoWrite status | `engram task update --status` |
| Agent assignment | `engram task --agent` |
| Review notes | engram context entities |
| Implementation history | engram reasoning entities |
| Task relationships | `engram relationship create` |
| Completion evidence | engram reasoning + git commit |

## Querying Subagent Progress

```bash
# Get all in-progress tasks
engram task list --status in_progress

# Get all tasks by agent
engram task list --agent implementer-subagent

# Get review status for parent task
engram relationship connected --entity-id [PARENT_TASK_ID] --relationship-type contains

# Get all reviews for a feature
engram context list | grep -E "review|[feature-tag]"
```

## Example Workflow

```bash
# Plan: Implement User Auth API (5 subtasks)
PARENT="task-uuid-for-auth-api"

# Task 1: Login endpoint
engram task update "login-subtask" --status in_progress --agent implementer-subagent
engram reasoning create --title "Auth Task 1: Dispatched" --task-id "login-subtask" \
  --content "Agent: implementer-subagent" --confidence 1.0 --tags "subagent,auth"

# Implementer works...

# Spec review
engram task update "login-subtask" --status pending_review_spec --agent spec-reviewer
engram context create --title "Auth Task 1: Spec Review" \
  --content "Review: PASSED - all requirements met" \
  --source "spec-review" --tags "review,auth"

# Code quality review
engram task update "login-subtask" --status pending_review_quality --agent code-quality-reviewer
engram context create --title "Auth Task 1: Code Quality Review" \
  --content "Review: APPROVED - clean code" \
  --source "code-quality-review" --tags "review,auth"

# Complete
engram task update "login-subtask" --status done --outcome "Approved"
engram reasoning create --title "Auth Task 1: Complete" --task-id "login-subtask" \
  --content "Commits: 2, Tests: 5/5 pass" --confidence 1.0 --tags "subagent,auth"

# Task 2: Register endpoint
# ... same pattern

# After all tasks: Final review
engram context create --title "Auth API: Final Review" \
  --content "All 5 tasks complete and approved" \
  --source "final-review" --tags "review,final,auth"

Agent: "All tasks complete. Ready for finishing-a-development-branch."
```

## Related Skills

This skill integrates with:
- `engram-delegate-to-agents` - Core delegation pattern
- `engram-dispatching-parallel-agents` - Parallel task execution
- `engram-plan-feature` - Break features into delegable tasks
- `engram-use-memory` - Track multi-agent workflow progress
- `engram-audit-trail` - Complete record of agent work
- `engram-requesting-code-review` - Final review before completion
