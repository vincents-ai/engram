//! Lesson entity — captures mistakes, corrections, and prevention rules
//! as first-class engram knowledge artifacts.

use super::{Entity, GenericEntity};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Lesson category — the broad domain the lesson falls into
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LessonCategory {
    Code,
    Domain,
    Process,
    Design,
}

impl std::fmt::Display for LessonCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LessonCategory::Code => write!(f, "code"),
            LessonCategory::Domain => write!(f, "domain"),
            LessonCategory::Process => write!(f, "process"),
            LessonCategory::Design => write!(f, "design"),
        }
    }
}

/// Severity of the lesson
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum LessonSeverity {
    #[default]
    Low,
    Medium,
    High,
}

impl std::fmt::Display for LessonSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LessonSeverity::Low => write!(f, "low"),
            LessonSeverity::Medium => write!(f, "medium"),
            LessonSeverity::High => write!(f, "high"),
        }
    }
}

/// Lesson entity — a structured record of a mistake, its correction, and
/// how to prevent recurrence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    /// Unique identifier
    pub id: String,

    /// Short title summarising the lesson
    pub title: String,

    /// Description of the mistake that was made
    pub mistake: String,

    /// Explanation of the correct approach
    pub correction: String,

    /// Rule or heuristic to prevent recurrence
    pub prevention_rule: String,

    /// Domain this lesson belongs to (e.g. "rust", "postgres", "agent-design")
    #[serde(default)]
    pub domain: String,

    /// Lesson category
    pub category: LessonCategory,

    /// Severity of the mistake
    #[serde(default)]
    pub severity: LessonSeverity,

    /// Agent that recorded this lesson
    pub agent: String,

    /// Tags for searchability
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub tags: Vec<String>,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
}

impl Lesson {
    /// Create a new Lesson.
    pub fn new(
        title: String,
        mistake: String,
        correction: String,
        prevention_rule: String,
        domain: String,
        category: LessonCategory,
        severity: LessonSeverity,
        agent: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            mistake,
            correction,
            prevention_rule,
            domain,
            category,
            severity,
            agent,
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Update the core lesson fields.
    pub fn update(
        &mut self,
        mistake: Option<String>,
        correction: Option<String>,
        prevention_rule: Option<String>,
    ) {
        if let Some(m) = mistake {
            self.mistake = m;
        }
        if let Some(c) = correction {
            self.correction = c;
        }
        if let Some(p) = prevention_rule {
            self.prevention_rule = p;
        }
        self.updated_at = Utc::now();
    }

    /// Add a tag (deduplicates).
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }
}

