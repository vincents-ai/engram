---
name: engram-doc-generate
description: "Generate project documentation from engram data using the `engram doc` subcommand. Covers topic discovery, source analysis, chunk writing, incremental updates, and mdBook assembly."
---

# Documentation Generation with `engram doc`

## Overview

`engram doc` is a CLI data tool — no LLM inside. You (the agent) use it to:

1. Pull structured engram entities for a topic (`fetch`)
2. Find relevant source files to ground your writing (`refs`)
3. Write named markdown chunks as `DocFragment` entities stored in git refs (`write`)
4. Assemble all chunks into an mdBook source tree (`build`)

Chunks are stored durably in engram. `build` is purely assembly — it can be re-run at any time from the stored chunks with no data loss.

## When to Use

- Generating or refreshing the project documentation site
- A human asks you to "write the docs" or "build the book"
- New ADRs, knowledge, or tasks have been added and existing docs are stale
- Auditing how much of the engram graph is covered by documentation

## Command Reference

```bash
# Discovery
engram doc topics list                              # list all available topics
engram doc status [--output <dir>]                  # coverage + staleness per topic

# Sourcing
engram doc fetch <topic> [--format json|md]         # pull existing engram entities for a topic
engram doc refs <query> [--dir <path>] [--regex]    # find relevant source files

# Chunk management
engram doc write <topic> <chunk-id> \
  --title "<title>" \
  [--stdin | --file <path> | --content "<text>"]    # write or overwrite a chunk
engram doc chunk list <topic>                       # list chunks stored for a topic
engram doc chunk delete <topic> <chunk-id>          # delete a chunk

# Assembly
engram doc build [--output <dir>]                   # emit mdBook source into <dir>/src/
```

### `engram doc write` — content input methods

There are three ways to supply content. Use `--stdin` for anything longer than a few words:

| Method | Flag | When to use |
|--------|------|-------------|
| Stdin pipe | `--stdin` | Long markdown content; avoids shell escaping issues |
| Stdin pipe (Unix convention) | `--content -` | Same as `--stdin`; `-` is treated as "read stdin" |
| Inline string | `--content "text"` | Short single-line content only |
| File | `--file <path>` | Content already exists in a file |

**`--stdin` is the recommended method for all substantive content.**

```bash
# Correct — pipe via --stdin
cat <<'EOF' | engram doc write adrs 01-overview --title "ADR Overview" --stdin
# Architecture Decision Records

This project uses ADRs to capture significant technical decisions...
EOF

# Also correct — pipe via --content - (Unix convention, same behaviour)
cat <<'EOF' | engram doc write adrs 01-overview --title "ADR Overview" --content -
# Architecture Decision Records
EOF

# Wrong — --content with a literal string gets stored verbatim, including any shell quoting issues
engram doc write adrs 01-overview --title "ADR Overview" --content "# Architecture..."
```

> **Historical footgun (fixed):** Prior to the `--content -` fix, passing `--content -` stored the
> literal string `-` as the chunk content. If you are running an older binary and pages appear blank,
> re-write all affected chunks using `--stdin` explicitly.

## Available Topics

| Topic | Entity source | Description |
|-------|---------------|-------------|
| `overview` | (auto-aggregated) | Project summary, aggregated from all other topics |
| `adrs` | `adr` entities | Architectural Decision Records |
| `decisions` | `reasoning` entities | Key decision chains and rationale |
| `tasks` | `task` entities | Task descriptions, history, and outcomes |
| `knowledge` | `knowledge` entities | Durable facts, rules, patterns, procedures |
| `theories` | `theory` entities | Mental models of the codebase |
| `workflows` | `workflow` definitions | State machine workflows and states |
| `sessions` | `session` entities | Session history and handoff records |
| `reasoning` | `reasoning` entities | Reasoning chains and logic |
| `standards` | `standard` entities | Coding standards and compliance rules |

## The Pattern

### Step 1: Assess current state

```bash
engram doc topics list
engram doc status
```

`status` shows each topic, its chunk count, staleness, and the timestamp of the latest write. A chunk is **stale** when its `written_at` is older than the `updated_at` of any source entity it covers.

### Step 2: Fetch source entities

For each topic you want to document, pull the underlying engram entities:

```bash
engram doc fetch adrs --format md        # human-readable
engram doc fetch knowledge --format json # structured for processing
```

### Step 3: Find relevant source files

Supplement entity data with actual source code or project files:

```bash
engram doc refs "authentication" --dir src/auth
engram doc refs "ADR" --dir .
engram doc refs "storage backend" --regex --ignore-case
```

`refs` searches file contents, auto-detects project type (code vs. non-code), and returns paths with line numbers and snippets. Use the results to ground your writing in real source material — never fabricate.

### Step 4: Write chunks

Plan the chunks for each topic before writing. A good chunk is:

- **50–200 lines** of markdown
- **One concept** — one module, one decision, one rule set
- **Numbered prefix** for narrative order: `01-`, `02-`, `03-`

