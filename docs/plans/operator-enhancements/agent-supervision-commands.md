# Agent Supervision Commands for Engram

**Date**: 2026-01-17
**Priority**: Medium
**Phase**: 2 - Human Operator Tools

## Overview

Implement comprehensive supervision and control commands that allow human operators to monitor, manage, and intervene in LLM agent operations with granular control and real-time visibility.

## Architecture

### Core Components

1. **Agent Monitor** - Real-time agent activity tracking
2. **Control Interface** - Agent state management and intervention
3. **Override System** - Emergency controls and manual overrides
4. **Session Management** - Agent session lifecycle control
5. **Coordination Engine** - Multi-agent orchestration

### Entity Design

```rust
// src/entities/agent_supervision.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSession {
    pub id: String,
    pub agent_id: String,
    pub session_type: SessionType,
    pub status: AgentStatus,
    pub current_task: Option<String>,
    pub started_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub resource_usage: ResourceUsage,
    pub performance_metrics: PerformanceMetrics,
    pub supervisor_notes: Vec<SupervisorNote>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentStatus {
    Active { 
        current_operation: String,
        progress_percentage: f32,
        estimated_completion: Option<DateTime<Utc>>,
    },
    Paused { 
        reason: PauseReason,
        paused_by: String,
        can_resume: bool,
    },
    Blocked { 
        blocking_factor: BlockingFactor,
        resolution_required: bool,
        escalation_level: EscalationLevel,
    },
    Idle {
        last_task_completed: Option<DateTime<Utc>>,
        available_for_assignment: bool,
    },
    Error {
        error_details: String,
        recovery_attempts: u32,
        requires_intervention: bool,
    },
    Terminated {
        reason: TerminationReason,
        terminated_by: String,
        can_restart: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentControl {
    pub control_type: ControlType,
    pub issued_by: String,
    pub issued_at: DateTime<Utc>,
    pub target_agents: Vec<String>,
    pub reason: String,
    pub conditions: Vec<ControlCondition>,
    pub auto_revert: Option<Duration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ControlType {
    Pause { graceful: bool },
    Resume { from_checkpoint: Option<String> },
    Terminate { immediate: bool },
    Restart { preserve_context: bool },
    Reassign { new_agent: String },
    Override { gate_name: String },
    Prioritize { new_priority: TaskPriority },
    Throttle { max_operations_per_minute: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupervisorOverride {
    pub id: String,
    pub override_type: OverrideType,
    pub target_entity: String,
    pub supervisor_id: String,
    pub justification: String,
    pub applied_at: DateTime<Utc>,
    pub expiry: Option<DateTime<Utc>>,
    pub conditions: Vec<OverrideCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OverrideType {
    SkipQualityGate { gate_name: String },
    ForceWorkflowTransition { to_stage: String },
    BypassValidation { validation_type: String },
    EmergencyCommit { bypass_all: bool },
    ResourceLimitIncrease { new_limit: ResourceLimit },
    TimeExtension { additional_minutes: u32 },
}
```

### Supervision Engine

