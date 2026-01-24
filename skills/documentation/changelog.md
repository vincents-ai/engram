---
name: engram-changelog
description: "Generate user-facing changelogs from commits and tasks, categorize changes (features, fixes, breaking, deprecated)."
---

# Changelog Generation (Engram-Integrated)

## Overview

Generate user-facing changelogs from git commits, pull requests, and Engram tasks. Categorize changes as features, fixes, breaking changes, deprecated features, and internal improvements. Follow Keep a Changelog format and Semantic Versioning principles. Store changelog history in Engram for release planning and communication.

## When to Use

Use this skill when:
- Preparing for a new release
- Generating release notes for users
- Documenting API changes and breaking changes
- Communicating what's new to customers
- Planning version bumps (major, minor, patch)
- Creating GitHub releases or blog posts
- Maintaining CHANGELOG.md file
- Need audit trail of user-facing changes

## The Pattern

### Step 1: Collect Changes Since Last Release

Gather all changes:

```bash
engram context create \
  --title "Changelog Sources: [Version]" \
  --content "## Version Information\n\n**Version:** [e.g., v1.2.0]\n**Previous Version:** [e.g., v1.1.0]\n**Release Date:** [YYYY-MM-DD]\n**Release Type:** [Major/Minor/Patch]\n\n## Git Commit Range\n\n```bash\n# Get all commits since last release\ngit log v1.1.0..HEAD --oneline\n\n# Get commit details\ngit log v1.1.0..HEAD --pretty=format:\"%h - %s (%an, %ar)\"\n```\n\n## Pull Requests Merged\n\n```bash\n# List PRs merged since last release\ngh pr list --state merged --search \"merged:>=$(git log v1.1.0 -1 --format=%ci | cut -d' ' -f1)\"\n```\n\n## Completed Tasks\n\n```bash\n# Get tasks completed since last release\nengram task list --status completed --created-after [last-release-date]\n```\n\n## Breaking Changes\n\n- Look for commits with: BREAKING CHANGE: in body\n- Look for PRs labeled: breaking-change\n- Look for API changes that affect existing behavior\n\n## Deprecated Features\n\n- Look for commits with: DEPRECATED: in body\n- Look for PRs labeled: deprecation\n- Look for features marked for removal\n\n## Security Fixes\n\n- Look for commits with: SECURITY: in body\n- Look for PRs labeled: security\n- Look for CVE fixes" \
  --source "changelog" \
  --tags "changelog,release,[version]"
```

### Step 2: Categorize Changes

Organize changes by type:

```bash
engram reasoning create \
  --title "Changelog Categorization: [Version]" \
  --task-id [TASK_ID] \
  --content "## Changelog Categories\n\n### Added (New Features)\n\n**Criteria:**\n- New user-facing features\n- New API endpoints\n- New configuration options\n- New integrations\n\n**From this release:**\n- [Feature 1]: [Description] (PR #123)\n- [Feature 2]: [Description] (PR #145)\n- [Feature 3]: [Description] (PR #167)\n\n**User Impact:** Positive - more capabilities\n\n### Changed (Enhancements)\n\n**Criteria:**\n- Improvements to existing features\n- UI/UX improvements\n- Performance improvements (user-visible)\n- Behavior changes (non-breaking)\n\n**From this release:**\n- [Change 1]: [Description] (PR #124)\n- [Change 2]: [Description] (PR #156)\n\n**User Impact:** Positive - better experience\n\n### Deprecated (Marked for Removal)\n\n**Criteria:**\n- Features marked for future removal\n- APIs to be removed in next major version\n- Configuration options to be replaced\n\n**From this release:**\n- [Deprecated 1]: [What is deprecated] - [Migration path] (PR #178)\n- [Deprecated 2]: [What is deprecated] - [Removal timeline] (PR #189)\n\n**User Impact:** Warning - need to migrate before next major version\n\n### Removed (Deleted Features)\n\n**Criteria:**\n- Features removed (previously deprecated)\n- APIs removed\n- Support for old versions dropped\n\n**From this release:**\n- [Removed 1]: [What was removed] - [Was deprecated in v1.0.0] (PR #134)\n\n**User Impact:** Breaking - may require code changes\n\n### Fixed (Bug Fixes)\n\n**Criteria:**\n- User-reported bugs fixed\n- Incorrect behavior corrected\n- Edge cases handled\n\n**From this release:**\n- [Fix 1]: [What bug was fixed] (PR #135, fixes #200)\n- [Fix 2]: [What bug was fixed] (PR #146, fixes #201)\n- [Fix 3]: [What bug was fixed] (PR #157, fixes #202)\n\n**User Impact:** Positive - more reliable\n\n### Security (Security Fixes)\n\n**Criteria:**\n- Security vulnerabilities fixed\n- CVE patches\n- Authentication/authorization improvements\n\n**From this release:**\n- [Security 1]: [What vulnerability fixed] - [Severity: High] (PR #140)\n- [Security 2]: [CVE-2026-1234 patched] (PR #150)\n\n**User Impact:** Critical - should upgrade immediately\n\n## Not in Changelog (Internal Only)\n\n**Criteria:**\n- Refactoring (no user-visible change)\n- Internal tooling improvements\n- Test improvements\n- Documentation updates (internal)\n- Dependency updates (no behavior change)\n\n**From this release:**\n- Refactored authentication module (PR #160)\n- Added integration tests (PR #170)\n- Updated CI/CD pipeline (PR #180)\n\n**User Impact:** None - internal improvements\n\n## Version Bump Decision\n\n**Semantic Versioning:** MAJOR.MINOR.PATCH\n\n**Rules:**\n- MAJOR: Breaking changes, removed features\n- MINOR: New features, deprecations (backward compatible)\n- PATCH: Bug fixes only (backward compatible)\n\n**This Release:**\n- Breaking changes? [Yes/No] - [List]\n- New features? [Yes/No] - [Count]\n- Bug fixes? [Yes/No] - [Count]\n\n**Recommendation:** [Version number]\n**Rationale:** [Why this version]\n\n**Confidence:** [0.0-1.0]" \
  --confidence [0.0-1.0] \
  --tags "changelog,categorization,[version]"
