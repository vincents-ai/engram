---
name: requesting-code-review-engram
description: "Engram-integrated version. Use when completing tasks or before merging - creates engram compliance entities for review requirements and context entities for review results."
---

# Requesting Code Review (Engram-Integrated)

## Overview

Dispatch code-reviewer subagent to catch issues before they cascade. Store all review artifacts in engram for audit trail and compliance tracking.

**Core principle:** Review early, review often. Everything documented in engram.

## Key Changes from Original

**Original:** Review in conversation, template-based dispatch
**Engram-integrated:** Creates engram compliance entities for review requirements, context entities for review results, reasoning entities for feedback tracking.

## When to Request Review with Engram

**Mandatory:**
- After each task in subagent-driven development
- After completing major feature
- Before merge to main
- After `engram validate check` passes

**Optional but valuable:**
- When stuck (fresh perspective)
- Before refactoring (baseline check)
- After fixing complex bug

## The Engram Review Pattern

### Step 1: Get Review Context from Engram

```bash
# Get task details
TASK_ID="[TASK_ID]"
TASK_TITLE=$(engram task show $TASK_ID --json | jq -r '.data.title')
TASK_DESC=$(engram task show $TASK_ID --json | jq -r '.data.description')

# Get related context (design docs, requirements)
engram relationship connected --entity-id $TASK_ID --relationship-type references

# Get changes (git diff)
BASE_SHA=$(git log --oneline -2 | tail -1 | awk '{print $1}')
HEAD_SHA=$(git rev-parse HEAD)
DIFF=$(git diff $BASE_SHA..$HEAD_SHA)
```

### Step 2: Create Compliance Entity for Review Requirements

```bash
# Create compliance entity for review checklist
engram compliance create \
  --title "Code Review: $TASK_TITLE" \
  --category code_review \
  --requirements "✅ Tests pass (verified via engram validate check)
✅ Code follows project patterns
✅ No hardcoded secrets
✅ Error handling complete
✅ Documentation updated
✅ No debug code left
✅ Edge cases handled
✅ Performance acceptable
✅ Security considerations met" \
  --tags "code-review,[task-tag]"
```

### Step 3: Create Review Context Entity

```bash
# Create context entity with review details
engram context create \
  --title "Code Review Context: $TASK_TITLE" \
  --content "## Task Information\n**ID:** $TASK_ID\n**Title:** $TASK_TITLE\n\n## Changes Reviewed\n**Base SHA:** $BASE_SHA\n**Head SHA:** $HEAD_SHA\n\n**Files changed:**\n\`\`\`\n$(git diff --stat $BASE_SHA..$HEAD_SHA)\n\`\`\`\n\n**Diff summary:**\n$DIFF\n\n## Requirements from Plan\n$(engram relationship connected --entity-id $TASK_ID --relationship-type references | grep -E "context|reasoning" | while read id; do engram context show $id --json | jq -r '.data.content'; done)\n\n## Review Checklist\nSee compliance entity for requirements." \
  --source "code-review" \
  --tags "code-review,context,[task-tag]"
```

### Step 4: Dispatch Code Reviewer Subagent

**Update task status for review:**

```bash
# Mark task as pending review
engram task update $TASK_ID \
  --status pending_review \
  --agent code-reviewer

# Create reasoning for review dispatch
engram reasoning create \
  --title "[Task] Code Review Dispatched" \
  --task-id $TASK_ID \
  --content "**Reviewer:** code-reviewer subagent\n**Dispatched:** [timestamp]\n\n**Context provided:**\n- Task details from engram task show\n- Design context from engram relationship connected\n- Git diff from $BASE_SHA to $HEAD_SHA\n\n**Review focus:**\n1. Spec compliance (matches plan requirements)\n2. Code quality (follows patterns, clean code)\n3. Test coverage (tests pass, edge cases covered)\n4. Security (no secrets, proper validation)" \
  --confidence 1.0 \
  --tags "code-review,dispatch,[task-tag]"
```

### Step 5: Receive and Process Review

**After reviewer completes, create review result context:**

