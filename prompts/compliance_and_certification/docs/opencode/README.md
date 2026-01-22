# OpenCode Configuration Documentation

This directory contains comprehensive documentation for configuring OpenCode with specialized AI subagents in any repository.

## What is OpenCode?

OpenCode is an AI-powered development tool that allows you to configure specialized AI agents for different aspects of your project. Each agent has specific capabilities, tools, and prompts tailored to their domain expertise.

## Documentation Structure

- **[opencode.json Reference](./opencode-json-reference.md)** - Complete configuration file documentation
- **[Subagent Architecture](./subagent-architecture.md)** - How to design and implement specialized agents
- **[Setup Guide](./setup-guide.md)** - Step-by-step implementation instructions
- **[Best Practices](./best-practices.md)** - Proven patterns and recommendations
- **[Models](./models.md)** - Available models and selection strategies
- **[Examples](./examples/)** - Real-world configuration examples

## Real Configuration Examples

This directory includes working examples:

- **[formatters-example.json](./formatters-example.json)** - Complete LSP and formatter configuration
- **[mcp-example.json](./mcp-example.json)** - Local and remote MCP server setup

## Quick Start

1. Create an `opencode.json` file in your repository root
2. Define your specialized agents in the `agent` section
3. Create prompt files in a `prompts/` directory
4. Configure tools and permissions for each agent
5. Optionally add LSP servers, formatters, and MCP servers
6. Test your configuration with `opencode validate`

## Key Benefits

- **Specialized Expertise**: Each agent focuses on specific domains (frontend, backend, design, etc.)
- **Consistent Context**: Agents maintain domain-specific knowledge and patterns
- **Controlled Access**: Fine-grained tool permissions per agent
- **Enhanced Development**: LSP integration and automatic formatting
- **Extended Capabilities**: MCP servers for external tool integration
- **Scalable Workflows**: Multiple agents can work in parallel on different aspects
- **Reproducible Results**: Consistent prompts and configurations across team members

## Example Configuration

```json
{
  "$schema": "https://opencode.ai/config.json",
  "agent": {
    "frontend-engineer": {
      "mode": "subagent",
      "model": "github-copilot/claude-3.5-sonnet",
      "prompt": "{file:./prompts/frontend-engineer.md}",
      "tools": {
        "read": true,
        "write": true,
        "edit": true,
        "bash": true
      }
    }
  },
  "formatter": {
    "prettier": {
      "disabled": false,
      "command": ["npx", "prettier", "--write", "$FILE"],
      "extensions": [".js", ".ts", ".jsx", ".tsx", ".json", ".md"]
    }
  }
}
```

## Getting Started

Read the [Setup Guide](./setup-guide.md) for detailed implementation instructions, or explore the [Examples](./examples/) directory for real-world configurations you can adapt to your project.