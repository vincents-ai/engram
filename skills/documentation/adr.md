---
name: engram-adr
description: "Document architectural decisions with context, options considered, rationale, and consequences. Store as reasoning entities for queryable decision history."
---

# Architecture Decision Records (Engram-Integrated)

## Overview

Document significant architectural decisions by systematically capturing the context, options considered, rationale for the chosen solution, and expected consequences. Store ADRs as reasoning entities in Engram to build a queryable history of design choices and their evolution.

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

### Step 1: Identify Decision to Document

Not every decision needs an ADR. Focus on:

**Document These:**
- Technology choices (database, language, framework)
- Architectural patterns (microservices vs monolith, event-driven)
- Data storage decisions (SQL vs NoSQL, normalization)
- External service integrations (payment provider, auth provider)
- Security approaches (encryption, authentication mechanism)
- Deployment strategies (CI/CD pipeline, infrastructure)

**Don't Document These:**
- Code style preferences (use linter config instead)
- Minor implementation details (code comments sufficient)
- Temporary workarounds (unless they become permanent)

### Step 2: Create ADR as Reasoning Entity

Use ADR template to document decision:

```bash
engram reasoning create \
  --title "ADR-[NUMBER]: [Short Title]" \
  --task-id [TASK_ID] \
  --content "## Status\n\n**Accepted** | **Proposed** | **Deprecated** | **Superseded by ADR-XXX**\n\n## Context\n\n**Problem Statement:**\n[What challenge or decision are we facing?]\n\n**Background:**\n[What led to this decision? What constraints do we have?]\n\n**Requirements:**\n1. [Requirement 1]\n2. [Requirement 2]\n3. [Requirement 3]\n\n**Assumptions:**\n- [Assumption 1: e.g., Traffic will grow 2x per year]\n- [Assumption 2: e.g., Team has expertise in Python]\n\n## Options Considered\n\n### Option 1: [Name]\n\n**Description:**\n[How would this work?]\n\n**Pros:**\n- [Advantage 1]\n- [Advantage 2]\n\n**Cons:**\n- [Disadvantage 1]\n- [Disadvantage 2]\n\n**Cost:** [Development cost, operational cost, learning curve]\n\n**Risk:** [Technical risks, business risks]\n\n### Option 2: [Name]\n\n[Same structure as Option 1]\n\n### Option 3: [Name]\n\n[Same structure as Option 1]\n\n## Decision\n\n**Chosen Option:** [Option X]\n\n**Rationale:**\n[Why did we choose this option over others?]\n\n**Key Factors:**\n1. [Factor 1: e.g., Team expertise]\n2. [Factor 2: e.g., Time to market]\n3. [Factor 3: e.g., Cost]\n\n**Decision Makers:**\n- [Name/Role 1]\n- [Name/Role 2]\n\n**Date:** [YYYY-MM-DD]\n\n## Consequences\n\n**Positive:**\n- [Expected benefit 1]\n- [Expected benefit 2]\n\n**Negative:**\n- [Expected drawback 1]\n- [Expected drawback 2]\n\n**Neutral:**\n- [Implications that are neither good nor bad]\n\n**Action Items:**\n1. [What needs to be done to implement this decision]\n2. [What needs to be communicated]\n3. [What needs to be monitored]\n\n## Revisit Criteria\n\n**Revisit this decision if:**\n- [Condition 1: e.g., Traffic exceeds 10x current level]\n- [Condition 2: e.g., New technology emerges that solves X problem]\n- [Condition 3: e.g., Team expertise changes significantly]\n\n**Next Review Date:** [YYYY-MM-DD or N/A]\n\n## References\n\n- [Link to related ADRs]\n- [Link to technical documentation]\n- [Link to research or benchmarks]\n- [Link to team discussion (Slack thread, RFC doc)]" \
  --confidence [0.0-1.0] \
  --tags "adr,architecture,[decision-domain]"
```

### Step 3: Link Related ADRs

Connect decisions that supersede, extend, or relate to each other:

```bash
# If this ADR supersedes an old one
engram relationship create \
  --source-id [NEW_ADR_ID] --source-type reasoning \
  --target-id [OLD_ADR_ID] --target-type reasoning \
  --relationship-type supersedes --agent [AGENT]

# If this ADR extends or builds on another
engram relationship create \
  --source-id [NEW_ADR_ID] --source-type reasoning \
  --target-id [RELATED_ADR_ID] --target-type reasoning \
  --relationship-type extends --agent [AGENT]

# Link ADR to task
engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [ADR_ID] --target-type reasoning \
  --relationship-type documents --agent [AGENT]
```

### Step 4: Update Status When Revisited