```

### Step 3: Write User-Facing Changelog

Create changelog entry:

```bash
engram context create \
  --title "Changelog: [Version]" \
  --content "# Changelog\n\nAll notable changes to this project will be documented in this file.\n\nThe format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),\nand this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).\n\n## [${VERSION}] - ${DATE}\n\n### Added\n\n- **[Feature Name]**: [Clear description of what was added and why users care]. Users can now [benefit]. ([#PR_NUMBER])\n  - Example: \`curl -X POST /api/v2/users -d '{...}'\`\n  - See: [Link to docs]\n  \n- **[Feature Name]**: [Description]. ([#PR_NUMBER])\n\n### Changed\n\n- **[Improvement]**: [What improved and how users benefit]. Response times reduced by 50%. ([#PR_NUMBER])\n  - **Migration**: [If behavior changed, how to adapt]\n  - **Before**: [Old behavior]\n  - **After**: [New behavior]\n  \n- **[Improvement]**: [Description]. ([#PR_NUMBER])\n\n### Deprecated\n\n- **[Feature Name]**: [What is deprecated]. This feature will be removed in v2.0.0. ([#PR_NUMBER])\n  - **Migration**: Use [new feature] instead\n  - **Example**: \`[new API call]\`\n  - **Timeline**: Deprecated in v1.2.0, removed in v2.0.0\n  \n- **[API Endpoint]**: \`GET /api/v1/users\` is deprecated. Use \`GET /api/v2/users\` instead. ([#PR_NUMBER])\n\n### Removed\n\n- **[Feature Name]**: [What was removed]. This was deprecated in v1.0.0. ([#PR_NUMBER])\n  - **Migration**: [How to migrate to replacement]\n  - **Breaking**: [How this breaks existing usage]\n  \n### Fixed\n\n- **[Bug Description]**: Fixed issue where [problem]. Now [correct behavior]. ([#PR_NUMBER], fixes [#ISSUE_NUMBER])\n  - **Impact**: Affected users who [scenario]\n  - **Workaround**: (if there was one before fix)\n  \n- **[Bug Description]**: [What was fixed]. ([#PR_NUMBER], fixes [#ISSUE_NUMBER])\n\n### Security\n\n- **[Severity]**: Fixed [vulnerability description]. All users should upgrade immediately. ([#PR_NUMBER])\n  - **CVE**: [CVE-YYYY-XXXXX] (if applicable)\n  - **Impact**: [Who is affected, what's the risk]\n  - **Mitigation**: [If users can't upgrade, what to do]\n\n---\n\n## [Previous Version] - [Date]\n\n[Previous changelog entries...]\n\n---\n\n## Upgrade Guide\n\n### Upgrading from v1.1.x to v1.2.0\n\n**Breaking Changes:**\n\nNone (this is a minor version).\n\n**Deprecations:**\n\n1. **Old API**: \`GET /api/v1/users\`\n   - **Action**: Update to \`GET /api/v2/users\`\n   - **Example**:\n     ```bash\n     # Old (still works, but deprecated)\n     curl -X GET https://api.example.com/api/v1/users\n     \n     # New (recommended)\n     curl -X GET https://api.example.com/api/v2/users\n     ```\n   - **Timeline**: v1 API will be removed in v2.0.0 (estimated Q3 2026)\n\n**New Features:**\n\n1. **Webhooks**: Configure webhooks to receive real-time notifications\n   - See: [Webhooks Documentation](docs/webhooks.md)\n   - Example: [repository]/examples/webhooks/basic.js\n\n**Bug Fixes:**\n\nAll bug fixes are backward compatible, no action needed.\n\n---\n\n## Format Guidelines\n\n**Each entry should:**\n- Start with bold feature/fix name\n- Explain WHAT changed and WHY users care\n- Include PR number for reference\n- Include issue number if fixing bug\n- Provide examples for API changes\n- Link to detailed documentation\n\n**User-Facing Language:**\n- ✓ \"You can now export data to CSV\"\n- ✓ \"API responses are now 50% faster\"\n- ✓ \"Fixed issue where login failed for Google accounts\"\n- ✗ \"Refactored authentication module\"\n- ✗ \"Updated dependency to v2.3.4\"\n- ✗ \"Improved code coverage to 90%\"\n\n**Technical vs User-Facing:**\n- User-Facing: Features, fixes, API changes, behavior changes\n- Internal Only: Refactoring, dependency updates, test improvements\n\n**Examples:**\n\n**Good:**\n> **Bulk User Import**: You can now import multiple users at once via CSV upload. This is much faster than creating users one by one. (#123)\n> - Upload via: Settings → Users → Import CSV\n> - See: [Import Guide](docs/import.md)\n\n**Bad:**\n> Implemented bulk import feature (#123)\n\n**Good:**\n> **Login Issue Fixed**: Fixed issue where users with long passwords (>50 characters) couldn't log in. (#145, fixes #200)\n> - Impact: Affected 0.1% of users\n> - All affected users can now log in normally\n\n**Bad:**\n> Fixed password validation bug (#145)" \
  --source "changelog" \
  --tags "changelog,release,[version]"
