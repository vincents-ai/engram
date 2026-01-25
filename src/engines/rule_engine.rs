//! Rule Execution Engine
//!
//! Provides business rule enforcement, validation, and automated
//! rule execution with conditions, actions, and audit trails.

use crate::entities::{GenericEntity, Rule};
use crate::error::EngramError;
use crate::storage::Storage;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Rule condition for evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleCondition {
    pub expression: String,
    pub description: Option<String>,
}

/// Rule action to execute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleAction {
    pub action_type: String,
    pub parameters: HashMap<String, String>,
    pub description: Option<String>,
}

/// Rule result enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleResult {
    Success,
    Failed(String),
    Skipped(String),
}

/// Rule execution context containing state and variables
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleExecutionContext {
    pub variables: HashMap<String, RuleValue>,
    pub current_entity: Option<GenericEntity>,
    pub executing_agent: String,
    pub execution_time: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
}

/// Values that can be used in rule conditions and actions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum RuleValue {
    String(String),
    Number(f64),
    Boolean(bool),
    DateTime(DateTime<Utc>),
    Array(Vec<RuleValue>),
    Object(HashMap<String, RuleValue>),
    Null,
}

/// Rule execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleExecutionResult {
    pub rule_id: String,
    pub condition_satisfied: bool,
    pub actions_executed: bool,
    pub context: RuleExecutionContext,
    pub errors: Vec<String>,
    pub actions_taken: Vec<String>,
    pub execution_duration_ms: u64,
}

/// Rule execution engine
pub struct RuleExecutionEngine {
    #[allow(dead_code)]
    builtin_functions: HashMap<String, Box<dyn Fn(&[RuleValue]) -> Result<RuleValue, String>>>,
}

