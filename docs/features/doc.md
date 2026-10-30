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

# Inspect documentation state
engram doc status
engram doc refs
engram doc fetch
```

## Built-in Topics

- `adrs`
- `decisions`
- `tasks`
- `knowledge`
- `theories`
- `workflows`
- `sessions`
- `reasoning`
- `standards`
- `overview`

## Staleness

Doc fragments can reference source entity IDs. When those entities change, the fragment can be marked stale and regenerated/reviewed.

## mdBook Deployment

The GitHub Pages workflow runs `mdbook build`. Every file referenced in `docs/SUMMARY.md` must exist under `docs/`, and `book.toml` must use keys supported by the mdBook version installed in CI.