When decision changes:

```bash
# Create new ADR with updated decision
engram reasoning create \
  --title "ADR-[NEW_NUMBER]: [Updated Decision]" \
  --task-id [TASK_ID] \
  --content "[ADR content with Status: Supersedes ADR-[OLD_NUMBER]]" \
  --confidence [0.0-1.0] \
  --tags "adr,architecture,[decision-domain]"

# Update old ADR status (via relationship)
engram relationship create \
  --source-id [NEW_ADR_ID] --source-type reasoning \
  --target-id [OLD_ADR_ID] --target-type reasoning \
  --relationship-type supersedes --agent [AGENT]
```

## Example

Team needs to decide on database for a new microservice.

### Step 1: Create ADR

```bash
ADR=$(engram reasoning create \
  --title "ADR-001: Database for User Service" \
  --task-id a1b2c3d4-5678-90ab-cdef-1234567890ab \
  --content "## Status\n\n**Accepted**\n\n## Context\n\n**Problem Statement:**\n\nWe need to choose a database for the User Service, which will store user accounts, profiles, and authentication data.\n\n**Background:**\n\n- User Service is a new microservice being extracted from monolith\n- Current monolith uses MySQL for all data\n- User Service will handle 100K users initially, growing to 1M in 2 years\n- Requirements: ACID transactions, strong consistency for auth data\n- Team has expertise in PostgreSQL and MySQL\n\n**Requirements:**\n\n1. Strong consistency (login must see latest password update)\n2. ACID transactions (user registration is multi-step)\n3. Handle 1M users within 2 years\n4. Support full-text search on user profiles\n5. Low operational overhead (small team)\n\n**Assumptions:**\n\n- Single datacenter deployment (multi-region not needed yet)\n- Read-heavy workload (10:1 read:write ratio)\n- Team can learn new database if needed\n\n## Options Considered\n\n### Option 1: PostgreSQL\n\n**Description:**\n\nUse PostgreSQL as primary database for User Service. Deploy as managed service (AWS RDS) with primary-replica replication.\n\n**Pros:**\n- ACID compliant with strong consistency\n- Excellent full-text search (tsvector, GIN indexes)\n- JSON support for flexible profile data\n- Team has production experience\n- Mature ecosystem and tooling\n- Free and open source\n\n**Cons:**\n- Vertical scaling limits (single-leader writes)\n- More complex than MySQL for simple queries\n- Replication lag on read replicas\n\n**Cost:**\n- AWS RDS: ~$200/month for db.t3.medium\n- Development: Low (team knows PostgreSQL)\n- Learning: None (team already expert)\n\n**Risk:**\n- Low: Proven technology, team expertise, managed service available\n\n### Option 2: MySQL\n\n**Description:**\n\nUse MySQL (or compatible like Aurora MySQL) as primary database. Aligns with existing monolith database.\n\n**Pros:**\n- Team already uses MySQL in monolith\n- Compatible with existing tools and workflows\n- Aurora MySQL offers better scalability\n- Simpler than PostgreSQL for basic operations\n\n**Cons:**\n- Weaker full-text search (vs PostgreSQL)\n- Less feature-rich (no JSONB, weaker indexes)\n- Aurora MySQL vendor lock-in\n\n**Cost:**\n- AWS RDS MySQL: ~$200/month\n- Aurora MySQL: ~$300/month\n- Development: Low (team knows MySQL)\n\n**Risk:**\n- Low: Proven, team expertise\n\n### Option 3: DynamoDB\n\n**Description:**\n\nUse AWS DynamoDB (NoSQL) for flexible scaling and managed operations.\n\n**Pros:**\n- Horizontal scaling (no single-leader bottleneck)\n- Fully managed (no operational overhead)\n- High availability built-in\n- Predictable performance at scale\n\n**Cons:**\n- No ACID transactions (eventual consistency)\n- No full-text search (need separate service)\n- Complex access patterns (must design for query patterns)\n- Team has no DynamoDB experience\n- Vendor lock-in (AWS only)\n\n**Cost:**\n- On-demand: ~$50/month initially, scales with usage\n- Development: High (learning curve)\n- Learning: 2-4 weeks for team to become proficient\n\n**Risk:**\n- Medium: Team learning curve, eventual consistency may cause auth bugs\n\n## Decision\n\n**Chosen Option:** Option 1 - PostgreSQL\n\n**Rationale:**\n\n1. **Strong consistency required:** Authentication cannot tolerate eventual consistency. PostgreSQL provides ACID guarantees.\n2. **Full-text search:** User profile search is a core feature. PostgreSQL's built-in full-text search avoids adding Elasticsearch.\n3. **Team expertise:** Team already has PostgreSQL experience, minimizing risk and development time.\n4. **Sufficient scale:** 1M users well within PostgreSQL capacity. Can add read replicas if needed.\n5. **JSON support:** Profile data has flexible schema. JSONB columns provide flexibility without sacrificing SQL.\n\n**Key Factors:**\n1. Strong consistency (eliminates DynamoDB)\n2. Team expertise (PostgreSQL > MySQL for full-text search)\n3. Full-text search requirement (PostgreSQL stronger than MySQL)\n\n**Decision Makers:**\n- Engineering Lead\n- Backend Team Lead\n- CTO\n\n**Date:** 2026-01-24\n\n## Consequences\n\n**Positive:**\n- Fast development (team already knows PostgreSQL)\n- Strong consistency ensures auth correctness\n- Full-text search built-in (no extra service)\n- JSONB enables flexible profile fields without schema migrations\n- Can use pg_cron for background jobs\n\n**Negative:**\n- Single-leader write bottleneck (if we reach 10M+ users)\n- Read replica lag (eventual consistency on reads from replica)\n- Must manage schema migrations carefully\n\n**Neutral:**\n- AWS RDS lock-in (but can migrate to self-hosted PostgreSQL if needed)\n- Need to monitor query performance and add indexes\n\n**Action Items:**\n1. Provision RDS PostgreSQL instance (db.t3.medium)\n2. Set up schema migration tool (Flyway or sqlx-migrate)\n3. Configure read replica for reporting queries\n4. Document connection pool settings for service\n5. Set up monitoring (query time, connection count, replication lag)\n\n## Revisit Criteria\n\n**Revisit this decision if:**\n- User count exceeds 5M (may need sharding or different architecture)\n- Write traffic exceeds 1K writes/sec (single-leader bottleneck)\n- Team loses PostgreSQL expertise (migration to MySQL may be easier)\n- Multi-region deployment required (PostgreSQL replication across regions is complex)\n- Cost exceeds $1K/month (consider optimizations or DynamoDB)\n\n**Next Review Date:** 2027-01-24 (1 year)\n\n## References\n\n- Related ADR: ADR-002 (Database migration strategy)\n- Benchmarks: https://example.com/postgresql-vs-mysql-benchmarks\n- Team discussion: https://slack.example.com/archives/C123/p1234567890\n- PostgreSQL docs: https://www.postgresql.org/docs/current/textsearch.html" \
  --confidence 0.90 \
  --tags "adr,architecture,database,user-service" \
  --json | jq -r '.id')

echo "Created ADR: $ADR"
```