```

### Step 4: Generate Changelog from Git History

Automate changelog generation:

```bash
engram context create \
  --title "Changelog Generation Script" \
  --content "# Automated Changelog Generation\n\n## Using Conventional Commits\n\nIf commits follow [Conventional Commits](https://www.conventionalcommits.org/):\n\n```\nfeat: Add CSV export for reports\nfix: Login fails for long passwords\nfix!: Remove support for API v1\nchore: Update dependencies\ndocs: Improve API documentation\n```\n\n## Generation Script\n\n```bash\n#!/bin/bash\n# generate-changelog.sh\n\nLAST_TAG=$(git describe --tags --abbrev=0)\nNEXT_VERSION=$1\nTODAY=$(date +%Y-%m-%d)\n\necho \"# Changelog\"\necho \"\"\necho \"## [${NEXT_VERSION}] - ${TODAY}\"\necho \"\"\n\n# Added (feat:)\necho \"### Added\"\necho \"\"\ngit log ${LAST_TAG}..HEAD --pretty=format:\"%s (%h)\" --grep=\"^feat:\" | \\\n  sed 's/^feat: /- /' | sed 's/ (\\([^)]*\\))/ ([#\\1])/'\necho \"\"\n\n# Changed (perf:, refactor: that affect users)\necho \"### Changed\"\necho \"\"\ngit log ${LAST_TAG}..HEAD --pretty=format:\"%s (%h)\" --grep=\"^perf:\" | \\\n  sed 's/^perf: /- /' | sed 's/ (\\([^)]*\\))/ ([#\\1])/'\necho \"\"\n\n# Fixed (fix:)\necho \"### Fixed\"\necho \"\"\ngit log ${LAST_TAG}..HEAD --pretty=format:\"%s (%h)\" --grep=\"^fix:\" | \\\n  sed 's/^fix: /- /' | sed 's/ (\\([^)]*\\))/ ([#\\1])/'\necho \"\"\n\n# Breaking changes (commits with ! or BREAKING CHANGE:)\necho \"### Removed\"\necho \"\"\ngit log ${LAST_TAG}..HEAD --pretty=format:\"%s%n%b\" | \\\n  grep -A 5 \"BREAKING CHANGE:\" | \\\n  sed 's/^BREAKING CHANGE: /- /'\necho \"\"\n\n# Security fixes\necho \"### Security\"\necho \"\"\ngit log ${LAST_TAG}..HEAD --pretty=format:\"%s (%h)\" --grep=\"^fix:.*security\" --grep=\"^sec:\" | \\\n  sed 's/^[^:]*: /- /' | sed 's/ (\\([^)]*\\))/ ([#\\1])/'\necho \"\"\n```\n\n## Usage\n\n```bash\n# Generate changelog for next version\n./scripts/generate-changelog.sh v1.2.0 > CHANGELOG-v1.2.0.md\n\n# Review and edit\nvim CHANGELOG-v1.2.0.md\n\n# Prepend to main CHANGELOG.md\ncat CHANGELOG-v1.2.0.md CHANGELOG.md > CHANGELOG-new.md\nmv CHANGELOG-new.md CHANGELOG.md\n\n# Commit\ngit add CHANGELOG.md\ngit commit -m \"docs: Update CHANGELOG for v1.2.0\"\n```\n\n## Using GitHub CLI\n\n```bash\n# Generate from merged PRs\ngh pr list \\\n  --state merged \\\n  --search \"merged:>=$(git log v1.1.0 -1 --format=%ci | cut -d' ' -f1)\" \\\n  --json number,title,labels \\\n  --jq '.[] | \"- \\(.title) (#\\(.number))\"'\n\n# Filter by label\ngh pr list --state merged --label feature --json number,title --jq '.[] | \"- \\(.title) (#\\(.number))\"'\ngh pr list --state merged --label bug --json number,title --jq '.[] | \"- \\(.title) (#\\(.number))\"'\n```\n\n## Using Engram Tasks\n\n```bash\n# Get completed features\nengram task list \\\n  --status completed \\\n  --tags feature \\\n  --created-after [last-release-date] \\\n  --json | jq -r '.[] | \"- \\(.title) (Task: \\(.id))\"'\n\n# Get fixed bugs\nengram task list \\\n  --status completed \\\n  --tags bug \\\n  --created-after [last-release-date] \\\n  --json | jq -r '.[] | \"- \\(.title) (Task: \\(.id))\"'\n```\n\n## Manual Curation\n\n**Automated generation is a starting point. Always:**\n\n1. Review all entries\n2. Rewrite technical language to user-facing\n3. Add examples for API changes\n4. Group related changes\n5. Remove internal-only changes\n6. Add \"Why users care\" context\n7. Link to documentation\n8. Highlight breaking changes prominently" \
  --source "changelog" \
  --tags "changelog,automation"
