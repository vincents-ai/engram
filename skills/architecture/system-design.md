---
name: engram-system-design
description: "Design distributed systems considering CAP theorem, consistency models, fault tolerance, and scalability patterns."
---

# System Design (Engram-Integrated)

## Overview

Design distributed systems and complex architectures by systematically evaluating trade-offs in consistency, availability, partition tolerance, and scalability. Store design decisions, architecture diagrams, and trade-off analysis in Engram for long-term reference and evolution.

## When to Use

Use this skill when:
- Designing a new service or system from scratch
- Scaling an existing system beyond single-machine capacity
- Adding distributed features (caching, message queues, replication)
- Evaluating consistency models for data access patterns
- Making architectural decisions with long-term implications
- A feature requires cross-service coordination or data synchronization

## The Pattern

### Step 1: Define System Requirements

Start with functional and non-functional requirements:

```bash
engram context create \
  --title "System Requirements: [System Name]" \
  --content "## Functional Requirements\n\n**What the system must do:**\n1. [Requirement 1] - [Description]\n2. [Requirement 2] - [Description]\n\n## Non-Functional Requirements\n\n**Performance:**\n- Latency: [e.g., p99 < 100ms]\n- Throughput: [e.g., 10K requests/sec]\n\n**Scalability:**\n- Users: [e.g., 1M daily active users]\n- Data: [e.g., 100TB stored data]\n- Growth: [e.g., 2x per year]\n\n**Availability:**\n- Uptime: [e.g., 99.9% SLA]\n- RTO: [Recovery Time Objective]\n- RPO: [Recovery Point Objective]\n\n**Consistency:**\n- Strong consistency: [Required for X]\n- Eventual consistency: [Acceptable for Y]\n\n**Other:**\n- Cost: [Budget constraints]\n- Compliance: [GDPR, HIPAA, etc.]\n- Operability: [Team skills, on-call requirements]" \
  --source "system-design" \
  --tags "system-design,requirements,[system-name]"
```

### Step 2: Apply CAP Theorem Analysis

Evaluate consistency, availability, and partition tolerance trade-offs:

```bash
engram reasoning create \
  --title "CAP Analysis: [System Name]" \
  --task-id [TASK_ID] \
  --content "## CAP Theorem Trade-offs\n\n**Network Partitions:**\n[Will partitions occur? Yes - distributed system / No - single datacenter]\n\n**If Partitions Occur, Choose Two:**\n\n### Option 1: Consistency + Partition Tolerance (CP)\n\n**Behavior:**\n- System becomes unavailable during partition\n- Returns errors or blocks until partition heals\n- All nodes see same data (strong consistency)\n\n**Use When:**\n- Correctness is critical (banking, inventory)\n- Better to fail than show stale data\n- System can tolerate downtime\n\n**Examples:** Etcd, ZooKeeper, HBase\n\n### Option 2: Availability + Partition Tolerance (AP)\n\n**Behavior:**\n- System remains available during partition\n- Nodes may return stale data\n- Eventually converges when partition heals\n\n**Use When:**\n- Uptime is critical (social media, analytics)\n- Stale data is acceptable temporarily\n- User experience degrades but doesn't break\n\n**Examples:** Cassandra, DynamoDB, Riak\n\n### Option 3: Consistency + Availability (CA)\n\n**Behavior:**\n- System cannot tolerate network partitions\n- Only works in single datacenter or reliable network\n- Both consistency and availability when network works\n\n**Use When:**\n- Single datacenter deployment\n- Network is highly reliable\n- Not planning geographic distribution\n\n**Examples:** PostgreSQL (single-leader), MySQL (single-leader)\n\n## Recommendation for [System Name]\n\n**Choice:** [CP/AP/CA]\n\n**Rationale:**\n[Why this choice fits the requirements]\n\n**Implications:**\n- [How this affects user experience]\n- [What monitoring is needed]\n- [What failure modes to handle]\n\n**Confidence:** [0.0-1.0]" \
  --confidence [0.0-1.0] \
  --tags "system-design,cap-theorem,[system-name]"
```

### Step 3: Select Consistency Model

Define read and write consistency semantics:

