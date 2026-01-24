---
name: engram-systematic-debugging
description: "Engram-integrated version. Use when encountering bugs - stores investigation in engram reasoning chains for audit trail and pattern analysis."
---

# Systematic Debugging (Engram-Integrated)

## Overview

Random fixes waste time and create new bugs. Quick patches mask underlying issues. Store all investigation in engram reasoning chains for persistent audit trail.

## Key Changes from Original

**Original:** Investigation in conversation, potentially lost
**Engram-integrated:** All investigation phases stored as reasoning entities in a chain, queryable and reviewable.

## The Iron Law

```
NO FIXES WITHOUT ROOT CAUSE INVESTIGATION FIRST
```

If you haven't completed Phase 1, you cannot propose fixes.

## The Four Phases with Engram

### Phase 1: Root Cause Investigation

BEFORE attempting ANY fix:

**Create reasoning entity for investigation:**

```bash
engram reasoning create \
  --title "[Issue] Investigation: Root Cause Analysis" \
  --task-id [TASK_ID] \
  --content "## Issue Description\n[Brief description of the bug/test failure/problem]\n\n## Error Message\n\`\`\`\n[Full error message and stack trace]\n\`\`\`\n\n## Steps to Reproduce\n1. [Step 1]\n2. [Step 2]\n3. [Step N]\n\n## Initial Observations\n- [Observation 1]\n- [Observation 2]\n- [Observation 3]\n\n## Files Examined\n- \`[file/path.rs]\`: [Finding]\n- \`[file/path.py]\`: [Finding]\n\n## Status:** IN_PROGRESS - Gathering evidence" \
  --confidence 0.5 \
  --tags "debugging,investigation,[issue-tag]"
```

**Step 1: Read Error Messages Carefully**

```bash
# Create reasoning for error analysis
engram reasoning create \
  --title "[Issue] Error Message Analysis" \
  --task-id [TASK_ID] \
  --content "**Error type:** [ErrorType]\n\n**Stack trace:**\n\`\`\`\n[Full stack trace]\n\`\`\`\n\n**Key findings from stack trace:**\n- Line [N]: \`[code]\` - [meaning]\n- File: \`[path]\` - [meaning]\n\n**Error codes/messages:**\n- \`[code]\`: [meaning]\n- \`[code]\`: [meaning]" \
  --confidence 0.8 \
  --tags "debugging,error-analysis,[issue-tag]"
```

**Step 2: Reproduce Consistently**

```bash
# Create reasoning for reproduction steps
engram reasoning create \
  --title "[Issue] Reproduction Steps" \
  --task-id [TASK_ID] \
  --content "**Reproducible:** [YES/NO]\n\n**Exact steps to reproduce:**\n1. \`[command 1]\`\n2. \`[command 2]\`\n3. \`[command 3]\`\n\n**Frequency:** [Every time/Occasionally/Rarely]\n\n**Environment details:**\n- OS: [version]\n- Language: [version]\n- Dependencies: [versions]" \
  --confidence 1.0 \
  --tags "debugging,reproduction,[issue-tag]"
```

**Step 3: Check Recent Changes**

```bash
# Create reasoning for recent changes analysis
engram reasoning create \
  --title "[Issue] Recent Changes Analysis" \
  --task-id [TASK_ID] \
  --content "**Recent commits examined:**\n- \`[commit]\`: [description] - [potential relevance]\n- \`[commit]\`: [description] - [potential relevance]\n\n**Files changed:**\n- \`[file]\`: [what changed] - [potential relevance]\n\n**Dependency changes:**\n- \`[package]\`: [version change] - [potential relevance]\n\n**Configuration changes:**\n- \`[config]\`: [what changed] - [potential relevance]" \
  --confidence 0.7 \
  --tags "debugging,changes,[issue-tag]"
```

**Step 4: Multi-Component Evidence (if applicable)**

For systems with multiple components (CI → build → signing, API → service → database):

