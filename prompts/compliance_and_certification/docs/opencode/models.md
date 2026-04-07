# Model Selection Guide

*Note: Consult your AI coding tool's documentation for available model identifiers. The guidance below focuses on capability levels and task matching.*

## Model Selection Guide

### By Task Type

Consult your AI coding tool's documentation for available model identifiers. When selecting models, consider task complexity:

| Task Type | Examples | Recommended Capability Level |
|-----------|----------|------------------------------|
| **Fast Operations** | Code formatting, simple edits, quick reviews, validation | Fast / lightweight |
| **Standard Development** | Code implementation, bug fixes, refactoring, API development | Standard / balanced |
| **Complex Architecture** | System design, architecture decisions, complex problem solving, strategic planning | Capable / high-performance |
| **Reasoning Tasks** | Complex analysis, multi-step planning, research synthesis, strategic thinking | Powerful / reasoning-focused |
| **Creative Work** | Content creation, UX design, creative problem solving | Capable / high-performance |

### Selection Principles

- **Match capability to complexity**: Avoid using a powerful model for simple, well-defined tasks — this increases cost and latency without improving output.
- **Use lightweight models as a "small model"**: Many tools support a `small_model` configuration for lightweight background tasks such as summarisation or routing decisions.
- **Consider context window**: Long-context tasks (large codebases, document synthesis) may require models with larger context windows regardless of raw capability.
- **Reasoning models for planning**: When an agent needs to break down a problem before acting, prefer a model with explicit reasoning capability over a standard generation model.

## Model Updates

### Maintenance Checklist
- [ ] Periodically review your AI coding tool's model catalog for new additions
- [ ] Remove references to deprecated model identifiers from configuration
- [ ] Update configuration examples when better models become available for a task type
- [ ] Test agents with new models before rolling out broadly

## Deprecated Models

*When a model identifier is retired by your provider, update all agent configurations that reference it before the deprecation deadline. Document the replacement model chosen and why.*
