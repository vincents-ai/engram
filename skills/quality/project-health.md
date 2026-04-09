---
name: engram-project-health
description: "Run git-based project health diagnostics before reading code. Churn hotspots, bus factor, bug clusters, velocity trends, firefighting frequency, and more."
---

# Project Health Diagnostics (Engram-Integrated)

## Overview

Run git-based diagnostics to assess project health before reading code. These commands reveal where a codebase hurts — churn hotspots, bus factor, bug clusters, velocity trends, and firefighting patterns. Store results as engram entities for tracking over time.

## When to Use

Use this skill when:
- Onboarding onto a new project or upstream dependency
- Starting a new work session on an unfamiliar codebase
- Evaluating project viability or team health
- Before planning a major refactoring effort
- Conducting a codebase audit
- Reviewing an open-source project before contributing

## The Five Core Checks

### Check 1: Churn Hotspots

What changes the most. High churn on a file nobody wants to own is the clearest signal of codebase drag.

```bash
# Top 20 most-changed files in the last year
git log --format=format: --name-only --since="1 year ago" | sort | uniq -c | sort -nr | head -20
```

**Interpretation:**
- High churn + nobody owns it = drag hotspot (patch on patch)
- High churn + active development = normal
- Cross-reference with bug cluster list — files on BOTH are highest risk
- A 2005 Microsoft Research study found churn predicted defects more reliably than complexity metrics alone

### Check 2: Bus Factor

Who built this. If one person accounts for 60%+ of commits, that's a bus factor risk.

```bash
# All-time contributor ranking
git shortlog -sn --no-merges

# Recent contributors (last 6 months)
git shortlog -sn --no-merges --since="6 months ago"
```

**Interpretation:**
- Compare all-time vs recent — if top all-time contributor isn't in recent list, flag it
- 30 contributors but only 3 active in the last year = knowledge concentration risk
- Squash-merge workflows compress authorship — ask about merge strategy before drawing conclusions

### Check 3: Bug Clusters

Where bugs concentrate. Compare against churn hotspots for highest-risk files.

```bash
# Top 20 files with bug-related commits
git log -i -E --grep="fix|bug|broken|regression" --name-only --format='' | sort | uniq -c | sort -nr | head -20
```

**Interpretation:**
- Files on both the churn AND bug lists are the single biggest risk
- Depends on commit message discipline — "update stuff" commits yield nothing
- Even a rough map of bug density is better than no map

### Check 4: Velocity Trend

Is this project accelerating or dying.

```bash
# Commit count by month (entire history)
git log --format='%ad' --date=format:'%Y-%m' | sort | uniq -c

# Recent velocity (last 12 months only)
git log --format='%ad' --date=format:'%Y-%m' --since="12 months ago" | sort | uniq -c
```

**Interpretation:**
- Steady rhythm = healthy
- Drop by half in a single month = someone probably left
- Declining curve over 6-12 months = losing momentum
- Periodic spikes + quiet months = batched releases instead of continuous shipping

### Check 5: Firefighting Frequency

How often the team is reverting, hotfixing, or rolling back.

```bash
# Revert and hotfix frequency (last year)
git log --oneline --since="1 year ago" | grep -iE 'revert|hotfix|emergency|rollback|outage|incident'

# Count
git log --oneline --since="1 year ago" | grep -iE 'revert|hotfix|emergency|rollback|outage|incident' | wc -l
```

**Interpretation:**
- A handful over a year = normal
- Reverts every couple weeks = deploy trust issues (unreliable tests, missing staging)
- Zero results = either very stable or nobody writes descriptive commit messages
- Crisis patterns are binary — they're there or they're not

## Extended Checks

### Check 6: Code Ownership Gaps

Files modified by many authors but owned by none.

```bash
# Files touched by most unique authors (last year)
git log --format=format: --name-only --since="1 year ago" | sort -u | while read f; do
  authors=$(git log --format='%aN' --since="1 year ago" -- "$f" | sort -u | wc -l)
  echo "$authors $f"
done | sort -nr | head -20
```

**Interpretation:**
- Many authors + high churn = no clear owner = coordination overhead
- Files with 1-2 authors are well-owned (even if high churn)

### Check 7: Commit Size Distribution

Are changes small and focused or large and risky?

```bash
# Average lines changed per commit (last 3 months)
git log --numstat --format="" --since="3 months ago" | awk 'NF==3 {add+=$1; del+=$2; n++} END {if(n>0) printf "Avg commit: +%.0f -%.0f lines (%d commits)\n", add/n, del/n, n}'

# Largest commits (potential risk)
git log --numstat --format="" --since="3 months ago" | awk 'NF==3 {total=$1+$2} {printf "%d %s\n", total, $0}' | sort -nr | head -10
```

**Interpretation:**
- Smaller commits = better (easier to review, revert, bisect)
- Average > 400 lines = commits may be too large
- Single commits > 2000 lines = high risk, should be split

### Check 8: Review Coverage

How many commits are being reviewed before merge.

```bash
# Commits with "Reviewed-by" or "Co-authored-by" (indicates review)
git log --format='%B' --since="6 months ago" | grep -iE 'reviewed-by|co-authored-by|pull request' | wc -l

# Total commits for comparison
git log --oneline --since="6 months ago" | wc -l
```

**Interpretation:**
- Review coverage < 50% = risk of unchecked code entering main
- 100% coverage with long review times = bottleneck

### Check 9: Dependency Freshness

How current are the project's dependencies.

