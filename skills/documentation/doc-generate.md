---
name: engram-doc-generate
description: "Generate project documentation from engram data using the `engram doc` subcommand. Covers topic discovery, source analysis, chunk writing, incremental updates, and mdBook assembly."
---

# Documentation Generation with `engram doc`

## Overview

The `engram doc` subcommand is a dumb data tool with no LLM inside. An external LLM agent (you) uses it as a CLI to fetch structured engram data, confirm claims against project files, write named content chunks as DocFragment entities stored in git refs, and assemble a final mdBook documentation site. It supports incremental updates: you can re-run any topic and update individual chunks without regenerating everything. Works on both coding and non-coding projects.

## When to Use

Use this skill when:
- You need to generate project documentation from engram's knowledge graph
- You want to produce an mdBook site covering ADRs, tasks, knowledge, workflows, and more
- You need to update documentation incrementally after new work is done
- A human asks you to "generate docs" or "build the docs site"
- You want to audit documentation coverage across all topics

## Command Reference

```bash
engram doc topics list                              # list all available doc topics
engram doc refs <query> [--dir <path>]              # find relevant source files
engram doc fetch <topic> [--format json|md]         # get existing content for a topic
engram doc write <topic> <chunk-id> --content -     # write a content chunk (reads stdin)
engram doc status [--output <dir>]                  # check coverage across all topics
engram doc build [--output <dir>]                   # assemble all chunks into mdBook source
engram doc chunk list <topic>                       # list chunks for a topic
engram doc chunk delete <topic> <chunk-id>          # delete a chunk
```

## Available Topics

| Topic | Source | Description |
|-------|--------|-------------|
| `overview` | auto-aggregated | Project summary, auto-generated from all other topics |
| `adrs` | ADR entities | Architecture Decision Records |
| `decisions` | Reasoning entities | Key technical decisions and rationale |
| `tasks` | Task entities | Task history, outcomes, and patterns |
| `knowledge` | Knowledge entities | Durable facts, rules, patterns, procedures |
| `theories` | Theory entities | Mental models of the codebase |
| `workflows` | Workflow definitions | State machine workflows and their states |
| `sessions` | Session entities | Session history and handoff records |
| `reasoning` | Reasoning entities | Reasoning chains and logic |
| `standards` | Knowledge (type=rule) | Coding standards and compliance rules |

## The Pattern

### Step 1: Discover Available Topics

See what documentation topics exist and their current coverage:

```bash
engram doc topics list
engram doc status
```

