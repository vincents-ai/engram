# RFC and Questions Entity Types Design

**Date:** 2026-01-19  
**Status:** Design Complete - Ready for Implementation  
**Purpose:** Add human collaboration capabilities to Engram's agent-centric memory system

## Problem Statement

Engram currently operates as an agent-centric memory system without mechanisms for human oversight and collaboration. This creates three critical gaps:

1. **No human feedback mechanism**: Agents cannot request clarification or approval from humans
2. **No blocking mechanism**: Agents can proceed with questionable decisions without human review  
3. **No governance controls**: No formal approval process for major architectural decisions

Projects using Engram need human oversight to prevent agents from making poor decisions or proceeding without proper guidance.

## Solution Overview

Add two new entity types that create **hard blocking mechanisms** where agents must stop and wait for human input:

- **Question Entity**: Task-level blocking for clarification requests
- **RFC Entity**: Workflow-level blocking for major design decisions

Key principle: **Agents can create blocks but only humans can resolve them** - preventing agents from bypassing oversight.

## Entity Designs

### Question Entity Structure

```rust
pub struct Question {
    pub id: String,
    pub agent: String, 
    pub timestamp: DateTime<Utc>,
    
    // Core question data
    pub question_text: String,
    pub question_type: QuestionType, // Clarification, Decision, Approval
    pub urgency_level: UrgencyLevel, // Low, Medium, High, Critical
    
    // Blocking mechanism
    pub blocking_entity_id: String,
    pub blocking_entity_type: String, // "task", "workflow", etc.
    pub estimated_impact: String, // "Delays feature X by N days"
    
    // Rich context for human decision-making
    pub background_reasoning: String,
    pub context_snippet: String,
    pub related_files: Vec<String>,
    pub what_happens_if_delayed: String,
    pub alternative_approaches_considered: Vec<String>,
    
    // Resolution tracking
    pub status: QuestionStatus, // Pending, Answered, Cancelled
    pub answer: Option<String>,
    pub answered_by: Option<String>,
    pub answered_at: Option<DateTime<Utc>>,
}

pub enum QuestionType {
    Clarification,  // "Which approach should I use?"
    Decision,       // "Should I proceed with X?"
    Approval,       // "Please approve this change"
}

pub enum UrgencyLevel {
    Low,       // Can wait days
    Medium,    // Should be answered within hours
    High,      // Blocks immediate work
    Critical,  // Blocks entire workflow
}

pub enum QuestionStatus {
    Pending,   // Waiting for human answer
    Answered,  // Human provided answer
    Cancelled, // No longer needed
}
```

### RFC Entity Structure

```rust
pub struct RFC {
    pub id: String,
    pub agent: String,
    pub timestamp: DateTime<Utc>,
    
    // Standard RFC fields
    pub title: String,
    pub summary: String,
    pub motivation: String,
    pub detailed_design: String,
    pub alternatives: Vec<Alternative>,
    pub unresolved_questions: Vec<String>,
    
    // Engram integration
    pub affected_workflows: Vec<String>, // Workflow IDs that will be blocked
    pub blocked_tasks: Vec<String>,      // Task IDs currently blocked
    pub implementation_timeline: String,
    pub rollback_plan: String,
    
    // Collaborative features  
    pub review_status: RFCStatus, // Draft, UnderReview, Approved, Rejected, Implemented
    pub reviewer_feedback: Vec<ReviewerFeedback>,
    pub approval_conditions: Vec<String>, // Conditions that must be met
    pub implementation_constraints: Vec<String>, // Constraints for implementation
    
    // Resolution tracking
    pub approved_by: Option<String>,
    pub approved_at: Option<DateTime<Utc>>,
    pub rejection_reason: Option<String>,
}

pub struct Alternative {
    pub name: String,
    pub description: String,
    pub pros: Vec<String>,
    pub cons: Vec<String>,
    pub effort_estimate: String,
}

pub struct ReviewerFeedback {
    pub reviewer: String,
    pub timestamp: DateTime<Utc>,
    pub feedback_type: FeedbackType, // Question, Concern, Suggestion, Approval
    pub content: String,
    pub blocking: bool, // Does this feedback block approval?
}

pub enum RFCStatus {
    Draft,         // Being written
    UnderReview,   // Waiting for human review
    Approved,      // Approved for implementation
    Rejected,      // Rejected with reason
    Implemented,   // Implementation complete
}

pub enum FeedbackType {
    Question,   // Clarification needed
    Concern,    // Issue that needs addressing
    Suggestion, // Improvement idea
    Approval,   // Reviewer approves
}
```

