# Available OpenCode Models

*Last updated: January 22, 2025*

## Current Model Catalog

The following models are available for use in OpenCode configurations:

### OpenCode Native
- `opencode/sonic` - Fast, efficient model for standard development tasks

### GitHub Copilot Models

#### GPT Models
- `github-copilot/gpt-5` - Latest GPT model with enhanced capabilities
- `github-copilot/gpt-5-mini` - Lightweight version of GPT-5
- `github-copilot/gpt-4.1` - Improved GPT-4 variant
- `github-copilot/gpt-4o` - Optimized GPT-4 for development tasks
- `github-copilot/o3` - Advanced reasoning model
- `github-copilot/o3-mini` - Lightweight reasoning model
- `github-copilot/o4-mini` - Next-generation compact model

#### Claude Models
- `github-copilot/claude-3.7-sonnet` - Latest Claude Sonnet variant
- `github-copilot/claude-3.5-sonnet` - Balanced performance and capability
- `github-copilot/claude-opus-4` - High-capability creative model
- `github-copilot/claude-opus-41` - Enhanced Opus variant
- `github-copilot/claude-3.7-sonnet-thought` - Reasoning-focused Claude
- `github-copilot/claude-sonnet-4` - Latest Sonnet generation

#### Gemini Models
- `github-copilot/gemini-2.5-pro` - Advanced Gemini professional model
- `github-copilot/gemini-2.0-flash-001` - Fast Gemini variant

## Model Selection Guide

### By Task Type

#### Standard Development (Recommended: `github-copilot/claude-3.5-sonnet`)
- Code implementation
- Bug fixes
- Refactoring
- API development

#### Complex Architecture (Recommended: `github-copilot/claude-opus-4`)
- System design
- Architecture decisions
- Complex problem solving
- Strategic planning

#### Fast Operations (Recommended: `github-copilot/o3-mini`)
- Code formatting
- Simple edits
- Quick reviews
- Validation

#### Creative Work (Recommended: `github-copilot/claude-opus-41`)
- Content creation
- Game design
- User experience design
- Creative problem solving

#### Reasoning Tasks (Recommended: `github-copilot/claude-3.7-sonnet-thought`)
- Complex analysis
- Multi-step planning
- Research synthesis
- Strategic thinking

### Configuration Examples

#### Balanced Configuration
```json
{
  "model": "github-copilot/claude-3.5-sonnet",
  "small_model": "github-copilot/o3-mini",
  "agent": {
    "frontend-engineer": {
      "model": "github-copilot/claude-3.5-sonnet"
    },
    "architect": {
      "model": "github-copilot/claude-opus-4"
    },
    "formatter": {
      "model": "github-copilot/o3-mini"
    }
  }
}
```

#### Performance-Optimized Configuration
```json
{
  "model": "github-copilot/claude-3.5-sonnet",
  "small_model": "github-copilot/gpt-5-mini",
  "agent": {
    "development": {
      "model": "opencode/sonic"
    },
    "research": {
      "model": "github-copilot/gemini-2.5-pro"
    }
  }
}
```

#### High-Capability Configuration
```json
{
  "model": "github-copilot/claude-opus-41",
  "small_model": "github-copilot/o3-mini",
  "agent": {
    "senior-architect": {
      "model": "github-copilot/o3"
    },
    "creative-designer": {
      "model": "github-copilot/claude-opus-4"
    }
  }
}
```

## Model Updates

### Update Schedule
- **Weekly**: Check for new models and updates
- **Review Date**: Every Wednesday
- **Update Command**: `opencode models`

### Update Process
1. Run `opencode models` to get current list
2. Compare with this documentation
3. Update model catalog section
4. Update last updated date
5. Review configuration examples for new model opportunities

### Maintenance Checklist
- [ ] Weekly model availability check
- [ ] Update catalog with new models
- [ ] Remove deprecated models
- [ ] Update configuration examples
- [ ] Test new models with existing agents

## Deprecated Models
*Models that are no longer available will be listed here with migration recommendations*

---

**Note**: Model availability may change. Always verify current models with `opencode models` before updating configurations.