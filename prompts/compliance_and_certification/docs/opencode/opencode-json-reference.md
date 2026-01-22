# OpenCode Configuration Reference (`opencode.json`)

This document provides comprehensive documentation for the `opencode.json` configuration file that enables specialized AI subagents in your repository.

## Overview

The `opencode.json` file is the central configuration for OpenCode, allowing you to define specialized AI agents with specific capabilities, tools, and prompts tailored to different aspects of your project.

## Schema

```json
{
  "$schema": "https://opencode.ai/config.json"
}
```

Always include the schema reference for validation and IDE support.

## Core Configuration Structure

### Basic Configuration

```json
{
  "$schema": "https://opencode.ai/config.json",
  "model": "github-copilot/claude-3.5-sonnet",
  "small_model": "anthropic/claude-3-haiku",
  "agent": {
    // Agent definitions go here
  }
}
```

### Global Settings

| Field | Type | Description | Default |
|-------|------|-------------|---------|
| `model` | string | Default model for all agents | `"anthropic/claude-3.5-sonnet"` |
| `small_model` | string | Model for lightweight tasks | `"anthropic/claude-3-haiku"` |
| `username` | string | Custom username for conversations | System username |
| `share` | enum | Sharing behavior: `"manual"`, `"auto"`, `"disabled"` | `"manual"` |
| `autoupdate` | boolean | Automatically update OpenCode | `true` |

## Agent Configuration

The `agent` object defines specialized AI agents. Each agent has specific capabilities and tools.

### Agent Structure

```json
{
  "agent": {
    "agent-name": {
      "mode": "subagent",
      "model": "github-copilot/claude-3.5-sonnet",
      "prompt": "{file:./prompts/agent-name.md}",
      "description": "Brief description of agent purpose",
      "tools": {
        "read": true,
        "write": true,
        "edit": true,
        "bash": true,
        "grep": true,
        "glob": true,
        "list": true,
        "webfetch": false
      },
      "permission": {
        "edit": "ask",
        "bash": "allow",
        "webfetch": "deny"
      },
      "temperature": 0.1,
      "top_p": 0.9
    }
  }
}
```

### Agent Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `mode` | enum | Yes | Agent mode: `"subagent"`, `"primary"`, `"all"` |
| `model` | string | No | Model override for this agent |
| `prompt` | string | Yes | Prompt file path or inline prompt |
| `description` | string | No | When to use this agent |
| `tools` | object | No | Tool permissions (see Tools section) |
| `permission` | object | No | Permission settings (see Permissions section) |
| `temperature` | number | No | Model temperature (0.0-1.0) |
| `top_p` | number | No | Model top_p (0.0-1.0) |
| `disable` | boolean | No | Disable this agent |

## Agent Modes

### `"subagent"`
- **Purpose**: Specialized agents called by the main agent
- **Usage**: Accessed via the Task tool from the main agent
- **Best for**: Domain-specific expertise (frontend, backend, design, etc.)

### `"primary"` 
- **Purpose**: Main conversational agent
- **Usage**: Direct user interaction
- **Best for**: General-purpose assistance

### `"all"`
- **Purpose**: Available in both contexts
- **Usage**: Can be called as subagent or used directly
- **Best for**: Utilities and cross-cutting concerns

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

### Tool Configuration Examples

```json
{
  "tools": {
    // Basic file operations
    "read": true,
    "write": true,
    "edit": true,
    
    // Development tools
    "bash": true,
    "grep": true,
    "glob": true,
    "list": true,
    
    // External access
    "webfetch": false
  }
}
```

## Permissions

Fine-grained control over agent capabilities.

### Permission Levels

- `"allow"`: Agent can use without asking
- `"ask"`: Agent must request permission
- `"deny"`: Agent cannot use this capability

### Permission Configuration

```json
{
  "permission": {
    "edit": "ask",           // Ask before editing files
    "bash": "allow",         // Allow shell commands
    "webfetch": "deny",      // Deny web access
    "bash": {                // Command-specific bash permissions
      "rm": "deny",
      "git": "allow",
      "npm": "allow"
    }
  }
}
```

## Prompt Configuration

Prompts define the agent's personality, expertise, and behavior.

### File-based Prompts (Recommended)

```json
{
  "prompt": "{file:./prompts/frontend-engineer.md}"
}
```

### Inline Prompts