`status` shows each topic, how many chunks exist, and which chunks are stale (their `written_at` is older than the source entity's last update).

### Step 2: Find Relevant Source Files

For each topic you want to document, find the source entities and files:

```bash
engram doc refs "authentication flow" --dir src/auth
engram doc refs "ADR" --dir .
```

The `refs` command searches all project files, auto-detects whether this is a code or non-code project, and returns paths with line numbers and snippets. Use the results to ground your documentation in actual source material.

### Step 3: Fetch Existing Content

Before writing, check what already exists for a topic:

```bash
engram doc fetch adrs --format md
engram doc fetch knowledge --format json
```

This returns all existing chunks for the topic. Use `--format md` for human reading and `--format json` for structured processing.

### Step 4: Analyze Sources and Write Chunks

For each topic, analyze the source entities and project files, then write content chunks. Each chunk should be a single coherent section of documentation.

**Chunk sizing guidelines:**
- **Target:** 50-200 lines of markdown per chunk
- **Minimum:** A chunk must be large enough to stand alone as a meaningful section (at least 20 lines)
- **Maximum:** If a chunk exceeds 300 lines, split it into sub-chunks
- **One concept per chunk:** Each chunk should cover one idea, one module, or one decision

**Chunk naming conventions:**
- Use descriptive kebab-case: `auth-setup`, `api-endpoints-users`, `database-schema`
- Prefix with ordering numbers for narrative flow: `01-overview`, `02-setup`, `03-configuration`
- Alphabetical ordering of chunk-ids determines the order in the final mdBook chapter

**Writing a chunk:**

```bash
echo '# Authentication Setup

This project uses JWT-based authentication with refresh tokens.

## Configuration

Set the following environment variables:

- `JWT_SECRET`: 256-bit key for signing tokens
- `JWT_EXPIRY`: Token lifetime (default: 1h)
- `REFRESH_EXPIRY`: Refresh token lifetime (default: 7d)

## Implementation

The auth middleware is implemented in `src/auth/middleware.rs`.
It validates the Bearer token on every request to protected routes.' | engram doc write adrs auth-setup --content -
```

**Writing multiple chunks for a topic:**

```bash
echo '# Overview of ADRs' | engram doc write adrs 01-overview --content -
echo '# ADR-001: Database Choice' | engram doc write adrs 02-database-choice --content -
echo '# ADR-002: API Framework' | engram doc write adrs 03-api-framework --content -
```

### Step 5: Check Coverage and Staleness

After writing chunks, check the overall status:

```bash
engram doc status
```

This reports:
- Which topics have chunks and which are empty
- Which chunks are stale (source entities were updated after the chunk was written)
- Total coverage percentage

**When chunks become stale:**
- A chunk is marked stale when its `written_at` timestamp is older than the `updated_at` of any source entity it references
- Re-fetch the topic and compare: `engram doc fetch <topic> --format json`
- Update only the stale chunks — no need to rewrite the entire topic

### Step 6: Build the mdBook

Assemble all chunks into mdBook source files:

```bash
engram doc build --output docs/
```

This creates:
- `docs/SUMMARY.md` — table of contents
- `docs/<topic>/` — one directory per topic with chunk files as chapters
- Standard mdBook structure ready for `mdbook build`

**Note:** `engram doc build` generates mdBook source files only. It does not invoke the `mdbook` binary. Run `mdbook build` separately if needed.

### Step 7: Iterate Incrementally

When source entities change (new ADRs, completed tasks, updated knowledge):

```bash
# Check what is stale
engram doc status

# Re-fetch the affected topic
engram doc fetch adrs --format md

# Update only the changed chunk
echo '# Updated ADR section...' | engram doc write adrs 02-database-choice --content -

# Verify
engram doc status

# Rebuild
engram doc build --output docs/
```

Overwriting a chunk with the same `<chunk-id>` replaces it. All other chunks remain untouched.

### Deleting a Chunk

If a chunk is no longer relevant:

```bash
engram doc chunk delete adrs old-deprecated-section
```

## Example: Full Documentation Generation

Generate documentation for a project with existing engram data.

### Step 1: Assess current state

```bash
engram doc topics list
engram doc status
```

Output shows:
```
Topic          Chunks   Stale
overview       0        -
adrs           3        1
knowledge      0        -
tasks          0        -
workflows      2        0
```

### Step 2: Gather sources for empty topics

```bash
engram doc refs "knowledge" --dir .
engram doc fetch knowledge --format json
engram doc refs "tasks completed" --dir .
engram doc fetch tasks --format json
```

### Step 3: Write chunks for the `knowledge` topic

```bash
echo '# Knowledge Base

This section captures durable project-wide knowledge stored in engram.' | engram doc write knowledge 01-overview --content -

echo '# Facts

## API Rate Limit
The external payments API enforces 100 req/s per key. (kno-001)

## Storage Backend
All engram data is stored in git refs under `.engram/refs/`. (kno-002)' | engram doc write knowledge 02-facts --content -

echo '# Rules

## Commit Convention
All commits must reference a valid engram task UUID. (kno-003)

## No --no-verify
Never use `--no-verify` to skip pre-commit hooks. (kno-004)' | engram doc write knowledge 03-rules --content -
```

### Step 4: Update the stale ADR chunk

```bash
engram doc fetch adrs --format md
# Identify which chunk is stale, then overwrite it:
echo '# ADR-005: Cache Strategy (Updated)

## Context
We need a caching layer for the API gateway.

## Decision
Use Redis with a 5-minute TTL for GET endpoints.' | engram doc write adrs 05-cache-strategy --content -
```

### Step 5: Verify and build

```bash
engram doc status
engram doc build --output docs/
```

### Step 6: Commit the generated docs

```bash
git add docs/
git commit -m "docs: generate project documentation [TASK_UUID]"
```

## Chunk Writing Tips

1. **Ground in source** — Always run `engram doc refs` and read the actual source entities before writing. Never fabricate content.
2. **Reference entity IDs** — Include engram entity IDs (e.g., `kno-001`, `adr-005`) so readers can trace back to the source.
3. **Use ordering prefixes** — `01-`, `02-`, `03-` ensures correct narrative flow in the final book.
4. **Keep chunks atomic** — One chunk = one section. If you find yourself writing "Part A" and "Part B" headings, split into two chunks.
5. **Write to stdin** — Always use `--content -` to pipe content via stdin. This avoids shell escaping issues with long markdown.
6. **Check staleness before builds** — Run `engram doc status` before `engram doc build` to catch outdated chunks.
7. **Non-code projects work too** — The `refs` command auto-detects project type. For a screenplay project, it finds beat sheets, character files, and scene files.

## Related Skills

- `engram-technical-writing` — Writing style, review checklists, and documentation quality
- `engram-knowledge` — Creating and managing knowledge entities that feed into documentation
- `engram-adr` — Architecture Decision Records that become ADR documentation chapters
- `engram-changelog` — Release notes that complement project documentation
- `engram-knowledge-transfer` — Onboarding docs and runbooks from engram data
- `engram-api-docs` — API reference documentation generation