```bash
# Create context entity for review results
engram context create \
  --title "Code Review Result: $TASK_TITLE" \
  --content "## Review Summary\n**Completed:** [timestamp]\n**Reviewer:** code-reviewer\n\n### Critical Issues\n- [ ] [None / Issue description]\n\n### Important Issues\n- [ ] [None / Issue 1]\n- [ ] [None / Issue 2]\n\n### Minor Issues\n- [ ] [None / Issue 1]\n- [ ] [None / Issue 2]\n\n### Strengths\n- [Positive 1]\n- [Positive 2]\n\n### Assessment\n✅ **APPROVED** - Ready to proceed\n⚠️ **CONDITIONAL** - Fix important issues before proceeding\n❌ **BLOCKED** - Critical issues must be fixed\n\n## Review Details\n\n### Changes Reviewed\n\`\`\`diff\n$DIFF\n\`\`\`\n\n### Code Quality Notes\n- [Quality observation 1]\n- [Quality observation 2]\n\n### Test Coverage Notes\n- [Test observation 1]\n- [Test observation 2]\n\n### Security Notes\n- [Security observation 1]\n- [Security observation 2]" \
  --source "code-review-result" \
  --tags "code-review,result,[task-tag]"
```

### Step 6: Act on Feedback

**If issues found, create reasoning for tracking:**

```bash
# If there are issues to fix
engram reasoning create \
  --title "[Task] Review Feedback: Issues to Address" \
  --task-id $TASK_ID \
  --content "## Critical Issues (Fix Immediately)\n- [Issue 1]: [detail] → [fix plan]\n\n## Important Issues (Fix Before Proceeding)\n- [Issue 1]: [detail] → [fix plan]\n- [Issue 2]: [detail] → [fix plan]\n\n## Minor Issues (Note for Later)\n- [Issue 1]: [detail]\n\n## Decision\n[Fix critical → re-review / Fix important → continue / Note minor → proceed]" \
  --confidence 0.9 \
  --tags "code-review,feedback,[task-tag]"
```

**If approved, update task:**

```bash
# Mark task as reviewed and approved
engram task update $TASK_ID \
  --status reviewed \
  --outcome "Approved - no critical issues, 0 important issues, 0 minor issues"

# Create completion reasoning
engram reasoning create \
  --title "[Task] Code Review Complete" \
  --task-id $TASK_ID \
  --content "**Review status:** ✅ APPROVED\n**Reviewed by:** code-reviewer subagent\n**Review context:** [context entity ID]\n**Review result:** [result entity ID]\n\n**Issues found:**\n- Critical: 0\n- Important: 0\n- Minor: 0\n\n**Next step:** Proceed to next task / Create PR / Merge" \
  --confidence 1.0 \
  --tags "code-review,complete,[task-tag]"
```

### Step 7: Update Compliance Entity

```bash
# Update compliance with actual review results
engram compliance update [COMPLIANCE_ID] \
  --violations "Review passed - no violations" \
  --remediation "None required"
```

## Integration with Engram Workflows

### Subagent-Driven Development

After each task in subagent-driven development:
1. Task completes → `engram task update --status done`
2. Immediately dispatch code review → `requesting-code-review-engram`
3. Fix issues if found
4. Only then proceed to next task

```bash
# In subagent-driven workflow:
engram task update [SUBTASK] --status done --outcome "Implementation complete"
engram compliance create --title "Code Review: [Subtask]" --category code_review
# ... dispatch review ...
engram task update [SUBTASK] --status reviewed --outcome "Approved"
# Now safe to proceed to next task
```

### Writing Plans

Before creating implementation plan:
1. Create compliance entity for review requirements
2. Include review checklist in plan
3. After implementation, enforce review before marking done

### Dispatching Parallel Agents

After parallel agents complete:
1. Create compliance entity for final integration review
2. Review all changes together
3. Ensure no conflicts introduced

```bash
# After parallel execution complete
engram compliance create \
  --title "Integration Review: [Feature]" \
  --category code_review \
  --requirements "✅ All domain tasks reviewed\n✅ No conflicts between changes\n✅ Full test suite passes\n✅ Integration tests pass"
```

## Querying Review History

