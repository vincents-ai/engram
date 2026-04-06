---
name: engram-adr
description: "Document architectural decisions with context, options considered, rationale, and consequences. Store as ADR entities for queryable decision history."
---

# Architecture Decision Records (Engram-Integrated)

## Overview

Document significant architectural decisions using Engram's dedicated `engram adr` entity type. ADRs capture context, alternatives, rationale, and consequences — and link to tasks and other ADRs to form a queryable decision history.

## When to Use

Use this skill when:
- Making significant architectural decisions (database choice, framework selection)
- Choosing between multiple viable technical approaches
- Setting standards or conventions for the codebase
- Making trade-offs with long-term implications
- Onboarding new team members who need to understand why things are the way they are
- Revisiting old decisions to evaluate if they still make sense
- Documenting decisions made during design reviews

## The Pattern

### Step 1: Search Before Creating

Check whether an ADR already exists for this decision:

```bash
# Natural language search
engram ask query "ADR database user service"

# List all ADRs
engram adr list

# Filter by status
engram adr list --status proposed
```

### Step 2: Create the ADR

Use `engram adr create` with the three required flags:

```bash
ADR_UUID=$(engram adr create \
  --title "Use PostgreSQL for User Service" \
  --number 1 \
  --context "We need a database for the User Service handling 100K users growing to 1M. Requirements: ACID transactions, strong consistency for auth data, full-text search on profiles. Team has PostgreSQL expertise. PostgreSQL was chosen over MySQL (less full-text capability) and MongoDB (no ACID transactions)." \
  --agent "architect" \
  --json | jq -r '.id')

echo "Created ADR: $ADR_UUID"
```

**Required flags:**
- `--title` — short, descriptive decision title
- `--number` — sequential integer (1, 2, 3...) — never reuse numbers
- `--context` — situation, problem statement, and what was decided and why

**No `--status` or `--decision` flag on create.** Status starts as `proposed`.

### Step 3: Add Alternatives Considered

Document each option that was evaluated:

```bash
# Add alternative options that were considered
engram adr add-alternative "$ADR_UUID" \
  --description "MySQL: Compatible with existing monolith stack. Pros: team already uses it in production. Cons: weaker full-text search than PostgreSQL, no native JSON support."

engram adr add-alternative "$ADR_UUID" \
  --description "MongoDB: Flexible document model for profile data. Pros: easy schema evolution. Cons: no ACID transactions, eventual consistency by default — unacceptable for auth data."
```

### Step 4: Add Stakeholders

Record who was involved in the decision:

```bash
engram adr add-stakeholder "$ADR_UUID" --stakeholder "Alice Chen, Lead Engineer"
engram adr add-stakeholder "$ADR_UUID" --stakeholder "Bob Patel, Engineering Manager"
```

### Step 5: Accept the ADR

When the team agrees on the decision, accept it with the formal decision and consequences:

```bash
engram adr accept "$ADR_UUID" \
  --decision "Use PostgreSQL 15 deployed as AWS RDS with primary + 2 read replicas. All user data lives in a single schema. Migrations managed with sqlx." \
  --consequences "Positive: ACID transactions, full-text search, team expertise. Negative: vertical write scaling limits at ~500 writes/sec. Neutral: must run migrations on deploy. Revisit if write traffic exceeds 400 writes/sec sustained."
```

### Step 6: Link ADR to Task

Connect the ADR to the task that triggered the decision:

```bash
engram relationship create \
  --source-id "$TASK_UUID" --source-type task \
  --target-id "$ADR_UUID" --target-type adr \
  --relationship-type explains \
  --agent "architect"
```

### Step 7: Link ADRs to Each Other

When decisions supersede or extend prior ones:

```bash
# New ADR supersedes an old one
engram relationship create \
  --source-id "$NEW_ADR_UUID" --source-type adr \
  --target-id "$OLD_ADR_UUID" --target-type adr \
  --relationship-type supersedes \
  --agent "architect"

# Mark old ADR as superseded (update its status)
engram adr update "$OLD_ADR_UUID" \
  --status superseded \
  --superseded-by "$NEW_ADR_UUID"

# New ADR extends (builds on) an existing one
engram relationship create \
  --source-id "$NEW_ADR_UUID" --source-type adr \
  --target-id "$RELATED_ADR_UUID" --target-type adr \
  --relationship-type extends \
  --agent "architect"
```

## Querying ADRs

```bash
# List all ADRs
engram adr list

# Filter by status
engram adr list --status accepted
engram adr list --status proposed

# Text search
engram adr list --search "database"
engram adr list --search "authentication"

# Get full details on a specific ADR
engram adr get "$ADR_UUID"

# Find all ADRs connected to a task
engram relationship connected --entity-id "$TASK_UUID" --max-depth 2

# Natural language query across all engram entities
engram ask query "what database decisions have we made"
engram ask query "why did we choose PostgreSQL"
```