```bash
engram reasoning create \
  --title "Consistency Model: [System Name]" \
  --task-id [TASK_ID] \
  --content "## Consistency Models Evaluated\n\n### Strong Consistency (Linearizability)\n\n**Guarantee:** All reads see the most recent write\n**Cost:** Higher latency, lower availability\n**Use for:** [Critical data like payment state]\n\n### Sequential Consistency\n\n**Guarantee:** All processes see operations in same order\n**Cost:** Moderate latency, moderate availability\n**Use for:** [Data where order matters but not real-time]\n\n### Causal Consistency\n\n**Guarantee:** Related operations seen in order, unrelated can differ\n**Cost:** Lower latency than sequential, complex implementation\n**Use for:** [Social feeds, comment threads]\n\n### Eventual Consistency\n\n**Guarantee:** All replicas converge eventually (no time bound)\n**Cost:** Lowest latency, highest availability, conflict resolution needed\n**Use for:** [Analytics, caches, views]\n\n### Read-Your-Writes Consistency\n\n**Guarantee:** After write, same client always reads that value\n**Cost:** Moderate latency, session affinity needed\n**Use for:** [User profile data, settings]\n\n## Selected Model\n\n**Primary Model:** [e.g., Eventual consistency]\n\n**Per-Data-Type:**\n- [User account]: Strong consistency (rare updates, critical correctness)\n- [User posts]: Eventual consistency (high volume, acceptable staleness)\n- [Likes count]: Eventual consistency (high volume, approximate is fine)\n- [Payment state]: Strong consistency (critical, low volume)\n\n**Rationale:**\n[Why this mix of consistency models fits the requirements]\n\n**Implementation:**\n- Strong: Use [consensus algorithm, single-leader replication]\n- Eventual: Use [multi-leader, last-write-wins, CRDT]\n\n**Conflict Resolution:**\n[How conflicts are handled for eventual consistency]\n\n**Confidence:** [0.0-1.0]" \
  --confidence [0.0-1.0] \
  --tags "system-design,consistency,[system-name]"
```

### Step 4: Design Architecture Diagram

Create visual architecture and store in Engram:

```bash
engram context create \
  --title "Architecture Diagram: [System Name]" \
  --content "## High-Level Architecture\n\n\`\`\`\n[ASCII diagram or description]\n\n┌──────────┐\n│  Client  │\n└────┬─────┘\n     │\n     ▼\n┌──────────────┐      ┌──────────────┐\n│  API Gateway │─────▶│  Auth Service│\n└──────┬───────┘      └──────────────┘\n       │\n       ├──────────┬──────────┐\n       ▼          ▼          ▼\n  ┌────────┐ ┌────────┐ ┌────────┐\n  │Service1│ │Service2│ │Service3│\n  └───┬────┘ └───┬────┘ └───┬────┘\n      │          │          │\n      └──────────┴──────────┘\n                 │\n                 ▼\n         ┌──────────────┐\n         │   Database   │\n         │  (Primary +  │\n         │   Replicas)  │\n         └──────────────┘\n\`\`\`\n\n## Components\n\n### API Gateway\n- **Role:** Single entry point, routing, rate limiting\n- **Technology:** [Nginx, Kong, AWS ALB]\n- **Scaling:** Stateless, horizontal\n- **Failover:** Active-active, health checks\n\n### Auth Service\n- **Role:** Authentication, authorization, JWT issuing\n- **Technology:** [Custom service, Auth0, Keycloak]\n- **Scaling:** Stateless, horizontal\n- **Data:** User credentials (strongly consistent)\n\n### Service1: [Name]\n- **Role:** [Responsibility]\n- **Technology:** [Language, framework]\n- **Scaling:** [Horizontal/Vertical, constraints]\n- **Data:** [What data it owns]\n- **Dependencies:** [Other services, external APIs]\n\n### Database\n- **Technology:** [PostgreSQL, Cassandra, etc.]\n- **Replication:** [Single-leader, multi-leader, leaderless]\n- **Consistency:** [Strong for writes, eventual for reads from replicas]\n- **Sharding:** [Strategy if applicable]\n- **Backup:** [Strategy and RPO]\n\n## Data Flow\n\n**Write Path:**\n1. Client → API Gateway (validate request)\n2. API Gateway → Auth Service (verify token)\n3. API Gateway → Service1 (route to handler)\n4. Service1 → Database primary (write)\n5. Database → Replicas (async replication)\n6. Service1 → Client (acknowledge)\n\n**Read Path:**\n1. Client → API Gateway\n2. API Gateway → Auth Service (verify token)\n3. API Gateway → Service1 (route to handler)\n4. Service1 → Database replica (read, eventual consistency)\n5. Service1 → Client (return data)\n\n## Failure Modes\n\n**API Gateway down:**\n- Impact: All requests fail\n- Mitigation: Active-active, multiple instances, health checks\n\n**Service1 down:**\n- Impact: Feature X unavailable\n- Mitigation: Multiple instances, graceful degradation\n\n**Database primary down:**\n- Impact: Writes fail, system read-only\n- Mitigation: Auto-failover to replica (30s downtime), split-brain protection\n\n**Network partition:**\n- Impact: [Based on CAP choice]\n- Mitigation: [Circuit breakers, retries, fallback responses]" \
  --source "system-design" \
  --tags "system-design,architecture,[system-name]"
```