```json
{
  "prompt": "You are a frontend engineer specializing in React and TypeScript..."
}
```

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

## Real-World Examples

This repository includes practical examples in the `docs/opencode/` directory:

### Formatter Configuration Example
See [`formatters-example.json`](./formatters-example.json) for a comprehensive example showing:
- LSP server configuration for TypeScript, Python, Go, Rust, and Nix
- Formatter setup for multiple languages including Prettier, Black, gofmt, rustfmt, clang-format, and nixpkgs-fmt
- Proper file extension mappings and command configurations

### MCP Server Configuration Example  
See [`mcp-example.json`](./mcp-example.json) for examples of:
- Local MCP server with opencode package manager integration
- Remote MCP server with SSE endpoint configuration
- Both types configured in the same project

### Complete Configuration Examples

### Frontend-Focused Project

```json
{
  "$schema": "https://opencode.ai/config.json",
  "model": "github-copilot/claude-3.5-sonnet",
  "agent": {
    "frontend-engineer": {
      "mode": "subagent",
      "prompt": "{file:./prompts/frontend-engineer.md}",
      "description": "React, TypeScript, and modern web development",
      "tools": {
        "read": true,
        "write": true,
        "edit": true,
        "bash": true,
        "grep": true,
        "glob": true
      }
    },
    "ui-designer": {
      "mode": "subagent", 
      "prompt": "{file:./prompts/ui-designer.md}",
      "description": "Design systems, UX, and component design",
      "tools": {
        "read": true,
        "write": true,
        "edit": true,
        "webfetch": true
      }
    }
  }
}
```

### Full-Stack Project

```json
{
  "$schema": "https://opencode.ai/config.json",
  "model": "github-copilot/claude-3.5-sonnet",
  "agent": {
    "frontend-engineer": {
      "mode": "subagent",
      "prompt": "{file:./prompts/frontend-engineer.md}",
      "tools": {
        "read": true,
        "write": true,
        "edit": true,
        "bash": true
      }
    },
    "backend-developer": {
      "mode": "subagent",
      "prompt": "{file:./prompts/backend-developer.md}",
      "tools": {
        "read": true,
        "write": true,
        "edit": true,
        "bash": true
      }
    },
    "devops-engineer": {
      "mode": "subagent",
      "prompt": "{file:./prompts/devops-engineer.md}",
      "tools": {
        "read": true,
        "write": true,
        "edit": true,
        "bash": true
      },
      "permission": {
        "bash": {
          "docker": "allow",
          "kubectl": "allow",
          "rm": "ask"
        }
      }
    }
  }
}
```

## Advanced Features

### Model Context Protocol (MCP)

OpenCode supports both local and remote MCP servers to extend agent capabilities with external tools.

#### Local MCP Servers

```json
{
  "mcp": {
    "weather": {
      "type": "local",
      "command": ["opencode", "x", "@h1deya/mcp-server-weather"],
      "enabled": true,
      "environment": {
        "API_KEY": "${WEATHER_API_KEY}"
      }
    }
  }
}
```

#### Remote MCP Servers

```json
{
  "mcp": {
    "context7": {
      "type": "remote",
      "url": "https://mcp.context7.com/sse",
      "enabled": true,
      "headers": {
        "Authorization": "Bearer ${CONTEXT7_TOKEN}"
      }
    }
  }
}
```

### Formatters

OpenCode automatically formats files after editing using language-specific formatters.

#### Built-in Formatters

| Formatter | Extensions | Requirements |
|-----------|------------|--------------|
| `prettier` | .js, .jsx, .ts, .tsx, .html, .css, .md, .json, .yaml | `prettier` dependency in `package.json` |
| `biome` | .js, .jsx, .ts, .tsx, .html, .css, .md, .json, .yaml | `biome.json(c)` config file |
| `gofmt` | .go | `gofmt` command available |
| `ruff` | .py, .pyi | `ruff` command available with config |
| `clang-format` | .c, .cpp, .h, .hpp, .ino | `.clang-format` config file |
| `mix` | .ex, .exs, .eex, .heex, .leex, .neex, .sface | `mix` command available |
| `zig` | .zig, .zon | `zig` command available |
| `ktlint` | .kt, .kts | `ktlint` command available |
| `rubocop` | .rb, .rake, .gemspec, .ru | `rubocop` command available |
| `standardrb` | .rb, .rake, .gemspec, .ru | `standardrb` command available |
| `htmlbeautifier` | .erb, .html.erb | `htmlbeautifier` command available |

