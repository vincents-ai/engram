# Engram Workflow Templates

This directory contains predefined workflow templates for common development patterns.

## Available Workflows

### feature-development.yaml
Complete BDD (Behavior-Driven Development) workflow for new features.

**Stages:**
1. **requirements** - Requirements gathering and brainstorming (engram entities only)
2. **planning** - Technical planning and design (engram entities only) 
3. **research** - Research and proof of concepts (docs/examples allowed)
4. **bdd** - Write failing tests (RED phase - tests only)
5. **development** - Implement to make tests pass (GREEN phase - code allowed)
6. **integration** - Full system validation (all quality gates)

**Quality Gates:**
- BDD stage enforces test failures (proves tests are real)
- Development stage requires test success (GREEN phase)
- Integration runs full build and test suite

### bug-fix.yaml
Streamlined workflow for bug fixes and hotfixes.

**Stages:**
1. **investigation** - Investigate and document the bug (engram entities only)
2. **reproduction** - Create tests that reproduce the bug (tests only)
3. **fix** - Implement the bug fix (code with tests)
4. **verification** - Verify fix and run full test suite (full validation)

**Quality Gates:**
- Reproduction stage enforces test failures (proves bug is reproduced)
- Fix stage requires test success and clean code
- Verification runs full build and test suite

### research.yaml
Workflow for research tasks and technical exploration.

**Stages:**
1. **scope** - Define research scope and questions (engram entities only)
2. **exploration** - Research and exploration phase (research artifacts allowed)
3. **analysis** - Analyze findings and draw conclusions (research artifacts allowed)
4. **documentation** - Document findings and recommendations (research artifacts allowed)

**Quality Gates:**
- Each stage validates proper documentation and progress
- Focus on research artifacts and documentation quality

## Usage

```bash
# Create workflow from template
engram workflow create --file workflows/feature-development.yaml
engram workflow create --file workflows/bug-fix.yaml
engram workflow create --file workflows/research.yaml

# Assign to task  
engram workflow assign --task-id [uuid] --workflow "Feature Development"
engram workflow assign --task-id [uuid] --workflow "Bug Fix"
engram workflow assign --task-id [uuid] --workflow "Research and Exploration"

# Advance through stages
engram task advance [uuid]

# Validate current stage
engram workflow validate [uuid]
```

## Custom Workflows

You can create custom workflows by copying and modifying these templates:

1. Copy existing template
2. Modify stages, commit policies, and quality gates
3. Create with `engram workflow create --file your-workflow.yaml`

See the design document for complete workflow DSL specification.