# Natural Language Query Interface for Engram

**Date**: 2026-01-17
**Priority**: High
**Phase**: 1 - Core LLM Agent Features

## Overview

Implement a natural language interface that allows LLM agents and human operators to query Engram using conversational language instead of complex CLI syntax.

## Architecture

### Core Components

1. **Query Parser** - Translates natural language to structured queries
2. **Intent Recognition** - Identifies query types and required entities
3. **Query Executor** - Maps intents to existing Engram operations
4. **Response Formatter** - Converts structured results to natural language

### Implementation Strategy

```rust
// src/nlq/mod.rs - Natural Language Query module
pub struct NLQEngine {
    intent_classifier: IntentClassifier,
    entity_extractor: EntityExtractor,
    query_mapper: QueryMapper,
    response_formatter: ResponseFormatter,
}

#[derive(Debug, Clone)]
pub enum QueryIntent {
    TaskStatus {
        filters: Vec<TaskFilter>,
        scope: QueryScope,
    },
    WorkflowInfo {
        workflow_id: Option<String>,
        stage_filter: Option<String>,
    },
    FailureAnalysis {
        timeframe: TimeFrame,
        component: Option<String>,
    },
    BlockedItems {
        reason_filter: Option<String>,
        priority: Option<TaskPriority>,
    },
    RelationshipQuery {
        entity_id: String,
        relationship_type: Option<String>,
        depth: u32,
    },
}

#[derive(Debug, Clone)]
pub struct QueryScope {
    agents: Vec<String>,
    timeframe: Option<TimeFrame>,
    task_types: Vec<String>,
}
```

### Query Pattern Recognition

```yaml
# .engram/nlq-patterns.yaml
intent_patterns:
  task_status:
    patterns:
      - "What tasks are {status}?"
      - "Show me {agent}'s work"
      - "What is {agent} working on?"
    entities:
      - status: ["completed", "in progress", "blocked", "todo"]
      - agent: ["agent_name_pattern"]

  workflow_info:
    patterns:
      - "What's the current workflow stage for {task_name}?"
      - "Where are we in the {workflow_name} process?"
      - "Show workflow status for {entity}"

  failure_analysis:
    patterns:
      - "What failed in the last {timeframe}?"
      - "Show me test failures from {timeframe}"
      - "Why did {task} fail?"
    
  blocked_items:
    patterns:
      - "What tasks are blocked?"
      - "What's preventing progress on {task}?"
      - "Show blocked {priority} tasks"
```

### CLI Integration

```bash
# New command
engram ask "What tasks are blocked and why?"
engram ask "Show me all failed tests from the last 3 commits"  
engram ask "What's the current workflow stage for authentication work?"
engram ask "Which agent worked on database migrations?"

# With context
engram ask --context task:auth-123 "What are the dependencies?"
engram ask --agent alice "What should I work on next?"
engram ask --verbose "Explain the workflow progression for BDD tasks"
```

### Query Execution Engine

```rust
impl NLQEngine {
    pub async fn process_query(&self, query: &str, context: QueryContext) -> Result<QueryResponse> {
        // 1. Parse natural language
        let intent = self.intent_classifier.classify(query)?;
        
        // 2. Extract entities and parameters
        let entities = self.entity_extractor.extract(query, &intent)?;
        
        // 3. Map to structured query
        let structured_query = self.query_mapper.map_to_query(&intent, &entities)?;
        
        // 4. Execute against Engram storage
        let results = self.execute_structured_query(structured_query).await?;
        
        // 5. Format natural language response
        let response = self.response_formatter.format_response(results, &intent)?;
        
        Ok(response)
    }
    
    async fn execute_structured_query(&self, query: StructuredQuery) -> Result<QueryResults> {
        match query.query_type {
            QueryType::TaskList(filters) => {
                self.storage.query_tasks(filters).await
            },
            QueryType::RelationshipTraversal { entity_id, path } => {
                self.storage.traverse_relationships(entity_id, path).await
            },
            QueryType::ExecutionHistory { filters } => {
                self.storage.query_execution_results(filters).await
            },
            // ... other query types
        }
    }
}
```

