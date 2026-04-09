//! Persona entity — a first-class engram entity for storing expert personas
//! with optional CoV/FAP/OV verification protocol fields.

use super::{Entity, GenericEntity};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Persona entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Persona {
    /// Unique identifier
    pub id: String,

    /// URL-safe slug — must match `[a-z0-9-]+`
    pub slug: String,

    /// Display title
    pub title: String,

    /// Short description of the persona's purpose
    #[serde(default)]
    pub description: String,

    /// Full system-prompt instructions
    pub instructions: String,

    /// Domain this persona specialises in (e.g. "rust", "security")
    #[serde(default)]
    pub domain: String,

    // ── CoV/FAP/OV ──────────────────────────────────────────────────────────
    /// Calibration of Values questions (3-5 items when present)
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub cov_questions: Vec<String>,

    /// Foundational Assumptions & Principles (key → value)
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub fap_table: HashMap<String, String>,

    /// Operational Values / requirements (free-form list)
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub ov_requirements: Vec<String>,

    // ── Inheritance ──────────────────────────────────────────────────────────
    /// Slug of a compiled-in base persona this one extends (warning-only if missing)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_persona: Option<String>,

    // ── Metadata ─────────────────────────────────────────────────────────────
    /// Agent that created this persona
    pub agent: String,

    /// Tags for categorisation
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub tags: Vec<String>,

    /// Semantic version string (default "1.0.0")
    #[serde(default = "default_version")]
    pub version: String,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
}

fn default_version() -> String {
    "1.0.0".to_string()
}

impl Persona {
    /// Create a new Persona.
    pub fn new(
        slug: String,
        title: String,
        description: String,
        instructions: String,
        domain: String,
        agent: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            slug,
            title,
            description,
            instructions,
            domain,
            cov_questions: Vec::new(),
            fap_table: HashMap::new(),
            ov_requirements: Vec::new(),
            base_persona: None,
            agent,
            tags: Vec::new(),
            version: default_version(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Add a CoV question (deduplicates).
    pub fn add_cov_question(&mut self, q: String) {
        if !self.cov_questions.contains(&q) {
            self.cov_questions.push(q);
            self.updated_at = Utc::now();
        }
    }

    /// Set a FAP entry.
    pub fn set_fap(&mut self, key: String, value: String) {
        self.fap_table.insert(key, value);
        self.updated_at = Utc::now();
    }

    /// Add an OV requirement (deduplicates).
    pub fn add_ov_requirement(&mut self, req: String) {
        if !self.ov_requirements.contains(&req) {
            self.ov_requirements.push(req);
            self.updated_at = Utc::now();
        }
    }

    /// Add a tag (deduplicates).
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.updated_at = Utc::now();
        }
    }

    /// Set the base persona slug.
    pub fn set_base_persona(&mut self, slug: String) {
        self.base_persona = Some(slug);
        self.updated_at = Utc::now();
    }

    /// Validate that the slug matches `[a-z0-9-]+`.
    pub fn slug_is_valid(slug: &str) -> bool {
        !slug.is_empty()
            && slug
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    }
}

impl Entity for Persona {
    fn entity_type() -> &'static str {
        "persona"
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn agent(&self) -> &str {
        &self.agent
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.created_at
    }

    fn validate_entity(&self) -> crate::Result<()> {
        if self.slug.is_empty() {
            return Err(crate::EngramError::Validation(
                "Persona slug cannot be empty".to_string(),
            ));
        }
        if !Self::slug_is_valid(&self.slug) {
            return Err(crate::EngramError::Validation(format!(
                "Persona slug '{}' is invalid — must match [a-z0-9-]+",
                self.slug
            )));
        }
        if self.title.is_empty() {
            return Err(crate::EngramError::Validation(
                "Persona title cannot be empty".to_string(),
            ));
        }
        if self.instructions.is_empty() {
            return Err(crate::EngramError::Validation(
                "Persona instructions cannot be empty".to_string(),
            ));
        }
        // CoV: when non-empty must have 3-5 items
        if !self.cov_questions.is_empty()
            && (self.cov_questions.len() < 3 || self.cov_questions.len() > 5)
        {
            return Err(crate::EngramError::Validation(
                "cov_questions must contain 3-5 items when non-empty".to_string(),
            ));
        }
        // FAP: when non-empty must include WHO, WHAT, WHY
        if !self.fap_table.is_empty() {
            for required_key in &["WHO", "WHAT", "WHY"] {
                if !self.fap_table.contains_key(*required_key) {
                    return Err(crate::EngramError::Validation(format!(
                        "fap_table must include '{}' when non-empty",
                        required_key
                    )));
                }
            }
        }
        Ok(())
    }

    fn to_generic(&self) -> GenericEntity {
        GenericEntity {
            id: self.id.clone(),
            entity_type: Self::entity_type().to_string(),
            agent: self.agent.clone(),
            timestamp: self.created_at,
            data: serde_json::to_value(self).unwrap_or_default(),
        }
    }

    fn from_generic(entity: GenericEntity) -> crate::Result<Self> {
        serde_json::from_value(entity.data).map_err(|e| {
            crate::EngramError::Deserialization(format!("Failed to deserialize Persona: {}", e))
        })
    }

