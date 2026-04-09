use crate::storage::{GitRefsStorage, RelationshipStorage};
use serde::{Deserialize, Serialize};
use std::collections::{HashSet, VecDeque};

const DEFAULT_MAX_DEPTH: usize = 2;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectedEntity {
    pub entity_id: String,
    pub entity_type: String,
    pub depth: usize,
    pub relationship_type: String,
    pub direction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepWalkResult {
    pub seed_entities: Vec<String>,
    pub max_depth: usize,
    pub connected_entities: Vec<ConnectedEntity>,
    pub total_connected: usize,
}

pub struct DeepWalker;

impl DeepWalker {
    pub fn walk_from_entities(
        storage: &GitRefsStorage,
        entity_ids: &[String],
        max_depth: Option<usize>,
    ) -> Result<DeepWalkResult, crate::error::EngramError> {
        let max_depth = max_depth.unwrap_or(DEFAULT_MAX_DEPTH);
        let mut visited: HashSet<String> = HashSet::new();
        let mut connected: Vec<ConnectedEntity> = Vec::new();
        let mut queue: VecDeque<(String, usize)> = VecDeque::new();

        for id in entity_ids {
            visited.insert(id.clone());
            queue.push_back((id.clone(), 0));
        }

        while let Some((entity_id, depth)) = queue.pop_front() {
            if depth >= max_depth {
                continue;
            }

            let relationships = storage.get_entity_relationships(&entity_id)?;
            for rel in relationships {
                if !rel.active {
                    continue;
                }

                let neighbor_id = if rel.source_id == entity_id {
                    &rel.target_id
                } else {
                    &rel.source_id
                };
                let neighbor_type = if rel.source_id == entity_id {
                    &rel.target_type
                } else {
                    &rel.source_type
                };
                let direction = if rel.source_id == entity_id {
                    "outbound"
                } else {
                    "inbound"
                };

                if !visited.contains(neighbor_id) {
                    visited.insert(neighbor_id.clone());
                    let entry = ConnectedEntity {
                        entity_id: neighbor_id.clone(),
                        entity_type: neighbor_type.clone(),
                        depth: depth + 1,
                        relationship_type: format!("{:?}", rel.relationship_type),
                        direction: direction.to_string(),
                    };
                    connected.push(entry);
                    queue.push_back((neighbor_id.clone(), depth + 1));
                }
            }
        }

        let total_connected = connected.len();
        Ok(DeepWalkResult {
            seed_entities: entity_ids.to_vec(),
            max_depth,
            connected_entities: connected,
            total_connected,
        })
    }

    pub fn resolve_entity_ids(data: &serde_json::Value) -> Vec<String> {
        let mut ids = Vec::new();

        if let Some(tasks) = data.get("tasks").and_then(|v| v.as_array()) {
            for task in tasks {
                if let Some(id) = task.get("id").and_then(|v| v.as_str()) {
                    ids.push(id.to_string());
                }
            }
        }

        if let Some(contexts) = data.get("contexts").and_then(|v| v.as_array()) {
            for ctx in contexts {
                if let Some(id) = ctx.get("id").and_then(|v| v.as_str()) {
                    ids.push(id.to_string());
                }
            }
        }

        if let Some(reasoning) = data.get("reasoning").and_then(|v| v.as_array()) {
            for rsn in reasoning {
                if let Some(id) = rsn.get("id").and_then(|v| v.as_str()) {
                    ids.push(id.to_string());
                }
            }
        }

        if let Some(knowledge) = data.get("knowledge").and_then(|v| v.as_array()) {
            for k in knowledge {
                if let Some(id) = k.get("id").and_then(|v| v.as_str()) {
                    ids.push(id.to_string());
                }
            }
        }

        if let Some(adrs) = data.get("adrs").and_then(|v| v.as_array()) {
            for adr in adrs {
                if let Some(id) = adr.get("id").and_then(|v| v.as_str()) {
                    ids.push(id.to_string());
                }
            }
        }

        if let Some(theories) = data.get("theories").and_then(|v| v.as_array()) {
            for t in theories {
                if let Some(id) = t.get("id").and_then(|v| v.as_str()) {
                    ids.push(id.to_string());
                }
            }
        }

        if let Some(rules) = data.get("rules").and_then(|v| v.as_array()) {
            for r in rules {
                if let Some(id) = r.get("id").and_then(|v| v.as_str()) {
                    ids.push(id.to_string());
                }
            }
        }

        if let Some(standards) = data.get("standards").and_then(|v| v.as_array()) {
            for s in standards {
                if let Some(id) = s.get("id").and_then(|v| v.as_str()) {
                    ids.push(id.to_string());
                }
            }
        }

        if let Some(compliance) = data.get("compliance").and_then(|v| v.as_array()) {
            for c in compliance {
                if let Some(id) = c.get("id").and_then(|v| v.as_str()) {
                    ids.push(id.to_string());
                }
            }
        }

        if let Some(sessions) = data.get("sessions").and_then(|v| v.as_array()) {
            for s in sessions {
                if let Some(id) = s.get("id").and_then(|v| v.as_str()) {
                    ids.push(id.to_string());
                }
            }
        }

        if let Some(workflows) = data.get("workflows").and_then(|v| v.as_array()) {
            for w in workflows {
                if let Some(id) = w.get("id").and_then(|v| v.as_str()) {
                    ids.push(id.to_string());
                }
            }
        }

        if let Some(task_val) = data.get("task") {
            if let Some(id) = task_val.get("id").and_then(|v| v.as_str()) {
                ids.push(id.to_string());
            }
        }

        ids
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_entity_ids_from_tasks() {
        let data = serde_json::json!({
            "tasks": [
                {"id": "task-1", "title": "Task 1"},
                {"id": "task-2", "title": "Task 2"},
            ],
            "count": 2
        });

        let ids = DeepWalker::resolve_entity_ids(&data);
        assert_eq!(ids, vec!["task-1", "task-2"]);
    }

    #[test]
    fn test_resolve_entity_ids_from_multiple_types() {
        let data = serde_json::json!({
            "tasks": [{"id": "t1"}],
            "contexts": [{"id": "c1"}],
            "reasoning": [{"id": "r1"}],
            "knowledge": [{"id": "k1"}],
        });

        let ids = DeepWalker::resolve_entity_ids(&data);
        assert_eq!(ids, vec!["t1", "c1", "r1", "k1"]);
    }

    #[test]
    fn test_resolve_entity_ids_from_single_task() {
        let data = serde_json::json!({
            "task": {"id": "task-abc", "title": "Single task"}
        });

        let ids = DeepWalker::resolve_entity_ids(&data);
        assert_eq!(ids, vec!["task-abc"]);
    }

    #[test]
    fn test_resolve_entity_ids_empty() {
        let data = serde_json::json!({"total_matches": 0});
        let ids = DeepWalker::resolve_entity_ids(&data);
        assert!(ids.is_empty());
    }

    #[test]
    fn test_deep_walk_result_serialization() {
        let result = DeepWalkResult {
            seed_entities: vec!["e1".to_string()],
            max_depth: 2,
            connected_entities: vec![ConnectedEntity {
                entity_id: "e2".to_string(),
                entity_type: "task".to_string(),
                depth: 1,
                relationship_type: "DependsOn".to_string(),
                direction: "outbound".to_string(),
            }],
            total_connected: 1,
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("e2"));
        assert!(json.contains("DependsOn"));
    }
}