```rust
// src/supervision/mod.rs
pub struct SupervisionEngine {
    agent_monitor: AgentMonitor,
    control_interface: ControlInterface,
    override_manager: OverrideManager,
    notification_system: NotificationSystem,
    session_manager: SessionManager,
}

impl SupervisionEngine {
    pub async fn monitor_agents(&self) -> Result<Vec<AgentSession>> {
        let active_sessions = self.session_manager.get_active_sessions().await?;
        let mut monitored_sessions = Vec::new();
        
        for session in active_sessions {
            let updated_session = self.update_session_status(&session).await?;
            monitored_sessions.push(updated_session);
        }
        
        // Check for alerts and notifications
        self.check_supervision_alerts(&monitored_sessions).await?;
        
        Ok(monitored_sessions)
    }
    
    pub async fn pause_agent(&mut self, 
        agent_id: &str, 
        reason: PauseReason,
        supervisor_id: &str
    ) -> Result<ControlResult> {
        // 1. Validate supervision authority
        self.validate_supervisor_authority(supervisor_id, agent_id).await?;
        
        // 2. Check if agent can be paused safely
        let current_state = self.agent_monitor.get_agent_state(agent_id).await?;
        let pause_safety = self.assess_pause_safety(&current_state).await?;
        
        if !pause_safety.safe_to_pause {
            return Err(EngramError::UnsafePause {
                reason: pause_safety.reason,
                suggested_actions: pause_safety.alternatives,
            });
        }
        
        // 3. Create pause control
        let control = AgentControl {
            control_type: ControlType::Pause { graceful: true },
            issued_by: supervisor_id.to_string(),
            target_agents: vec![agent_id.to_string()],
            reason: format!("{:?}", reason),
            // ... other fields
        };
        
        // 4. Execute pause
        let result = self.control_interface.execute_control(&control).await?;
        
        // 5. Log supervision action
        self.log_supervision_action(&control, &result).await?;
        
        Ok(result)
    }
    
    pub async fn emergency_override(&mut self,
        override_type: OverrideType,
        target_entity: &str,
        supervisor_id: &str,
        justification: &str
    ) -> Result<OverrideResult> {
        // 1. Validate emergency authority
        if !self.validate_emergency_authority(supervisor_id).await? {
            return Err(EngramError::InsufficientAuthority);
        }
        
        // 2. Create override record
        let override_record = SupervisorOverride {
            override_type: override_type.clone(),
            target_entity: target_entity.to_string(),
            supervisor_id: supervisor_id.to_string(),
            justification: justification.to_string(),
            applied_at: Utc::now(),
            expiry: Some(Utc::now() + Duration::hours(24)), // Auto-expire in 24h
            // ... other fields
        };
        
        // 3. Execute override
        let result = self.override_manager.apply_override(&override_record).await?;
        
        // 4. Send emergency notification
        self.notification_system.send_emergency_alert(
            &format!("Override applied: {:?} on {} by {}", 
                override_type, target_entity, supervisor_id)
        ).await?;
        
        Ok(result)
    }
}

impl ControlInterface {
    pub async fn execute_control(&self, control: &AgentControl) -> Result<ControlResult> {
        match &control.control_type {
            ControlType::Pause { graceful } => {
                self.pause_agents(&control.target_agents, *graceful).await
            },
            ControlType::Resume { from_checkpoint } => {
                self.resume_agents(&control.target_agents, from_checkpoint.as_deref()).await
            },
            ControlType::Reassign { new_agent } => {
                self.reassign_tasks(&control.target_agents, new_agent).await
            },
            ControlType::Override { gate_name } => {
                self.skip_quality_gate(&control.target_agents, gate_name).await
            },
            // ... other control types
        }
    }
    
    async fn pause_agents(&self, agent_ids: &[String], graceful: bool) -> Result<ControlResult> {
        let mut results = HashMap::new();
        
        for agent_id in agent_ids {
            let result = if graceful {
                self.graceful_pause(agent_id).await
            } else {
                self.immediate_pause(agent_id).await
            };
            
            results.insert(agent_id.clone(), result);
        }
        
        Ok(ControlResult::Multiple(results))
    }
    
    async fn graceful_pause(&self, agent_id: &str) -> Result<PauseResult> {
        // 1. Send pause signal to agent
        self.send_pause_signal(agent_id).await?;
        
        // 2. Wait for current operation to complete
        let timeout = Duration::from_secs(30);
        let completion = self.wait_for_operation_completion(agent_id, timeout).await;
        
        match completion {
            Ok(()) => {
                // 3. Set agent status to paused
                self.set_agent_status(agent_id, AgentStatus::Paused {
                    reason: PauseReason::SupervisorRequest,
                    paused_by: "supervisor".to_string(),
                    can_resume: true,
                }).await?;
                
                Ok(PauseResult::Success { graceful: true })
            },
            Err(_) => {
                // Fallback to immediate pause
                self.immediate_pause(agent_id).await
            }
        }
    }
}
```

## CLI Commands

```bash
# Agent monitoring
engram agents status                          # Overview of all agents
engram agents status --agent alice           # Specific agent status
engram agents activity --live                # Real-time activity stream
engram agents performance --agent alice      # Performance metrics
engram agents workload --summary             # Workload distribution

# Agent control
engram agent pause alice --reason "maintenance"         # Graceful pause
engram agent pause alice --immediate                    # Immediate pause
engram agent resume alice                               # Resume from pause
engram agent restart alice --preserve-context          # Restart with context
engram agent terminate alice --reason "system_update"   # Terminate agent

# Task management
engram task reassign auth-123 --from alice --to bob     # Reassign task
engram task priority auth-123 --level critical          # Change priority
engram task cancel auth-123 --reason "requirements_changed"  # Cancel task
engram task force-complete auth-123                     # Mark as complete

# Emergency overrides
engram override skip-gate auth-123 --gate cargo-test --reason "urgent_hotfix"
engram override force-transition auth-123 --to development --reason "blocking_issue"
engram override bypass-validation auth-123 --type commit --reason "emergency_fix"
engram override emergency-commit --bypass-all --reason "production_down"

# Session management
engram session list                          # Active agent sessions
engram session terminate <session-id>       # End specific session
engram session archive --older-than 7d      # Archive old sessions
engram session restore <session-id>         # Restore terminated session

# Supervision reporting
engram supervision report --daily            # Daily supervision summary
engram supervision alerts                    # Current alerts requiring attention
engram supervision history --supervisor john # Supervision action history
engram supervision audit --agent alice       # Agent supervision audit trail
```

## Advanced Control Features

### Conditional Controls
```bash
# Pause agent if error rate exceeds threshold
engram agent control alice --action pause \
  --condition "error_rate > 10% in last 1h" \
  --reason "high_error_rate"

# Auto-resume when condition clears
engram agent control alice --action resume \
  --when "error_rate < 5% for 15min" \
  --notify supervisor

# Throttle agent based on system load
engram agent throttle alice --max-ops 30/min \
  --condition "system_cpu > 80%" \
  --auto-adjust
```

