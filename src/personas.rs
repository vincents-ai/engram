use serde::Deserialize;

include!(concat!(env!("OUT_DIR"), "/embedded_personas.rs"));

#[derive(Debug, Clone, Deserialize)]
pub struct PersonaDef {
    pub version: Option<String>,
    pub title: String,
    pub description: String,
    pub instructions: String,
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

/// Find a persona by slug or by partial name match (e.g. "the-architect" matches "03-the-architect")
pub fn find_persona(query: &str) -> Option<(String, PersonaDef)> {
    get_embedded_personas().into_iter().find(|(slug, def)| {
        slug == query
            || slug.ends_with(&format!("-{}", query))
            || def.title.to_lowercase().contains(&query.to_lowercase())
    })
}