### Step 5: Evaluate Scalability Patterns

Document how system scales:

```bash
engram reasoning create \
  --title "Scalability Strategy: [System Name]" \
  --task-id [TASK_ID] \
  --content "## Scaling Dimensions\n\n### Horizontal Scaling (Scale Out)\n\n**What scales horizontally:**\n- [API Gateway]: Stateless, add more instances\n- [Service1, Service2]: Stateless, add more instances\n- [Cache layer]: Consistent hashing, add more nodes\n\n**What does NOT scale horizontally:**\n- [Database primary]: Single-leader replication (writes bottleneck)\n- [Background job coordinator]: Distributed locking needed\n\n### Vertical Scaling (Scale Up)\n\n**Temporary solution for:**\n- [Database]: Upgrade to larger instance until sharding needed\n- [In-memory cache]: More RAM until distributed cache needed\n\n**Limitations:**\n- Single-leader database: [Max size before sharding]\n- Cost: [Linear increase vs horizontal]\n\n## Bottlenecks\n\n**Current:**\n1. [Database write throughput]: Single-leader can handle [N writes/sec]\n2. [Service1 CPU]: Compute-intensive operation takes [N ms]\n\n**At 10x scale:**\n1. [Database]: Will hit write limit at [N users]\n2. [Network]: Cross-AZ traffic costs at [N requests/sec]\n\n**At 100x scale:**\n1. [Database]: Must shard by [user_id]\n2. [Cache]: Must use distributed cache (Redis cluster)\n\n## Sharding Strategy\n\n**When to shard:** [At N TB or M writes/sec]\n\n**Shard key:** [user_id]\n**Reason:** Even distribution, most queries scoped to single user\n\n**Shard count:** [16 initially, can split later]\n\n**Cross-shard queries:**\n- [Query type]: Scatter-gather pattern\n- [Query type]: Denormalize into separate table\n\n## Caching Strategy\n\n**Cache layers:**\n1. [Browser]: Static assets (CDN)\n2. [API Gateway]: Rate limit counters (Redis)\n3. [Service1]: User sessions (Redis)\n4. [Database]: Query results (Application cache)\n\n**Cache invalidation:**\n- [User data]: TTL 5 minutes\n- [Static content]: Invalidate on publish\n- [Aggregates]: Eventual consistency acceptable\n\n## Monitoring\n\n**Key metrics:**\n- Request rate (per service)\n- Latency percentiles (p50, p95, p99)\n- Error rate\n- Database query time\n- Queue depth (if using message queues)\n\n**Alerts:**\n- p99 latency > 500ms\n- Error rate > 1%\n- Database replication lag > 10s\n\n**Confidence:** [0.0-1.0]" \
  --confidence [0.0-1.0] \
  --tags "system-design,scalability,[system-name]"
```

### Step 6: Link All Design Entities

