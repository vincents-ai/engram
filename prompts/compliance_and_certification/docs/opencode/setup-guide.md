# AI Coding Tool Setup Guide

This guide provides step-by-step instructions for implementing your AI coding tool with specialized AI subagents in any repository.

## Detailed Implementation

### Step 1: Project Analysis

Identify the key domains in your project:

```bash
# Analyze your codebase structure
find . -type f -name "*.js" -o -name "*.ts" -o -name "*.py" -o -name "*.go" | head -20

# Identify primary technologies
ls package.json requirements.txt go.mod pom.xml 2>/dev/null

# Check build and deployment systems
ls Dockerfile docker-compose.yml .github/workflows/ .gitlab-ci.yml 2>/dev/null
```

### Step 2: Define Agent Roles

Based on analysis, create specialized agents.

#### Web Development Project

Agents to define for a typical web project:

- **frontend-engineer** — React, TypeScript, modern web development
- **backend-developer** — APIs, databases, server architecture
- **devops-engineer** — CI/CD, deployment, infrastructure

#### Data Science Project

Agents to define for a data science project:

- **data-scientist** — ML models, data analysis, experimentation
- **ml-engineer** — Model deployment, MLOps, infrastructure
- **research-analyst** — Domain research, literature review, insights

### Step 3: Create Agent Prompts

#### Template Structure

```markdown
# Agent Name

## Role
Clear description of the agent's primary function.

## Primary Prompt
Detailed instructions including:
- Core responsibilities
- Technical expertise
- Quality standards
- Integration requirements

## Additional Capabilities
- Secondary functions
- Cross-domain knowledge
- Research abilities

## Important: Project Guidelines
**MANDATORY**: Before starting any work, read the `AGENTS.md` file in the project root. It contains essential build commands, code style guidelines, git workflow requirements, and architectural conventions that must be followed for all work in this codebase.

## Session Timing
**REQUIRED**: For performance tracking:
1. Run `date` command at the very start of your session and note the timestamp
2. Run `date` command at the very end before responding with final outcome  
3. Calculate and report the total duration in your final response
```

#### Frontend Engineer Example

```markdown
# Frontend Engineer AI

## Role
A specialized AI focused on modern frontend development using React, TypeScript, and contemporary web technologies.

## Primary Prompt
You are a senior frontend engineer AI specializing in React and TypeScript development. Your primary responsibilities include:

- **Component Development**: Create reusable, well-typed React components following modern patterns
- **State Management**: Implement and maintain state using appropriate libraries (Redux, Zustand, Context)
- **TypeScript Implementation**: Write type-safe code with proper interfaces and type definitions
- **Performance Optimization**: Implement best practices for React performance (memoization, lazy loading, etc.)
- **Testing**: Write comprehensive unit and integration tests using modern testing frameworks
- **Build Optimization**: Configure and optimize webpack, Vite, or similar build tools
- **Accessibility**: Ensure WCAG compliance and semantic HTML

When working on tasks:
1. Always check existing code patterns and follow project conventions
2. Use TypeScript strictly - avoid `any` types
3. Implement proper error boundaries and error handling
4. Follow the project's component architecture patterns
5. Write tests for new functionality
6. Consider performance implications of your implementations

## Additional Capabilities
- UI/UX implementation and improvement suggestions
- CSS-in-JS and styling system optimization
- Progressive Web App (PWA) implementation
- Frontend build pipeline optimization
- Code review and refactoring recommendations
- Integration with backend APIs and real-time features

## Important: Project Guidelines
**MANDATORY**: Before starting any work, read the `AGENTS.md` file in the project root. It contains essential build commands, code style guidelines, git workflow requirements, and architectural conventions that must be followed for all work in this codebase.

## Session Timing
**REQUIRED**: For performance tracking:
1. Run `date` command at the very start of your session and note the timestamp
2. Run `date` command at the very end before responding with final outcome
3. Calculate and report the total duration in your final response
```

#### Backend Developer Example

```markdown
# Backend Developer AI

## Role
A specialized AI focused on server-side development, API design, and database architecture.

## Primary Prompt
You are a senior backend developer AI. Your primary responsibilities include:

- **API Development**: Design and implement RESTful and GraphQL APIs
- **Database Architecture**: Schema design, query optimization, migrations
- **Authentication & Authorization**: Secure auth flows and access control
- **Performance**: Caching strategies, query optimization, profiling
- **Testing**: Unit, integration, and API tests

## Important: Project Guidelines
**MANDATORY**: Before starting any work, read the `AGENTS.md` file in the project root.
```