impl Entity for Lesson {
    fn entity_type() -> &'static str {
        "lesson"
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
        if self.title.is_empty() {
            return Err(crate::EngramError::Validation(
                "Lesson title cannot be empty".to_string(),
            ));
        }
        if self.mistake.is_empty() {
            return Err(crate::EngramError::Validation(
                "Lesson mistake cannot be empty".to_string(),
            ));
        }
        if self.correction.is_empty() {
            return Err(crate::EngramError::Validation(
                "Lesson correction cannot be empty".to_string(),
            ));
        }
        if self.prevention_rule.is_empty() {
            return Err(crate::EngramError::Validation(
                "Lesson prevention_rule cannot be empty".to_string(),
            ));
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
            crate::EngramError::Deserialization(format!("Failed to deserialize Lesson: {}", e))
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

    fn make_lesson() -> Lesson {
        Lesson::new(
            "Always validate slug format".to_string(),
            "Used free-text slugs with spaces".to_string(),
            "Use slugs that match [a-z0-9-]+ only".to_string(),
            "Validate slug with regex before storing".to_string(),
            "rust".to_string(),
            LessonCategory::Code,
            LessonSeverity::Medium,
            "agent".to_string(),
        )
    }

    #[test]
    fn test_lesson_creation() {
        let l = make_lesson();
        assert_eq!(l.title, "Always validate slug format");
        assert_eq!(l.category, LessonCategory::Code);
        assert_eq!(l.severity, LessonSeverity::Medium);
        assert!(!l.id.is_empty());
    }

    #[test]
    fn test_lesson_entity_type() {
        assert_eq!(Lesson::entity_type(), "lesson");
    }

    #[test]
    fn test_lesson_validate_ok() {
        let l = make_lesson();
        assert!(l.validate_entity().is_ok());
    }

    #[test]
    fn test_lesson_validate_empty_mistake() {
        let mut l = make_lesson();
        l.mistake = String::new();
        assert!(matches!(
            l.validate_entity(),
            Err(crate::EngramError::Validation(_))
        ));
    }

    #[test]
    fn test_lesson_validate_empty_correction() {
        let mut l = make_lesson();
        l.correction = String::new();
        assert!(matches!(
            l.validate_entity(),
            Err(crate::EngramError::Validation(_))
        ));
    }

    #[test]
    fn test_lesson_validate_empty_prevention_rule() {
        let mut l = make_lesson();
        l.prevention_rule = String::new();
        assert!(matches!(
            l.validate_entity(),
            Err(crate::EngramError::Validation(_))
        ));
    }

    #[test]
    fn test_lesson_validate_empty_title() {
        let mut l = make_lesson();
        l.title = String::new();
        assert!(matches!(
            l.validate_entity(),
            Err(crate::EngramError::Validation(_))
        ));
    }

    #[test]
    fn test_lesson_update() {
        let mut l = make_lesson();
        let original_created = l.created_at;
        l.update(
            Some("Updated mistake".to_string()),
            None,
            Some("Updated rule".to_string()),
        );
        assert_eq!(l.mistake, "Updated mistake");
        assert_eq!(l.correction, "Use slugs that match [a-z0-9-]+ only"); // unchanged
        assert_eq!(l.prevention_rule, "Updated rule");
        assert_eq!(l.created_at, original_created); // created_at unchanged
    }

    #[test]
    fn test_lesson_add_tag_deduplication() {
        let mut l = make_lesson();
        l.add_tag("validation".to_string());
        l.add_tag("validation".to_string()); // duplicate
        l.add_tag("slug".to_string());
        assert_eq!(l.tags.len(), 2);
        assert!(l.tags.contains(&"validation".to_string()));
        assert!(l.tags.contains(&"slug".to_string()));
    }

    #[test]
    fn test_lesson_round_trip_generic() {
        let l = make_lesson();
        let generic = l.to_generic();
        assert_eq!(generic.entity_type, "lesson");
        assert_eq!(generic.id, l.id);

        let restored = Lesson::from_generic(generic).expect("round-trip failed");
        assert_eq!(restored.id, l.id);
        assert_eq!(restored.title, l.title);
        assert_eq!(restored.mistake, l.mistake);
        assert_eq!(restored.correction, l.correction);
        assert_eq!(restored.prevention_rule, l.prevention_rule);
        assert_eq!(restored.category, l.category);
        assert_eq!(restored.severity, l.severity);
    }

    #[test]
    fn test_lesson_severity_default() {
        let s: LessonSeverity = Default::default();
        assert_eq!(s, LessonSeverity::Low);
    }

    #[test]
    fn test_lesson_category_display() {
        assert_eq!(LessonCategory::Code.to_string(), "code");
        assert_eq!(LessonCategory::Domain.to_string(), "domain");
        assert_eq!(LessonCategory::Process.to_string(), "process");
        assert_eq!(LessonCategory::Design.to_string(), "design");
    }

    #[test]
    fn test_lesson_severity_display() {
        assert_eq!(LessonSeverity::Low.to_string(), "low");
        assert_eq!(LessonSeverity::Medium.to_string(), "medium");
        assert_eq!(LessonSeverity::High.to_string(), "high");
    }

    #[test]
    fn test_lesson_tags_skip_serialized_when_empty() {
        let l = make_lesson();
        let json = serde_json::to_string(&l).unwrap();
        // tags should be omitted when empty
        assert!(!json.contains("\"tags\""));
    }

    #[test]
    fn test_lesson_tags_present_when_non_empty() {
        let mut l = make_lesson();
        l.add_tag("rust".to_string());
        let json = serde_json::to_string(&l).unwrap();
        assert!(json.contains("\"tags\""));
        assert!(json.contains("rust"));
    }
}