```bash
# Link all design documents to task
engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [REQUIREMENTS_ID] --target-type context \
  --relationship-type references --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [CAP_ANALYSIS_ID] --target-type reasoning \
  --relationship-type documents --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [CONSISTENCY_ID] --target-type reasoning \
  --relationship-type documents --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [ARCHITECTURE_ID] --target-type context \
  --relationship-type references --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [SCALABILITY_ID] --target-type reasoning \
  --relationship-type documents --agent [AGENT]
```

## Example

User wants to design a collaborative document editing system.

### Step 1: Define Requirements

```bash
REQUIREMENTS=$(engram context create \
  --title "System Requirements: Collaborative Document Editor" \
  --content "## Functional Requirements\n\n**What the system must do:**\n1. Real-time collaborative editing - multiple users edit same document simultaneously\n2. Conflict-free merging - edits from different users don't cause data loss\n3. Presence awareness - show who is currently editing\n4. Document persistence - save documents durably\n5. Revision history - view and restore previous versions\n\n## Non-Functional Requirements\n\n**Performance:**\n- Latency: p99 < 100ms for local edits, < 300ms for remote syncs\n- Throughput: 100 edits/sec per document\n\n**Scalability:**\n- Users: 100K concurrent users\n- Documents: 10M documents\n- Growth: 3x per year\n\n**Availability:**\n- Uptime: 99.9% SLA (43 minutes downtime/month)\n- RTO: 5 minutes\n- RPO: 0 (no data loss acceptable)\n\n**Consistency:**\n- Strong consistency: Document persistence (no lost edits)\n- Eventual consistency: Presence info (acceptable staleness)\n- Causal consistency: Edit operations (preserve intent)\n\n**Other:**\n- Cost: Target $0.01 per user per month\n- Compliance: GDPR (data privacy)\n- Operability: Small team (2-3 engineers on-call)" \
  --source "system-design" \
  --tags "system-design,requirements,collab-editor" \
  --json | jq -r '.id')
```

### Step 2: CAP Analysis

```bash
CAP=$(engram reasoning create \
  --title "CAP Analysis: Collaborative Document Editor" \
  --task-id a3f8b2c1-1234-5678-90ab-cdef12345678 \
  --content "## CAP Theorem Trade-offs\n\n**Network Partitions:**\nYes - distributed system with users globally, partitions will occur.\n\n**If Partitions Occur, Choose Two:**\n\n## Recommendation: AP (Availability + Partition Tolerance)\n\n**Rationale:**\nCollaborative editing requires high availability - users must be able to continue editing even during network issues. Document convergence is more important than immediate consistency. Better to merge conflicts later than block users from editing.\n\n**Behavior During Partition:**\n- Users continue editing locally (offline mode)\n- Edits queue locally in browser\n- When partition heals, edits sync and merge using CRDT\n- Conflicts resolved automatically (CRDTs are conflict-free)\n\n**Trade-off:**\n- May see brief divergence during partition\n- Eventual consistency means users may see different views temporarily\n- Presence info may be stale during partition\n\n**Implications:**\n- Use CRDT for conflict-free merging (Yjs, Automerge)\n- Implement local-first architecture (IndexedDB for persistence)\n- Show \"Syncing...\" indicator when offline\n- Test partition scenarios frequently\n\n**Alternative Considered:**\nCP (Consistency + Partition): Would block editing during partition, unacceptable UX for collaborative editing.\n\n**Confidence:** 0.85" \
  --confidence 0.85 \
  --tags "system-design,cap-theorem,collab-editor" \
  --json | jq -r '.id')
```

### Step 3: Consistency Model

