use crate::entities::{
    AgentSandbox, Compliance, Context, EntityRelationship, EscalationRequest, ExecutionResult,
    Knowledge, ProgressiveGateConfig, Reasoning, Rule, Session, Standard, StateReflection, Task,
    TaskStatus, Theory, Workflow, WorkflowInstance, ADR,
};
use std::collections::HashMap;
use std::time::Instant;

/// Which pane has focus in the Relationships view.
#[derive(Debug, Clone, PartialEq)]
pub enum RelationshipFocus {
    Nodes,
    Edges,
}

/// A directed edge in the relationships graph.
#[derive(Debug, Clone)]
pub struct RelationshipEdge {
    pub from_id: String,
    pub to_id: String,
    pub to_type: String,
    pub relationship_type: String, // e.g. "depends_on", "relates_to", "explains"
    pub to_title: String,
    pub agent: String,
}

/// A node in the relationships graph (adjacency-list representation).
#[derive(Debug, Clone)]
pub struct RelationshipNode {
    pub id: String,
    pub title: String,
    pub entity_type: String, // "task", "context", "reasoning", "adr"
    pub edges: Vec<RelationshipEdge>,
}

/// A single node in the reasoning tree.
#[derive(Debug, Clone)]
pub struct ReasoningNode {
    pub id: String,
    pub title: String,
    pub content_preview: String, // first 80 chars of content
    pub task_id: Option<String>,
    pub depth: usize, // for indentation (0 = root)
    pub expanded: bool,
}

impl ReasoningNode {
    /// Return the indentation prefix string for this node's depth.
    pub fn indent_prefix(&self) -> String {
        " ".repeat(self.depth * 2)
    }

    /// Return the expand/collapse indicator glyph.
    pub fn expand_glyph(&self) -> &'static str {
        if self.expanded {
            "▼"
        } else {
            "▶"
        }
    }
}

/// Summary counts for tasks across all statuses.
#[derive(Debug, Clone, Default)]
pub struct TaskSummary {
    pub total: usize,
    pub todo: usize,
    pub in_progress: usize,
    pub done: usize,
}

/// A single row for the recent-tasks table.
#[derive(Debug, Clone)]
pub struct TaskRow {
    pub id: String,
    pub title: String,
    pub status: String,
    pub priority: String,
    pub created: String,
}

/// Detail view for a single task (shown as modal overlay).
#[derive(Debug, Clone)]
pub struct TaskDetail {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: String,
    pub priority: String,
    pub agent: String,
    pub created: String,
    pub tags: Vec<String>,
    pub outcome: Option<String>,
}

/// A single row in search results.
#[derive(Debug, Clone)]
pub struct SearchResultRow {
    pub entity_type: String,
    pub title: String,
    pub preview: String,
}

/// The active view currently displayed in the TUI.
#[derive(Debug, Clone, PartialEq)]
pub enum ActiveView {
    Dashboard,
    Tasks,
    Reasoning,
    Relationships,
    Contexts,
    Adrs,
    Theories,
    Search,
    Workflows,
    WorkflowInstances,
    Knowledge,
    Sessions,
    Compliance,
    Rules,
    Standards,
    StateReflections,
    Escalations,
    Sandboxes,
    ExecutionResults,
    ProgressiveConfigs,
}

impl ActiveView {
    /// All variants in cycle order.
    fn all() -> &'static [ActiveView] {
        use ActiveView::*;
        &[
            Dashboard,
            Tasks,
            Reasoning,
            Relationships,
            Contexts,
            Adrs,
            Theories,
            Workflows,
            WorkflowInstances,
            Knowledge,
            Sessions,
            Compliance,
            Rules,
            Standards,
            StateReflections,
            Escalations,
            Sandboxes,
            ExecutionResults,
            ProgressiveConfigs,
            Search,
        ]
    }

    fn index(&self) -> usize {
        Self::all().iter().position(|v| v == self).unwrap_or(0)
    }
}