#### Custom Formatter Configuration

```json
{
  "formatter": {
    "prettier": {
      "disabled": false,
      "command": ["npx", "prettier", "--write", "$FILE"],
      "extensions": [".js", ".ts", ".jsx", ".tsx", ".json", ".md", ".css", ".html"],
      "environment": {
        "NODE_ENV": "development"
      }
    },
    "nixpkgs-fmt": {
      "disabled": false,
      "command": ["nixpkgs-fmt", "$FILE"],
      "extensions": [".nix"]
    }
  }
}
```

#### Formatter Configuration Fields

| Field | Type | Description |
|-------|------|-------------|
| `disabled` | boolean | Set to `true` to disable the formatter |
| `command` | string[] | The command to run for formatting (use `$FILE` placeholder) |
| `extensions` | string[] | File extensions this formatter should handle |
| `environment` | object | Environment variables to set when running the formatter |

### Language Server Protocol (LSP)

OpenCode can integrate with language servers to provide intelligent code analysis and editing capabilities.

```json
{
  "lsp": {
    "typescript": {
      "command": ["typescript-language-server", "--stdio"],
      "extensions": [".ts", ".tsx", ".js", ".jsx"],
      "initialization": {
        "preferences": {
          "includeInlayParameterNameHints": "all",
          "includeInlayVariableTypeHints": true,
          "includeInlayFunctionParameterTypeHints": true
        }
      }
    },
    "python": {
      "command": ["pylsp"],
      "extensions": [".py"],
      "initialization": {
        "settings": {
          "pylsp": {
            "plugins": {
              "pycodestyle": {"enabled": true},
              "pyflakes": {"enabled": true},
              "pylint": {"enabled": true}
            }
          }
        }
      }
    },
    "rust": {
      "command": ["rust-analyzer", "--stdio"],
      "extensions": [".rs"],
      "initialization": {
        "settings": {
          "rust-analyzer": {
            "checkOnSave": {
              "command": "clippy"
            }
          }
        }
      }
    },
    "nix": {
      "command": ["nil"],
      "extensions": [".nix"],
      "initialization": {
        "settings": {
          "nil": {
            "formatting": {
              "command": ["nixpkgs-fmt"]
            }
          }
        }
      }
    }
  }
}
```

## Best Practices

### Agent Design
1. **Single Responsibility**: Each agent should have a clear, focused purpose
2. **Minimal Tools**: Only grant necessary tools to each agent  
3. **Clear Prompts**: Write specific, actionable prompts
4. **Consistent Naming**: Use descriptive, hyphenated agent names

### Security
1. **Principle of Least Privilege**: Grant minimal necessary permissions
2. **Sensitive Commands**: Use `"ask"` permission for destructive operations
3. **External Access**: Carefully control `webfetch` permissions
4. **Environment Variables**: Never commit secrets in configuration

### Performance
1. **Model Selection**: Use appropriate models for task complexity
2. **Tool Optimization**: Enable only required tools
3. **Prompt Efficiency**: Keep prompts focused and concise
4. **Caching**: Leverage OpenCode's built-in caching

### Maintenance
1. **Version Control**: Track configuration changes in git
2. **Documentation**: Keep agent prompts well-documented
3. **Testing**: Validate agent behavior regularly
4. **Updates**: Stay current with OpenCode schema changes

## Troubleshooting

### Common Issues

1. **Agent Not Found**: Check agent name spelling and case sensitivity
2. **Tool Access Denied**: Review tool permissions in configuration
3. **Prompt Errors**: Verify file paths and syntax in prompt files
4. **Model Errors**: Ensure specified models are available

### Validation

```bash
# Validate configuration
opencode validate

# Test specific agent
opencode test-agent frontend-engineer

# Check tool permissions
opencode check-permissions
```

## Migration Guide

### From Legacy `mode` to `agent`

Old format:
```json
{
  "mode": {
    "build": { "prompt": "..." }
  }
}
```

New format:
```json
{
  "agent": {
    "build": { "prompt": "..." }
  }
}
```

### Schema Updates

Always use the latest schema:
```json
{
  "$schema": "https://opencode.ai/config.json"
}
```

This ensures compatibility with new features and validation.