```bash
CONSISTENCY=$(engram reasoning create \
  --title "Consistency Model: Collaborative Document Editor" \
  --task-id a3f8b2c1-1234-5678-90ab-cdef12345678 \
  --content "## Consistency Models Evaluated\n\n## Selected Model\n\n**Primary Model:** Causal consistency + eventual convergence (via CRDT)\n\n**Per-Data-Type:**\n- Document edits: Causal consistency - preserve editing intent and causality\n- Document metadata (title, owner): Strong consistency - rare updates, single source\n- Presence (who's editing): Eventual consistency - high frequency, approximate is fine\n- Revision snapshots: Strong consistency - immutable once created\n\n**Rationale:**\nCausal consistency ensures that if Edit B depends on Edit A (e.g., user reads updated text then replies), all users see them in that order. For independent edits, ordering can differ without issue. CRDT ensures convergence - all replicas reach same state eventually.\n\n**Implementation:**\n- Edits: Yjs CRDT library handles causality and convergence\n- Metadata: PostgreSQL single-leader for strong consistency\n- Presence: WebSocket broadcast, last-update-wins\n- Revisions: Immutable snapshots in object storage (S3)\n\n**Conflict Resolution:**\nCRDT guarantees conflict-free merging:\n- Concurrent inserts at same position: Deterministic ordering by client ID\n- Concurrent delete and update: Delete wins (deleted text stays deleted)\n- Format conflicts (bold vs italic): Last-writer-wins per character\n\n**User Experience:**\n- Edits appear immediately locally (optimistic UI)\n- Remote edits arrive within 300ms (p99)\n- No \"merge conflict\" dialogs - automatic resolution\n- Show other users' cursors to reduce collision\n\n**Confidence:** 0.90" \
  --confidence 0.90 \
  --tags "system-design,consistency,collab-editor" \
  --json | jq -r '.id')
```

### Step 4: Architecture Diagram

```bash
ARCHITECTURE=$(engram context create \
  --title "Architecture Diagram: Collaborative Document Editor" \
  --content "## High-Level Architecture\n\n\`\`\`\n┌─────────────┐\n│   Browser   │ (Yjs CRDT, IndexedDB)\n└──────┬──────┘\n       │ WebSocket (bidirectional sync)\n       ▼\n┌──────────────────┐\n│  WebSocket Server│ (Stateful, sticky sessions)\n│  (Node.js)       │\n└────┬─────────────┘\n     │\n     ├──────────────┬──────────────┐\n     │              │              │\n     ▼              ▼              ▼\n┌─────────┐  ┌──────────┐  ┌───────────┐\n│  Redis  │  │ Document │  │ Revision  │\n│Pub/Sub  │  │ Service  │  │ Service   │\n│(Presence)│  │ (REST)   │  │ (REST)    │\n└─────────┘  └────┬─────┘  └─────┬─────┘\n                  │              │\n                  ▼              ▼\n           ┌─────────────┐ ┌──────────┐\n           │ PostgreSQL  │ │    S3    │\n           │ (Metadata)  │ │(Snapshots)│\n           └─────────────┘ └──────────┘\n\`\`\`\n\n## Components\n\n### Browser (Client)\n- **Role:** Local-first editing, CRDT state, offline support\n- **Technology:** Yjs (CRDT), IndexedDB (persistence), WebSocket client\n- **Scaling:** N/A (client-side)\n- **Data:** Full document CRDT state, local edits queue\n\n### WebSocket Server\n- **Role:** Relay edits between clients, broadcast presence\n- **Technology:** Node.js, ws library, Yjs sync protocol\n- **Scaling:** Horizontal with sticky sessions (user pinned to instance)\n- **State:** Active document connections, user presence\n- **Failover:** Reconnect to different instance, sync from last known state\n\n### Redis Pub/Sub\n- **Role:** Cross-instance communication for presence broadcast\n- **Technology:** Redis 7+\n- **Scaling:** Single instance (presence is ephemeral)\n- **Failover:** Acceptable to lose presence on restart\n\n### Document Service\n- **Role:** CRUD for document metadata, permissions\n- **Technology:** Rust, actix-web, PostgreSQL client\n- **Scaling:** Stateless, horizontal\n- **Data:** document_id, title, owner, created_at, updated_at\n\n### Revision Service\n- **Role:** Create and retrieve document snapshots\n- **Technology:** Rust, actix-web, S3 client\n- **Scaling:** Stateless, horizontal\n- **Data:** Immutable CRDT snapshots in S3\n\n### PostgreSQL\n- **Technology:** PostgreSQL 15\n- **Replication:** Single-leader with 2 read replicas\n- **Consistency:** Strong for writes, eventual for replica reads\n- **Backup:** Daily full backup, WAL archiving (RPO: 0)\n\n### S3\n- **Technology:** AWS S3\n- **Purpose:** Immutable revision snapshots\n- **Lifecycle:** Archive to Glacier after 90 days\n\n## Data Flow\n\n**Edit Path (Real-time Collaboration):**\n1. User types in browser → Yjs CRDT updates local state\n2. Browser → WebSocket Server (send edit delta)\n3. WebSocket Server → Other connected clients (broadcast delta)\n4. Other clients → Apply delta to local CRDT state\n5. Periodically: Browser → IndexedDB (persist local state)\n6. Periodically: WebSocket Server → Revision Service (snapshot for recovery)\n\n**Presence Path:**\n1. Browser → WebSocket Server (\"User X is editing\")\n2. WebSocket Server → Redis Pub/Sub (broadcast to all instances)\n3. Other WebSocket instances → Connected clients (show User X cursor)\n\n**Document Metadata Path:**\n1. Browser → Document Service (REST: create/update document)\n2. Document Service → PostgreSQL (write metadata)\n3. Document Service → Browser (acknowledge)\n\n**Revision Retrieval Path:**\n1. Browser → Revision Service (GET /revisions/:doc_id/:version)\n2. Revision Service → S3 (fetch snapshot)\n3. Revision Service → Browser (return snapshot)\n4. Browser → Load snapshot into Yjs CRDT\n\n## Failure Modes\n\n**WebSocket Server down:**\n- Impact: Users disconnected, editing continues locally\n- Mitigation: Auto-reconnect, sticky session to different instance, sync queued edits\n\n**Redis down:**\n- Impact: Presence info not shared across instances (degraded UX)\n- Mitigation: Acceptable - users still see presence on same instance\n\n**PostgreSQL primary down:**\n- Impact: Cannot create new documents or update metadata (read-only for metadata)\n- Mitigation: Failover to replica (30s), editing continues (uses cached metadata)\n\n**Network partition (browser to server):**\n- Impact: User edits locally in offline mode\n- Mitigation: IndexedDB persistence, auto-sync when reconnected, CRDT merges conflicts\n\n**Confidence:** 0.85" \
  --source "system-design" \
  --tags "system-design,architecture,collab-editor" \
  --json | jq -r '.id')
```

