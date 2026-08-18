# Doc

The `doc` command manages documentation fragments stored in engram and can assemble them into mdBook-compatible source files.

## CLI Usage

```bash
# Build mdBook source from stored documentation fragments
engram doc build --output docs

# Discover topics and chunks
engram doc topics list
engram doc chunk list <topic>

# Write a chunk
engram doc write tasks task-overview --title "Task Overview" --order 10 --stdin
engram doc write tasks task-overview --title "Task Overview" --file overview.md

# Read and maintain chunks
engram doc show tasks task-overview
engram doc stale
engram doc search "validation"
```

## Topics

Built-in topics include tasks, knowledge, reasoning, workflows, sessions, standards, ADRs, decisions, theories, and overview.

## mdBook Deployment

The GitHub Pages workflow runs `mdbook build`, so every path referenced in `docs/SUMMARY.md` must exist in the `docs/` tree.