### Step 2: Later - Create Superseding ADR

6 months later, team reaches scale limits and decides to shard database:

```bash
NEW_ADR=$(engram reasoning create \
  --title "ADR-005: Shard User Database by User ID" \
  --task-id a1b2c3d4-5678-90ab-cdef-1234567890ab \
  --content "## Status\n\n**Accepted** | **Supersedes ADR-001**\n\n## Context\n\n**Problem Statement:**\n\nUser Service has grown to 8M users. Write traffic (500 writes/sec) is approaching single-leader PostgreSQL limit. Need to scale writes horizontally.\n\n**Background:**\n\n- ADR-001 chose PostgreSQL for strong consistency and team expertise\n- Current deployment: Single primary + 3 read replicas\n- Primary CPU reaching 80% during peak hours\n- Forecast: Will hit capacity limit in 6 months at current growth rate\n\n**Requirements:**\n\n1. Scale write capacity to 2K writes/sec\n2. Maintain strong consistency within shard\n3. Minimize application changes\n4. Zero-downtime migration\n\n## Options Considered\n\n### Option 1: Shard by User ID\n\n**Description:**\n\nSplit users across 4 PostgreSQL instances, hashing user_id to determine shard.\n\n**Pros:**\n- Most queries scoped to single user (single shard)\n- Even distribution (hash-based sharding)\n- Linear write scaling\n\n**Cons:**\n- Cross-shard queries are complex\n- Requires sharding logic in application\n- Database migrations must run on all shards\n\n### Option 2: Migrate to Distributed Database (CockroachDB)\n\n**Description:**\n\nMigrate from PostgreSQL to CockroachDB (PostgreSQL-compatible distributed DB).\n\n**Pros:**\n- Automatic sharding (no application changes)\n- Horizontal scaling built-in\n- PostgreSQL compatibility (minimal code changes)\n\n**Cons:**\n- Unproven technology (less mature than PostgreSQL)\n- Team has no experience\n- Higher latency than single PostgreSQL instance\n- Migration risk\n\n**Cost:**\n- CockroachDB Cloud: ~$1K/month\n- Development: High (testing migration, performance validation)\n\n### Option 3: Vertical Scaling\n\n**Description:**\n\nUpgrade to larger RDS instance (db.r5.2xlarge).\n\n**Pros:**\n- Simplest option (no architecture changes)\n- Buys time (6-12 months)\n\n**Cons:**\n- Temporary solution (will hit limit again)\n- Expensive ($800/month vs $200/month)\n- Still single point of failure\n\n## Decision\n\n**Chosen Option:** Option 1 - Shard by User ID\n\n**Rationale:**\n\n1. **Proven approach:** Well-understood sharding pattern\n2. **Application queries:** 95% of queries scoped to single user (efficient sharding)\n3. **Linear scaling:** Can add more shards as needed\n4. **Team control:** Full control vs managed distributed DB\n5. **Cost:** 4 shards = $800/month vs CockroachDB $1K/month\n\n**Decision Makers:**\n- Engineering Lead\n- CTO\n\n**Date:** 2026-07-24\n\n## Consequences\n\n**Positive:**\n- Write capacity scales to 2K writes/sec (4x improvement)\n- Can add more shards if needed\n- Most queries unchanged (single-user scope)\n\n**Negative:**\n- Cross-shard queries are slow (admin dashboard)\n- Must maintain 4 databases (migrations, backups, monitoring)\n- Application complexity (shard routing logic)\n\n**Action Items:**\n1. Implement shard routing library\n2. Add shard_id to all queries\n3. Set up 4 PostgreSQL instances\n4. Migrate data with zero downtime (copy + sync)\n5. Update monitoring to aggregate across shards\n\n## Revisit Criteria\n\n- If write traffic exceeds 1.5K writes/sec (near capacity of 4 shards)\n- If cross-shard queries become frequent (>10% of queries)\n- If operational overhead of 4 databases is too high\n\n**Next Review Date:** 2027-01-24" \
  --confidence 0.85 \
  --tags "adr,architecture,database,sharding,user-service" \
  --json | jq -r '.id')

# Link new ADR to old one
engram relationship create \
  --source-id $NEW_ADR --source-type reasoning \
  --target-id $ADR --target-type reasoning \
  --relationship-type supersedes --agent default

echo "Created superseding ADR: $NEW_ADR"
```