### Step 5: Scalability Strategy

```bash
SCALABILITY=$(engram reasoning create \
  --title "Scalability Strategy: Collaborative Document Editor" \
  --task-id a3f8b2c1-1234-5678-90ab-cdef12345678 \
  --content "## Scaling Dimensions\n\n### Horizontal Scaling (Scale Out)\n\n**What scales horizontally:**\n- WebSocket Server: Add more instances, use load balancer with sticky sessions\n- Document Service: Stateless, add more instances\n- Revision Service: Stateless, add more instances\n- PostgreSQL read replicas: Add more for read scaling\n\n**What does NOT scale horizontally:**\n- PostgreSQL primary: Single-leader (write bottleneck at ~10K writes/sec)\n- Redis Pub/Sub: Single instance (not a bottleneck for presence)\n\n### Vertical Scaling (Scale Up)\n\n**Temporary solution for:**\n- PostgreSQL: Upgrade instance until sharding needed (~1M documents)\n- WebSocket Server: More RAM for more connections per instance\n\n## Bottlenecks\n\n**Current (100K users):**\n1. WebSocket connections: 10K connections per instance (100 instances needed)\n2. PostgreSQL writes: Metadata updates at ~100 writes/sec (well below limit)\n\n**At 10x scale (1M users):**\n1. WebSocket: Need 1000 instances or optimize connections (consider WebRTC mesh)\n2. PostgreSQL: Will hit write limit if metadata updates increase proportionally\n\n**At 100x scale (10M users):**\n1. Must shard PostgreSQL by document_id or user_id\n2. Consider separate CRDT sync servers per region\n3. CDN for static assets becomes critical\n\n## Sharding Strategy\n\n**When to shard:** At 1M documents or 1K metadata writes/sec\n\n**Shard key:** document_id (consistent hash)\n**Reason:** Most queries scoped to single document, even distribution\n\n**Shard count:** 16 initially (64K docs per shard)\n\n**Cross-shard queries:**\n- List user's documents: Scatter-gather to all shards (cached)\n- Search: Use separate Elasticsearch cluster\n\n## Caching Strategy\n\n**Cache layers:**\n1. Browser: CRDT state in memory + IndexedDB\n2. WebSocket Server: Active document connections (in-memory)\n3. Document Service: Metadata cache (Redis, 5 min TTL)\n4. CDN: Static assets (CloudFront)\n\n**Cache invalidation:**\n- Document metadata: Invalidate on update (pub/sub)\n- CRDT state: Never cached server-side (always in-memory at client)\n\n## Cost Optimization\n\n**Current cost per user:**\n- WebSocket Server: $0.002/mo (100 users per $0.20 instance)\n- PostgreSQL: $0.003/mo (100K users per $300/mo instance)\n- S3 storage: $0.001/mo (10 MB per user at $0.023/GB)\n- **Total: $0.006/user/mo** (below $0.01 target)\n\n**Optimization opportunities:**\n- Use Glacier for old revisions (90+ days)\n- Compress CRDT snapshots (50% reduction)\n- Shut down WebSocket instances during low traffic\n\n## Monitoring\n\n**Key metrics:**\n- WebSocket connections per instance\n- Edit latency (local and remote)\n- Sync lag (client to server)\n- PostgreSQL query time\n- S3 upload/download time\n- Redis Pub/Sub latency\n\n**Alerts:**\n- WebSocket connections > 15K per instance (scale out)\n- Remote edit latency p99 > 500ms\n- PostgreSQL replication lag > 10s\n- Sync failures > 1% (network issues)\n\n**Confidence:** 0.80" \
  --confidence 0.80 \
  --tags "system-design,scalability,collab-editor" \
  --json | jq -r '.id')
```