    fn as_any(&self) -> &dyn std::any::Any
    where
        Self: Sized,
    {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_persona() -> Persona {
        Persona::new(
            "rust-expert".to_string(),
            "Rust Expert".to_string(),
            "Expert in Rust systems programming".to_string(),
            "You are a seasoned Rust engineer.".to_string(),
            "rust".to_string(),
            "agent".to_string(),
        )
    }

    #[test]
    fn test_persona_creation() {
        let p = make_persona();
        assert_eq!(p.slug, "rust-expert");
        assert_eq!(p.title, "Rust Expert");
        assert_eq!(p.version, "1.0.0");
        assert!(!p.id.is_empty());
    }

    #[test]
    fn test_persona_entity_type() {
        assert_eq!(Persona::entity_type(), "persona");
    }

    #[test]
    fn test_persona_validate_ok() {
        let p = make_persona();
        assert!(p.validate_entity().is_ok());
    }

    #[test]
    fn test_persona_validate_empty_slug() {
        let mut p = make_persona();
        p.slug = String::new();
        assert!(matches!(
            p.validate_entity(),
            Err(crate::EngramError::Validation(_))
        ));
    }

    #[test]
    fn test_persona_validate_invalid_slug() {
        let mut p = make_persona();
        p.slug = "Rust Expert".to_string(); // spaces + capitals — invalid
        assert!(matches!(
            p.validate_entity(),
            Err(crate::EngramError::Validation(_))
        ));
    }

    #[test]
    fn test_persona_validate_empty_title() {
        let mut p = make_persona();
        p.title = String::new();
        assert!(matches!(
            p.validate_entity(),
            Err(crate::EngramError::Validation(_))
        ));
    }

    #[test]
    fn test_persona_validate_empty_instructions() {
        let mut p = make_persona();
        p.instructions = String::new();
        assert!(matches!(
            p.validate_entity(),
            Err(crate::EngramError::Validation(_))
        ));
    }

    #[test]
    fn test_persona_cov_valid() {
        let mut p = make_persona();
        p.cov_questions = vec!["Q1".into(), "Q2".into(), "Q3".into()];
        assert!(p.validate_entity().is_ok());
    }

    #[test]
    fn test_persona_cov_too_few() {
        let mut p = make_persona();
        p.cov_questions = vec!["Q1".into(), "Q2".into()]; // only 2
        assert!(matches!(
            p.validate_entity(),
            Err(crate::EngramError::Validation(_))
        ));
    }

    #[test]
    fn test_persona_cov_too_many() {
        let mut p = make_persona();
        p.cov_questions = vec![
            "Q1".into(),
            "Q2".into(),
            "Q3".into(),
            "Q4".into(),
            "Q5".into(),
            "Q6".into(),
        ];
        assert!(matches!(
            p.validate_entity(),
            Err(crate::EngramError::Validation(_))
        ));
    }

    #[test]
    fn test_persona_fap_valid() {
        let mut p = make_persona();
        p.fap_table.insert("WHO".into(), "Rust developers".into());
        p.fap_table
            .insert("WHAT".into(), "Produce safe code".into());
        p.fap_table.insert("WHY".into(), "Eliminate bugs".into());
        assert!(p.validate_entity().is_ok());
    }

    #[test]
    fn test_persona_fap_missing_key() {
        let mut p = make_persona();
        p.fap_table.insert("WHO".into(), "Rust developers".into());
        p.fap_table
            .insert("WHAT".into(), "Produce safe code".into());
        // missing WHY
        assert!(matches!(
            p.validate_entity(),
            Err(crate::EngramError::Validation(_))
        ));
    }

    #[test]
    fn test_persona_add_tag_deduplication() {
        let mut p = make_persona();
        p.add_tag("rust".to_string());
        p.add_tag("rust".to_string()); // duplicate
        p.add_tag("systems".to_string());
        assert_eq!(p.tags.len(), 2);
    }

    #[test]
    fn test_persona_round_trip_generic() {
        let mut p = make_persona();
        p.add_cov_question("Q1".into());
        p.add_cov_question("Q2".into());
        p.add_cov_question("Q3".into());

        let generic = p.to_generic();
        assert_eq!(generic.entity_type, "persona");

        let restored = Persona::from_generic(generic).expect("round-trip failed");
        assert_eq!(restored.id, p.id);
        assert_eq!(restored.slug, p.slug);
        assert_eq!(restored.cov_questions.len(), 3);
    }

    #[test]
    fn test_slug_is_valid() {
        assert!(Persona::slug_is_valid("rust-expert"));
        assert!(Persona::slug_is_valid("01-the-one"));
        assert!(Persona::slug_is_valid("abc123"));
        assert!(!Persona::slug_is_valid("Rust Expert"));
        assert!(!Persona::slug_is_valid("rust_expert")); // underscore not allowed
        assert!(!Persona::slug_is_valid(""));
    }

    #[test]
    fn test_persona_set_base_persona() {
        let mut p = make_persona();
        p.set_base_persona("01-the-one".to_string());
        assert_eq!(p.base_persona, Some("01-the-one".to_string()));
    }

    #[test]
    fn test_persona_tags_skip_serialized_when_empty() {
        let p = make_persona();
        let json = serde_json::to_string(&p).unwrap();
        assert!(!json.contains("\"tags\""));
    }

    #[test]
    fn test_persona_version_default_in_deserialization() {
        // Serialise without version field to test default
        let p = make_persona();
        let mut val = serde_json::to_value(&p).unwrap();
        val.as_object_mut().unwrap().remove("version");
        let json = val.to_string();
        let restored: Persona = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.version, "1.0.0");
    }
}