impl Default for RuleExecutionEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl RuleExecutionEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            builtin_functions: HashMap::new(),
        };

        engine.register_builtin_functions();
        engine
    }

    pub fn execute_rule(
        &self,
        rule: &Rule,
        context: &mut RuleExecutionContext,
    ) -> Result<RuleExecutionResult, EngramError> {
        let start_time = std::time::Instant::now();
        let mut result = RuleExecutionResult {
            rule_id: rule.id.clone(),
            condition_satisfied: false,
            actions_executed: false,
            context: context.clone(),
            errors: Vec::new(),
            actions_taken: Vec::new(),
            execution_duration_ms: 0,
        };

        match self.evaluate_rule_condition(&rule.condition, context) {
            Ok(condition_result) => {
                result.condition_satisfied = condition_result;

                if condition_result {
                    match self.execute_rule_action(&rule.action, context) {
                        Ok(action_descriptions) => {
                            result.actions_executed = true;
                            result.actions_taken = action_descriptions;
                        }
                        Err(e) => {
                            result
                                .errors
                                .push(format!("Action execution failed: {}", e));
                        }
                    }
                }
            }
            Err(e) => {
                result
                    .errors
                    .push(format!("Condition evaluation failed: {}", e));
            }
        }

        result.execution_duration_ms = start_time.elapsed().as_millis() as u64;
        result.context = context.clone();

        Ok(result)
    }

    pub fn execute_rules_for_entity<S: Storage>(
        &self,
        storage: &S,
        entity: &GenericEntity,
        agent: &str,
    ) -> Result<Vec<RuleExecutionResult>, EngramError> {
        let mut context = RuleExecutionContext {
            variables: HashMap::new(),
            current_entity: Some(entity.clone()),
            executing_agent: agent.to_string(),
            execution_time: Utc::now(),
            metadata: HashMap::new(),
        };

        self.populate_entity_variables(&mut context, entity);

        let rules = storage.query_by_agent(agent, Some("rule"))?;

        let mut results = Vec::new();

        for generic_rule in rules {
            if let Ok(rule_json) = serde_json::to_string(&generic_rule.data) {
                if let Ok(rule) = serde_json::from_str::<Rule>(&rule_json) {
                    if self.rule_applies_to_entity(&rule, entity) {
                        match self.execute_rule(&rule, &mut context) {
                            Ok(result) => results.push(result),
                            Err(e) => {
                                eprintln!("Failed to execute rule {}: {}", rule.id, e);
                            }
                        }
                    }
                }
            }
        }

        Ok(results)
    }

    fn evaluate_rule_condition(
        &self,
        condition: &serde_json::Value,
        context: &RuleExecutionContext,
    ) -> Result<bool, String> {
        match condition {
            serde_json::Value::String(expr) => self.evaluate_expression(expr, context),
            serde_json::Value::Object(obj) => {
                if let Some(expr_str) = obj.get("expression").and_then(|v| v.as_str()) {
                    self.evaluate_expression(expr_str, context)
                } else {
                    Ok(true)
                }
            }
            serde_json::Value::Bool(b) => Ok(*b),
            serde_json::Value::Null => Ok(true),
            _ => Err("Invalid condition format".to_string()),
        }
    }

    fn execute_rule_action(
        &self,
        action: &serde_json::Value,
        context: &mut RuleExecutionContext,
    ) -> Result<Vec<String>, String> {
        let mut action_descriptions = Vec::new();

        match action {
            serde_json::Value::String(action_str) => {
                action_descriptions.push(format!("Executed: {}", action_str));
            }
            serde_json::Value::Object(obj) => {
                if let Some(serde_json::Value::String(action_type)) = obj.get("type") {
                    match action_type.as_str() {
                        "log" => {
                            if let Some(serde_json::Value::String(message)) = obj.get("message") {
                                println!("RULE LOG: {}", message);
                                action_descriptions.push(format!("Logged: {}", message));
                            }
                        }
                        "set_metadata" => {
                            if let Some(serde_json::Value::String(key)) = obj.get("key") {
                                if let Some(serde_json::Value::String(value)) = obj.get("value") {
                                    context.metadata.insert(key.clone(), value.clone());
                                    action_descriptions
                                        .push(format!("Set metadata {} = {}", key, value));
                                }
                            }
                        }
                        "validate" => {
                            if let Some(serde_json::Value::String(field)) = obj.get("field") {
                                if !context.variables.contains_key(field) {
                                    return Err(format!("Required field '{}' is missing", field));
                                }
                                action_descriptions.push(format!("Validated field: {}", field));
                            }
                        }
                        _ => {
                            action_descriptions.push(format!("Unknown action: {}", action_type));
                        }
                    }
                }
            }
            _ => {
                action_descriptions.push("Executed unknown action".to_string());
            }
        }

        Ok(action_descriptions)
    }

    fn evaluate_expression(
        &self,
        expression: &str,
        context: &RuleExecutionContext,
    ) -> Result<bool, String> {
        let parts: Vec<&str> = expression.split_whitespace().collect();
        if parts.len() < 3 {
            return Err(format!("Invalid expression: {}", expression));
        }

        let variable_name = parts[0];
        let operator = parts[1];
        let expected_value = parts[2..].join(" ");

        let variable_value = context
            .variables
            .get(variable_name)
            .ok_or_else(|| format!("Variable '{}' not found", variable_name))?;

        match operator {
            "equals" | "==" => {
                let expected = self.parse_value(&expected_value)?;
                Ok(*variable_value == expected)
            }
            "not_equals" | "!=" => {
                let expected = self.parse_value(&expected_value)?;
                Ok(*variable_value != expected)
            }
            "greater_than" | ">" => {
                self.compare_numeric(variable_value, &expected_value, |a, b| a > b)
            }
            "less_than" | "<" => {
                self.compare_numeric(variable_value, &expected_value, |a, b| a < b)
            }
            "contains" => match variable_value {
                RuleValue::String(s) => Ok(s.contains(&expected_value)),
                RuleValue::Array(arr) => {
                    let expected = self.parse_value(&expected_value)?;
                    Ok(arr.contains(&expected))
                }
                _ => Err(format!(
                    "Contains operator not supported for {:?}",
                    variable_value
                )),
            },
            _ => Err(format!("Unknown operator: {}", operator)),
        }
    }

    fn rule_applies_to_entity(&self, rule: &Rule, entity: &GenericEntity) -> bool {
        rule.entity_types.is_empty() || rule.entity_types.contains(&entity.entity_type)
    }

    fn populate_entity_variables(
        &self,
        context: &mut RuleExecutionContext,
        entity: &GenericEntity,
    ) {
        context
            .variables
            .insert("id".to_string(), RuleValue::String(entity.id.clone()));
        context.variables.insert(
            "entity_type".to_string(),
            RuleValue::String(entity.entity_type.clone()),
        );
        context
            .variables
            .insert("agent".to_string(), RuleValue::String(entity.agent.clone()));
        context.variables.insert(
            "timestamp".to_string(),
            RuleValue::DateTime(entity.timestamp),
        );

        self.extract_variables_from_json(&entity.data, "", &mut context.variables);
    }

    fn extract_variables_from_json(
        &self,
        json: &serde_json::Value,
        prefix: &str,
        variables: &mut HashMap<String, RuleValue>,
    ) {
        match json {
            serde_json::Value::Object(map) => {
                for (key, value) in map {
                    let var_name = if prefix.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", prefix, key)
                    };

                    match value {
                        serde_json::Value::String(s) => {
                            variables.insert(var_name, RuleValue::String(s.clone()));
                        }
                        serde_json::Value::Number(n) => {
                            if let Some(f) = n.as_f64() {
                                variables.insert(var_name, RuleValue::Number(f));
                            }
                        }
                        serde_json::Value::Bool(b) => {
                            variables.insert(var_name, RuleValue::Boolean(*b));
                        }
                        serde_json::Value::Null => {
                            variables.insert(var_name, RuleValue::Null);
                        }
                        serde_json::Value::Object(_) => {
                            self.extract_variables_from_json(value, &var_name, variables);
                        }
                        serde_json::Value::Array(_) => {}
                    }
                }
            }
            _ => {}
        }
    }

    fn parse_value(&self, value_str: &str) -> Result<RuleValue, String> {
        if let Ok(num) = value_str.parse::<f64>() {
            return Ok(RuleValue::Number(num));
        }

        if let Ok(boolean) = value_str.parse::<bool>() {
            return Ok(RuleValue::Boolean(boolean));
        }

        if value_str.eq_ignore_ascii_case("null") {
            return Ok(RuleValue::Null);
        }

        Ok(RuleValue::String(value_str.to_string()))
    }

    fn compare_numeric<F>(
        &self,
        left: &RuleValue,
        right_str: &str,
        comparator: F,
    ) -> Result<bool, String>
    where
        F: Fn(f64, f64) -> bool,
    {
        let left_num = match left {
            RuleValue::Number(n) => *n,
            _ => return Err("Left operand is not numeric".to_string()),
        };

        let right_num = right_str
            .parse::<f64>()
            .map_err(|_| format!("Right operand '{}' is not numeric", right_str))?;

        Ok(comparator(left_num, right_num))
    }

    fn register_builtin_functions(&mut self) {}
}

