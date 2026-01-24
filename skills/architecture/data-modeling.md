---
name: engram-data-modeling
description: "Design database schemas, define relationships, plan indexes, design migrations for schema evolution, and ensure backward compatibility."
---

# Data Modeling (Engram-Integrated)

## Overview

Design robust database schemas by systematically modeling entities, relationships, access patterns, and constraints. Plan schema migrations with versioning and backward compatibility. Store data models, normalization decisions, and migration strategies in Engram for long-term evolution.

## When to Use

Use this skill when:
- Starting a new project and defining initial schema
- Adding new features that require new data entities
- Refactoring existing schema for performance or clarity
- Planning a database migration (version upgrade, cloud migration)
- Choosing between SQL and NoSQL databases
- Optimizing queries by designing indexes
- Ensuring data integrity with constraints and validation

## The Pattern

### Step 1: Identify Entities and Attributes

Start by documenting core entities and their properties:

```bash
engram context create \
  --title "Data Model: [System Name] - Entities" \
  --content "## Core Entities\n\n### Entity: User\n\n**Description:** Registered user of the system\n\n**Attributes:**\n- id (UUID, primary key)\n- email (string, unique, not null)\n- password_hash (string, not null)\n- name (string, nullable)\n- created_at (timestamp, not null, default now())\n- updated_at (timestamp, not null, default now())\n- deleted_at (timestamp, nullable) -- soft delete\n\n**Access Patterns:**\n1. Find by id (primary key lookup)\n2. Find by email (unique index)\n3. List active users (filter deleted_at IS NULL)\n\n**Constraints:**\n- email must be valid format (CHECK constraint)\n- password_hash must be bcrypt/argon2 format\n\n**Estimated Volume:** 1M users, growth 10K/month\n\n### Entity: Document\n\n**Description:** User-created document\n\n**Attributes:**\n- id (UUID, primary key)\n- owner_id (UUID, foreign key → users.id)\n- title (string, not null)\n- content (text, nullable)\n- visibility (enum: private, public, shared)\n- created_at (timestamp, not null)\n- updated_at (timestamp, not null)\n\n**Access Patterns:**\n1. Find by id (primary key)\n2. List by owner_id (foreign key index)\n3. List public documents (filter visibility = 'public')\n4. Full-text search on title + content\n\n**Constraints:**\n- owner_id must reference existing user\n- visibility must be one of (private, public, shared)\n\n**Estimated Volume:** 10M documents, growth 100K/month\n\n### Entity: Share\n\n**Description:** Document shared with another user\n\n**Attributes:**\n- id (UUID, primary key)\n- document_id (UUID, foreign key → documents.id)\n- user_id (UUID, foreign key → users.id)\n- permission (enum: read, write)\n- created_at (timestamp, not null)\n\n**Access Patterns:**\n1. Find shares by document_id (index)\n2. Find shares by user_id (index)\n3. Check if user can access document (composite index: document_id, user_id)\n\n**Constraints:**\n- Unique (document_id, user_id) -- can't share same doc twice to same user\n- permission must be one of (read, write)\n\n**Estimated Volume:** 1M shares, growth 10K/month" \
  --source "data-modeling" \
  --tags "data-model,entities,[system-name]"
```

### Step 2: Define Relationships

Map how entities connect:

