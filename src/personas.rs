use serde::Deserialize;
use std::collections::HashMap;

include!(concat!(env!("OUT_DIR"), "/embedded_personas.rs"));

/// A persona definition deserialized from a YAML file.
///
/// The `cov_questions`, `fap_table`, and `ov_requirements` fields implement
/// the CoV/FAP/OV verification protocols from the Structured Expert Prompting
/// (SEP) methodology by @lwedel (https://github.com/lwedel/PersonaArchitect).
/// The schema design and engram integration are original to this project.
#[derive(Debug, Clone, Deserialize)]
pub struct PersonaDef {
    pub version: Option<String>,
    pub title: String,
    pub description: String,
    pub instructions: String,

    /// Chain of Verification (CoV) — 3-5 domain-specific challenge questions
    /// that verify the persona's outputs are grounded and accurate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cov_questions: Option<Vec<String>>,

    /// Forensic Analysis Protocol (FAP) — 5W2H table for diagnosing failures.
    /// Keys: WHO, WHAT, WHEN, WHERE, WHY, HOW, HOW_MUCH
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fap_table: Option<HashMap<String, String>>,

    /// Operational Verification (OV) — evidence requirements that must be
    /// satisfied to confirm the persona's outputs are complete and consistent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ov_requirements: Option<Vec<String>>,
}

/// Returns all embedded personas parsed from YAML.
/// Key is the slug (filename without .yaml), value is parsed PersonaDef.
pub fn get_embedded_personas() -> Vec<(String, PersonaDef)> {
    EMBEDDED_PERSONA_SOURCES
        .iter()
        .filter_map(
            |(slug, src)| match serde_yaml::from_str::<PersonaDef>(src) {
                Ok(def) => Some((slug.to_string(), def)),
                Err(_) => None,
            },
        )
        .collect()
}

/// Find a persona by slug or title match, searching storage first then compiled-in fallback.
///
/// Search order:
/// 1. **GitRefsStorage first** — lists all stored `persona` entities and matches by:
///    - exact slug match
///    - slug ends with `-{query}`
///    - case-insensitive title substring match
///
///    On a match, constructs a `PersonaDef` from the stored `Persona` fields.
///
/// 2. **Compiled-in fallback** — if no match in storage, searches `EMBEDDED_PERSONA_SOURCES`.
pub fn find_persona(
    query: &str,
    storage: &dyn crate::storage::Storage,
) -> Option<(String, PersonaDef)> {
    use crate::entities::persona::Persona;
    use crate::entities::Entity;

    // Tier 1: search storage
    if let Ok(ids) = storage.list_ids("persona") {
        for id in ids {
            if let Ok(Some(generic)) = storage.get(&id, "persona") {
                if let Ok(persona) = Persona::from_generic(generic) {
                    let slug = &persona.slug;
                    let matches = slug == query
                        || slug.ends_with(&format!("-{}", query))
                        || persona.title.to_lowercase().contains(&query.to_lowercase());

                    if matches {
                        let def = PersonaDef {
                            version: Some(persona.version.clone()),
                            title: persona.title.clone(),
                            description: persona.description.clone(),
                            instructions: persona.instructions.clone(),
                            cov_questions: if persona.cov_questions.is_empty() {
                                None
                            } else {
                                Some(persona.cov_questions.clone())
                            },
                            fap_table: if persona.fap_table.is_empty() {
                                None
                            } else {
                                Some(persona.fap_table.clone())
                            },
                            ov_requirements: if persona.ov_requirements.is_empty() {
                                None
                            } else {
                                Some(persona.ov_requirements.clone())
                            },
                        };
                        return Some((slug.clone(), def));
                    }
                }
            }
        }
    }

    // Tier 2: compiled-in fallback
    get_embedded_personas().into_iter().find(|(slug, def)| {
        slug == query
            || slug.ends_with(&format!("-{}", query))
            || def.title.to_lowercase().contains(&query.to_lowercase())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_persona_def_without_cov_fap_ov_deserialises() {
        let yaml = r#"
version: "1.0"
title: "Test Persona"
description: "A simple test persona"
instructions: "Do things well."
"#;
        let def: PersonaDef = serde_yaml::from_str(yaml)
            .expect("PersonaDef without CoV/FAP/OV fields must deserialise cleanly");
        assert_eq!(def.title, "Test Persona");
        assert!(def.cov_questions.is_none());
        assert!(def.fap_table.is_none());
        assert!(def.ov_requirements.is_none());
    }

    #[test]
    fn test_persona_def_with_cov_fap_ov_deserialises() {
        let yaml = r#"
version: "1.0"
title: "Expert Persona"
description: "An expert persona with verification protocols"
instructions: "Apply the SEP methodology."
cov_questions:
  - "Is the domain expertise verifiable?"
  - "Are credentials domain-specific?"
  - "Does the methodology include self-correction?"
fap_table:
  WHO: "Which domain expert?"
  WHAT: "Which output is wrong?"
  WHEN: "At which step did it fail?"
  WHERE: "Which file has the gap?"
  WHY: "Root cause of the failure"
  HOW: "How to reproduce the issue"
  HOW_MUCH: "Impact severity"
ov_requirements:
  - "All CoV questions answered"
  - "FAP table populated with domain-specific content"
  - "Instructions reference the methodology"
"#;
        let def: PersonaDef = serde_yaml::from_str(yaml)
            .expect("PersonaDef with CoV/FAP/OV fields must deserialise correctly");
        assert_eq!(def.title, "Expert Persona");

        let cov = def.cov_questions.expect("cov_questions should be Some");
        assert_eq!(cov.len(), 3);

        let fap = def.fap_table.expect("fap_table should be Some");
        assert!(fap.contains_key("WHO"));
        assert!(fap.contains_key("WHAT"));
        assert!(fap.contains_key("WHY"));

        let ov = def.ov_requirements.expect("ov_requirements should be Some");
        assert_eq!(ov.len(), 3);
    }
}