## Implementation Phases

### Phase 1: Basic Intent Recognition (2 weeks)
- Simple pattern matching for common queries
- Integration with existing CLI commands
- Basic response formatting

```bash
engram ask "show my tasks"          # -> engram task list --agent $USER
engram ask "what failed recently"   # -> engram execution-results --status failed --since 1day
```

### Phase 2: Entity Extraction (2 weeks)  
- Named entity recognition for:
  - Agent names
  - Task titles/IDs
  - Time expressions
  - Status values
  - Workflow names

### Phase 3: Complex Query Support (3 weeks)
- Multi-entity queries
- Conditional logic
- Aggregation operations
- Cross-entity relationships

### Phase 4: Learning & Adaptation (2 weeks)
- Query refinement based on user feedback
- Custom pattern learning
- Domain-specific vocabulary expansion

## File Structure

```
src/
├── nlq/
│   ├── mod.rs              # Main NLQ engine
│   ├── intent_classifier.rs # Intent recognition
│   ├── entity_extractor.rs  # Named entity extraction  
│   ├── query_mapper.rs      # Intent -> structured query
│   ├── response_formatter.rs # Results -> natural language
│   └── patterns/
│       ├── task_patterns.rs
│       ├── workflow_patterns.rs
│       └── relationship_patterns.rs
├── cli/
│   └── ask.rs              # `engram ask` command
└── entities/
    └── nlq_session.rs      # Query session tracking
```

## Example Interactions

```bash
# Basic task queries
$ engram ask "What am I working on?"
→ You have 3 active tasks:
  • auth-system-refactor (In Progress, started 2h ago)
  • database-migration-fix (Blocked, waiting for review)
  • api-documentation (Todo, priority: High)

# Workflow status
$ engram ask "How's the authentication workflow progressing?"
→ Authentication workflow (auth-123) is in Development stage:
  ✓ Requirements (completed 2 days ago)
  ✓ Planning (completed 1 day ago) 
  ✓ BDD Red (tests failing, as expected)
  → Development (current stage, 3 quality gates pending)
  ○ Integration (not started)

# Failure analysis  
$ engram ask "Why did the build fail?"
→ Last build failure (commit abc123, 30min ago):
  • cargo test failed in auth module
  • 3 tests failing: test_login_validation, test_token_refresh, test_logout
  • Error: "Database connection timeout"
  • Suggested fix: Check database service status

# Cross-agent collaboration
$ engram ask "Who else worked on the payment system?"
→ Payment system contributors (last 30 days):
  • alice-agent: 15 tasks (database schema, API endpoints)
  • bob-agent: 8 tasks (frontend integration, testing)
  • security-agent: 3 tasks (audit, compliance checks)
  
  Most recent activity: alice-agent working on payment-webhooks (2h ago)
```

## Dependencies

- Existing Engram entity storage system
- Relationship traversal capabilities
- CLI command infrastructure
- Basic text processing libraries (regex, string matching)

## Integration Points

- Uses existing Storage trait for data access
- Leverages current CLI command patterns
- Integrates with relationship system for entity connections
- Compatible with current authentication/agent system

## Success Metrics

1. **Query Coverage**: 80% of common developer questions answerable
2. **Response Accuracy**: >90% for simple queries, >70% for complex queries
3. **Response Time**: <2 seconds for most queries
4. **User Adoption**: Measurable increase in Engram query frequency
5. **Error Handling**: Graceful degradation with helpful suggestions

## Future Enhancements

- Voice interface integration
- Multi-turn conversations with context retention
- Query suggestion and auto-completion
- Integration with external LLM APIs for complex reasoning
- Custom domain vocabulary learning