### Step 4: Configure Tools and Permissions

Agents should be granted access to tools according to the principle of least privilege.

#### Basic Development Setup (Tier 1)

Tools for standard file-level development work:

- `read`, `write`, `edit` — file operations
- `bash`, `grep`, `glob`, `list` — development tooling

#### Research-Enabled Setup (Tier 2)

All basic tools plus:

- `webfetch` — external documentation and research access

#### Security-Conscious Setup (Tier 3)

All basic tools with command-level bash restrictions:

- `bash.rm` → `"ask"` — confirm before deleting
- `bash.sudo` → `"deny"` — block privilege escalation
- `bash.git` → `"allow"` — permit git operations
- `bash.npm` → `"allow"` — permit package operations
- `bash.docker` → `"ask"` — confirm container operations
- `edit` → `"ask"` — confirm file modifications

### Step 5: Project Integration

#### Create AGENTS.md

```markdown
# AGENTS.md - Codebase Guide for AI Agents

## Build Commands
- **Build**: `npm run build` 
- **Dev**: `npm run dev`
- **Test**: `npm test`
- **Lint**: `npm run lint`
- **Format**: `npm run format`

## Code Style
- **Language**: TypeScript strict mode
- **Formatting**: Prettier with 2 spaces, single quotes
- **Imports**: Use absolute imports with path mapping
- **Components**: Function components with TypeScript

## Architecture
- Frontend: React + TypeScript + Vite
- State: Zustand for global state, React Query for server state
- Styling: CSS Modules with SCSS
- Testing: Vitest + React Testing Library

## Git Workflow
- **Commits**: Conventional commits with detailed descriptions
- **Branches**: feature/task-description format
- **Reviews**: All changes require review before merge
```

#### Update package.json Scripts

```json
{
  "scripts": {
    "build": "vite build",
    "dev": "vite",
    "test": "vitest",
    "test:ui": "vitest --ui",
    "lint": "eslint . --ext ts,tsx --report-unused-disable-directives --max-warnings 0",
    "lint:fix": "eslint . --ext ts,tsx --fix",
    "format": "prettier --write \"src/**/*.{ts,tsx,js,jsx,json,css,md}\"",
    "format:check": "prettier --check \"src/**/*.{ts,tsx,js,jsx,json,css,md}\"",
    "typecheck": "tsc --noEmit"
  }
}
```

## Advanced Configuration

### Environment-Specific Configuration

For production-sensitive agents, apply stricter permission tiers. For example, a production deployer agent might be granted only `read` and `bash`, with command-level restrictions:

- `bash.kubectl` → `"ask"`
- `bash.docker` → `"allow"`
- `bash.rm` → `"deny"`

This pattern limits blast radius while still enabling necessary deployment operations.

## Troubleshooting

### Common Issues

#### Agent Not Found

- Check agent names in your configuration for correct spelling and case
- Review your agent configuration file to confirm the agent is defined

#### Permission Denied

- Review the tool permissions assigned to the agent
- Check command-specific bash permission settings

#### Prompt File Errors

```bash
# Verify prompt file exists
ls -la prompts/

# Check file path matches your configuration
```

### Performance Optimization

#### Model Selection
- Use a fast/lightweight model for simple tasks (formatting, quick reviews, validation)
- Use a standard balanced model for general development work
- Use a high-capability model for complex architecture and planning tasks

#### Tool Optimization
- Only enable necessary tools per agent
- Use specific bash permissions rather than blanket allow
- Limit webfetch for security-sensitive agents

#### Prompt Optimization
- Keep prompts focused and specific
- Include relevant context and examples
- Reference project guidelines clearly

## Migration from Manual AI Usage

1. Extract common prompts into agent files
2. Define tool boundaries for each use case
3. Create agent-specific configurations
4. Test workflows with new agents
5. Iterate on prompt quality based on output

## Best Practices Summary

### Security
- Apply principle of least privilege
- Use `"ask"` permission for sensitive operations
- Regularly review and audit permissions
- Avoid hardcoding secrets

### Maintenance
- Keep prompts updated with project evolution
- Monitor agent performance and accuracy
- Collect feedback and iterate on configurations
- Document changes and decisions

### Team Adoption
- Provide training on agent capabilities
- Create usage guidelines and examples
- Share successful patterns and configurations
- Establish review processes for agent changes

This setup guide provides the foundation for implementing AI subagents in any project. Adapt the examples and configurations to match your specific technology stack and team requirements.
