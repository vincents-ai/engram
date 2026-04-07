> **Note:** This documentation describes multi-agent AI tool configuration patterns. Examples were originally written for OpenCode but the concepts apply to any AI coding tool.

# Multi-Agent Configuration Documentation

This directory contains comprehensive documentation for configuring your AI coding tool with specialized AI subagents in any repository.

## Documentation Structure

- **[Subagent Architecture](./subagent-architecture.md)** - How to design and implement specialized agents
- **[Best Practices](./best-practices.md)** - Proven patterns and recommendations

## Key Benefits

- **Specialized Expertise**: Each agent focuses on specific domains (frontend, backend, design, etc.)
- **Consistent Context**: Agents maintain domain-specific knowledge and patterns
- **Controlled Access**: Fine-grained tool permissions per agent
- **Scalable Workflows**: Multiple agents can work in parallel on different aspects
- **Reproducible Results**: Consistent prompts and configurations across team members

## The `prompts/` Pattern

The recommended approach is to store agent system prompts as individual Markdown files in a `prompts/` directory at the root of your repository. Your AI coding tool configuration then references these files by path.

This pattern:
- Keeps prompts version-controlled alongside the code they govern
- Allows per-agent iteration without touching the tool config
- Makes agent responsibilities visible to the whole team

Example directory layout:

```
prompts/
├── README.md                         # Agent overview and index
├── frontend-typescript-engineer.md
├── backend-developer.md
├── devops-engineer.md
└── researcher.md
```

## Getting Started

1. Identify the key domains in your project
2. Create a `prompts/` directory at your repository root
3. Write one Markdown prompt file per agent role
4. Configure your AI coding tool to reference those prompt files
5. Set per-agent tool permissions following the principle of least privilege

See [Subagent Architecture](./subagent-architecture.md) for design guidance, and [Best Practices](./best-practices.md) for proven configuration patterns.