### Step 6: Link Everything

```bash
engram relationship create \
  --source-id a3f8b2c1-1234-5678-90ab-cdef12345678 --source-type task \
  --target-id $REQUIREMENTS --target-type context \
  --relationship-type references --agent default

engram relationship create \
  --source-id a3f8b2c1-1234-5678-90ab-cdef12345678 --source-type task \
  --target-id $CAP --target-type reasoning \
  --relationship-type documents --agent default

engram relationship create \
  --source-id a3f8b2c1-1234-5678-90ab-cdef12345678 --source-type task \
  --target-id $CONSISTENCY --target-type reasoning \
  --relationship-type documents --agent default

engram relationship create \
  --source-id a3f8b2c1-1234-5678-90ab-cdef12345678 --source-type task \
  --target-id $ARCHITECTURE --target-type context \
  --relationship-type references --agent default

engram relationship create \
  --source-id a3f8b2c1-1234-5678-90ab-cdef12345678 --source-type task \
  --target-id $SCALABILITY --target-type reasoning \
  --relationship-type documents --agent default
```

### Step 7: Communicate to User

Agent presents system design:

"I've designed the collaborative document editor as an AP system (prioritizing availability during partitions). Using Yjs CRDT for conflict-free merging with causal consistency. Architecture is local-first with WebSocket sync and PostgreSQL for metadata. Scales horizontally to 1M users before sharding needed. Cost: $0.006/user/mo (below target). All design docs in Engram. Ready to review architecture?"

## Querying System Design

After creating system design, agents can retrieve:

```bash
# Get all design documents for a system
engram relationship connected --entity-id [TASK_ID] | grep -E "Requirements|Architecture"

# Get design decisions
engram relationship connected --entity-id [TASK_ID] | grep -E "CAP|Consistency|Scalability"

# Get all system design contexts
engram context list | grep "System"

# Get CAP analyses across systems
engram reasoning list | grep "CAP Analysis"
```

## Related Skills

This skill integrates with:
- `engram-brainstorming` - Use during design exploration phase
- `engram-risk-assessment` - Assess architectural risks (data loss, downtime)
- `engram-spike-investigation` - Validate technology choices with prototypes
- `engram-dependency-mapping` - Map service dependencies and integration points
- `engram-writing-plans` - Create implementation plan from architecture