```

### Step 5: Link Changelog to Release Task

```bash
# Link changelog to release task
engram relationship create \
  --source-id [RELEASE_TASK_ID] --source-type task \
  --target-id [CHANGELOG_ID] --target-type context \
  --relationship-type documents --agent default

engram relationship create \
  --source-id [RELEASE_TASK_ID] --source-type task \
  --target-id [CATEGORIZATION_ID] --target-type reasoning \
  --relationship-type documents --agent default
```

## Example

Generate changelog for v1.2.0 release.

### Step 1: Collect Changes

```bash
SOURCES=$(engram context create \
  --title "Changelog Sources: v1.2.0" \
  --content "Version: v1.2.0\nPrevious: v1.1.0\nDate: 2026-01-24\nType: Minor\n\nCommits:\n- feat: Add webhook support for real-time notifications (#123)\n- feat: CSV export for reports (#145)\n- fix: Login fails for users with long passwords (#150, fixes #200)\n- fix: Timezone issue in date picker (#155, fixes #205)\n- perf: Reduce API response time by 50% (#160)\n- chore: Update dependencies (#170)\n- docs: Improve API documentation (#180)\n\nPRs with breaking-change label: None\nSecurity fixes: None" \
  --source "changelog" \
  --tags "changelog,release,v1.2.0" \
  --json | jq -r '.id')