## Updating an ADR

Use `engram adr update` to amend any field after creation:

```bash
engram adr update "$ADR_UUID" \
  --context "Updated context with new constraints discovered post-review." \
  --status deprecated
```

Available `--status` values: `proposed`, `accepted`, `deprecated`, `superseded`

## Full Example

Team needs to decide on database for a new microservice.

```bash
# Step 1: Search first
engram ask query "database decision user service"
# Returns nothing relevant — proceed

# Step 2: Create ADR
ADR=$(engram adr create \
  --title "Use PostgreSQL for User Service" \
  --number 1 \
  --context "User Service needs a database for 100K users growing to 1M. Auth data requires strong consistency and ACID transactions. Profile search requires full-text capability. Team has 3 years of PostgreSQL production experience. PostgreSQL was selected over MySQL (inferior full-text search) and MongoDB (lacks ACID)." \
  --agent "architect" \
  --json | jq -r '.id')

echo "ADR created: $ADR"

# Step 3: Add alternatives
engram adr add-alternative "$ADR" \
  --description "MySQL: Team uses it in monolith. Pros: familiar tooling, good ACID support. Cons: weaker full-text search, no native JSONB."

engram adr add-alternative "$ADR" \
  --description "MongoDB: Flexible document model. Pros: easy profile schema evolution. Cons: no multi-doc ACID transactions, eventual consistency unacceptable for login flows."

# Step 4: Add stakeholders
engram adr add-stakeholder "$ADR" --stakeholder "Alice Chen, Lead Engineer"
engram adr add-stakeholder "$ADR" --stakeholder "Bob Patel, Engineering Manager"

# Step 5: Accept
engram adr accept "$ADR" \
  --decision "Deploy PostgreSQL 15 on AWS RDS (db.t3.medium). Primary + 2 read replicas. Migrations via sqlx-cli." \
  --consequences "Positive: ACID, full-text search, team expertise reduces ramp-up. Negative: single-writer limit ~500 writes/sec. Revisit when sustained writes exceed 400/sec."

# Step 6: Link to task
engram relationship create \
  --source-id "$TASK_UUID" --source-type task \
  --target-id "$ADR" --target-type adr \
  --relationship-type explains \
  --agent "architect"

# Step 7: Verify
engram adr get "$ADR"
```

Six months later, scale requires sharding:

```bash
# Create superseding ADR
NEW_ADR=$(engram adr create \
  --title "Shard User Database by User ID" \
  --number 5 \
  --context "User Service has grown to 8M users. Write traffic (500 writes/sec) is hitting the single-leader PostgreSQL limit (ADR-001). CockroachDB migration was evaluated but rejected due to team unfamiliarity and migration risk. Hash sharding across 4 PostgreSQL instances was chosen as lowest-risk horizontal scale path." \
  --agent "architect" \
  --json | jq -r '.id')

engram adr accept "$NEW_ADR" \
  --decision "Shard users across 4 PostgreSQL instances using hash(user_id) % 4. Sharding logic lives in the data layer. Cross-shard queries are prohibited." \
  --consequences "Positive: 4x write capacity. Negative: cross-shard queries require application-level joins. All migrations must run on each shard. Revisit if shards become uneven due to hot users."

# Link: new ADR supersedes old
engram relationship create \
  --source-id "$NEW_ADR" --source-type adr \
  --target-id "$ADR" --target-type adr \
  --relationship-type supersedes \
  --agent "architect"

engram adr update "$ADR" --status superseded --superseded-by "$NEW_ADR"
```

## ADR Numbering

- Use sequential integers: 1, 2, 3, 4, 5...
- Never reuse a number, even for superseded ADRs
- Gaps in sequence are acceptable

## ADR Anti-Patterns

**DON'T: Use `engram reasoning create` for ADRs**
Engram has a dedicated `engram adr` entity type — use it. `reasoning` is for logic chains, not decisions.

**DON'T: Write ADRs After the Fact**
ADRs should be written during decision-making. Context and trade-offs are freshest then.

**DON'T: Write One-Sided ADRs**
Always document alternatives considered. This shows due diligence and helps future readers understand what was rejected and why.

**DON'T: Skip the Accept Step**
An ADR in `proposed` state is not actionable. Run `engram adr accept` when the team agrees.

**DON'T: Forget to Link**
Always link the ADR to its triggering task with `engram relationship create`. This is what makes ADRs queryable from task context.

## Related Skills

- `engram-system-design` — ADRs document design decisions
- `engram-api-design` — API design choices warrant ADRs
- `engram-security-architecture` — Security decisions should be ADRs
- `engram-data-modeling` — Database design decisions warrant ADRs
- `engram-risk-assessment` — ADRs document risk mitigation strategies
- `engram-orchestrator` — Orchestrators create ADRs for architectural choices in the decision step