```bash
engram reasoning create \
  --title "Data Model: [System Name] - Relationships" \
  --task-id [TASK_ID] \
  --content "## Entity Relationships\n\n### Relationship Type: One-to-Many\n\n**User → Documents**\n\n**Cardinality:** One user owns many documents\n\n**Implementation:**\n- documents.owner_id references users.id\n- Foreign key constraint (CASCADE on delete? or SET NULL?)\n\n**Decision: CASCADE DELETE**\n\nWhen user deleted, delete all their documents.\n\n**Rationale:**\n- GDPR right to deletion requires removing all user data\n- Orphaned documents have no owner (who can access?)\n\n**Alternative Considered:**\nSET NULL: Keep documents but mark owner as NULL. Rejected because documents without owner violate business logic.\n\n### Relationship Type: Many-to-Many\n\n**Documents ↔ Users (Sharing)**\n\n**Cardinality:** \n- One document can be shared with many users\n- One user can have many documents shared with them\n\n**Implementation:**\n- Join table: shares (document_id, user_id, permission)\n- Foreign keys to both documents and users\n\n**Delete Behavior:**\n- Document deleted → CASCADE delete shares (no orphan shares)\n- User deleted → CASCADE delete shares (revoke access)\n\n### Relationship Type: Self-Referential\n\n**Document → Document (Versions)**\n\n**Cardinality:** One document may have one parent (previous version)\n\n**Implementation:**\n\n```sql\nALTER TABLE documents ADD COLUMN parent_id UUID REFERENCES documents(id) ON DELETE SET NULL;\n```\n\n**Delete Behavior:**\nSET NULL: If parent deleted, child survives (becomes root version).\n\n**Query:**\n\n```sql\n-- Get version history\nWITH RECURSIVE versions AS (\n  SELECT * FROM documents WHERE id = ?\n  UNION ALL\n  SELECT d.* FROM documents d\n  JOIN versions v ON d.id = v.parent_id\n)\nSELECT * FROM versions;\n```\n\n### Relationship Type: One-to-One\n\n**User → UserProfile**\n\n**Cardinality:** One user has one profile\n\n**Implementation Option 1: Separate Table**\n\n```sql\nCREATE TABLE user_profiles (\n  user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,\n  bio TEXT,\n  avatar_url TEXT,\n  timezone TEXT\n);\n```\n\n**Implementation Option 2: Same Table**\n\n```sql\nALTER TABLE users ADD COLUMN bio TEXT;\nALTER TABLE users ADD COLUMN avatar_url TEXT;\n```\n\n**Decision: Same Table**\n\n**Rationale:**\n- Profile always loaded with user (not lazy-loaded)\n- Simpler queries (no join needed)\n- Fewer tables to maintain\n\n**Trade-off:**\n- If profile becomes large (many fields), separate table better for performance\n\n## Referential Integrity\n\n**Enforce at Database Level:**\n\n```sql\nALTER TABLE documents\nADD CONSTRAINT fk_owner\nFOREIGN KEY (owner_id) REFERENCES users(id)\nON DELETE CASCADE;\n```\n\n**Why Database-Level:**\n- Ensures integrity even if application has bugs\n- Prevents orphaned records\n- Single source of truth for constraints\n\n**Application-Level Checks:**\n\nStill validate in app for better error messages:\n\n```rust\nif !user_exists(owner_id) {\n  return Err(\"User not found\");\n}\n```\n\n**Confidence:** [0.0-1.0]" \
  --confidence [0.0-1.0] \
  --tags "data-model,relationships,[system-name]"
```

### Step 3: Normalize or Denormalize

Decide on normalization level:

```bash
engram reasoning create \
  --title "Data Model: [System Name] - Normalization Strategy" \
  --task-id [TASK_ID] \
  --content "## Normalization Analysis\n\n### 1NF (First Normal Form)\n\n**Rule:** No repeating groups, atomic values\n\n**Violation Example:**\n\n```sql\n-- BAD: tags as comma-separated string\nCREATE TABLE documents (\n  id UUID,\n  title TEXT,\n  tags TEXT  -- \"travel,food,photography\"\n);\n```\n\n**Problem:**\n- Can't query \"documents with tag 'travel'\" efficiently\n- Can't add constraints (tag must be from allowed list)\n\n**Solution: Separate Table**\n\n```sql\nCREATE TABLE tags (\n  id UUID PRIMARY KEY,\n  name TEXT UNIQUE\n);\n\nCREATE TABLE document_tags (\n  document_id UUID REFERENCES documents(id),\n  tag_id UUID REFERENCES tags(id),\n  PRIMARY KEY (document_id, tag_id)\n);\n```\n\n**Benefit:**\n- Query \"documents with tag X\" is simple join\n- Tag uniqueness enforced\n- Can add tag metadata (description, color)\n\n### 2NF (Second Normal Form)\n\n**Rule:** No partial dependencies (all non-key attributes depend on entire primary key)\n\n**Violation Example:**\n\n```sql\n-- BAD: Composite key but name depends only on user_id\nCREATE TABLE shares (\n  document_id UUID,\n  user_id UUID,\n  user_name TEXT,  -- Only depends on user_id (partial dependency)\n  permission TEXT,\n  PRIMARY KEY (document_id, user_id)\n);\n```\n\n**Problem:**\n- User name duplicated for every share\n- Update anomaly: If user changes name, must update all shares\n\n**Solution: Reference User Table**\n\n```sql\nCREATE TABLE shares (\n  document_id UUID,\n  user_id UUID REFERENCES users(id),\n  permission TEXT,\n  PRIMARY KEY (document_id, user_id)\n);\n\n-- user_name comes from JOIN with users table\n```\n\n### 3NF (Third Normal Form)\n\n**Rule:** No transitive dependencies (non-key attributes depend only on primary key)\n\n**Violation Example:**\n\n```sql\n-- BAD: country_name depends on country_code (transitive dependency)\nCREATE TABLE users (\n  id UUID PRIMARY KEY,\n  email TEXT,\n  country_code TEXT,\n  country_name TEXT  -- Depends on country_code, not id\n);\n```\n\n**Problem:**\n- Country name duplicated for all users in same country\n- Update anomaly: If country renames, must update all users\n\n**Solution: Separate Countries Table**\n\n```sql\nCREATE TABLE countries (\n  code TEXT PRIMARY KEY,\n  name TEXT\n);\n\nCREATE TABLE users (\n  id UUID PRIMARY KEY,\n  email TEXT,\n  country_code TEXT REFERENCES countries(code)\n);\n```\n\n## When to Denormalize\n\n**Trade-off:** Normalization reduces redundancy but increases joins.\n\n### Denormalization Pattern 1: Cached Counts\n\n**Scenario:** Show document count per user\n\n**Normalized:**\n\n```sql\nSELECT u.id, u.email, COUNT(d.id) AS doc_count\nFROM users u\nLEFT JOIN documents d ON d.owner_id = u.id\nGROUP BY u.id;\n```\n\n**Problem:** Slow for large tables (must scan all documents)\n\n**Denormalized:**\n\n```sql\nALTER TABLE users ADD COLUMN document_count INT DEFAULT 0;\n\n-- Update on insert/delete\nCREATE TRIGGER update_doc_count\nAFTER INSERT ON documents\nFOR EACH ROW\nEXECUTE FUNCTION increment_user_doc_count();\n```\n\n**Trade-off:**\n- Pro: Fast reads (no join, no count)\n- Con: Complex writes (must maintain trigger)\n- Con: Can drift out of sync (must periodically reconcile)\n\n**Decision: Denormalize**\n\nRationale: Read-heavy workload (users frequently list documents), acceptable to be slightly stale.\n\n### Denormalization Pattern 2: Embed Related Data (JSON)\n\n**Scenario:** Store document metadata (tags, share count)\n\n**Normalized:**\n\nQuery requires 2 joins (tags, shares).\n\n**Denormalized:**\n\n```sql\nALTER TABLE documents ADD COLUMN metadata JSONB;\n\n-- Store frequently accessed data\n{\n  \"tags\": [\"travel\", \"food\"],\n  \"share_count\": 5,\n  \"last_accessed\": \"2026-01-24T12:00:00Z\"\n}\n```\n\n**Trade-off:**\n- Pro: Single query (no joins)\n- Con: Redundant data (tags in both tags table and JSON)\n- Con: Must update JSON when tags change\n\n**Decision: Don't Denormalize**\n\nRationale: Tags change infrequently but queried frequently. Better to keep normalized and add index.\n\n## Final Decision\n\n**Normalization Level:** 3NF for most tables\n\n**Denormalization Exceptions:**\n- Cached counts (document_count in users table)\n- Audit fields (created_by_email in addition to created_by_id)\n\n**Rationale:**\n3NF balances integrity and performance. Denormalize only where read performance critical and data changes infrequently.\n\n**Confidence:** [0.0-1.0]" \
  --confidence [0.0-1.0] \
  --tags "data-model,normalization,[system-name]"
```

### Step 4: Design Indexes

Plan indexes based on access patterns:

```bash
engram reasoning create \
  --title "Data Model: [System Name] - Index Strategy" \
  --task-id [TASK_ID] \
  --content "## Index Design\n\n### Primary Indexes (Automatic)\n\n**Users:**\n- PRIMARY KEY (id) → B-tree index\n\n**Documents:**\n- PRIMARY KEY (id) → B-tree index\n\n### Secondary Indexes (Must Create)\n\n**Users Table:**\n\n```sql\nCREATE UNIQUE INDEX idx_users_email ON users(email);\n```\n\n**Rationale:**\n- Login query: WHERE email = ? (executed frequently)\n- Unique constraint enforces business rule\n- B-tree index enables O(log n) lookup\n\n**Documents Table:**\n\n```sql\nCREATE INDEX idx_documents_owner_id ON documents(owner_id);\n```\n\n**Rationale:**\n- Query: Find all documents by user (SELECT * FROM documents WHERE owner_id = ?)\n- Common access pattern (user views their documents)\n\n```sql\nCREATE INDEX idx_documents_visibility ON documents(visibility) WHERE visibility = 'public';\n```\n\n**Rationale:**\n- Query: Find all public documents (explore page)\n- Partial index (only index public docs, smaller index)\n- WHERE clause makes index even faster\n\n**Shares Table:**\n\n```sql\nCREATE INDEX idx_shares_user_id ON shares(user_id);\nCREATE INDEX idx_shares_document_id ON shares(document_id);\nCREATE UNIQUE INDEX idx_shares_doc_user ON shares(document_id, user_id);\n```\n\n**Rationale:**\n- Query 1: Find documents shared with user (WHERE user_id = ?)\n- Query 2: Find who document is shared with (WHERE document_id = ?)\n- Query 3: Check specific permission (WHERE document_id = ? AND user_id = ?)\n- Composite index serves both Query 3 and Query 2 (leftmost prefix)\n\n### Full-Text Search Index\n\n**Documents Table:**\n\n```sql\nALTER TABLE documents ADD COLUMN search_vector tsvector;\n\nCREATE INDEX idx_documents_search ON documents USING GIN(search_vector);\n\nCREATE TRIGGER update_search_vector\nBEFORE INSERT OR UPDATE ON documents\nFOR EACH ROW\nEXECUTE FUNCTION\n  tsvector_update_trigger(search_vector, 'pg_catalog.english', title, content);\n```\n\n**Rationale:**\n- Query: Full-text search on title and content\n- GIN index enables fast text search (vs LIKE '%keyword%')\n- Trigger keeps search_vector in sync automatically\n\n### Index on JSON Column\n\n**Documents Table (if using JSONB metadata):**\n\n```sql\nCREATE INDEX idx_documents_metadata ON documents USING GIN(metadata);\n\n-- Query: Find documents with specific tag in metadata\nSELECT * FROM documents WHERE metadata @> '{\"tags\": [\"travel\"]}';\n```\n\n**Rationale:**\n- GIN index enables fast JSON containment queries\n- Without index, must scan entire table\n\n## Index Trade-offs\n\n**Benefit:**\n- Fast reads (O(log n) instead of O(n) table scan)\n\n**Cost:**\n- Slower writes (must update index on INSERT/UPDATE/DELETE)\n- Storage space (index size can exceed table size)\n- Maintenance (must VACUUM and ANALYZE regularly)\n\n**Rule of Thumb:**\n- Index columns in WHERE clauses\n- Index foreign keys (for joins)\n- Index columns used in ORDER BY\n- Don't index low-cardinality columns (boolean, gender)\n- Don't index columns that change frequently\n\n## Index Monitoring\n\n**Unused Indexes:**\n\n```sql\nSELECT schemaname, tablename, indexname, idx_scan\nFROM pg_stat_user_indexes\nWHERE idx_scan = 0\n  AND indexname NOT LIKE 'pg_toast%';\n```\n\n**Action:** Drop unused indexes (wasting space and slowing writes)\n\n**Missing Indexes:**\n\n```sql\nSELECT schemaname, tablename, attname, n_distinct, correlation\nFROM pg_stats\nWHERE schemaname = 'public'\n  AND n_distinct > 100  -- High cardinality\n  AND correlation < 0.1;  -- Not clustered\n```\n\n**Action:** Analyze slow queries (EXPLAIN ANALYZE), add indexes\n\n## Composite Index Order\n\n**Rule:** Most selective column first\n\n**Example:**\n\n```sql\n-- Query: WHERE status = 'active' AND created_at > '2026-01-01'\n-- Selectivity: status (50% active), created_at (5% match)\n\n-- GOOD: More selective column first\nCREATE INDEX idx_documents_created_status ON documents(created_at, status);\n\n-- BAD: Less selective column first\nCREATE INDEX idx_documents_status_created ON documents(status, created_at);\n```\n\n**Rationale:** Index can narrow down results faster with more selective column.\n\n**Confidence:** [0.0-1.0]" \
  --confidence [0.0-1.0] \
  --tags "data-model,indexes,[system-name]"
```

### Step 5: Plan Schema Migrations

Design migration strategy for schema evolution:

```bash
engram reasoning create \
  --title "Data Model: [System Name] - Migration Strategy" \
  --task-id [TASK_ID] \
  --content "## Migration Philosophy\n\n**Zero-Downtime Deployments:**\nSchema changes must not break running application instances.\n\n**Backward Compatibility:**\nOld code must work with new schema (during rolling deploy).\n\n**Forward Compatibility:**\nNew code must work with old schema (for rollback).\n\n## Migration Patterns\n\n### Pattern 1: Add Nullable Column\n\n**Safe:** Yes (backward compatible)\n\n```sql\n-- Migration 001: Add bio column\nALTER TABLE users ADD COLUMN bio TEXT NULL;\n```\n\n**Compatibility:**\n- Old code: Ignores new column (works fine)\n- New code: Can read/write bio (works fine)\n\n**Rollback:**\n\n```sql\nALTER TABLE users DROP COLUMN bio;\n```\n\n### Pattern 2: Add Non-Nullable Column\n\n**Unsafe:** Breaks old code (INSERT fails without new column)\n\n**Solution: Expand-Contract Pattern**\n\n**Step 1 (Expand):** Add column as nullable with default\n\n```sql\n-- Migration 002\nALTER TABLE users ADD COLUMN timezone TEXT DEFAULT 'UTC';\n```\n\n**Deploy:** Application now can set timezone (but doesn't require it yet)\n\n**Step 2 (Contract):** Make non-nullable\n\n```sql\n-- Migration 003 (after all instances deployed)\nUPDATE users SET timezone = 'UTC' WHERE timezone IS NULL;\nALTER TABLE users ALTER COLUMN timezone SET NOT NULL;\n```\n\n### Pattern 3: Rename Column\n\n**Unsafe:** Breaks old code (column not found)\n\n**Solution: 3-Step Process**\n\n**Step 1:** Add new column, copy data\n\n```sql\n-- Migration 004\nALTER TABLE users ADD COLUMN full_name TEXT;\nUPDATE users SET full_name = name;\nCREATE TRIGGER sync_name_to_full_name\n  AFTER UPDATE ON users\n  FOR EACH ROW\n  EXECUTE FUNCTION copy_name_to_full_name();\n```\n\n**Deploy:** Application uses both columns (writes to both)\n\n**Step 2:** Update application to use new column\n\n**Deploy:** Application only uses full_name\n\n**Step 3:** Drop old column\n\n```sql\n-- Migration 005\nDROP TRIGGER sync_name_to_full_name;\nALTER TABLE users DROP COLUMN name;\n```\n\n### Pattern 4: Change Column Type\n\n**Unsafe:** May require table rewrite (downtime)\n\n**Solution: Shadow Column Pattern**\n\n**Scenario:** Change user_id from INTEGER to UUID\n\n**Step 1:** Add new column\n\n```sql\n-- Migration 006\nALTER TABLE users ADD COLUMN id_uuid UUID DEFAULT gen_random_uuid();\nCREATE UNIQUE INDEX idx_users_id_uuid ON users(id_uuid);\n\n-- Backfill existing rows\nUPDATE users SET id_uuid = gen_random_uuid() WHERE id_uuid IS NULL;\n```\n\n**Step 2:** Update application to use both IDs\n\nApp writes both id (INT) and id_uuid (UUID) during transition.\n\n**Step 3:** Swap primary key\n\n```sql\n-- Migration 007\nALTER TABLE users DROP CONSTRAINT users_pkey;\nALTER TABLE users ADD PRIMARY KEY (id_uuid);\nALTER TABLE users ALTER COLUMN id DROP NOT NULL;\n```\n\n**Step 4:** Update foreign keys\n\n```sql\n-- Migration 008\nALTER TABLE documents ADD COLUMN owner_id_uuid UUID REFERENCES users(id_uuid);\nUPDATE documents SET owner_id_uuid = (SELECT id_uuid FROM users WHERE id = documents.owner_id);\n```\n\n**Step 5:** Drop old column\n\n```sql\n-- Migration 009\nALTER TABLE users DROP COLUMN id;\nALTER TABLE users RENAME COLUMN id_uuid TO id;\nALTER TABLE documents DROP COLUMN owner_id;\nALTER TABLE documents RENAME COLUMN owner_id_uuid TO owner_id;\n```\n\n### Pattern 5: Drop Column\n\n**Safe:** If column not used by any application version\n\n**Process:**\n1. Stop writing to column (deploy application)\n2. Wait for all instances to deploy\n3. Drop column\n\n```sql\n-- Migration 010\nALTER TABLE users DROP COLUMN deprecated_field;\n```\n\n**Note:** Don't drop column in same release that stops using it (allow rollback).\n\n## Migration Versioning\n\n**File Naming Convention:**\n\n```\nmigrations/\n  001_initial_schema.up.sql\n  001_initial_schema.down.sql\n  002_add_user_bio.up.sql\n  002_add_user_bio.down.sql\n  003_add_documents_table.up.sql\n  003_add_documents_table.down.sql\n```\n\n**Tracking Table:**\n\n```sql\nCREATE TABLE schema_migrations (\n  version INT PRIMARY KEY,\n  applied_at TIMESTAMP DEFAULT now()\n);\n```\n\n**Migration Tool:** Use library (Flyway, Liquibase, diesel-migrations, sqlx-migrate)\n\n## Testing Migrations\n\n**Pre-Deploy:**\n1. Test migration on staging database (same size as production)\n2. Measure migration time (estimate downtime if any)\n3. Test rollback (down migration)\n\n**During Deploy:**\n1. Run migration before deploying application\n2. Monitor database locks (pg_locks)\n3. Monitor application errors (check for schema mismatches)\n\n**Post-Deploy:**\n1. Verify data integrity (counts, checksums)\n2. Verify application behavior (smoke tests)\n3. Monitor performance (new indexes, new columns)\n\n## Migration Anti-Patterns\n\n**DON'T: Modify Existing Migrations**\n\nOnce migration applied to production, never modify it. Create new migration instead.\n\n**DON'T: Drop Column in Same Release**\n\nAllow at least one release cycle between stopping use and dropping.\n\n**DON'T: Assume Migration is Fast**\n\nALTER TABLE on large table can take hours. Use CONCURRENTLY for indexes:\n\n```sql\nCREATE INDEX CONCURRENTLY idx_documents_owner ON documents(owner_id);\n```\n\n**DON'T: Use Transactions for Long Migrations**\n\nPostgreSQL locks table during ALTER TABLE. Split into smaller migrations or use tools like pg_repack.\n\n**Confidence:** [0.0-1.0]" \
  --confidence [0.0-1.0] \
  --tags "data-model,migrations,[system-name]"
```

### Step 6: Link All Data Model Entities

```bash
# Link all data model documents to task
engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [ENTITIES_ID] --target-type context \
  --relationship-type references --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [RELATIONSHIPS_ID] --target-type reasoning \
  --relationship-type documents --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [NORMALIZATION_ID] --target-type reasoning \
  --relationship-type documents --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [INDEXES_ID] --target-type reasoning \
  --relationship-type documents --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [MIGRATIONS_ID] --target-type reasoning \
  --relationship-type documents --agent [AGENT]
```

## Example

User wants to add multi-tenancy to an existing SaaS application.

### Step 1: Identify New Entities

```bash
ENTITIES=$(engram context create \
  --title "Data Model: Multi-Tenancy - Entities" \
  --content "## New Entity: Organization\n\n**Description:** Tenant in multi-tenant system\n\n**Attributes:**\n- id (UUID, primary key)\n- name (string, not null)\n- slug (string, unique, not null) -- URL-friendly name\n- plan (enum: free, pro, enterprise)\n- created_at (timestamp)\n\n**Access Patterns:**\n1. Find by id (primary key)\n2. Find by slug (unique index)\n3. List all organizations (admin only)\n\n**Estimated Volume:** 10K organizations\n\n## Modified Entity: User\n\n**New Attributes:**\n- organization_id (UUID, foreign key → organizations.id)\n\n**Access Patterns:**\n1. List users in organization (WHERE organization_id = ?)\n\n## Modified Entity: Document\n\n**New Attributes:**\n- organization_id (UUID, foreign key → organizations.id)\n\n**Access Patterns:**\n1. List documents in organization (WHERE organization_id = ?)\n2. Ensure user can only access documents in their organization\n\n**Constraint:**\n- Document owner must be in same organization as document" \
  --source "data-modeling" \
  --tags "data-model,entities,multi-tenancy" \
  --json | jq -r '.id')
```

### Step 2: Define Relationships

```bash
RELATIONSHIPS=$(engram reasoning create \
  --title "Data Model: Multi-Tenancy - Relationships" \
  --task-id c7d9e1f2-3456-7890-abcd-ef1234567890 \
  --content "## New Relationships\n\n### Organization → Users (One-to-Many)\n\n**Cardinality:** One organization has many users\n\n**Implementation:**\n\n```sql\nALTER TABLE users ADD COLUMN organization_id UUID REFERENCES organizations(id) ON DELETE CASCADE;\n```\n\n**Delete Behavior:**\nCASCADE: When organization deleted, delete all users (expected behavior).\n\n### Organization → Documents (One-to-Many)\n\n**Cardinality:** One organization has many documents\n\n**Implementation:**\n\n```sql\nALTER TABLE documents ADD COLUMN organization_id UUID REFERENCES organizations(id) ON DELETE CASCADE;\n```\n\n### Data Isolation Constraint\n\n**Rule:** User can only access documents in their organization\n\n**Implementation:**\n\n```sql\n-- Check constraint: document owner must be in same org\nALTER TABLE documents ADD CONSTRAINT documents_owner_same_org\nCHECK (\n  (SELECT organization_id FROM users WHERE id = owner_id) = organization_id\n);\n```\n\n**Alternative: Row-Level Security (PostgreSQL)**\n\n```sql\nALTER TABLE documents ENABLE ROW LEVEL SECURITY;\n\nCREATE POLICY documents_tenant_isolation ON documents\nUSING (organization_id = current_setting('app.current_organization_id')::uuid);\n```\n\n**Application Sets Context:**\n\n```sql\nSET LOCAL app.current_organization_id = 'org-uuid';\nSELECT * FROM documents;  -- Only returns documents in org\n```\n\n**Confidence:** 0.85" \
  --confidence 0.85 \
  --tags "data-model,relationships,multi-tenancy" \
  --json | jq -r '.id')
```

### Step 3: Plan Migration

```bash
MIGRATION=$(engram reasoning create \
  --title "Data Model: Multi-Tenancy - Migration Plan" \
  --task-id c7d9e1f2-3456-7890-abcd-ef1234567890 \
  --content "## Migration Strategy\n\n**Challenge:** Existing data has no organization_id\n\n**Solution: 3-Phase Migration**\n\n### Phase 1: Add Organizations Table\n\n```sql\n-- Migration 020\nCREATE TABLE organizations (\n  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),\n  name TEXT NOT NULL,\n  slug TEXT UNIQUE NOT NULL,\n  plan TEXT NOT NULL DEFAULT 'free',\n  created_at TIMESTAMP NOT NULL DEFAULT now()\n);\n\n-- Create default organization for existing users\nINSERT INTO organizations (name, slug, plan)\nVALUES ('Default Organization', 'default', 'pro');\n```\n\n### Phase 2: Add organization_id to Users and Documents\n\n```sql\n-- Migration 021\nALTER TABLE users ADD COLUMN organization_id UUID REFERENCES organizations(id);\n\n-- Backfill: Assign all existing users to default org\nUPDATE users SET organization_id = (\n  SELECT id FROM organizations WHERE slug = 'default'\n);\n\n-- Make non-nullable after backfill\nALTER TABLE users ALTER COLUMN organization_id SET NOT NULL;\n\n-- Same for documents\nALTER TABLE documents ADD COLUMN organization_id UUID REFERENCES organizations(id);\n\nUPDATE documents SET organization_id = (\n  SELECT organization_id FROM users WHERE users.id = documents.owner_id\n);\n\nALTER TABLE documents ALTER COLUMN organization_id SET NOT NULL;\n```\n\n### Phase 3: Add Indexes and Constraints\n\n```sql\n-- Migration 022\nCREATE INDEX idx_users_organization_id ON users(organization_id);\nCREATE INDEX idx_documents_organization_id ON documents(organization_id);\n\n-- Composite index for common query\nCREATE INDEX idx_documents_org_owner ON documents(organization_id, owner_id);\n```\n\n## Application Changes\n\n**Phase 1:**\nNo application changes (organization table exists but unused).\n\n**Phase 2:**\nUpdate application to:\n1. Set organization_id when creating users/documents\n2. Filter queries by organization_id\n3. Validate user and document are in same org\n\n**Phase 3:**\nEnable multi-tenancy features:\n1. Organization creation\n2. User invitation to organization\n3. Billing per organization\n\n## Rollback Plan\n\n**If issues found:**\n\n```sql\n-- Remove constraints\nALTER TABLE documents DROP CONSTRAINT documents_owner_same_org;\n\n-- Make nullable again\nALTER TABLE users ALTER COLUMN organization_id DROP NOT NULL;\nALTER TABLE documents ALTER COLUMN organization_id DROP NOT NULL;\n\n-- Eventually drop columns (after rolling back application)\nALTER TABLE users DROP COLUMN organization_id;\nALTER TABLE documents DROP COLUMN organization_id;\nDROP TABLE organizations;\n```\n\n**Confidence:** 0.80" \
  --confidence 0.80 \
  --tags "data-model,migrations,multi-tenancy" \
  --json | jq -r '.id')
```

### Step 4: Link Everything

```bash
engram relationship create \
  --source-id c7d9e1f2-3456-7890-abcd-ef1234567890 --source-type task \
  --target-id $ENTITIES --target-type context \
  --relationship-type references --agent default

engram relationship create \
  --source-id c7d9e1f2-3456-7890-abcd-ef1234567890 --source-type task \
  --target-id $RELATIONSHIPS --target-type reasoning \
  --relationship-type documents --agent default

engram relationship create \
  --source-id c7d9e1f2-3456-7890-abcd-ef1234567890 --source-type task \
  --target-id $MIGRATION --target-type reasoning \
  --relationship-type documents --agent default
```

## Querying Data Models

After creating data models, agents can retrieve:

```bash
# Get all data modeling documents
engram context list | grep "Data Model"

# Get all migration plans
engram reasoning list | grep "Migration"

# Get all data modeling reasoning for a task
engram relationship connected --entity-id [TASK_ID] --relationship-type documents | grep -i "data model\|migration"
```

## Related Skills

This skill integrates with:
- `engram-system-design` - Database is part of system architecture
- `engram-api-design` - API endpoints map to data entities
- `engram-security-architecture` - Data encryption, access control
- `engram-scalability-analysis` - Database sharding, replication strategy
- `engram-test-driven-development` - Test migrations and data integrity