### Multi-Agent Coordination
```bash
# Coordinate multiple agents
engram agents coordinate --agents alice,bob,charlie \
  --action "sequential_execution" \
  --task auth-project

# Load balancing
engram agents rebalance --criteria "workload,performance,availability"

# Emergency stop for agent group
engram agents emergency-stop --group backend-team \
  --reason "security_incident"
```

## Implementation Phases

### Phase 1: Basic Monitoring (2 weeks)
- Agent status tracking and reporting
- Simple pause/resume functionality
- Basic session management
- CLI command infrastructure

### Phase 2: Advanced Controls (3 weeks)
- Task reassignment capabilities
- Emergency override system
- Conditional control logic
- Notification system integration

### Phase 3: Multi-Agent Coordination (2 weeks)
- Agent group management
- Load balancing algorithms
- Coordinated control actions
- Performance optimization

### Phase 4: Advanced Analytics (2 weeks)
- Supervision reporting dashboards
- Predictive supervision alerts
- Historical analysis tools
- Audit trail enhancements

## File Structure

```
src/
├── supervision/
│   ├── mod.rs                  # Main supervision engine
│   ├── agent_monitor.rs        # Real-time agent monitoring
│   ├── control_interface.rs    # Agent control operations
│   ├── override_manager.rs     # Emergency override handling
│   ├── session_manager.rs      # Agent session lifecycle
│   └── notification_system.rs  # Alert and notification system
├── entities/
│   ├── agent_session.rs        # Agent session entities
│   ├── agent_control.rs        # Control command entities
│   └── supervisor_override.rs  # Override record entities
└── cli/
    ├── agents.rs               # Agent management commands
    ├── supervision.rs          # Supervision commands
    └── emergency.rs            # Emergency control commands
```

## Security & Authorization

### Role-Based Access Control
```yaml
# .engram/supervision-roles.yaml
roles:
  supervisor:
    permissions:
      - pause_agent
      - resume_agent
      - reassign_task
      - view_all_sessions
    restrictions:
      - no_data_deletion
      - no_system_shutdown
      
  emergency_operator:
    permissions:
      - all_supervisor_permissions
      - emergency_override
      - skip_quality_gates
      - bypass_validation
    restrictions:
      - require_justification
      - audit_all_actions
      - auto_expire_overrides
      
  administrator:
    permissions:
      - all_emergency_permissions
      - terminate_agent
      - modify_supervision_config
      - access_audit_logs
```

## Example Supervision Scenarios

### Scenario 1: High Error Rate Detection
```bash
$ engram agents status --alerts
⚠️  ALERT: alice-agent error rate: 15% (threshold: 10%)
   Last errors:
   • cargo test failed (3 times in 30min)
   • git push rejected (timeout, 2 times)
   
$ engram agent pause alice --reason high_error_rate
✓ alice-agent paused gracefully
   Current task saved: auth-refactor-456
   Can resume: Yes
   
$ engram supervision investigate alice --focus errors
→ Error pattern analysis:
   • Network timeouts correlate with system load spikes
   • Cargo test failures in auth module (flaky test detected)
   • Suggested action: Resume with throttling + retry logic
```

### Scenario 2: Emergency Override
```bash
$ engram override emergency-commit --bypass-all --reason "production_down"
⚠️  EMERGENCY OVERRIDE REQUESTED
   Type: Bypass all validation
   Reason: production_down
   Supervisor: john@company.com
   
   This will skip:
   • Commit validation
   • Quality gates
   • Workflow stage checks
   
   Continue? [y/N]: y
   
✓ Emergency override applied (expires in 24h)
✓ All team leads notified
✓ Audit trail created: override_2026_01_17_001
```

### Scenario 3: Multi-Agent Coordination
```bash
$ engram agents coordinate --agents alice,bob --task payment-system-upgrade
→ Coordinating 2 agents for payment-system-upgrade:
   
   Phase 1: alice (database migration) 
   Phase 2: bob (API updates)
   Estimated total time: 45 minutes
   
   Start coordination? [y/N]: y
   
✓ alice started: database-migration
✓ bob waiting for alice completion
→ Phase 1 completed (12min)
✓ bob started: api-updates  
→ All phases completed (32min total)
```

## Success Metrics

1. **Response Time**: <30 seconds for critical supervision actions
2. **Override Accuracy**: 95% of emergency overrides justified by outcomes
3. **Agent Downtime**: <5% unplanned agent downtime
4. **Coordination Efficiency**: 30% improvement in multi-agent task completion
5. **Incident Resolution**: 80% faster resolution of agent-related issues

## Integration Points

- Uses existing agent identification system
- Integrates with task and workflow management
- Leverages existing storage and entity systems
- Compatible with current CLI patterns
- Extends existing validation and quality gate systems

## Future Enhancements

- AI-powered supervision recommendations
- Predictive intervention based on patterns
- Integration with external monitoring systems
- Mobile supervision interface
- Automated supervision policies
- Cross-repository agent coordination