## Blocking and Resolution System

### Hard Blocking Behavior

When an agent creates a Question or RFC:

**Question Creation:**
- Target entity (Task/Workflow) status → `blocked_pending_human_input`
- Agent cannot progress that specific entity
- Related dependent tasks also blocked via relationship traversal

**RFC Creation:**
- All `affected_workflows[]` → `blocked_pending_approval`
- All `blocked_tasks[]` → `blocked_pending_approval` 
- Dependent entities discovered through relationship system also blocked

### Automatic Unblocking

Upon human resolution:

**Question Answered** (`status: Answered`):
- `blocking_entity_id` status → previous state (e.g., `inprogress`)
- Relationship engine unblocks dependent entities
- Agent discovers unblocked state on next entity check

**RFC Approved** (`review_status: Approved`):
- All `affected_workflows[]` and `blocked_tasks[]` → previous states
- Implementation constraints stored for agent reference
- Cascade unblocking through relationship dependencies
- Agents discover approval on next workflow/task status check

## CLI Command Distribution

### Engram CLI (Agent Operations)

Agents can create blocking entities and read status but cannot resolve them:

```bash
# Question management
engram question create --text "Should we use JWT or sessions?" \
  --blocking-task-id <task-id> --urgency high \
  --context "Building authentication system" \
  --impact "Delays API implementation by 2 days"

engram question show <question-id>
engram question list --status pending --agent alice

# RFC management  
engram rfc create --title "Authentication Architecture" \
  --file path/to/rfc.md --blocking-workflow-id <workflow-id>

engram rfc create --title "Database Migration Strategy" \
  --summary "Migrate from SQLite to PostgreSQL" \
  --motivation "Performance and scalability requirements" \
  --design "Three-phase migration with zero downtime" \
  --blocking-workflow-id <workflow-id>

engram rfc update <rfc-id> --file sections/design.md --section detailed_design
engram rfc show <rfc-id>
engram rfc list --status under-review
```

### Locus CLI (Human Operations)

Humans resolve blocking entities through governance interface:

```bash
# Question resolution
locus question answer <question-id> --answer "Use JWT for stateless API"
locus question list --urgent  # High/Critical priority only

# RFC resolution
locus rfc approve <rfc-id> --conditions "Must include rate limiting"
locus rfc reject <rfc-id> --reason "Security concerns with proposed approach"
locus rfc review <rfc-id>  # Structured review interface

# Dashboard views
locus dashboard --view impact   # "3 workflows blocked" vs "5 tasks blocked"
locus dashboard --view agents   # "Agent Alice waiting on 2 items"
```

### RFC Content Import

RFCs support rich content through multiple mechanisms:

```bash
# Import existing RFC document with markdown parsing
engram rfc create --title "API Design" --file rfc-001.md --blocking-workflow-id <id>

# Link existing context entities for background
engram rfc create --title "Security Model" --context-id <context-id> --reasoning-id <reasoning-id>

# Build content iteratively
engram rfc create --title "Database Architecture" --blocking-workflow-id <id>
engram rfc update <rfc-id> --file motivation.md --section motivation
engram rfc update <rfc-id> --file design.md --section detailed_design
```

Markdown parsing extracts standard sections: `## Motivation`, `## Design`, `## Alternatives`.

## Integration with Existing Systems

### Relationship System Integration

