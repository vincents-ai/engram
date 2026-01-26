# Engram Feedback & Fixes

## CLI & Environment

- [x] **Issue:** The `engram` CLI seems to default to Nix store paths for `ENGRAM_PROMPTS_PATH` and `ENGRAM_SKILLS_PATH` which are read-only and often non-existent in the dev shell environment.
- [x] **Fix:** Default to looking in the current workspace's `.engram/prompts` and `.engram/skills` if the environment variables are not explicitly set or point to invalid paths.

## Command Output Consistency

- [x] **Issue:** `engram task create --json` fails when arguments like `--title` are mixed in. It seems `--json` mode forces _input_ to be JSON via stdin, preventing mixed usage (flags for input, JSON for output).
- [x] **Fix:** Decouple input method from output format. Support `--output json` to strictly control output format while still allowing CLI flags for input.
- [x] **Issue:** `engram compliance list --json` and `engram workflow list --json` sometimes return text output (headers like "📋 Workflows List") instead of pure JSON, making programmatic parsing brittle.
- [x] **Fix:** Ensure `--json` (or a new `--output json` flag) suppresses _all_ human-readable headers and decorations.

## Content Discovery

- [x] **Issue:** `engram prompts list` and `engram skills list` returned empty results even after valid Markdown files were created in the target directories.
- [x] **Fix:** Add a verbose mode (`-v`) to these commands to show _where_ it is looking and _why_ files might be ignored (e.g., missing frontmatter, wrong extension).

## Task Management

- [x] **Issue:** `engram task create` fails with "Invalid JSON" when piping JSON input if the format isn't exactly as expected, with opaque error messages.
- [x] **Fix:** Improve error messages for JSON parsing failures to indicate which field or line caused the issue.

## Compliance & Identity

- [x] **Issue:** `engram compliance list` displays short IDs (e.g., `[ca99ea33]`) but `engram compliance show ca99ea33` failed with "not found".
- [x] **Fix:** Ensure `show` commands accept the short IDs displayed in `list` views, or ensure `list` displays the full UUIDs that `show` requires.

## Workflow Context Injection

- [x] **Issue:** `engram workflow start` accepts `--variables` in `key=value` format, which is insufficient for passing complex, nested context objects (like an entire AST analysis or file content) to a workflow.
- [x] **Fix:** Add a `--context-json` or `--context-file` flag to `workflow start` and `workflow transition` to allow passing rich, structured data.

## API Consistency


- [x] **Issue:** `engram task list` and `engram compliance list` use different visual metaphors and output structures (bullets vs icons).
- [x] **Fix:** Standardize the "list" view output format across all entities for a consistent developer experience.