```

### Step 2: Categorize

```bash
CATEGORIES=$(engram reasoning create \
  --title "Changelog Categorization: v1.2.0" \
  --task-id release-v1.2.0 \
  --content "## Added\n- Webhook support for real-time notifications (#123)\n- CSV export for reports (#145)\n\n## Changed\n- API responses now 50% faster (#160)\n\n## Fixed\n- Login fails for long passwords (#150, fixes #200)\n- Timezone issue in date picker (#155, fixes #205)\n\n## Not in Changelog\n- Dependency updates (#170)\n- Documentation improvements (#180)\n\n## Version Bump\nNew features + no breaking changes = MINOR version\nRecommendation: v1.2.0" \
  --confidence 0.95 \
  --tags "changelog,categorization,v1.2.0" \
  --json | jq -r '.id')
```

### Step 3: Write Changelog

```bash
CHANGELOG=$(engram context create \
  --title "Changelog: v1.2.0" \
  --content "# Changelog\n\n## [1.2.0] - 2026-01-24\n\n### Added\n\n- **Webhooks**: You can now configure webhooks to receive real-time notifications when events occur (orders placed, users registered, payments completed). This eliminates the need to poll the API for updates. (#123)\n  - Configure via: Settings → Webhooks → Add Webhook\n  - Example: \`POST https://your-server.com/webhook\` with \`{event: 'order.created', data: {...}}\`\n  - See: [Webhook Documentation](docs/webhooks.md)\n\n- **CSV Export**: Export your reports to CSV format for analysis in Excel or other tools. Available for all report types. (#145)\n  - Export via: Reports → Select report → Export → CSV\n  - See: [Export Guide](docs/export.md)\n\n### Changed\n\n- **Faster API**: API response times improved by 50% (p95: 300ms → 150ms). You should notice faster page loads and smoother user experience. (#160)\n  - Technical: Database query optimization and caching improvements\n  - No changes needed to your code\n\n### Fixed\n\n- **Long Password Login**: Fixed issue where users with passwords longer than 50 characters couldn't log in. All affected users can now log in normally. (#150, fixes #200)\n  - Impact: Affected approximately 0.1% of users\n  - No action needed - issue is resolved\n\n- **Date Picker Timezone**: Fixed date picker showing wrong dates for users in certain timezones (GMT+12 to GMT+14). Dates now display correctly for all timezones. (#155, fixes #205)\n  - Impact: Affected users in Pacific island timezones\n  - No action needed - issue is resolved" \
  --source "changelog" \
  --tags "changelog,release,v1.2.0" \
  --json | jq -r '.id')
```

### Step 4: Update CHANGELOG.md

```bash
# Generate and prepend to CHANGELOG.md
cat > /tmp/new-entry.md << 'EOF'
## [1.2.0] - 2026-01-24

### Added

- **Webhooks**: Configure webhooks to receive real-time notifications. (#123)
- **CSV Export**: Export reports to CSV format. (#145)

### Changed

- **Faster API**: Response times improved by 50%. (#160)

### Fixed

- **Long Password Login**: Fixed login for passwords > 50 chars. (#150, fixes #200)
- **Date Picker Timezone**: Fixed timezone issues. (#155, fixes #205)

EOF

# Prepend to CHANGELOG.md
cat /tmp/new-entry.md CHANGELOG.md > CHANGELOG-new.md
mv CHANGELOG-new.md CHANGELOG.md

# Commit
git add CHANGELOG.md
git commit -m "docs: Update CHANGELOG for v1.2.0"
```

### Step 5: Create GitHub Release

```bash
# Create release with changelog
gh release create v1.2.0 \
  --title "Release v1.2.0" \
  --notes-file /tmp/new-entry.md

# Or with interactive editor
gh release create v1.2.0 --notes "" --edit
```

## Querying Changelogs

```bash
# Get all changelogs
engram context list | grep "Changelog:"

# Get changelog for specific version
engram context list | grep "Changelog: v1.2.0"

# Get all release categorizations
engram reasoning list | grep "Changelog Categorization"

# Find breaking changes across releases
engram context list | grep -i "breaking"
```

## Related Skills

This skill integrates with:
- `engram-release-planning` - Plan releases based on changelog categories
- `engram-technical-writing` - Write clear, user-facing changelog entries
- `engram-api-design` - Document API changes in changelog