/// Top-level application state for the Locus TUI.
pub struct AppState {
    pub active_view: ActiveView,
    pub should_quit: bool,
    pub status_message: Option<String>,
    /// Selected row index for list views.
    pub selected_index: usize,
    /// Search query string (used in the Search view).
    pub search_query: String,
    /// Active colour theme.
    pub theme: crate::locus_tui::theme::AppTheme,
    /// Summary counts shown on the Dashboard.
    pub task_summary: TaskSummary,
    /// Recent tasks shown in the Dashboard table (up to 10).
    pub recent_tasks: Vec<TaskRow>,
    /// Full Task entities for detail view.
    pub all_tasks: Vec<Task>,
    /// Contexts loaded from backend.
    pub contexts: Vec<Context>,
    /// Status filter: None = show all, Some("todo") etc.
    pub filter_status: Option<String>,
    /// Substring filter on task title (case-insensitive).
    pub filter_text: String,
    /// Reasoning tree nodes.
    pub reasoning_nodes: Vec<ReasoningNode>,
    /// Full Reasoning entities.
    pub all_reasoning: Vec<Reasoning>,
    /// Selected index within the reasoning view list.
    pub reasoning_selected: usize,
    /// Nodes in the relationships graph (adjacency list).
    pub relationship_nodes: Vec<RelationshipNode>,
    /// Selected index within the relationships node list.
    pub relationship_selected: usize,
    /// Which pane is focused in the relationships view (Nodes or Edges).
    pub relationship_focus: RelationshipFocus,
    /// Selected edge index within the currently selected node's edge list.
    pub relationship_edge_selected: usize,
    /// Selected index within the contexts view list.
    pub contexts_selected: usize,
    /// All ADR entities loaded from backend.
    pub all_adrs: Vec<ADR>,
    /// Selected index within the ADRs view list.
    pub adrs_selected: usize,
    /// All Theory entities loaded from backend.
    pub all_theories: Vec<Theory>,
    /// Selected index within the Theories view list.
    pub theories_selected: usize,
    /// Task detail overlay (None = not shown).
    pub task_detail: Option<TaskDetail>,
    /// Whether the app is in search input mode.
    pub search_mode: bool,
    /// Search results.
    pub search_results: Vec<SearchResultRow>,
    /// Selected index within search results list.
    pub search_result_selected: usize,
    /// Auto-refresh interval in seconds (0 = disabled).
    /// Driven by `WorkspaceConfig::refresh_interval_secs`.
    pub refresh_interval_secs: u64,
    /// Instant of the last data refresh (used for tick-based auto-refresh).
    pub last_refresh: Instant,
    /// Whether the global help overlay is shown.
    pub show_help: bool,
    /// Cached workflow count (populated by load_all_data, not re-fetched every frame).
    pub workflow_count: usize,
    // ── New entity collections ────────────────────────────────────────────
    pub all_workflows: Vec<Workflow>,
    pub workflows_selected: usize,
    pub all_workflow_instances: Vec<WorkflowInstance>,
    pub workflow_instances_selected: usize,
    pub all_knowledge: Vec<Knowledge>,
    pub knowledge_selected: usize,
    pub all_sessions: Vec<Session>,
    pub sessions_selected: usize,
    pub all_compliance: Vec<Compliance>,
    pub compliance_selected: usize,
    pub all_rules: Vec<Rule>,
    pub rules_selected: usize,
    pub all_standards: Vec<Standard>,
    pub standards_selected: usize,
    pub all_state_reflections: Vec<StateReflection>,
    pub state_reflections_selected: usize,
    pub all_escalations: Vec<EscalationRequest>,
    pub escalations_selected: usize,
    pub all_sandboxes: Vec<AgentSandbox>,
    pub sandboxes_selected: usize,
    pub all_execution_results: Vec<ExecutionResult>,
    pub execution_results_selected: usize,
    pub all_progressive_configs: Vec<ProgressiveGateConfig>,
    pub progressive_configs_selected: usize,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            active_view: ActiveView::Dashboard,
            should_quit: false,
            status_message: None,
            selected_index: 0,
            search_query: String::new(),
            theme: crate::locus_tui::theme::AppTheme::dark(),
            task_summary: TaskSummary::default(),
            recent_tasks: Vec::new(),
            all_tasks: Vec::new(),
            contexts: Vec::new(),
            filter_status: None,
            filter_text: String::new(),
            reasoning_nodes: Vec::new(),
            all_reasoning: Vec::new(),
            reasoning_selected: 0,
            relationship_nodes: Vec::new(),
            relationship_selected: 0,
            relationship_focus: RelationshipFocus::Nodes,
            relationship_edge_selected: 0,
            contexts_selected: 0,
            all_adrs: Vec::new(),
            adrs_selected: 0,
            all_theories: Vec::new(),
            theories_selected: 0,
            task_detail: None,
            search_mode: false,
            search_results: Vec::new(),
            search_result_selected: 0,
            refresh_interval_secs: 30,
            last_refresh: Instant::now(),
            show_help: false,
            workflow_count: 0,
            all_workflows: Vec::new(),
            workflows_selected: 0,
            all_workflow_instances: Vec::new(),
            workflow_instances_selected: 0,
            all_knowledge: Vec::new(),
            knowledge_selected: 0,
            all_sessions: Vec::new(),
            sessions_selected: 0,
            all_compliance: Vec::new(),
            compliance_selected: 0,
            all_rules: Vec::new(),
            rules_selected: 0,
            all_standards: Vec::new(),
            standards_selected: 0,
            all_state_reflections: Vec::new(),
            state_reflections_selected: 0,
            all_escalations: Vec::new(),
            escalations_selected: 0,
            all_sandboxes: Vec::new(),
            sandboxes_selected: 0,
            all_execution_results: Vec::new(),
            execution_results_selected: 0,
            all_progressive_configs: Vec::new(),
            progressive_configs_selected: 0,
        }
    }

    /// Cycle forward through all ActiveView variants.
    pub fn next_view(&mut self) {
        let variants = ActiveView::all();
        let next = (self.active_view.index() + 1) % variants.len();
        self.active_view = variants[next].clone();
    }

    /// Cycle backward through all ActiveView variants.
    pub fn prev_view(&mut self) {
        let variants = ActiveView::all();
        let current = self.active_view.index();
        let prev = if current == 0 {
            variants.len() - 1
        } else {
            current - 1
        };
        self.active_view = variants[prev].clone();
    }

    /// Increment the selected row index.
    pub fn select_next(&mut self) {
        self.selected_index = self.selected_index.saturating_add(1);
    }

    /// Decrement the selected row index, saturating at 0.
    pub fn select_prev(&mut self) {
        self.selected_index = self.selected_index.saturating_sub(1);
    }

    /// Move selection to the last row in a list of known length.
    pub fn select_bottom_of(&mut self, len: usize) {
        if len > 0 {
            self.selected_index = len - 1;
        }
    }

    /// Set the status filter (None = show all).
    pub fn set_filter_status(&mut self, status: Option<String>) {
        self.filter_status = status;
    }

    /// Set the text substring filter.
    pub fn set_filter_text(&mut self, text: String) {
        self.filter_text = text;
    }

    /// Return tasks that match both the active status filter and text filter.
    pub fn filtered_tasks(&self) -> Vec<&TaskRow> {
        self.recent_tasks
            .iter()
            .filter(|t| {
                let status_ok = self
                    .filter_status
                    .as_ref()
                    .map(|s| t.status.to_lowercase() == s.to_lowercase())
                    .unwrap_or(true);
                let text_ok = self.filter_text.is_empty()
                    || t.title
                        .to_lowercase()
                        .contains(&self.filter_text.to_lowercase());
                status_ok && text_ok
            })
            .collect()
    }

    /// Signal that the application should quit.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// Return `true` if auto-refresh is enabled and the interval has elapsed.
    /// Resets `last_refresh` to now when it returns `true`.
    pub fn should_auto_refresh(&mut self) -> bool {
        if self.refresh_interval_secs == 0 {
            return false;
        }
        let elapsed = self.last_refresh.elapsed().as_secs();
        if elapsed >= self.refresh_interval_secs {
            self.last_refresh = Instant::now();
            true
        } else {
            false
        }
    }

    /// Reset the auto-refresh timer (called after a manual or automatic refresh).
    pub fn reset_refresh_timer(&mut self) {
        self.last_refresh = Instant::now();
    }

    /// Set a status bar message.
    pub fn set_status(&mut self, msg: impl Into<String>) {
        self.status_message = Some(msg.into());
    }

    /// Clear the status bar message.
    pub fn clear_status(&mut self) {
        self.status_message = None;
    }

    /// Toggle between dark and light themes.
    pub fn toggle_theme(&mut self) {
        self.theme = self.theme.toggle();
    }

    /// Toggle the expanded state of the currently selected reasoning node.
    pub fn toggle_reasoning_node(&mut self) {
        if let Some(node) = self.reasoning_nodes.get_mut(self.reasoning_selected) {
            node.expanded = !node.expanded;
        }
    }

    /// Return all reasoning nodes as a flat list (depth drives visual indent).
    pub fn visible_reasoning_nodes(&self) -> Vec<&ReasoningNode> {
        self.reasoning_nodes.iter().collect()
    }

    /// Return the currently selected relationship node, if any.
    pub fn selected_relationship_node(&self) -> Option<&RelationshipNode> {
        self.relationship_nodes.get(self.relationship_selected)
    }

    /// Build a TaskDetail from the currently selected TaskRow + full Task entity.
    pub fn open_task_detail(&mut self) {
        let idx = self.selected_index;
        if let Some(row) = self.recent_tasks.get(idx) {
            // Try to find the full entity by matching on id prefix or full id
            let full = self.all_tasks.iter().find(|t| {
                t.id == row.id
                    || t.id.starts_with(&row.id)
                    || row.id.starts_with(&t.id[..8.min(t.id.len())])
            });
            let detail = TaskDetail {
                id: row.id.clone(),
                title: row.title.clone(),
                description: full.map(|t| t.description.clone()).unwrap_or_default(),
                status: row.status.clone(),
                priority: row.priority.clone(),
                agent: full.map(|t| t.agent.clone()).unwrap_or_default(),
                created: row.created.clone(),
                tags: full.map(|t| t.tags.clone()).unwrap_or_default(),
                outcome: full.and_then(|t| t.outcome.clone()),
            };
            self.task_detail = Some(detail);
        }
    }

    /// Close the task detail overlay.
    pub fn close_task_detail(&mut self) {
        self.task_detail = None;
    }

    /// Cycle the status of the currently selected ADR: Proposed → Accepted → Deprecated → Superseded → Proposed.
    pub fn cycle_selected_adr_status(&mut self) -> Option<(String, crate::entities::AdrStatus)> {
        use crate::entities::AdrStatus;
        if let Some(adr) = self.all_adrs.get_mut(self.adrs_selected) {
            adr.status = match adr.status {
                AdrStatus::Proposed => AdrStatus::Accepted,
                AdrStatus::Accepted => AdrStatus::Deprecated,
                AdrStatus::Deprecated => AdrStatus::Superseded,
                AdrStatus::Superseded => AdrStatus::Proposed,
            };
            Some((adr.id.clone(), adr.status.clone()))
        } else {
            None
        }
    }

    /// Run an in-memory search across all loaded entities.
    pub fn run_search(&mut self) {
        let query = self.search_query.to_lowercase();
        if query.is_empty() {
            self.search_results.clear();
            return;
        }

        let mut results: Vec<SearchResultRow> = Vec::new();

        // Search tasks
        for task in &self.all_tasks {
            if task.title.to_lowercase().contains(&query)
                || task.description.to_lowercase().contains(&query)
            {
                results.push(SearchResultRow {
                    entity_type: "task".to_string(),
                    title: task.title.clone(),
                    preview: task.description.chars().take(60).collect(),
                });
            }
        }

        // Search contexts
        for ctx in &self.contexts {
            if ctx.title.to_lowercase().contains(&query)
                || ctx.content.to_lowercase().contains(&query)
            {
                results.push(SearchResultRow {
                    entity_type: "context".to_string(),
                    title: ctx.title.clone(),
                    preview: ctx.content.chars().take(60).collect(),
                });
            }
        }

        // Search reasoning
        for rsn in &self.all_reasoning {
            if rsn.title.to_lowercase().contains(&query)
                || rsn.conclusion.to_lowercase().contains(&query)
            {
                results.push(SearchResultRow {
                    entity_type: "reasoning".to_string(),
                    title: rsn.title.clone(),
                    preview: rsn.conclusion.chars().take(60).collect(),
                });
            }
        }

        // Search ADRs
        for adr in &self.all_adrs {
            if adr.title.to_lowercase().contains(&query)
                || adr.context.to_lowercase().contains(&query)
            {
                results.push(SearchResultRow {
                    entity_type: "adr".to_string(),
                    title: adr.title.clone(),
                    preview: adr.context.chars().take(60).collect(),
                });
            }
        }

        // Search theories
        for theory in &self.all_theories {
            if theory.domain_name.to_lowercase().contains(&query) {
                results.push(SearchResultRow {
                    entity_type: "theory".to_string(),
                    title: theory.domain_name.clone(),
                    preview: format!("iter: {}", theory.iteration_count),
                });
            }
        }

        // Search workflows
        for w in &self.all_workflows {
            if w.title.to_lowercase().contains(&query)
                || w.description.to_lowercase().contains(&query)
            {
                results.push(SearchResultRow {
                    entity_type: "workflow".to_string(),
                    title: w.title.clone(),
                    preview: w.description.chars().take(60).collect(),
                });
            }
        }

        // Search knowledge
        for k in &self.all_knowledge {
            if k.title.to_lowercase().contains(&query) || k.content.to_lowercase().contains(&query)
            {
                results.push(SearchResultRow {
                    entity_type: "knowledge".to_string(),
                    title: k.title.clone(),
                    preview: k.content.chars().take(60).collect(),
                });
            }
        }

        // Search sessions
        for s in &self.all_sessions {
            if s.title.to_lowercase().contains(&query) {
                results.push(SearchResultRow {
                    entity_type: "session".to_string(),
                    title: s.title.clone(),
                    preview: format!("agent: {}", s.agent),
                });
            }
        }

        // Search compliance
        for c in &self.all_compliance {
            if c.title.to_lowercase().contains(&query)
                || c.description.to_lowercase().contains(&query)
                || c.category.to_lowercase().contains(&query)
            {
                results.push(SearchResultRow {
                    entity_type: "compliance".to_string(),
                    title: c.title.clone(),
                    preview: c.description.chars().take(60).collect(),
                });
            }
        }

        // Search rules
        for r in &self.all_rules {
            if r.title.to_lowercase().contains(&query)
                || r.description.to_lowercase().contains(&query)
            {
                results.push(SearchResultRow {
                    entity_type: "rule".to_string(),
                    title: r.title.clone(),
                    preview: r.description.chars().take(60).collect(),
                });
            }
        }

        // Search standards
        for s in &self.all_standards {
            if s.title.to_lowercase().contains(&query)
                || s.description.to_lowercase().contains(&query)
            {
                results.push(SearchResultRow {
                    entity_type: "standard".to_string(),
                    title: s.title.clone(),
                    preview: s.description.chars().take(60).collect(),
                });
            }
        }

        // Search state reflections
        for sr in &self.all_state_reflections {
            if sr.observed_state.to_lowercase().contains(&query) {
                results.push(SearchResultRow {
                    entity_type: "state_reflection".to_string(),
                    title: sr.observed_state.chars().take(60).collect(),
                    preview: format!("dissonance: {:.2}", sr.dissonance_score),
                });
            }
        }

        // Search escalations
        for e in &self.all_escalations {
            let op = format!("{:?}", e.operation_type).to_lowercase();
            if op.contains(&query) || e.agent_id.to_lowercase().contains(&query) {
                results.push(SearchResultRow {
                    entity_type: "escalation".to_string(),
                    title: format!("{:?}", e.operation_type),
                    preview: format!("agent: {}", e.agent_id),
                });
            }
        }

        // Search progressive configs
        for c in &self.all_progressive_configs {
            if c.name.to_lowercase().contains(&query)
                || c.description.to_lowercase().contains(&query)
            {
                results.push(SearchResultRow {
                    entity_type: "progressive_config".to_string(),
                    title: c.name.clone(),
                    preview: c.description.chars().take(60).collect(),
                });
            }
        }

        self.search_results = results;
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

// ── Conversion functions ─────────────────────────────────────────────────────

pub fn task_to_row(task: &Task) -> TaskRow {
    let status = match &task.status {
        TaskStatus::Todo => "todo".to_string(),
        TaskStatus::InProgress => "in_progress".to_string(),
        TaskStatus::Done => "done".to_string(),
        TaskStatus::Blocked => "blocked".to_string(),
        TaskStatus::Cancelled => "cancelled".to_string(),
    };
    let priority = format!("{:?}", task.priority).to_lowercase();
    TaskRow {
        id: task.id.chars().take(8).collect(),
        title: task.title.clone(),
        status,
        priority,
        created: task.start_time.format("%Y-%m-%d").to_string(),
    }
}

pub fn compute_summary(rows: &[TaskRow]) -> TaskSummary {
    TaskSummary {
        total: rows.len(),
        todo: rows
            .iter()
            .filter(|r| r.status.to_lowercase().contains("todo"))
            .count(),
        in_progress: rows
            .iter()
            .filter(|r| {
                r.status.to_lowercase().contains("inprogress")
                    || r.status.to_lowercase().contains("in_progress")
            })
            .count(),
        done: rows
            .iter()
            .filter(|r| r.status.to_lowercase().contains("done"))
            .count(),
    }
}

pub fn reasoning_to_node(r: &Reasoning) -> ReasoningNode {
    let preview: String = if !r.conclusion.is_empty() {
        r.conclusion.chars().take(80).collect()
    } else {
        r.steps
            .first()
            .map(|s| s.description.chars().take(80).collect())
            .unwrap_or_default()
    };
    ReasoningNode {
        id: r.id.clone(),
        title: r.title.clone(),
        content_preview: preview,
        task_id: Some(r.task_id.clone()),
        depth: 0,
        expanded: false,
    }
}

pub fn build_relationship_nodes(
    rels: &[EntityRelationship],
    title_map: &HashMap<String, String>,
) -> Vec<RelationshipNode> {
    let mut map: HashMap<String, RelationshipNode> = HashMap::new();
    for rel in rels {
        if !rel.active {
            continue;
        }
        let node = map
            .entry(rel.source_id.clone())
            .or_insert(RelationshipNode {
                id: rel.source_id.clone(),
                title: title_map
                    .get(&rel.source_id)
                    .cloned()
                    .unwrap_or_else(|| rel.source_id.chars().take(8).collect()),
                entity_type: rel.source_type.clone(),
                edges: vec![],
            });
        node.edges.push(RelationshipEdge {
            from_id: rel.source_id.clone(),
            to_id: rel.target_id.clone(),
            to_type: rel.target_type.clone(),
            relationship_type: rel.relationship_type.to_string(),
            to_title: title_map
                .get(&rel.target_id)
                .cloned()
                .unwrap_or_else(|| rel.target_id.chars().take(8).collect()),
            agent: rel.agent.clone(),
        });
    }
    map.into_values().collect()
}

/// Build a title map from all loaded entities: id -> title.
#[allow(clippy::too_many_arguments)]
pub fn build_title_map(
    tasks: &[Task],
    contexts: &[crate::entities::Context],
    reasoning: &[Reasoning],
    adrs: &[crate::entities::ADR],
    theories: &[crate::entities::Theory],
    workflows: &[Workflow],
    workflow_instances: &[WorkflowInstance],
    knowledge: &[Knowledge],
    sessions: &[Session],
    compliance: &[Compliance],
    rules: &[Rule],
    standards: &[Standard],
    state_reflections: &[StateReflection],
    escalations: &[EscalationRequest],
) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for t in tasks {
        map.insert(t.id.clone(), t.title.clone());
    }
    for c in contexts {
        map.insert(c.id.clone(), c.title.clone());
    }
    for r in reasoning {
        map.insert(r.id.clone(), r.title.clone());
    }
    for a in adrs {
        map.insert(a.id.clone(), a.title.clone());
    }
    for t in theories {
        map.insert(t.id.clone(), t.domain_name.clone());
    }
    for w in workflows {
        map.insert(w.id.clone(), w.title.clone());
    }
    for wi in workflow_instances {
        // WorkflowInstance has no title; use workflow_id prefix as display name
        map.insert(
            wi.id.clone(),
            format!(
                "instance:{}",
                wi.workflow_id.chars().take(8).collect::<String>()
            ),
        );
    }
    for k in knowledge {
        map.insert(k.id.clone(), k.title.clone());
    }
    for s in sessions {
        map.insert(s.id.clone(), s.title.clone());
    }
    for c in compliance {
        map.insert(c.id.clone(), c.title.clone());
    }
    for r in rules {
        map.insert(r.id.clone(), r.title.clone());
    }
    for s in standards {
        map.insert(s.id.clone(), s.title.clone());
    }
    for sr in state_reflections {
        map.insert(sr.id.clone(), sr.observed_state.chars().take(40).collect());
    }
    for e in escalations {
        map.insert(
            e.id.clone(),
            format!(
                "{:?}:{}",
                e.operation_type,
                e.agent_id.chars().take(8).collect::<String>()
            ),
        );
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{ContextRelevance, TaskPriority};
    use crate::locus_tui::theme::AppTheme;

    #[test]
    fn test_next_view_cycles_through_all_variants() {
        // all() order: Dashboard, Tasks, Reasoning, Relationships, Contexts, Adrs, Theories,
        // Workflows, WorkflowInstances, Knowledge, Sessions, Compliance, Rules, Standards,
        // StateReflections, Escalations, Sandboxes, ExecutionResults, ProgressiveConfigs, Search
        let mut state = AppState::new();
        assert_eq!(state.active_view, ActiveView::Dashboard);
        state.next_view();
        assert_eq!(state.active_view, ActiveView::Tasks);
        state.next_view();
        assert_eq!(state.active_view, ActiveView::Reasoning);
        state.next_view();
        assert_eq!(state.active_view, ActiveView::Relationships);
        state.next_view();
        assert_eq!(state.active_view, ActiveView::Contexts);
        state.next_view();
        assert_eq!(state.active_view, ActiveView::Adrs);
        state.next_view();
        assert_eq!(state.active_view, ActiveView::Theories);
        state.next_view();
        assert_eq!(state.active_view, ActiveView::Workflows);
        state.next_view();
        assert_eq!(state.active_view, ActiveView::WorkflowInstances);
        state.next_view();
        assert_eq!(state.active_view, ActiveView::Knowledge);
        state.next_view();
        assert_eq!(state.active_view, ActiveView::Sessions);
        state.next_view();
        assert_eq!(state.active_view, ActiveView::Compliance);
        state.next_view();
        assert_eq!(state.active_view, ActiveView::Rules);
        state.next_view();
        assert_eq!(state.active_view, ActiveView::Standards);
        state.next_view();
        assert_eq!(state.active_view, ActiveView::StateReflections);
        state.next_view();
        assert_eq!(state.active_view, ActiveView::Escalations);
        state.next_view();
        assert_eq!(state.active_view, ActiveView::Sandboxes);
        state.next_view();
        assert_eq!(state.active_view, ActiveView::ExecutionResults);
        state.next_view();
        assert_eq!(state.active_view, ActiveView::ProgressiveConfigs);
        state.next_view();
        assert_eq!(state.active_view, ActiveView::Search);
        // Should wrap back to Dashboard on step 21
        state.next_view();
        assert_eq!(state.active_view, ActiveView::Dashboard);
    }

    #[test]
    fn test_prev_view_cycles_backward() {
        // Reverse of all() order, last element is Search
        let mut state = AppState::new();
        assert_eq!(state.active_view, ActiveView::Dashboard);
        // Going backward from Dashboard should wrap to Search (last in all())
        state.prev_view();
        assert_eq!(state.active_view, ActiveView::Search);
        state.prev_view();
        assert_eq!(state.active_view, ActiveView::ProgressiveConfigs);
        state.prev_view();
        assert_eq!(state.active_view, ActiveView::ExecutionResults);
        state.prev_view();
        assert_eq!(state.active_view, ActiveView::Sandboxes);
        state.prev_view();
        assert_eq!(state.active_view, ActiveView::Escalations);
        state.prev_view();
        assert_eq!(state.active_view, ActiveView::StateReflections);
        state.prev_view();
        assert_eq!(state.active_view, ActiveView::Standards);
        state.prev_view();
        assert_eq!(state.active_view, ActiveView::Rules);
        state.prev_view();
        assert_eq!(state.active_view, ActiveView::Compliance);
        state.prev_view();
        assert_eq!(state.active_view, ActiveView::Sessions);
        state.prev_view();
        assert_eq!(state.active_view, ActiveView::Knowledge);
        state.prev_view();
        assert_eq!(state.active_view, ActiveView::WorkflowInstances);
        state.prev_view();
        assert_eq!(state.active_view, ActiveView::Workflows);
        state.prev_view();
        assert_eq!(state.active_view, ActiveView::Theories);
        state.prev_view();
        assert_eq!(state.active_view, ActiveView::Adrs);
        state.prev_view();
        assert_eq!(state.active_view, ActiveView::Contexts);
        state.prev_view();
        assert_eq!(state.active_view, ActiveView::Relationships);
        state.prev_view();
        assert_eq!(state.active_view, ActiveView::Reasoning);
        state.prev_view();
        assert_eq!(state.active_view, ActiveView::Tasks);
        state.prev_view();
        assert_eq!(state.active_view, ActiveView::Dashboard);
    }

    #[test]
    fn test_select_next_increments() {
        let mut state = AppState::new();
        assert_eq!(state.selected_index, 0);
        state.select_next();
        assert_eq!(state.selected_index, 1);
        state.select_next();
        assert_eq!(state.selected_index, 2);
    }

    #[test]
    fn test_select_prev_saturates_at_zero() {
        let mut state = AppState::new();
        assert_eq!(state.selected_index, 0);
        state.select_prev();
        assert_eq!(state.selected_index, 0);
        state.select_next();
        state.select_next();
        assert_eq!(state.selected_index, 2);
        state.select_prev();
        assert_eq!(state.selected_index, 1);
        state.select_prev();
        assert_eq!(state.selected_index, 0);
        state.select_prev();
        assert_eq!(state.selected_index, 0);
    }

    #[test]
    fn test_quit_sets_should_quit() {
        let mut state = AppState::new();
        assert!(!state.should_quit);
        state.quit();
        assert!(state.should_quit);
    }

    #[test]
    fn test_set_and_clear_status() {
        let mut state = AppState::new();
        assert!(state.status_message.is_none());
        state.set_status("hello");
        assert_eq!(state.status_message, Some("hello".to_string()));
        state.clear_status();
        assert!(state.status_message.is_none());
    }

    #[test]
    fn test_default_theme_is_dark() {
        let state = AppState::new();
        assert!(matches!(state.theme, AppTheme::Dark(_)));
    }

    #[test]
    fn test_toggle_theme_switches_dark_to_light() {
        let mut state = AppState::new();
        assert!(matches!(state.theme, AppTheme::Dark(_)));
        state.toggle_theme();
        assert!(matches!(state.theme, AppTheme::Light(_)));
    }

    #[test]
    fn test_toggle_theme_switches_light_to_dark() {
        let mut state = AppState::new();
        state.toggle_theme(); // dark -> light
        state.toggle_theme(); // light -> dark
        assert!(matches!(state.theme, AppTheme::Dark(_)));
    }

    #[test]
    fn test_task_summary_default_is_zero() {
        let s = TaskSummary::default();
        assert_eq!(s.total, 0);
        assert_eq!(s.todo, 0);
        assert_eq!(s.in_progress, 0);
        assert_eq!(s.done, 0);
    }

    #[test]
    fn test_app_state_new_has_empty_lists() {
        let state = AppState::new();
        assert!(state.recent_tasks.is_empty());
        assert!(state.reasoning_nodes.is_empty());
        assert!(state.relationship_nodes.is_empty());
        assert!(state.contexts.is_empty());
        assert_eq!(state.task_summary.total, 0);
    }

    #[test]
    fn test_task_row_fields() {
        let row = TaskRow {
            id: "abc12345".to_string(),
            title: "Test task".to_string(),
            status: "todo".to_string(),
            priority: "high".to_string(),
            created: "2026-01-01".to_string(),
        };
        assert_eq!(row.id, "abc12345");
        assert_eq!(row.title, "Test task");
        assert_eq!(row.status, "todo");
        assert_eq!(row.priority, "high");
        assert_eq!(row.created, "2026-01-01");
    }

    #[test]
    fn test_filtered_tasks_returns_all_when_no_filter() {
        let mut state = AppState::new();
        state.recent_tasks = vec![
            TaskRow {
                id: "a".to_string(),
                title: "Alpha".to_string(),
                status: "todo".to_string(),
                priority: "high".to_string(),
                created: "2026-01-01".to_string(),
            },
            TaskRow {
                id: "b".to_string(),
                title: "Beta".to_string(),
                status: "done".to_string(),
                priority: "low".to_string(),
                created: "2026-01-02".to_string(),
            },
        ];
        let filtered = state.filtered_tasks();
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_filtered_tasks_filters_by_status() {
        let mut state = AppState::new();
        state.recent_tasks = vec![
            TaskRow {
                id: "a".to_string(),
                title: "Alpha".to_string(),
                status: "todo".to_string(),
                priority: "high".to_string(),
                created: "2026-01-01".to_string(),
            },
            TaskRow {
                id: "b".to_string(),
                title: "Beta".to_string(),
                status: "done".to_string(),
                priority: "low".to_string(),
                created: "2026-01-02".to_string(),
            },
        ];
        state.set_filter_status(Some("todo".to_string()));
        let filtered = state.filtered_tasks();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].status, "todo");
    }

    #[test]
    fn test_filtered_tasks_filters_by_text_case_insensitive() {
        let mut state = AppState::new();
        state.recent_tasks = vec![
            TaskRow {
                id: "a".to_string(),
                title: "CLI parser".to_string(),
                status: "todo".to_string(),
                priority: "high".to_string(),
                created: "2026-01-01".to_string(),
            },
            TaskRow {
                id: "b".to_string(),
                title: "Storage impl".to_string(),
                status: "done".to_string(),
                priority: "low".to_string(),
                created: "2026-01-02".to_string(),
            },
        ];
        state.set_filter_text("CLI".to_string());
        let filtered = state.filtered_tasks();
        assert_eq!(filtered.len(), 1);
        assert!(filtered[0].title.to_lowercase().contains("cli"));
    }

    #[test]
    fn test_select_bottom_of_sets_last_index() {
        let mut state = AppState::new();
        state.select_bottom_of(5);
        assert_eq!(state.selected_index, 4);
    }

    #[test]
    fn test_select_bottom_of_zero_length_noop() {
        let mut state = AppState::new();
        state.selected_index = 3;
        state.select_bottom_of(0);
        assert_eq!(state.selected_index, 3);
    }

    #[test]
    fn test_select_bottom_of_single_element() {
        let mut state = AppState::new();
        state.select_bottom_of(1);
        assert_eq!(state.selected_index, 0);
    }

    // ── Conversion function tests ────────────────────────────────────────────

    #[test]
    fn test_task_to_row_maps_fields_correctly() {
        let task = Task::new(
            "My Task".to_string(),
            "desc".to_string(),
            "agent-1".to_string(),
            TaskPriority::High,
            None,
        );
        let row = task_to_row(&task);
        assert_eq!(row.title, "My Task");
        assert_eq!(row.status, "todo");
        assert_eq!(row.priority, "high");
        assert!(!row.created.is_empty());
        assert_eq!(row.id.len(), 8);
    }

    #[test]
    fn test_task_to_row_status_in_progress() {
        let mut task = Task::new(
            "Task".to_string(),
            "d".to_string(),
            "a".to_string(),
            TaskPriority::Medium,
            None,
        );
        task.start();
        let row = task_to_row(&task);
        assert_eq!(row.status, "in_progress");
    }

    #[test]
    fn test_task_to_row_status_done() {
        let mut task = Task::new(
            "Task".to_string(),
            "d".to_string(),
            "a".to_string(),
            TaskPriority::Low,
            None,
        );
        task.complete("outcome".to_string());
        let row = task_to_row(&task);
        assert_eq!(row.status, "done");
    }

    #[test]
    fn test_compute_summary_counts_statuses() {
        let rows = vec![
            TaskRow {
                id: "1".to_string(),
                title: "A".to_string(),
                status: "todo".to_string(),
                priority: "high".to_string(),
                created: "2026-01-01".to_string(),
            },
            TaskRow {
                id: "2".to_string(),
                title: "B".to_string(),
                status: "in_progress".to_string(),
                priority: "medium".to_string(),
                created: "2026-01-02".to_string(),
            },
            TaskRow {
                id: "3".to_string(),
                title: "C".to_string(),
                status: "done".to_string(),
                priority: "low".to_string(),
                created: "2026-01-03".to_string(),
            },
            TaskRow {
                id: "4".to_string(),
                title: "D".to_string(),
                status: "done".to_string(),
                priority: "low".to_string(),
                created: "2026-01-04".to_string(),
            },
        ];
        let summary = compute_summary(&rows);
        assert_eq!(summary.total, 4);
        assert_eq!(summary.todo, 1);
        assert_eq!(summary.in_progress, 1);
        assert_eq!(summary.done, 2);
    }

    #[test]
    fn test_reasoning_to_node_uses_conclusion_as_preview() {
        let mut r = Reasoning::new(
            "My reasoning".to_string(),
            "task-1".to_string(),
            "agent".to_string(),
        );
        r.set_conclusion("This is the conclusion".to_string(), 0.9);
        let node = reasoning_to_node(&r);
        assert_eq!(node.title, "My reasoning");
        assert_eq!(node.content_preview, "This is the conclusion");
        assert_eq!(node.depth, 0);
        assert!(!node.expanded);
        assert_eq!(node.task_id, Some("task-1".to_string()));
    }

    #[test]
    fn test_reasoning_to_node_falls_back_to_step_description() {
        let mut r = Reasoning::new(
            "My reasoning".to_string(),
            "task-1".to_string(),
            "agent".to_string(),
        );
        r.add_step(
            "Step one description".to_string(),
            "step conclusion".to_string(),
            0.8,
        );
        let node = reasoning_to_node(&r);
        assert_eq!(node.content_preview, "Step one description");
    }

    #[test]
    fn test_reasoning_to_node_empty_gives_empty_preview() {
        let r = Reasoning::new(
            "Empty".to_string(),
            "task-1".to_string(),
            "agent".to_string(),
        );
        let node = reasoning_to_node(&r);
        assert_eq!(node.content_preview, "");
    }

    #[test]
    fn test_build_relationship_nodes_groups_by_source() {
        use crate::entities::{EntityRelationType, EntityRelationship};
        let rel1 = EntityRelationship::new(
            "r1".to_string(),
            "agent".to_string(),
            "src-001".to_string(),
            "task".to_string(),
            "tgt-002".to_string(),
            "context".to_string(),
            EntityRelationType::References,
        );
        let rel2 = EntityRelationship::new(
            "r2".to_string(),
            "agent".to_string(),
            "src-001".to_string(),
            "task".to_string(),
            "tgt-003".to_string(),
            "reasoning".to_string(),
            EntityRelationType::DependsOn,
        );
        let mut title_map = HashMap::new();
        title_map.insert("src-001".to_string(), "Source Task".to_string());
        title_map.insert("tgt-002".to_string(), "Target Context".to_string());
        title_map.insert("tgt-003".to_string(), "Target Reasoning".to_string());

        let nodes = build_relationship_nodes(&[rel1, rel2], &title_map);
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].title, "Source Task");
        assert_eq!(nodes[0].edges.len(), 2);
    }

    #[test]
    fn test_build_relationship_nodes_skips_inactive() {
        use crate::entities::{EntityRelationType, EntityRelationship};
        let mut rel = EntityRelationship::new(
            "r1".to_string(),
            "agent".to_string(),
            "src-001".to_string(),
            "task".to_string(),
            "tgt-002".to_string(),
            "context".to_string(),
            EntityRelationType::References,
        );
        rel.active = false;
        let title_map = HashMap::new();
        let nodes = build_relationship_nodes(&[rel], &title_map);
        assert!(nodes.is_empty());
    }

    #[test]
    fn test_build_title_map_includes_all_entity_types() {
        let task = Task::new(
            "Task title".to_string(),
            "d".to_string(),
            "a".to_string(),
            TaskPriority::High,
            None,
        );
        let ctx = Context::new(
            "Context title".to_string(),
            "content".to_string(),
            "source".to_string(),
            ContextRelevance::Medium,
            "a".to_string(),
        );
        let rsn = Reasoning::new(
            "Reasoning title".to_string(),
            "t1".to_string(),
            "a".to_string(),
        );

        let task_id = task.id.clone();
        let ctx_id = ctx.id.clone();
        let rsn_id = rsn.id.clone();

        let map = build_title_map(
            &[task],
            &[ctx],
            &[rsn],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
        );
        assert_eq!(map.get(&task_id), Some(&"Task title".to_string()));
        assert_eq!(map.get(&ctx_id), Some(&"Context title".to_string()));
        assert_eq!(map.get(&rsn_id), Some(&"Reasoning title".to_string()));
    }

    #[test]
    fn test_build_title_map_includes_workflow_and_knowledge() {
        use crate::entities::{Knowledge, KnowledgeType, Workflow};
        let wf = Workflow::new(
            "Deploy Pipeline".to_string(),
            "desc".to_string(),
            "agent".to_string(),
        );
        let kn = Knowledge::new(
            "Auth Patterns".to_string(),
            "content".to_string(),
            KnowledgeType::Pattern,
            0.9,
            "agent".to_string(),
        );
        let wf_id = wf.id.clone();
        let kn_id = kn.id.clone();
        let map = build_title_map(
            &[],
            &[],
            &[],
            &[],
            &[],
            &[wf],
            &[],
            &[kn],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
        );
        assert_eq!(map.get(&wf_id), Some(&"Deploy Pipeline".to_string()));
        assert_eq!(map.get(&kn_id), Some(&"Auth Patterns".to_string()));
    }

    // ── Reasoning node tests ────────────────────────────────────────────────

    #[test]
    fn test_visible_reasoning_nodes_returns_all() {
        let mut state = AppState::new();
        state.reasoning_nodes = vec![
            ReasoningNode {
                id: "1".to_string(),
                title: "A".to_string(),
                content_preview: "p".to_string(),
                task_id: None,
                depth: 0,
                expanded: false,
            },
            ReasoningNode {
                id: "2".to_string(),
                title: "B".to_string(),
                content_preview: "q".to_string(),
                task_id: None,
                depth: 0,
                expanded: false,
            },
        ];
        let visible = state.visible_reasoning_nodes();
        assert_eq!(visible.len(), 2);
    }

    #[test]
    fn test_toggle_reasoning_node_flips_expanded() {
        let mut state = AppState::new();
        state.reasoning_nodes = vec![ReasoningNode {
            id: "1".to_string(),
            title: "A".to_string(),
            content_preview: "p".to_string(),
            task_id: None,
            depth: 0,
            expanded: true,
        }];
        state.reasoning_selected = 0;
        state.toggle_reasoning_node();
        assert!(!state.reasoning_nodes[0].expanded);
        state.toggle_reasoning_node();
        assert!(state.reasoning_nodes[0].expanded);
    }

    #[test]
    fn test_toggle_reasoning_node_on_collapsed_node() {
        let mut state = AppState::new();
        state.reasoning_nodes = vec![
            ReasoningNode {
                id: "1".to_string(),
                title: "A".to_string(),
                content_preview: "p".to_string(),
                task_id: None,
                depth: 0,
                expanded: false,
            },
            ReasoningNode {
                id: "2".to_string(),
                title: "B".to_string(),
                content_preview: "q".to_string(),
                task_id: None,
                depth: 0,
                expanded: false,
            },
        ];
        state.reasoning_selected = 1;
        state.toggle_reasoning_node();
        assert!(state.reasoning_nodes[1].expanded);
    }

    #[test]
    fn test_toggle_reasoning_node_out_of_bounds_noop() {
        let mut state = AppState::new();
        state.reasoning_selected = 999;
        // Should not panic
        state.toggle_reasoning_node();
    }

    #[test]
    fn test_reasoning_node_indent_prefix_depth_zero() {
        let node = ReasoningNode {
            id: "x".to_string(),
            title: "t".to_string(),
            content_preview: "c".to_string(),
            task_id: None,
            depth: 0,
            expanded: false,
        };
        assert_eq!(node.indent_prefix(), "");
    }

    #[test]
    fn test_reasoning_node_indent_prefix_depth_one() {
        let node = ReasoningNode {
            id: "x".to_string(),
            title: "t".to_string(),
            content_preview: "c".to_string(),
            task_id: None,
            depth: 1,
            expanded: false,
        };
        assert_eq!(node.indent_prefix(), "  ");
    }

    #[test]
    fn test_reasoning_node_indent_prefix_depth_two() {
        let node = ReasoningNode {
            id: "x".to_string(),
            title: "t".to_string(),
            content_preview: "c".to_string(),
            task_id: None,
            depth: 2,
            expanded: true,
        };
        assert_eq!(node.indent_prefix(), "    ");
    }

    #[test]
    fn test_reasoning_node_expand_glyph_collapsed() {
        let node = ReasoningNode {
            id: "x".to_string(),
            title: "t".to_string(),
            content_preview: "c".to_string(),
            task_id: None,
            depth: 0,
            expanded: false,
        };
        assert_eq!(node.expand_glyph(), "▶");
    }

    #[test]
    fn test_reasoning_node_expand_glyph_expanded() {
        let node = ReasoningNode {
            id: "x".to_string(),
            title: "t".to_string(),
            content_preview: "c".to_string(),
            task_id: None,
            depth: 0,
            expanded: true,
        };
        assert_eq!(node.expand_glyph(), "▼");
    }

    // ── Relationship node tests ──────────────────────────────────────────────

    #[test]
    fn test_selected_relationship_node_returns_correct_node() {
        let mut state = AppState::new();
        state.relationship_nodes = vec![
            RelationshipNode {
                id: "r1".to_string(),
                title: "Node A".to_string(),
                entity_type: "task".to_string(),
                edges: vec![],
            },
            RelationshipNode {
                id: "r2".to_string(),
                title: "Node B".to_string(),
                entity_type: "context".to_string(),
                edges: vec![],
            },
        ];
        state.relationship_selected = 0;
        let node = state
            .selected_relationship_node()
            .expect("should have node");
        assert_eq!(node.title, "Node A");

        state.relationship_selected = 1;
        let node = state
            .selected_relationship_node()
            .expect("should have node");
        assert_eq!(node.title, "Node B");
    }

    #[test]
    fn test_selected_relationship_node_returns_none_when_empty() {
        let mut state = AppState::new();
        state.relationship_nodes.clear();
        state.relationship_selected = 0;
        assert!(state.selected_relationship_node().is_none());
    }

    #[test]
    fn test_selected_relationship_node_returns_none_out_of_bounds() {
        let mut state = AppState::new();
        state.relationship_selected = 999;
        assert!(state.selected_relationship_node().is_none());
    }

    #[test]
    fn test_run_search_filters_tasks_and_contexts() {
        let mut state = AppState::new();
        state.all_tasks = vec![
            Task::new(
                "Implement OAuth".to_string(),
                "auth stuff".to_string(),
                "a".to_string(),
                TaskPriority::High,
                None,
            ),
            Task::new(
                "Write tests".to_string(),
                "testing".to_string(),
                "a".to_string(),
                TaskPriority::Medium,
                None,
            ),
        ];
        state.contexts = vec![Context::new(
            "OAuth spec".to_string(),
            "RFC content".to_string(),
            "manual".to_string(),
            ContextRelevance::High,
            "a".to_string(),
        )];
        state.search_query = "oauth".to_string();
        state.run_search();
        assert_eq!(state.search_results.len(), 2);
        assert!(state.search_results.iter().any(|r| r.entity_type == "task"));
        assert!(state
            .search_results
            .iter()
            .any(|r| r.entity_type == "context"));
    }

    #[test]
    fn test_run_search_empty_query_clears_results() {
        let mut state = AppState::new();
        state.search_results = vec![SearchResultRow {
            entity_type: "task".to_string(),
            title: "x".to_string(),
            preview: "y".to_string(),
        }];
        state.search_query = String::new();
        state.run_search();
        assert!(state.search_results.is_empty());
    }

    // ── Auto-refresh tests ───────────────────────────────────────────────────

    #[test]
    fn test_should_auto_refresh_disabled_when_zero() {
        let mut state = AppState::new();
        state.refresh_interval_secs = 0;
        assert!(!state.should_auto_refresh());
    }

    #[test]
    fn test_should_auto_refresh_not_fired_before_interval() {
        let mut state = AppState::new();
        state.refresh_interval_secs = 9999; // very long interval
                                            // last_refresh was just set to Instant::now() in new()
        assert!(!state.should_auto_refresh());
    }

    #[test]
    fn test_reset_refresh_timer_updates_last_refresh() {
        use std::time::Duration;
        let mut state = AppState::new();
        // Backdate last_refresh so it looks stale
        state.last_refresh = std::time::Instant::now()
            .checked_sub(Duration::from_secs(100))
            .unwrap_or(std::time::Instant::now());
        state.refresh_interval_secs = 30;
        // Should fire once
        assert!(state.should_auto_refresh());
        // After firing, timer is reset — should not fire again immediately
        assert!(!state.should_auto_refresh());
    }

    #[test]
    fn test_should_auto_refresh_fires_when_interval_elapsed() {
        use std::time::Duration;
        let mut state = AppState::new();
        state.refresh_interval_secs = 30;
        // Backdate by more than the interval
        state.last_refresh = std::time::Instant::now()
            .checked_sub(Duration::from_secs(60))
            .unwrap_or(std::time::Instant::now());
        assert!(state.should_auto_refresh());
    }
}