```bash
# Create reasoning for multi-component analysis
engram reasoning create \
  --title "[Issue] Multi-Component Analysis" \
  --task-id [TASK_ID] \
  --content "## Component Boundaries Examined\n\n### Component 1: [Name]\n**Input:** [what enters]\n**Output:** [what exits]\n**State:** [what state]\n**Finding:** [result]\n\n### Component 2: [Name]\n**Input:** [what enters]\n**Output:** [what exits]\n**State:** [what state]\n**Finding:** [result]\n\n## Evidence Collection\n\`\`\`bash\n# Commands run for evidence\n[command 1]\n[command 2]\n[command 3]\n\`\`\`\n\n## Results\n**Where it breaks:** [Component]\n**Why it breaks:** [Initial hypothesis]" \
  --confidence 0.6 \
  --tags "debugging,multi-component,[issue-tag]"
```

### Phase 2: Pattern Analysis

```bash
# Create reasoning for pattern analysis
engram reasoning create \
  --title "[Issue] Pattern Analysis" \
  --task-id [TASK_ID] \
  --content "## Similar Issues Found\n- \`[issue-id]\`: [similarity] - [root cause]\n- \`[issue-id]\`: [similarity] - [root cause]\n\n## Recurring Patterns\n1. [Pattern 1]: [description]\n2. [Pattern 2]: [description]\n\n## Systemic vs Isolated\n**Assessment:** [Systemic/Isolated]\n**Reason:** [explanation]\n\n## Related Code Patterns\n- \`[pattern]\`: found in [files] - [implication]" \
  --confidence 0.7 \
  --tags "debugging,pattern-analysis,[issue-tag]"
```

### Phase 3: Hypothesis Testing

```bash
# Create reasoning for hypothesis
engram reasoning create \
  --title "[Issue] Hypothesis: [Root Cause]" \
  --task-id [TASK_ID] \
  --content "## Hypothesis\n**[Root cause hypothesis statement]**\n\n## Supporting Evidence\n1. [Evidence 1]\n2. [Evidence 2]\n3. [Evidence 3]\n\n## Test to Verify\n\`\`\`bash\n[command to test hypothesis]\n\`\`\`\n\n## Expected Result\nIf hypothesis is correct: [result]\nIf hypothesis is incorrect: [result]" \
  --confidence 0.6 \
  --tags "debugging,hypothesis,[issue-tag]"
```

**After testing hypothesis:**

```bash
# Update hypothesis with test results
engram reasoning create \
  --title "[Issue] Hypothesis Test Results" \
  --task-id [TASK_ID] \
  --content "## Hypothesis Tested\n[Hypothesis statement]\n\n## Test Executed\n\`\`\`bash\n[command run]\n\`\`\`\n\n## Test Output\n\`\`\`\n[output]\n\`\`\`\n\n## Conclusion\n**VERIFIED** or **REFUTED**\n\n**If verified:** [explanation]\n**If refuted:** [what this means for root cause]" \
  --confidence 0.9 \
  --tags "debugging,hypothesis-result,[issue-tag]"
```

### Phase 4: Implementation

```bash
# Create reasoning for fix
engram reasoning create \
  --title "[Issue] Fix Implementation" \
  --task-id [TASK_ID] \
  --content "## Root Cause Confirmed\n[Final root cause statement]\n\n## Fix Applied\n\`\`\`[language]\n[fixed code]\n\`\`\`\n\n## Files Modified\n- \`[file]\`: [what changed]\n\n## Test Added\n\`\`\`[language]\n[regression test]\n\`\`\`\n\n## Verification Steps\n1. \`[command 1]\` - [expected result]\n2. \`[command 2]\` - [expected result]" \
  --confidence 1.0 \
  --tags "debugging,fix,[issue-tag]"
```

**After verification:**