## Querying ADRs

After creating ADRs, agents can query:

```bash
# Get all ADRs
engram reasoning list | grep "ADR-"

# Get ADRs for specific domain
engram reasoning list --tags architecture,database

# Get ADRs for a task
engram relationship connected --entity-id [TASK_ID] --relationship-type documents | grep "ADR"

# Find superseded ADRs (to understand evolution)
engram relationship list --relationship-type supersedes

# Get all ADRs with high confidence
engram reasoning list --min-confidence 0.8 | grep "ADR"
```

## ADR Numbering

**Sequential Numbering:**
- ADR-001, ADR-002, ADR-003, ...
- Never reuse numbers (even for superseded ADRs)
- Gaps in sequence are fine (some ADRs may be deleted)

**Date-Based Numbering:**
- ADR-20260124-database-choice
- Self-documenting (shows chronology)
- No need to track sequence

## ADR Anti-Patterns

**DON'T: Write ADRs After the Fact**

ADRs should be written during decision-making, not months later. The context and trade-offs are fresh.

**DON'T: Write One-Sided ADRs**

Always document alternatives considered, not just the chosen option. This shows due diligence.

**DON'T: Make ADRs Too Technical**

ADRs should be understandable by engineers who join later. Avoid assuming too much context.

**DON'T: Never Revisit ADRs**

Set revisit criteria and dates. Technology evolves, and yesterday's good decision may be today's technical debt.

**DON'T: Skip ADRs for "Obvious" Decisions**

What's obvious to you may not be obvious to others. Document it anyway.

## Example ADR Titles

**Good:**
- ADR-001: Use PostgreSQL for User Service
- ADR-002: Implement JWT-Based Authentication
- ADR-003: Adopt Microservices Architecture
- ADR-004: Use Docker for Local Development
- ADR-005: Shard Database by User ID

**Bad:**
- ADR-001: Database (too vague)
- ADR-002: We Should Use JWT (no context)
- ADR-003: Decision About Microservices (not descriptive)

## Related Skills

This skill integrates with:
- `engram-system-design` - ADRs document design decisions
- `engram-api-design` - API design choices warrant ADRs
- `engram-security-architecture` - Security decisions should be ADRs
- `engram-data-modeling` - Database design decisions warrant ADRs
- `engram-risk-assessment` - ADRs document risk mitigation strategies
- `engram-use-engram-memory` - ADRs stored in Engram for future reference
