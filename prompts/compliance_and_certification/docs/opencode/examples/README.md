# Real-World Examples for AI Coding Tools

This directory contains practical examples of configurations and workflows for different types of projects using your AI coding tool.

## Example Categories

### [Game Development](./game-development/)
Complete configuration for a cannabis cultivation game project with specialized agents for game design, cannabis expertise, and technical implementation.

### [Web Application](./web-application/)
Frontend and backend agents for modern web development with React, TypeScript, and Node.js.

### [Data Science](./data-science/)
ML engineers, data scientists, and research analysts for machine learning projects.

### [DevOps & Infrastructure](./devops/)
System architects, deployment engineers, and monitoring specialists.

### [Content & Documentation](./content/)
Technical writers, content creators, and documentation specialists.

## Usage Patterns

### Development Workflow

A typical development workflow uses your AI coding tool's primary agent to coordinate across specialisms: the agent researches requirements, then delegates implementation across architecture, frontend, backend, and testing roles before consolidating.

### Code Review Process

A multi-stage review workflow routes a pull request through successive specialist reviewers — architecture first, then security, then performance — each building on the previous agent's findings.

### Research and Implementation

A three-phase pattern:
1. **Research phase** — a researcher agent surveys best practices and relevant library options.
2. **Implementation phase** — a domain engineer agent implements the researched patterns.
3. **Documentation phase** — a technical-writer agent documents the new implementation.

## Configuration Templates

### By Project Type

#### Startup/MVP
- Focus on speed and MVP features
- Minimal agent specialization
- Balanced capability across domains

#### Enterprise
- Security-focused permissions
- Comprehensive review processes
- Compliance and audit capabilities

#### Open Source
- Documentation-heavy configuration
- Community contribution support
- Code quality and consistency focus

#### Research/Academic
- Research and analysis emphasis
- Literature review capabilities
- Experimental feature development

### By Team Size

#### Solo Developer
Two agents cover most needs: a versatile full-stack helper for implementation tasks and a researcher/learning assistant for exploratory work.

#### Small Team (2–5)
Three specialisms map cleanly onto typical feature work: a frontend engineer, a backend engineer, and a devops engineer covering deployment and infrastructure.

#### Large Team (10+)
Dedicated agents per discipline — senior architect, frontend, backend, mobile, devops, QA, security engineer, and technical writer — allow each team member to work with an agent that matches their domain context without overlap.

## Best Practice Examples

### Security-First Configuration
Restrict dangerous shell commands (`rm`, `sudo`, `docker`) to require explicit approval, deny `sudo` outright, and set `webfetch` to ask-before-use. Allow safe read-only operations (`git`) without interruption.

### Performance-Optimized Configuration
Route low-complexity tasks (formatting, linting) to a fast, low-temperature small model. Reserve the primary model with a slightly higher temperature for reasoning-heavy tasks (review, architecture).

### Research-Enhanced Configuration
Grant `webfetch` permission to research and analyst agents only. Keep all other agents restricted to local filesystem tools to reduce surface area.

## Troubleshooting

### Agent Not Responding
1. Confirm the agent's prompt file exists at the referenced path.
2. Validate the configuration syntax (JSON must be well-formed and reference a valid schema).
3. Test with a minimal prompt to isolate whether the problem is configuration or model availability.

### Permission Denied
1. Check the agent's declared tool list — a tool must be listed to be available.
2. Review global permission overrides, which take precedence over agent-level settings.
3. Use a minimal test prompt to confirm which tool is being blocked.

### Slow Performance
1. Check the model assigned to the agent — large models have higher latency.
2. Remove tools from agent configurations that the agent does not actually need.
3. Route formatting and simple tasks to a smaller, faster model.

## Language Server and Formatter Reference

The following LSP servers and formatters are commonly used with AI coding tools that support language server integration:

| Language | LSP Server Command | Formatter Command |
|----------|--------------------|-------------------|
| TypeScript/JavaScript | `typescript-language-server --stdio` | `npx prettier --write $FILE` |
| Python | `pylsp` | `black $FILE` |
| Go | `gopls` | `gofmt -w $FILE` |
| Rust | `rust-analyzer --stdio` | `rustfmt $FILE` |
| C/C++ | `clangd` | `clang-format -i $FILE` |
| Nix | `nil` | `nixpkgs-fmt $FILE` |

These examples provide practical starting points for implementing your AI coding tool across various project contexts. Adapt the configurations to match your specific requirements and technology stack.