```bash
# Get all reviews for a task
engram relationship connected --entity-id [TASK_ID] | grep -E "context|reasoning" | grep code-review

# Get all compliance entities for code review
engram compliance list | grep code_review

# Get pending reviews
engram task list --status pending_review

# Get review results for a feature
engram context list | grep -E "code-review.*result|[feature-tag]"

# Search for review feedback
engram reasoning list | grep "Review Feedback"
```

## Example Workflow

```bash
# Task: Implement login endpoint (from writing-plans)

# Step 1: Get context
TASK=$(engram task show login-subtask --json | jq -r '.id')
DIFF=$(git diff HEAD~1..HEAD)
BASE_SHA=$(git rev-parse HEAD~1)
HEAD_SHA=$(git rev-parse HEAD)

# Step 2: Create compliance
COMPLIANCE=$(engram compliance create \
  --title "Code Review: Login Endpoint Implementation" \
  --category code_review \
  --requirements "✅ Tests pass\n✅ Error handling complete\n✅ No hardcoded secrets\n✅ Follows auth patterns" \
  --tags "code-review,login,auth" \
  --json | jq -r '.id')

# Step 3: Create review context
engram context create \
  --title "Code Review Context: Login Endpoint" \
  --content "Task: Implement login endpoint\nDiff: $DIFF" \
  --source "code-review" \
  --tags "code-review,login"

# Step 4: Dispatch reviewer
engram task update $TASK --status pending_review --agent code-reviewer
engram reasoning create \
  --title "Login Task: Review Dispatched" \
  --task-id $TASK \
  --content "Reviewer: code-reviewer" \
  --confidence 1.0 \
  --tags "code-review,dispatch"

# Step 5: Receive review (after reviewer completes)
engram context create \
  --title "Code Review Result: Login Endpoint" \
  --content "Critical: 0\nImportant: 1 (missing rate limiting)\nMinor: 2 (style nits)\nAssessment: CONDITIONAL - fix rate limiting" \
  --source "code-review-result" \
  --tags "code-review,result,login"

# Step 6: Act on feedback
engram reasoning create \
  --title "Login Task: Review Feedback" \
  --task-id $TASK \
  --content "Issue: Missing rate limiting\nFix: Add Redis-based rate limiter\nStatus: WILL FIX" \
  --confidence 0.9 \
  --tags "code-review,feedback"

# Fix rate limiting...

# Step 7: Re-review and approve
engram context create \
  --title "Code Review Result: Login Endpoint (Re-review)" \
  --content "Critical: 0\nImportant: 0 (rate limiting added)\nMinor: 0 (style nits fixed)\nAssessment: APPROVED" \
  --source "code-review-result" \
  --tags "code-review,result,login,approved"

engram task update $TASK --status reviewed --outcome "Approved after fixing rate limiting"
engram reasoning create \
  --title "Login Task: Review Complete" \
  --task-id $TASK \
  --content "Status: APPROVED\nIssues: 1 important (fixed), 0 remaining" \
  --confidence 1.0 \
  --tags "code-review,complete"

echo "Review complete. Ready for next task."
```

## Engram Integration Summary

| Original Tracking | Engram Integration |
|-------------------|-------------------|
| Review template | engram context entity |
| Review checklist | engram compliance entity |
| Review results | engram context entity (result) |
| Feedback tracking | engram reasoning entity |
| Task status | `engram task update --status reviewed` |
| Audit trail | Full engram history |

## Benefits of Engram Integration

1. **Compliance Tracking:** `engram compliance` enforces review requirements
2. **Queryable History:** `engram context list | grep code-review` shows all reviews
3. **Issue Tracking:** `engram reasoning list | grep feedback` shows all feedback
4. **Integration with TDD:** Review after `engram validate check` passes
5. **Multi-Agent Support:** Review context available to all agents
6. **Audit Trail:** Complete review history for team learning

## Related Skills

This skill integrates with:
- `engram-use-memory` - Store review feedback for learning
- `engram-audit-trail` - Track review process and outcomes
- `engram-test-driven-development` - Review after tests pass
- `engram-check-compliance` - Code review for compliance requirements
- `engram-subagent-driven-development` - Reviews between agent tasks