impl fmt::Display for RuleValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuleValue::String(s) => write!(f, "{}", s),
            RuleValue::Number(n) => write!(f, "{}", n),
            RuleValue::Boolean(b) => write!(f, "{}", b),
            RuleValue::DateTime(dt) => write!(f, "{}", dt.to_rfc3339()),
            RuleValue::Array(arr) => write!(
                f,
                "[{}]",
                arr.iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            RuleValue::Object(obj) => write!(
                f,
                "{{{}}}",
                obj.iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            RuleValue::Null => write!(f, "null"),
        }
    }
}

pub struct RuleEngineBuilder {
    rules: Vec<Rule>,
}

impl RuleEngineBuilder {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_rule(mut self, rule: Rule) -> Self {
        self.rules.push(rule);
        self
    }

    pub fn build(self) -> RuleExecutionEngine {
        RuleExecutionEngine::new()
    }
}

impl Default for RuleEngineBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{RulePriority, RuleStatus, RuleType};
    use crate::storage::Storage;
    use serde_json::json;

    fn create_test_rule() -> Rule {
        Rule {
            id: "test-rule-1".to_string(),
            title: "Test Rule".to_string(),
            description: "A test rule for validation".to_string(),
            rule_type: RuleType::Validation,
            status: RuleStatus::Active,
            priority: RulePriority::Medium,
            agent: "test-agent".to_string(),
            condition: json!({
                "expression": "priority equals high"
            }),
            action: json!({
                "type": "log",
                "message": "High priority task detected"
            }),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            entity_types: vec!["task".to_string()],
            execution_history: vec![],
            tags: vec!["test".to_string()],
            related_rules: vec![],
            metadata: HashMap::new(),
        }
    }

    fn create_test_entity() -> GenericEntity {
        GenericEntity {
            id: "test-entity-1".to_string(),
            entity_type: "task".to_string(),
            agent: "test-agent".to_string(),
            timestamp: Utc::now(),
            data: json!({
                "title": "Test Task",
                "priority": "high",
                "status": "pending"
            }),
        }
    }

    #[test]
    fn test_rule_engine_creation() {
        let engine = RuleExecutionEngine::new();
        assert!(engine.builtin_functions.is_empty());
    }

    #[test]
    fn test_context_variable_population() {
        let engine = RuleExecutionEngine::new();
        let entity = create_test_entity();
        let mut context = RuleExecutionContext {
            variables: HashMap::new(),
            current_entity: Some(entity.clone()),
            executing_agent: "test-agent".to_string(),
            execution_time: Utc::now(),
            metadata: HashMap::new(),
        };

        engine.populate_entity_variables(&mut context, &entity);

        assert!(context.variables.contains_key("id"));
        assert!(context.variables.contains_key("entity_type"));
        assert!(context.variables.contains_key("priority"));

        if let Some(RuleValue::String(priority)) = context.variables.get("priority") {
            assert_eq!(priority, "high");
        } else {
            panic!("Priority variable not found or wrong type");
        }
    }

    #[test]
    fn test_expression_evaluation_equals() {
        let engine = RuleExecutionEngine::new();
        let mut context = RuleExecutionContext {
            variables: HashMap::new(),
            current_entity: None,
            executing_agent: "test-agent".to_string(),
            execution_time: Utc::now(),
            metadata: HashMap::new(),
        };

        context.variables.insert(
            "priority".to_string(),
            RuleValue::String("high".to_string()),
        );

        let result = engine.evaluate_expression("priority equals high", &context);
        assert!(result.is_ok());
        assert!(result.unwrap());

        let result = engine.evaluate_expression("priority equals low", &context);
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_execute_rule_success() {
        let engine = RuleExecutionEngine::new();
        let rule = create_test_rule();
        let entity = create_test_entity();

        let mut context = RuleExecutionContext {
            variables: HashMap::new(),
            current_entity: Some(entity.clone()),
            executing_agent: "test-agent".to_string(),
            execution_time: Utc::now(),
            metadata: HashMap::new(),
        };

        engine.populate_entity_variables(&mut context, &entity);

        let result = engine.execute_rule(&rule, &mut context).unwrap();

        assert!(result.condition_satisfied);
        assert!(result.actions_executed);
        assert_eq!(result.actions_taken.len(), 1);
        assert!(result.actions_taken[0].contains("Logged: High priority task detected"));
    }

    #[test]
    fn test_execute_rule_condition_fail() {
        let engine = RuleExecutionEngine::new();
        let rule = create_test_rule();
        // Create entity with low priority so condition fails
        let mut entity = create_test_entity();
        if let serde_json::Value::Object(ref mut map) = entity.data {
            map.insert("priority".to_string(), json!("low"));
        }

        let mut context = RuleExecutionContext {
            variables: HashMap::new(),
            current_entity: Some(entity.clone()),
            executing_agent: "test-agent".to_string(),
            execution_time: Utc::now(),
            metadata: HashMap::new(),
        };

        engine.populate_entity_variables(&mut context, &entity);

        let result = engine.execute_rule(&rule, &mut context).unwrap();

        assert!(!result.condition_satisfied);
        assert!(!result.actions_executed);
    }
}
