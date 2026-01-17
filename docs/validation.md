# Engram Validation Configuration

## Validation Rules Configuration

The validation system supports flexible configuration through YAML files or CLI options.

### Configuration File

Create a `.engram/validation.yaml` file in your project root:

```yaml
enabled: true
require_task_reference: true
require_reasoning_relationship: true
require_context_relationship: true
require_file_scope_match: true

task_id_patterns:
  - pattern: '\[([A-Z]+-\d+)\]'
    name: "Brackets format"
    example: "[TASK-123]"
  - pattern: '\[task:([a-z0-9-]+)\]'
    name: "Colon format"
    example: "[task:auth-impl-001]"
  - pattern: 'Refs:\s*#(\d+)'
    name: "Refs format"
    example: "Refs: #456"

exemptions:
  - message_pattern: '^(chore|docs):'
    skip_validation: false
    skip_specific:
      - "require_task_reference"
  - message_pattern: '^fixup!'
    skip_validation: true
  - message_pattern: '^amend!'
    skip_validation: true

performance:
  cache_ttl_seconds: 300
  max_cache_entries: 1000
  enable_parallel_validation: true
  validation_timeout_seconds: 30
```

### Environment Variables

You can override configuration with environment variables:

- `ENGRAM_VALIDATION_ENABLED` - Enable/disable validation
- `ENGRAM_VALIDATION_STRICT` - Enable strict mode (no exemptions)
- `ENGRAM_VALIDATION_CACHE_TTL` - Cache TTL in seconds
- `ENGRAM_VALIDATION_TIMEOUT` - Validation timeout in seconds

### CLI Commands

```bash
# Validate a commit message
engram validation commit --message "feat: implement authentication [TASK-123]"

# Validate with dry run (doesn't require git repo)
engram validation commit --message "test commit" --dry-run

# Install pre-commit hook
engram validation hook install

# Check hook status
engram validation hook status

# Uninstall hook
engram validation hook uninstall
```

### Configuration Priority

1. Environment variables (highest priority)
2. Project configuration file (`.engram/validation.yaml`)
3. User configuration file (`~/.engram/validation.yaml`)
4. Default configuration

### Performance Tuning

For optimal performance:

1. **Cache Warming**: Pre-load common task IDs
   ```bash
   engram validation warm-cache TASK-123 TASK-456
   ```

2. **Parallel Validation**: Enable multi-threaded validation
   ```yaml
   performance:
     enable_parallel_validation: true
   ```

3. **Memory Management**: Configure appropriate cache sizes
   ```yaml
   performance:
     max_cache_entries: 5000  # For large repositories
     cache_ttl_seconds: 600   # 10 minutes
   ```

### Advanced Configuration

#### Custom Task ID Patterns

Add your own task ID patterns:

```yaml
task_id_patterns:
  - pattern: 'PROJECT-(\d+)'
    name: "Project format"
    example: "PROJECT-123"
  - pattern: 'jira/([A-Z]+-\d+)'
    name: "JIRA format"
    example: "jira/PROJ-123"
```

#### Custom Exemptions

Create exemptions for specific commit types:

```yaml
exemptions:
  - message_pattern: '^(hotfix|emergency):'
    skip_validation: false
    skip_specific:
      - "require_reasoning_relationship"
      - "require_context_relationship"
  - message_pattern: '^WIP:'
    skip_validation: true
    reason: "Work in progress commits"
```

### Integration with CI/CD

#### GitHub Actions

```yaml
- name: Validate Commit
  run: |
    engram validation commit --message "${{ github.event.head_commit.message }}"
```

#### GitLab CI

```yaml
validate-commit:
  stage: validate
  script:
    - engram validation commit --message "$CI_COMMIT_MESSAGE"
```

### Troubleshooting

#### Common Issues

1. **Hook not found**
   ```bash
   # Check if engram is in PATH
   which engram
   
   # Install hook manually
   engram validation hook install
   ```

2. **Validation timeout**
   ```yaml
   performance:
     validation_timeout_seconds: 60  # Increase timeout
   ```

3. **False positives**
   ```yaml
   # Add custom exemptions
   exemptions:
     - message_pattern: '^merge:'
       skip_validation: true
   ```

### Best Practices

1. **Consistent Format**: Use one task ID format across your team
2. **Regular Updates**: Keep cache TTL reasonable for your commit frequency
3. **Appropriate Exemptions**: Use exemptions for automated commits, not to bypass quality
4. **Performance Monitoring**: Use cache stats to monitor effectiveness

### Migration Guide

When upgrading validation configuration:

1. **Backup**: Export current configuration
   ```bash
   engram validation config export > validation-backup.yaml
   ```

2. **Update**: Modify configuration file
3. **Validate**: Test new configuration
   ```bash
   engram validation check
   ```

4. **Deploy**: Roll out to team with documentation