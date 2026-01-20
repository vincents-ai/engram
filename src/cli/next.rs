use crate::entities::task::Task;
use crate::storage::Storage;
use crate::EngramError;
use std::collections::HashMap;

pub fn interpolate(template: &str, context: &HashMap<String, String>) -> String {
    let mut result = template.to_string();
    for (key, value) in context {
        let placeholder = format!("{{{{{}}}}}", key);
        result = result.replace(&placeholder, value);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpolation() {
        let template = "Hello {{AGENT_NAME}}, working on {{TASK_ID}}";
        let mut context = HashMap::new();
        context.insert("AGENT_NAME".to_string(), "Alice".to_string());
        context.insert("TASK_ID".to_string(), "123".to_string());

        let result = interpolate(template, &context);
        assert_eq!(result, "Hello Alice, working on 123");
    }
}
