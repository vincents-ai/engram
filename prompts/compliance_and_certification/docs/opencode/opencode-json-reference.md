# AI Coding Tool Configuration Reference

This document provides comprehensive documentation for configuring specialized AI subagents in your repository using your AI coding tool.

## Agent Configuration

The agent configuration object defines specialized AI agents. Each agent has specific capabilities, tools, and prompts tailored to different aspects of your project.

### Agent Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `model` | string | No | Model override for this agent |
| `prompt` | string | Yes | Prompt file path or inline prompt |
| `description` | string | No | When to use this agent |
| `tools` | object | No | Tool enable/disable flags (see Tools section) |
| `permission` | object | No | Permission settings (see Permissions section) |
| `temperature` | number | No | Model temperature (0.0–1.0) |
| `top_p` | number | No | Model top_p (0.0–1.0) |
| `disable` | boolean | No | Disable this agent |

## Tools Configuration

Tools define what capabilities each agent has access to.

### Available Tools

| Tool | Description | Typical Use Cases |
|------|-------------|-------------------|
| `read` | Read files from filesystem | Code analysis, documentation review |
| `write` | Create new files | Generate new components, configs |
| `edit` | Modify existing files | Code updates, refactoring |
| `bash` | Execute shell commands | Build, test, deploy operations |
| `grep` | Search file contents | Find code patterns, debug issues |
| `glob` | Find files by pattern | Locate specific file types |
| `list` | List directory contents | Explore project structure |
| `webfetch` | Fetch web content | Research, API documentation |

## Permissions

Fine-grained control over agent capabilities.

### Permission Levels

- `"allow"` — Agent can use without asking
- `"ask"` — Agent must request permission before proceeding
- `"deny"` — Agent cannot use this capability

### Permission Configuration

Command-specific bash permissions allow precise control. Examples:

```
permission.bash.rm       = "deny"    # Prevent file deletion
permission.bash.git      = "allow"   # Permit git operations freely
permission.bash.npm      = "allow"   # Permit package manager operations
permission.bash.sudo     = "deny"    # Block privilege escalation
permission.bash.docker   = "ask"     # Confirm before container operations
permission.bash.kubectl  = "ask"     # Confirm before cluster operations
permission.edit          = "ask"     # Confirm before any file modification
permission.webfetch      = "deny"    # Block external network access
```

## Prompt Configuration

Prompts define the agent's personality, expertise, and behavior.

### File-based vs Inline Prompts

**File-based prompts (recommended)**: Store the prompt in a separate markdown file and reference it from agent configuration. This enables version control, reuse across agents, and easier editing.

**Inline prompts**: Embed the prompt string directly in the configuration. Suitable for short, simple prompts but becomes hard to maintain as prompts grow.

### Prompt File Structure

```markdown
# Agent Name

## Role
Brief description of the agent's role and expertise.

## Primary Prompt
Core instructions for the agent's behavior and capabilities.

## Additional Capabilities
- Secondary use cases
- Extended functionality
- Related skills

## Important: Project Guidelines
Reference to project-specific requirements like AGENTS.md.

## Session Timing
Performance tracking requirements if needed.
```

## Complete Configuration: Team Composition Examples

### Frontend-Focused Project

A frontend project typically benefits from two agents: one for implementation and one for design. The implementation agent gets full file and shell access to run builds and tests. The design agent gets file access plus webfetch to research design systems and component libraries.

Example agent roles:
- **frontend-engineer** — React, TypeScript, and modern web development; tools: read, write, edit, bash, grep, glob
- **ui-designer** — Design systems, UX, and component design; tools: read, write, edit, webfetch

### Full-Stack Project

A full-stack project typically separates frontend, backend, and infrastructure concerns into distinct agents. Each agent gets only the tools relevant to its domain. The infrastructure agent carries stricter bash permissions to prevent accidental destructive operations.

Example agent roles:
- **frontend-engineer** — UI components and client-side logic; tools: read, write, edit, bash
- **backend-developer** — APIs, databases, server architecture; tools: read, write, edit, bash
- **devops-engineer** — CI/CD, deployment, infrastructure; tools: read, write, edit, bash; bash permissions: docker=allow, kubectl=ask, rm=ask

### Data Science Project

Data science projects benefit from separating model development from infrastructure and analysis.

Example agent roles:
- **data-scientist** — ML models, data analysis, experimentation; tools: read, write, edit, bash, webfetch
- **ml-engineer** — Model deployment, MLOps, infrastructure; tools: read, write, edit, bash
- **research-analyst** — Literature review, insights, external research; tools: read, webfetch

## Agent Design Best Practices

1. **Single Responsibility** — Each agent should have a clear, focused purpose
2. **Minimal Tools** — Only grant necessary tools to each agent
3. **Clear Prompts** — Write specific, actionable prompts
4. **Consistent Naming** — Use descriptive, hyphenated agent names
5. **AGENTS.md Reference** — Always instruct agents to read the project's AGENTS.md before starting work

## Security Best Practices

1. **Principle of Least Privilege** — Grant minimal necessary permissions
2. **Sensitive Commands** — Use `"ask"` permission for destructive operations
3. **External Access** — Carefully control `webfetch` permissions; deny it for agents that don't need external access
4. **Environment Variables** — Never commit secrets in configuration files
5. **Regular Audits** — Periodically review agent permissions as project requirements evolve

## Troubleshooting

### Common Issues

1. **Agent Not Found** — Check agent name spelling and case sensitivity in configuration
2. **Tool Access Denied** — Review tool permissions assigned to the agent
3. **Prompt Errors** — Verify file paths and syntax in prompt files
4. **Model Errors** — Ensure specified model identifiers are currently available from your provider