```bash
cat <<'EOF' | engram doc write adrs 01-overview --title "ADR Index" --stdin
# Architecture Decision Records

This project uses numbered ADRs stored in engram to capture significant
technical decisions with context, options considered, and rationale.

## Index

| Number | Title | Status |
|--------|-------|--------|
| ADR-001 | Use gix over git2 | Accepted |
| ADR-002 | SOPS secrets with AGE | Accepted |
EOF

cat <<'EOF' | engram doc write adrs 02-gix-migration --title "ADR-001: gix Migration" --stdin
# ADR-001: Migrate from git2 to gix

## Context
...
EOF
```

Overwriting an existing `chunk-id` replaces it in-place. All other chunks are unaffected.

### Step 5: Check coverage and staleness

```bash
engram doc status
engram doc chunk list adrs
```

Repeat steps 2–4 for any topics still empty or stale.

### Step 6: Build the mdBook source

```bash
engram doc build --output docs/
```

This emits mdBook source files into `<output>/src/`:

```
docs/
└── src/
    ├── SUMMARY.md          ← table of contents (auto-generated)
    ├── overview/
    │   ├── 01-overview.md
    │   └── 02-repos-structure.md
    ├── adrs/
    │   ├── 01-overview.md
    │   └── 02-gix-migration.md
    └── knowledge/
        └── 01-project-facts.md
```

`engram doc build` writes source files only. It does **not** invoke the `mdbook` binary.

### Step 7: Build and serve with mdbook

```bash
# Build the HTML book
nix run nixpkgs#mdbook -- build docs/

# Serve locally with live reload
nix run nixpkgs#mdbook -- serve docs/ --port 3000
```

Ensure `docs/book.toml` has `src = "src"` set, otherwise mdbook looks in the wrong directory.

### Step 8: Iterate incrementally

When new engram entities are created or existing ones are updated:

```bash
engram doc status                              # find stale chunks

engram doc fetch adrs --format md              # re-read the source

cat <<'EOF' | engram doc write adrs 03-new-decision --title "ADR-010: ..." --stdin
# ADR-010: ...
EOF

engram doc build --output docs/
```

Only the updated chunks need rewriting — all others remain stored in engram unchanged.

## Full Example

```bash
# 1. Survey
engram doc topics list
engram doc status

# 2. Source the knowledge topic
engram doc fetch knowledge --format json
engram doc refs "project facts" --dir .

# 3. Write three chunks
cat <<'EOF' | engram doc write knowledge 01-project-facts --title "Project Facts" --stdin
# Project Facts

Key immutable facts about this project stored in the engram knowledge graph.

- **Platform**: ADP — Agentic Development Platform
- **Repos**: engram, agentic-repos, adp
- **License**: AGPL-3.0-or-later OR LicenseRef-Commercial
EOF

cat <<'EOF' | engram doc write knowledge 02-api-surface --title "API Surface" --stdin
# REST API Surface

The agentic-repos backend exposes a REST API on port 8090.

## Namespaces

- `GET/POST /api/repos` — repository CRUD
- `GET /api/repos/:id/branches` — branch listing
...
EOF

cat <<'EOF' | engram doc write knowledge 03-build-rules --title "Build Rules" --stdin
# Build Rules

## Rust

Always run before committing:

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```
EOF

# 4. Verify
engram doc status
engram doc chunk list knowledge

# 5. Build
engram doc build --output docs/
nix run nixpkgs#mdbook -- build docs/

# 6. Spot-check
curl -s http://localhost:3000/knowledge/01-project-facts.html | grep '<h1'
```

## Chunk Writing Rules

1. **Ground in source** — run `engram doc fetch` and `engram doc refs` before writing. Never fabricate content.
2. **Use `--stdin` for all substantive content** — avoids shell escaping issues with quotes, backticks, and special characters in markdown.
3. **Reference entity IDs** — include engram IDs (e.g. `kno-001`, `adr-005`) so readers can trace to the source.
4. **One concept per chunk** — if you find yourself writing "Part A" and "Part B" headings, split into two chunks.
5. **Number your chunks** — `01-`, `02-`, `03-` prefixes determine chapter order in the final book.
6. **Check staleness before building** — `engram doc status` before `engram doc build`.
7. **`book.toml` must have `src = "src"`** — `engram doc build` outputs to `<output>/src/`; mdbook defaults to looking in `src/` relative to the book root.

## Troubleshooting

### Pages render blank

The chunk content was stored as the literal string `-`. This happens when `--content -`
was used on a binary that predated the stdin-convention fix.

**Fix:** Re-write the affected chunks using `--stdin`:

```bash
# Identify blank chunks
engram doc chunk list <topic>

# Re-write each one
cat <<'EOF' | engram doc write <topic> <chunk-id> --title "<title>" --stdin
<real content here>
EOF

# Rebuild
engram doc build --output docs/
nix run nixpkgs#mdbook -- build docs/
```

### mdbook cannot find source files

`engram doc build` outputs to `<output>/src/`. Ensure `docs/book.toml` has:

```toml
[book]
src = "src"
```

### `--content -` stores literal `-`

You are running an old `engram` binary. Use `--stdin` instead, or upgrade the binary.

## Related Skills

- `engram-technical-writing` — Writing style and documentation quality
- `engram-knowledge` — Creating knowledge entities that feed the `knowledge` topic
- `engram-adr` — Architecture Decision Records that become `adrs` chapters
- `engram-changelog` — Release notes that complement project docs
- `engram-knowledge-transfer` — Onboarding docs and runbooks from engram data
- `engram-api-docs` — API reference documentation
