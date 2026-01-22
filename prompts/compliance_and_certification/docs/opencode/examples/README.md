# Real-World OpenCode Examples

This directory contains practical examples of OpenCode configurations for different types of projects.

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

## Quick Start Examples

### Minimal Configuration
```json
{
  "$schema": "https://opencode.ai/config.json",
  "model": "github-copilot/claude-3.5-sonnet",
  "agent": {
    "developer": {
      "mode": "subagent",
      "prompt": "{file:./prompts/developer.md}",
      "tools": {
        "read": true,
        "write": true,
        "edit": true,
        "bash": true
      }
    }
  }
}
```

### Multi-Domain Configuration
```json
{
  "$schema": "https://opencode.ai/config.json",
  "model": "github-copilot/claude-3.5-sonnet",
  "agent": {
    "frontend-engineer": {
      "mode": "subagent",
      "prompt": "{file:./prompts/frontend-engineer.md}",
      "description": "React, TypeScript, UI development"
    },
    "backend-engineer": {
      "mode": "subagent", 
      "prompt": "{file:./prompts/backend-engineer.md}",
      "description": "APIs, databases, server logic"
    },
    "researcher": {
      "mode": "subagent",
      "prompt": "{file:./prompts/researcher.md}",
      "description": "Research, documentation, analysis",
      "tools": {
        "webfetch": true
      }
    }
  }
}
```

## Usage Patterns

### Development Workflow
```bash
# Get list of available agents
opencode run "List all available agents"

# Use specific agent for task
opencode run --agent frontend-engineer "Create a login component"

# Sequential agent collaboration
opencode run "Design and implement a user authentication system"
# Primary agent coordinates: architect → frontend → backend → tester
```

### Code Review Process
```bash
# Multiple review stages
opencode run --agent senior-reviewer "Review this pull request for architecture"
opencode run --agent security-reviewer "Check for security vulnerabilities"
opencode run --agent performance-reviewer "Analyze performance implications"
```

### Research and Implementation
```bash
# Research phase
opencode run --agent researcher "Research best practices for React 18 concurrent features"

# Implementation phase  
opencode run --agent frontend-engineer "Implement the researched React patterns"

# Documentation phase
opencode run --agent technical-writer "Document the new implementation patterns"
```

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
```json
{
  "agent": {
    "full-stack-helper": {
      "description": "Versatile agent for solo development"
    },
    "researcher": {
      "description": "Research and learning assistant"
    }
  }
}
```

#### Small Team (2-5)
```json
{
  "agent": {
    "frontend-engineer": { "description": "Frontend specialist" },
    "backend-engineer": { "description": "Backend specialist" },
    "devops-engineer": { "description": "Deployment and infrastructure" }
  }
}
```

#### Large Team (10+)
```json
{
  "agent": {
    "senior-architect": { "description": "System design and decisions" },
    "frontend-engineer": { "description": "React and TypeScript" },
    "backend-engineer": { "description": "APIs and databases" },
    "mobile-engineer": { "description": "React Native development" },
    "devops-engineer": { "description": "CI/CD and infrastructure" },
    "qa-engineer": { "description": "Testing and quality assurance" },
    "security-engineer": { "description": "Security review and compliance" },
    "technical-writer": { "description": "Documentation and guides" }
  }
}
```

## Best Practice Examples

### Security-First Configuration
```json
{
  "permission": {
    "bash": {
      "rm": "ask",
      "sudo": "deny",
      "docker": "ask",
      "git": "allow"
    },
    "edit": "ask",
    "webfetch": "ask"
  }
}
```

### Performance-Optimized Configuration
```json
{
  "model": "github-copilot/claude-3.5-sonnet",
  "small_model": "github-copilot/o3-mini",
  "agent": {
    "formatter": {
      "model": "github-copilot/o3-mini",
      "temperature": 0.0
    },
    "reviewer": {
      "model": "github-copilot/claude-3.5-sonnet",
      "temperature": 0.1
    }
  }
}
```

### Research-Enhanced Configuration
```json
{
  "agent": {
    "researcher": {
      "tools": {
        "webfetch": true
      },
      "permission": {
        "webfetch": "allow"
      }
    },
    "analyst": {
      "tools": {
        "webfetch": true
      }
    }
  }
}
```

## Integration Examples

### GitHub Actions Integration
```yaml
name: AI Code Review
on: [pull_request]
jobs:
  ai-review:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install OpenCode
        run: npm install -g opencode
      - name: Run AI Review
        run: opencode run --agent code-reviewer "Review this PR"
```

### Pre-commit Hook Integration
```bash
#!/bin/bash
# .git/hooks/pre-commit
opencode run --agent formatter "Format staged files"
opencode run --agent linter "Check code quality"
```

### VS Code Integration
```json
{
  "tasks": [
    {
      "label": "OpenCode Review",
      "type": "shell",
      "command": "opencode run --agent code-reviewer 'Review current file'"
    }
  ]
}
```

## Troubleshooting Examples

### Common Issues and Solutions

#### Agent Not Responding
```bash
# Check agent configuration
cat opencode.json | jq '.agent.["agent-name"]'

# Verify prompt file exists
ls -la prompts/

# Test basic connectivity
opencode run "Hello, test agent communication"
```

#### Permission Denied
```bash
# Check tool permissions
cat opencode.json | jq '.agent.["agent-name"].tools'

# Review global permissions
cat opencode.json | jq '.permission'

# Test specific tool access
opencode run --agent developer "List current directory files"
```

#### Slow Performance
```bash
# Check model configuration
cat opencode.json | jq '.model, .small_model'

# Optimize tool access
# Remove unnecessary tools from agent configurations

# Monitor response times
time opencode run --agent developer "Simple task"
```

## Migration Examples

### From Manual AI Usage
```bash
# Before: Copy/paste prompts manually
# After: Configured agents

# Extract common prompts
mkdir prompts
echo "You are a React expert..." > prompts/frontend-engineer.md

# Create configuration
cat > opencode.json << EOF
{
  "agent": {
    "frontend-engineer": {
      "prompt": "{file:./prompts/frontend-engineer.md}"
    }
  }
}
EOF
```

### From Other AI Tools
```bash
# Map existing workflows to OpenCode agents
# Convert tool-specific prompts to OpenCode format
# Test equivalent functionality
# Gradually migrate workflows
```

These examples provide practical starting points for implementing OpenCode in various project contexts. Adapt the configurations to match your specific requirements and technology stack.