Questions and RFCs automatically create relationships:

```bash
# Question creation creates blocking relationship
engram question create --blocking-task-id <task-id>
# Result: Question --blocks--> Task relationship

# RFC creation with multiple relationships
engram rfc create --blocking-workflow-id <workflow-id> --context-id <context-id>
# Result: RFC --blocks--> Workflow relationship
# Result: RFC --references--> Context relationship

# Cascade blocking through dependency chains
engram relationship connected --entity-id <blocked-task> --relationship-type depends_on
# All dependent tasks automatically receive blocked status
```

### Storage and Validation

- Questions/RFCs stored in same `.engram/.git` memory repository
- Commit validation enforces proper relationships (Questions must reference blocking entities) 
- Workflow engine respects blocking states during progression
- Entity registry automatically includes new types in dynamic operations

### Workflow Engine Integration

Workflow progression must check for blocking states:

```rust
// Before progressing any workflow step
if entity.has_pending_questions() || entity.has_pending_rfcs() {
    return Err("Entity blocked pending human input");
}

// Before allowing task status changes
if task.status == "blocked_pending_human_input" {
    return Err("Task blocked by pending Question");
}
```

## Implementation Plan

### Phase 1: Core Entity Implementation
1. Add `src/entities/question.rs` with full Entity trait implementation
2. Add `src/entities/rfc.rs` with full Entity trait implementation  
3. Update `src/entities/mod.rs` to include new types
4. Register new entities in EntityRegistry

### Phase 2: CLI Command Implementation
5. Extend `src/cli/` with question/rfc subcommands for engram binary
6. Extend `src/locus_cli/` with answer/approve/review commands for locus binary
7. Add markdown parsing support for RFC content import
8. Implement structured review interface for Locus

### Phase 3: Blocking System Integration
9. Update relationship system to handle blocking/unblocking cascade logic
10. Add blocking status checks to workflow engine and task progression  
11. Implement automatic unblocking when Questions answered / RFCs approved
12. Add commit validation rules for Questions/RFCs

### Phase 4: Dashboard and Views
13. Implement Locus dashboard with impact-based and agent-based views
14. Add filtering and prioritization for human review queues
15. Create structured RFC review interface
16. Add urgency-based Question sorting

## Success Criteria

### Functional Requirements
- ✅ Agents can create Questions that hard-block specific entities
- ✅ Agents can create RFCs with rich content that block workflows
- ✅ Only humans through Locus can resolve blocking entities  
- ✅ Automatic unblocking occurs when humans provide answers/approval
- ✅ Relationship system handles cascade blocking/unblocking
- ✅ Dashboard prioritizes by impact and agent productivity

### Governance Requirements  
- ✅ Agents cannot self-approve or bypass blocking mechanisms
- ✅ All blocking entities require human resolution
- ✅ Clear separation: engram creates, locus resolves
- ✅ Rich context provided for informed human decision-making
- ✅ Audit trail maintained through git storage

### Integration Requirements
- ✅ Seamless integration with existing Entity trait system
- ✅ Compatible with git-based storage architecture  
- ✅ Relationship system handles new entity types
- ✅ Workflow engine respects blocking states
- ✅ CLI commands follow established patterns

## Benefits

### For Human Operators
- Complete oversight of agent decisions through hard blocking
- Rich context for rapid decision-making without codebase diving
- Prioritized dashboard showing highest-impact blocks first
- Formal governance process for major architectural changes

### For Agents  
- Clear mechanism to request human guidance when uncertain
- Structured way to propose major changes requiring approval
- Automatic resumption of work when human input provided
- Rich relationship system for understanding dependencies

### For Projects
- Prevents agents from making poor decisions autonomously  
- Maintains human control while keeping agents productive
- Formal approval process for significant changes
- Complete audit trail of human decisions and reasoning

This design provides comprehensive human-agent collaboration while maintaining Engram's core principles of git-based storage, relationship modeling, and extensible entity architecture.