```bash
# Create reasoning for fix verification
engram reasoning create \
  --title "[Issue] Fix Verification" \
  --task-id [TASK_ID] \
  --content "## Verification Commands Executed\n\`\`\`bash\n[command 1]\n[command 2]\n\`\`\`\n\n## Verification Results\n\`\`\`\n[output]\n\`\`\`\n\n## Status\n✅ **FIX VERIFIED** - Issue resolved\n✅ **Regression test passes**\n✅ **No new issues introduced**" \
  --confidence 1.0 \
  --tags "debugging,fix-verified,[issue-tag]"
```

## Why Engram Integration Matters

1. **Persistent Audit Trail:** Debug sessions aren't lost in conversation
2. **Pattern Recognition:** Can query all debugging sessions for patterns
3. **Knowledge Reuse:** Future agents can query: `engram reasoning list | grep debugging`
4. **Root Cause追溯:** Full reasoning chain from symptom → hypothesis → test → fix
5. **Team Learning:** Debug knowledge is stored, not forgotten

## Querying Debug History

```bash
# Get all debugging sessions
engram reasoning list | grep debugging

# Get specific debug session
engram reasoning show [REASONING_ID]

# Search for similar issues
engram reasoning list | grep -E "[issue-tag]|[symptom]"

# Get full debug chain for an issue
engram relationship connected --entity-id [TASK_ID] --relationship-type documents | grep debugging
```

## Example Workflow

```bash
# Issue: API returns 500 error on user creation

# Phase 1: Investigation
engram reasoning create \
  --title "API 500 Error: Investigation" \
  --task-id $BUG_TASK \
  --content "Error: 500 Internal Server Error\nStack trace: [trace]\nReproducible: YES" \
  --confidence 0.5 \
  --tags "debugging,api-error,user-creation"

engram reasoning create \
  --title "API 500 Error: Error Analysis" \
  --task-id $BUG_TASK \
  --content "Error type: NullPointerException\nAt: UserService.java:142\nFinding: user.getProfile() called when profile is null" \
  --confidence 0.8 \
  --tags "debugging,error-analysis,user-creation"

# Phase 2: Pattern Analysis
engram reasoning create \
  --title "API 500 Error: Pattern Analysis" \
  --task-id $BUG_TASK \
  --content "Similar issues: 2 previous null pointer issues\nPattern: Missing null checks before property access\nSystemic: YES - inconsistent null handling" \
  --confidence 0.7 \
  --tags "debugging,pattern,user-creation"

# Phase 3: Hypothesis
engram reasoning create \
  --title "API 500 Error: Hypothesis" \
  --task-id $BUG_TASK \
  --content "Hypothesis: Profile is not being loaded in create flow\nTest: Add logging to check profile state before access" \
  --confidence 0.6 \
  --tags "debugging,hypothesis,user-creation"

engram reasoning create \
  --title "API 500 Error: Hypothesis Verified" \
  --task-id $BUG_TASK \
  --content "Test: Logging added\nResult: Profile is null at access point\nConclusion: VERIFIED - profile not loaded in create flow" \
  --confidence 0.9 \
  --tags "debugging,hypothesis-result,user-creation"

# Phase 4: Fix
engram reasoning create \
  --title "API 500 Error: Fix" \
  --task-id $BUG_TASK \
  --content "Fix: Added profile loading in UserService.create()\nTest: Added null check regression test" \
  --confidence 1.0 \
  --tags "debugging,fix,user-creation"

engram reasoning create \
  --title "API 500 Error: Fix Verified" \
  --task-id $BUG_TASK \
  --content "Verification: Tests pass, 500 error no longer occurs\nStatus: RESOLVED" \
  --confidence 1.0 \
  --tags "debugging,fix-verified,user-creation"
```

## Related Skills

This skill integrates with:
- `engram-use-memory` - Store debugging insights for similar issues
- `engram-audit-trail` - Track debugging process and root cause analysis
- `engram-test-driven-development` - Write tests to reproduce bugs
- `engram-testing` - Verify fixes with comprehensive tests
- `engram-check-compliance` - Document security bug fixes for compliance
