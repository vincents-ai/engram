# Agent Memory & Learning System for Engram

**Date**: 2026-01-17
**Priority**: High
**Phase**: 1 - Core LLM Agent Features

## Overview

Implement a comprehensive memory and learning system that enables LLM agents to remember past experiences, learn from successes and failures, and improve decision-making over time.

## Architecture

### Core Components

1. **Agent Memory Store** - Persistent storage for agent experiences
2. **Pattern Recognition Engine** - Identifies recurring patterns and anti-patterns
3. **Decision Learning System** - Learns from decision outcomes
4. **Knowledge Transfer Interface** - Shares learnings across agents
5. **Memory Retrieval System** - Context-aware memory access

### Entity Design

```rust
// src/entities/agent_memory.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMemory {
    pub id: String,
    pub agent_id: String,
    pub memory_type: MemoryType,
    pub context_embedding: Vec<f32>,  // Vector representation of context
    pub content: MemoryContent,
    pub confidence_score: f32,
    pub created_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
    pub access_count: u32,
    pub success_rate: f32,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryType {
    DecisionPattern {
        decision_context: String,
        chosen_action: String,
        outcome_quality: OutcomeQuality,
        alternative_actions: Vec<String>,
    },
    FailurePattern {
        error_signature: String,
        root_cause: String,
        solution_applied: String,
        prevention_strategy: Option<String>,
    },
    SuccessPattern {
        context_signature: String,
        successful_approach: String,
        key_factors: Vec<String>,
        replication_steps: Vec<String>,
    },
    Collaboration {
        partner_agents: Vec<String>,
        interaction_type: CollaborationType,
        outcome: CollaborationOutcome,
        lessons_learned: Vec<String>,
    },
    ContextKnowledge {
        domain: String,
        knowledge_type: KnowledgeType,
        expertise_level: ExpertiseLevel,
        related_entities: Vec<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionTemplate {
    pub pattern_id: String,
    pub context_signature: String,
    pub decision_tree: Vec<DecisionBranch>,
    pub confidence_threshold: f32,
    pub success_history: Vec<DecisionOutcome>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailurePattern {
    pub pattern_id: String,
    pub error_signature: String,
    pub occurrence_count: u32,
    pub last_seen: DateTime<Utc>,
    pub known_solutions: Vec<Solution>,
    pub prevention_checks: Vec<PreventionCheck>,
}
```

### Learning Engine

```rust
// src/learning/mod.rs
pub struct LearningEngine {
    memory_store: Box<dyn MemoryStorage>,
    pattern_recognizer: PatternRecognizer,
    decision_evaluator: DecisionEvaluator,
    knowledge_synthesizer: KnowledgeSynthesizer,
}

impl LearningEngine {
    pub async fn record_decision(&mut self, 
        agent_id: &str,
        context: &DecisionContext,
        action_taken: &str,
        outcome: &ActionOutcome
    ) -> Result<()> {
        // 1. Create memory record
        let memory = AgentMemory {
            agent_id: agent_id.to_string(),
            memory_type: MemoryType::DecisionPattern {
                decision_context: context.to_signature(),
                chosen_action: action_taken.to_string(),
                outcome_quality: self.evaluate_outcome(outcome),
                alternative_actions: context.available_actions.clone(),
            },
            context_embedding: self.generate_embedding(context).await?,
            // ... other fields
        };
        
        // 2. Store memory
        self.memory_store.store_memory(memory).await?;
        
        // 3. Update patterns
        self.update_decision_patterns(agent_id, context, action_taken, outcome).await?;
        
        Ok(())
    }
    
    pub async fn learn_from_failure(&mut self,
        agent_id: &str,
        error: &ExecutionError,
        solution: &Solution
    ) -> Result<()> {
        // 1. Extract failure signature
        let signature = self.extract_error_signature(error);
        
        // 2. Find similar past failures
        let similar_failures = self.find_similar_failures(&signature).await?;
        
        // 3. Update or create failure pattern
        if let Some(pattern) = similar_failures.first() {
            self.update_failure_pattern(pattern.pattern_id.clone(), solution).await?;
        } else {
            self.create_failure_pattern(signature, error, solution).await?;
        }
        
        // 4. Generate prevention strategies
        let prevention_checks = self.generate_prevention_strategies(error, solution).await?;
        self.store_prevention_checks(agent_id, prevention_checks).await?;
        
        Ok(())
    }
    
    pub async fn retrieve_relevant_memories(&self,
        agent_id: &str,
        current_context: &TaskContext
    ) -> Result<Vec<RelevantMemory>> {
        // 1. Generate embedding for current context
        let context_embedding = self.generate_embedding(current_context).await?;
        
        // 2. Vector similarity search
        let similar_memories = self.memory_store
            .find_similar_memories(&context_embedding, 0.8).await?;
        
        // 3. Filter by agent and relevance
        let agent_memories: Vec<_> = similar_memories.into_iter()
            .filter(|m| m.agent_id == agent_id || m.is_shareable())
            .take(10)
            .collect();
        
        // 4. Rank by relevance and recency
        Ok(self.rank_memories_by_relevance(agent_memories, current_context))
    }
}
```

### Memory Integration with Task Execution