```bash
# Rust: check Cargo.toml dependencies
cargo outdated 2>/dev/null || echo "cargo-outdated not installed"

# Node: check npm dependencies
npm outdated 2>/dev/null || echo "Not a Node project"

# Python: check pip dependencies
pip list --outdated 2>/dev/null | head -20 || echo "Not a Python project"
```

### Check 10: Test Signal

How healthy is the test suite based on commit patterns.

```bash
# Test-related commit frequency
git log --oneline --since="1 year ago" | grep -iE 'test|spec|coverage|fixture' | wc -l

# Total commits for ratio
git log --oneline --since="1 year ago" | wc -l

# Files most frequently changed for test reasons
git log -i -E --grep="test|spec" --name-only --format='' --since="1 year ago" | sort | uniq -c | sort -nr | head -10
```

**Interpretation:**
- Test commits < 5% of total = testing may be an afterthought
- Files frequently changed "for tests" may have flaky or inadequate test coverage

## Running the Full Audit

```bash
# Step 1: Create audit task
TASK_ID=$(engram task create \
  --title "Project Health Audit: $(basename $(git rev-parse --show-toplevel))" \
  --description "Running git-based project health diagnostics" \
  --priority medium \
  --output json | jq -r '.id')

engram task update $TASK_ID --status in_progress
```

```bash
# Step 2: Run all checks and capture output
AUDIT_DIR=$(mktemp -d)
{
  echo "## Churn Hotspots"
  git log --format=format: --name-only --since="1 year ago" | sort | uniq -c | sort -nr | head -20

  echo -e "\n## Bus Factor"
  git shortlog -sn --no-merges
  echo -e "\n### Recent (6 months)"
  git shortlog -sn --no-merges --since="6 months ago"

  echo -e "\n## Bug Clusters"
  git log -i -E --grep="fix|bug|broken|regression" --name-only --format='' | sort | uniq -c | sort -nr | head -20

  echo -e "\n## Velocity Trend"
  git log --format='%ad' --date=format:'%Y-%m' --since="12 months ago" | sort | uniq -c

  echo -e "\n## Firefighting"
  REVERTS=$(git log --oneline --since="1 year ago" | grep -iE 'revert|hotfix|emergency|rollback' | wc -l)
  echo "Reverts/hotfixes in last year: $REVERTS"

  echo -e "\n## Commit Size"
  git log --numstat --format="" --since="3 months ago" | awk 'NF==3 {add+=$1; del+=$2; n++} END {if(n>0) printf "Avg: +%.0f -%.0f lines (%d commits)\n", add/n, del/n, n}'

  echo -e "\n## Test Signal"
  TEST_COMMITS=$(git log --oneline --since="1 year ago" | grep -iE 'test|spec|coverage' | wc -l)
  TOTAL_COMMITS=$(git log --oneline --since="1 year ago" | wc -l)
  echo "Test commits: $TEST_COMMITS / $TOTAL_COMMITS ($(( TEST_COMMITS * 100 / TOTAL_COMMITS ))%)"
} > "$AUDIT_DIR/audit.md" 2>&1
```

```bash
# Step 3: Store results in engram
CONTEXT_ID=$(engram context create \
  --title "Project Health Audit: $(basename $(git rev-parse --show-toplevel))" \
  --content "$(cat $AUDIT_DIR/audit.md)" \
  --source "project-health" \
  --tags "project-health,audit,$(basename $(git rev-parse --show-toplevel))" \
  --json | jq -r '.id')
```

```bash
# Step 4: Record interpretation as reasoning
REASONING_ID=$(engram reasoning create \
  --title "Project Health Interpretation" \
  --task-id $TASK_ID \
  --content "[Fill in your interpretation of the audit results. Key findings, risks, and recommendations.]" \
  --tags "project-health,interpretation" \
  --json | jq -r '.id')
```

```bash
# Step 5: Link everything
engram relationship create \
  --source-id $TASK_ID --source-type task \
  --target-id $CONTEXT_ID --target-type context \
  --relationship-type documents --agent default

engram relationship create \
  --source-id $TASK_ID --source-type task \
  --target-id $REASONING_ID --target-type reasoning \
  --relationship-type explains --agent default

engram task update $TASK_ID --status done

rm -rf "$AUDIT_DIR"
```

## Engram CLI Integration

The existing engram health commands complement these checks:

```bash
# Stale task detection (already implemented)
engram task list --stale --stale-threshold 24h

# Zombie session detection (already implemented)
engram session zombies --max-age-hours 24 --check-git

# DORA metrics (already implemented)
engram analytics dora --window-days 30
```

## Health Score Formula

Combine all signals into a single health score (0-100):

| Signal | Weight | Healthy | Warning | Critical |
|--------|--------|---------|---------|----------|
| Bus factor | 20% | 3+ active contributors | 1-2 active | 1 or 0 |
| Churn concentration | 15% | No file > 5% of changes | Top file 5-10% | Top file > 10% |
| Bug/churn overlap | 15% | No overlap | 1-2 files | 3+ files |
| Velocity trend | 15% | Stable or growing | < 20% decline | > 20% decline |
| Firefighting | 15% | < 2% of commits | 2-5% | > 5% |
| Commit size | 10% | Avg < 200 lines | 200-400 | > 400 |
| Test signal | 10% | > 10% test commits | 5-10% | < 5% |

## Related Skills

- `engram-tech-debt` - Identify and quantify technical debt
- `engram-risk-assessment` - Assess project risks
- `engram-capacity-planning` - Plan team capacity based on health data
- `engram-dora` - DORA metrics for delivery performance
- `engram-test-harness-review` - Audit test suite adequacy