```rust
// Enhanced task execution with memory
impl TaskExecutor {
    pub async fn execute_with_memory(&mut self, task: &Task) -> Result<TaskResult> {
        let agent_id = &task.agent;
        
        // 1. Retrieve relevant memories
        let memories = self.learning_engine
            .retrieve_relevant_memories(agent_id, &task.context).await?;
        
        // 2. Apply learned patterns
        let execution_strategy = self.apply_learned_patterns(&memories, task)?;
        
        // 3. Execute task
        let start_time = Instant::now();
        let result = self.execute_task_with_strategy(task, execution_strategy).await;
        let execution_time = start_time.elapsed();
        
        // 4. Record decision and outcome
        let outcome = ActionOutcome::from_result(&result, execution_time);
        self.learning_engine.record_decision(
            agent_id,
            &task.context,
            &execution_strategy.chosen_approach,
            &outcome
        ).await?;
        
        // 5. Learn from failures if any
        if let Err(error) = &result {
            if let Some(solution) = self.attempt_error_recovery(error).await? {
                self.learning_engine.learn_from_failure(agent_id, error, &solution).await?;
            }
        }
        
        result
    }
}
```

## CLI Integration

```bash
# Memory management commands
engram memory list --agent alice                    # List memories for agent
engram memory search "authentication failure"       # Search memories by content
engram memory patterns --type failure              # Show learned patterns
engram memory share --from alice --to bob --pattern auth-pattern

# Learning insights
engram learning stats --agent alice                # Learning statistics
engram learning patterns --success-rate ">80%"     # High-success patterns
engram learning failures --recent 7d               # Recent failure patterns
engram learning suggest --task auth-123            # Memory-based suggestions

# Knowledge transfer
engram knowledge export --agent alice --domain auth # Export domain knowledge
engram knowledge import --file auth-knowledge.json  # Import shared knowledge
engram knowledge merge --from-agent bob --domain db # Merge knowledge from another agent
```

## Implementation Phases

### Phase 1: Basic Memory Storage (3 weeks)
- AgentMemory entity implementation
- Memory storage backend
- Simple pattern recognition
- CLI commands for memory management

```rust
// Basic memory operations
engram memory record --agent alice --type decision --context "auth-failure" --action "retry-with-backoff" --outcome success
engram memory retrieve --agent alice --context "auth-failure"
```

### Phase 2: Pattern Recognition (4 weeks)
- Decision pattern learning
- Failure pattern identification
- Success pattern extraction
- Pattern matching algorithms

### Phase 3: Context-Aware Retrieval (3 weeks)
- Vector embeddings for context similarity
- Relevance ranking algorithms
- Memory consolidation strategies
- Cross-agent knowledge sharing

### Phase 4: Advanced Learning (4 weeks)
- Outcome prediction based on patterns
- Strategy recommendation system
- Automatic prevention check generation
- Learning quality assessment

## File Structure

```
src/
├── entities/
│   ├── agent_memory.rs         # Core memory entities
│   ├── decision_pattern.rs     # Decision learning entities
│   └── failure_pattern.rs      # Failure pattern entities
├── learning/
│   ├── mod.rs                  # Main learning engine
│   ├── pattern_recognizer.rs   # Pattern identification
│   ├── decision_evaluator.rs   # Decision outcome evaluation
│   ├── memory_retrieval.rs     # Context-aware memory access
│   └── knowledge_transfer.rs   # Cross-agent learning
├── storage/
│   ├── memory_storage.rs       # Memory persistence layer
│   └── embedding_store.rs      # Vector similarity storage
└── cli/
    ├── memory.rs               # Memory management commands
    └── learning.rs             # Learning insight commands
```

## Example Learning Scenarios

### 1. Authentication Failure Pattern
```bash
# Agent encounters auth failure, tries multiple solutions
$ engram memory record --agent alice --type decision \
  --context "api-auth-timeout" --action "increase-timeout" --outcome failure

$ engram memory record --agent alice --type decision \
  --context "api-auth-timeout" --action "retry-with-exponential-backoff" --outcome success

# Later, when similar context occurs:
$ engram learning suggest --context "api-auth-timeout"
→ Based on past experience:
  • retry-with-exponential-backoff (80% success rate, 3 attempts)
  • increase-timeout (20% success rate, 1 attempt)
  Recommended: retry-with-exponential-backoff
```

### 2. Cross-Agent Learning
```bash
# Bob encounters database migration issue
$ engram memory search "database migration rollback"
→ Found 2 relevant memories from alice-agent:
  • migration-rollback-strategy (95% success rate)
  • backup-verification-steps (100% success rate)
  
# Apply Alice's learned pattern
$ engram learning apply-pattern migration-rollback-strategy --task db-migration-456
```

### 3. Failure Prevention
```bash
# System identifies recurring failure pattern
$ engram learning patterns --type failure --recurring
→ Identified pattern: "npm-install-timeout"
  • Occurs in: CI environment, large dependency trees
  • Prevention check: Verify npm cache, check network timeout settings
  • Success rate after applying prevention: 95%

# Auto-generate prevention check
$ engram prevention generate --pattern npm-install-timeout
→ Created prevention check: verify-npm-cache-status
  Added to workflow stage: development
```

## Success Metrics

1. **Learning Effectiveness**: 30% improvement in task success rates within 30 days
2. **Memory Utilization**: >50% of decisions informed by relevant memories
3. **Pattern Recognition**: Automatic identification of 90% of recurring issues
4. **Cross-Agent Transfer**: Successful knowledge sharing reducing duplicate learning
5. **Prevention Success**: 80% reduction in recurring failures after pattern identification

## Integration Points

- Extends existing Task and ExecutionResult entities
- Integrates with workflow engine for decision points
- Uses existing relationship system for memory connections
- Compatible with current agent identification system
- Leverages existing storage abstractions

## Security & Privacy

- Agent-specific memory isolation by default
- Explicit consent required for cross-agent memory sharing
- Sensitive context filtering (passwords, tokens, etc.)
- Memory retention policies and automatic cleanup
- Audit trail for memory access and sharing

## Future Enhancements

- Integration with external LLM APIs for enhanced pattern recognition
- Collaborative filtering for cross-agent recommendation systems
- Memory compression and archival strategies
- Real-time learning from ongoing executions
- Integration with code change analysis for proactive suggestions
