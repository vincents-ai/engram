//! Persona command implementations

use crate::entities::{Entity, Persona};
use crate::error::EngramError;
use crate::storage::Storage;
use clap::Subcommand;
use std::io::Write;

/// Persona commands
#[derive(Debug, Subcommand)]
pub enum PersonaCommands {
    /// Create a new persona
    Create {
        /// URL-safe slug — must match [a-z0-9-]+
        #[arg(long, short = 's')]
        slug: String,

        /// Display title
        #[arg(long, short = 'T')]
        title: String,

        /// Short description
        #[arg(long, short = 'd', default_value = "")]
        description: String,

        /// Full system-prompt instructions
        #[arg(long, short = 'i')]
        instructions: String,

        /// Domain (e.g. "rust", "security")
        #[arg(long, short = 'D', default_value = "")]
        domain: String,

        /// Base persona slug to extend
        #[arg(long)]
        base_persona: Option<String>,

        /// Assigned agent
        #[arg(long, short)]
        agent: Option<String>,

        /// Tags (comma-separated)
        #[arg(long)]
        tags: Option<String>,

        /// CoV question (can be repeated 3-5 times)
        #[arg(long = "cov-question")]
        cov_questions: Vec<String>,

        /// FAP entry in KEY=VALUE format (WHO, WHAT, WHY required when any provided)
        #[arg(long = "fap")]
        fap_entries: Vec<String>,

        /// OV requirement (can be repeated)
        #[arg(long = "ov-requirement")]
        ov_requirements: Vec<String>,
    },
    /// List personas
    List {
        /// Agent filter
        #[arg(long, short)]
        agent: Option<String>,

        /// Domain filter
        #[arg(long, short = 'd')]
        domain: Option<String>,

        /// Tag filter
        #[arg(long, short = 't')]
        tag: Option<String>,

        /// Limit results
        #[arg(long, short)]
        limit: Option<usize>,

        /// Show all results (no limit)
        #[arg(long, conflicts_with = "limit")]
        all: bool,

        /// Offset for pagination
        #[arg(long, short)]
        offset: Option<usize>,
    },
    /// Show persona details (accepts slug or UUID prefix)
    Show {
        /// Persona slug or UUID prefix
        #[arg(long, short)]
        id: String,
    },
    /// Update a persona
    Update {
        /// Persona slug or UUID prefix
        #[arg(long, short)]
        id: String,

        /// New title
        #[arg(long)]
        title: Option<String>,

        /// New description
        #[arg(long)]
        description: Option<String>,

        /// New instructions
        #[arg(long)]
        instructions: Option<String>,

        /// New domain
        #[arg(long)]
        domain: Option<String>,

        /// Add a tag
        #[arg(long)]
        add_tag: Option<String>,

        /// Add a CoV question
        #[arg(long = "add-cov")]
        add_cov: Option<String>,

        /// Add an OV requirement
        #[arg(long = "add-ov")]
        add_ov: Option<String>,

        /// Add a FAP entry (KEY=VALUE)
        #[arg(long = "add-fap")]
        add_fap: Option<String>,
    },
    /// Delete a persona
    Delete {
        /// Persona UUID
        #[arg(long, short)]
        id: String,
    },
    /// Submit a persona to the engram-personas repository as a GitHub issue
    Submit {
        /// Persona slug or UUID prefix
        id: String,

        /// Submission type: "new" or "improvement"
        #[arg(long, default_value = "new", value_parser = ["new", "improvement"])]
        submit_type: String,

        /// Target repository (owner/repo). Defaults to engram_personas_remote in workspace config,
        /// then falls back to vincents-ai/engram-personas.
        #[arg(long)]
        repo: Option<String>,

        /// Additional message to include in the issue body
        #[arg(long)]
        message: Option<String>,
    },
}

// ── helpers ──────────────────────────────────────────────────────────────────

fn parse_fap_entry(entry: &str) -> Result<(String, String), EngramError> {
    let parts: Vec<&str> = entry.splitn(2, '=').collect();
    if parts.len() != 2 || parts[0].is_empty() {
        return Err(EngramError::Validation(format!(
            "FAP entry '{}' must be in KEY=VALUE format",
            entry
        )));
    }
    Ok((parts[0].to_string(), parts[1].to_string()))
}

/// Resolve persona by slug or UUID prefix.
/// First tries direct UUID lookup; if not found, scans by slug field.
fn resolve_persona<S: Storage>(storage: &S, id: &str) -> Result<Persona, EngramError> {
    // Try direct get (UUID prefix match)
    if let Ok(Some(entity)) = storage.get(id, Persona::entity_type()) {
        return Persona::from_generic(entity).map_err(|e| EngramError::Validation(e.to_string()));
    }

    // Scan all personas and match by slug
    let ids = storage.list_ids(Persona::entity_type())?;
    for pid in ids {
        if let Some(entity) = storage.get(&pid, Persona::entity_type())? {
            if let Ok(p) = Persona::from_generic(entity) {
                if p.slug == id {
                    return Ok(p);
                }
            }
        }
    }

    Err(EngramError::NotFound(format!("Persona not found: {}", id)))
}

// ── CRUD functions ────────────────────────────────────────────────────────────

/// Create a new persona
pub fn create_persona<S: Storage>(
    storage: &mut S,
    slug: String,
    title: String,
    description: String,
    instructions: String,
    domain: String,
    base_persona: Option<String>,
    agent: Option<String>,
    tags: Option<String>,
    cov_questions: Vec<String>,
    fap_entries: Vec<String>,
    ov_requirements: Vec<String>,
) -> Result<(), EngramError> {
    // Validate slug format early
    if !Persona::slug_is_valid(&slug) {
        return Err(EngramError::Validation(format!(
            "Slug '{}' is invalid — must match [a-z0-9-]+",
            slug
        )));
    }

    let agent_name = agent.unwrap_or_else(|| "default".to_string());
    let mut persona = Persona::new(slug, title, description, instructions, domain, agent_name);

    if let Some(base) = base_persona {
        // Warn but don't error if the base persona slug is not found in the embedded set
        let found = crate::personas::get_embedded_personas()
            .iter()
            .any(|(s, _)| *s == base);
        if !found {
            eprintln!(
                "Warning: base persona slug '{}' was not found in embedded personas",
                base
            );
        }
        persona.set_base_persona(base);
    }

    if let Some(tags_str) = tags {
        for tag in tags_str.split(',') {
            persona.add_tag(tag.trim().to_string());
        }
    }

    for q in cov_questions {
        persona.add_cov_question(q);
    }

    for entry in fap_entries {
        let (key, val) = parse_fap_entry(&entry)?;
        persona.set_fap(key, val);
    }

    for req in ov_requirements {
        persona.add_ov_requirement(req);
    }

    persona.validate_entity()?;

    let generic = persona.to_generic();
    storage.store(&generic)?;

    println!("Persona created successfully with ID: {}", persona.id);
    Ok(())
}

use crate::cli::utils::{create_table, truncate};
use prettytable::row;

/// List personas
pub fn list_personas<S: Storage>(
    storage: &S,
    agent: Option<String>,
    domain: Option<String>,
    tag: Option<String>,
    limit: Option<usize>,
    all: bool,
    offset: Option<usize>,
) -> Result<(), EngramError> {
    let ids = storage.list_ids(Persona::entity_type())?;

    let mut items: Vec<Persona> = Vec::new();

    for id in ids {
        if let Some(entity) = storage.get(&id, Persona::entity_type())? {
            if let Ok(persona) = Persona::from_generic(entity) {
                if let Some(ref f) = agent {
                    if persona.agent != *f {
                        continue;
                    }
                }
                if let Some(ref f) = domain {
                    if persona.domain != *f {
                        continue;
                    }
                }
                if let Some(ref f) = tag {
                    if !persona.tags.contains(f) {
                        continue;
                    }
                }
                items.push(persona);
            }
        }
    }

    let total_count = items.len();

    if let Some(off) = offset {
        items = items.into_iter().skip(off).collect();
    }

    if !all {
        if let Some(lim) = limit {
            items.truncate(lim);
        }
    }

    if items.is_empty() {
        println!("No personas found matching the criteria.");
        return Ok(());
    }

    let mut table = create_table();
    table.set_titles(row![
        "Slug", "Title", "Domain", "Version", "Agent", "Updated"
    ]);

    for persona in &items {
        table.add_row(row![
            truncate(&persona.slug, 30),
            truncate(&persona.title, 40),
            truncate(&persona.domain, 15),
            &persona.version,
            truncate(&persona.agent, 15),
            persona.updated_at.format("%Y-%m-%d")
        ]);
    }

    table.printstd();

    if total_count > items.len() {
        println!(
            "(Showing {} of {} — use --all, --offset N, or --limit N)",
            items.len(),
            total_count
        );
    }

    Ok(())
}

/// Show persona details (accepts slug or UUID prefix)
pub fn show_persona<S: Storage>(storage: &S, id: &str) -> Result<(), EngramError> {
    let persona = resolve_persona(storage, id)?;

    println!("Persona Details:");
    println!("================");
    println!("ID: {}", persona.id);
    println!("Slug: {}", persona.slug);
    println!("Title: {}", persona.title);
    println!("Version: {}", persona.version);
    println!("Domain: {}", persona.domain);
    println!("Agent: {}", persona.agent);
    println!("Created: {}", persona.created_at);
    println!("Updated: {}", persona.updated_at);

    if !persona.description.is_empty() {
        println!();
        println!("Description: {}", persona.description);
    }

    if let Some(ref base) = persona.base_persona {
        println!("Base Persona: {}", base);
    }

    println!();
    println!("Instructions:");
    println!("{}", persona.instructions);

    if !persona.cov_questions.is_empty() {
        println!();
        println!("CoV Questions:");
        for (i, q) in persona.cov_questions.iter().enumerate() {
            println!("  {}. {}", i + 1, q);
        }
    }

    if !persona.fap_table.is_empty() {
        println!();
        println!("FAP Table:");
        let mut keys: Vec<&String> = persona.fap_table.keys().collect();
        keys.sort();
        for k in keys {
            println!("  {}: {}", k, persona.fap_table[k]);
        }
    }

    if !persona.ov_requirements.is_empty() {
        println!();
        println!("OV Requirements:");
        for req in &persona.ov_requirements {
            println!("  - {}", req);
        }
    }

    if !persona.tags.is_empty() {
        println!();
        println!("Tags: {}", persona.tags.join(", "));
    }

    Ok(())
}

/// Update a persona (accepts slug or UUID prefix)
pub fn update_persona<S: Storage>(
    storage: &mut S,
    id: &str,
    title: Option<String>,
    description: Option<String>,
    instructions: Option<String>,
    domain: Option<String>,
    add_tag: Option<String>,
    add_cov: Option<String>,
    add_ov: Option<String>,
    add_fap: Option<String>,
) -> Result<(), EngramError> {
    let mut persona = resolve_persona(storage, id)?;

    if let Some(t) = title {
        persona.title = t;
        persona.updated_at = chrono::Utc::now();
    }
    if let Some(d) = description {
        persona.description = d;
        persona.updated_at = chrono::Utc::now();
    }
    if let Some(i) = instructions {
        persona.instructions = i;
        persona.updated_at = chrono::Utc::now();
    }
    if let Some(d) = domain {
        persona.domain = d;
        persona.updated_at = chrono::Utc::now();
    }
    if let Some(tag) = add_tag {
        persona.add_tag(tag);
    }
    if let Some(cov) = add_cov {
        persona.add_cov_question(cov);
    }
    if let Some(ov) = add_ov {
        persona.add_ov_requirement(ov);
    }
    if let Some(fap) = add_fap {
        let (key, val) = parse_fap_entry(&fap)?;
        persona.set_fap(key, val);
    }

    persona.validate_entity()?;

    let generic = persona.to_generic();
    storage.store(&generic)?;

    println!("Persona updated successfully: {}", id);
    Ok(())
}

/// Delete a persona
pub fn delete_persona<S: Storage>(storage: &mut S, id: &str) -> Result<(), EngramError> {
    storage.delete(id, Persona::entity_type())?;
    println!("Persona deleted successfully: {}", id);
    Ok(())
}

fn format_issue_body(persona: &Persona, message: Option<&str>, submit_type: &str) -> String {
    let mut body = String::new();

    body.push_str(&format!("### Persona Slug\n\n{}\n\n", persona.slug));
    body.push_str(&format!("### Title\n\n{}\n\n", persona.title));

    if !persona.description.is_empty() {
        body.push_str(&format!("### Description\n\n{}\n\n", persona.description));
    }

    body.push_str(&format!("### Instructions\n\n{}\n\n", persona.instructions));

    if !persona.domain.is_empty() {
        body.push_str(&format!("### Domain\n\n{}\n\n", persona.domain));
    }

    if !persona.cov_questions.is_empty() {
        body.push_str("### CoV Questions\n\n");
        for (i, q) in persona.cov_questions.iter().enumerate() {
            body.push_str(&format!("{}. {}\n", i + 1, q));
        }
        body.push('\n');
    }

    if !persona.fap_table.is_empty() {
        body.push_str("### FAP Table\n\n");
        let mut keys: Vec<&String> = persona.fap_table.keys().collect();
        keys.sort();
        for k in keys {
            body.push_str(&format!("**{}**: {}\n", k, persona.fap_table[k]));
        }
        body.push('\n');
    }

    if !persona.ov_requirements.is_empty() {
        body.push_str("### OV Requirements\n\n");
        for req in &persona.ov_requirements {
            body.push_str(&format!("- {}\n", req));
        }
        body.push('\n');
    }

    if !persona.tags.is_empty() {
        body.push_str(&format!("### Tags\n\n{}\n\n", persona.tags.join(", ")));
    }

    if let Some(ref base) = persona.base_persona {
        body.push_str(&format!("### Base Persona\n\n{}\n\n", base));
    }

    body.push_str(&format!("### Version\n\n{}\n\n", persona.version));

    body.push_str(&format!("### Submission Type\n\n{}\n", submit_type));

    if let Some(msg) = message {
        if !msg.is_empty() {
            body.push_str(&format!("\n### Additional Message\n\n{}\n", msg));
        }
    }

    body
}

fn resolve_repo(repo_arg: Option<String>) -> Result<String, EngramError> {
    if let Some(r) = repo_arg {
        return Ok(r);
    }

    if let Ok(config) = crate::config::Config::load_with_defaults() {
        if let Some(ref remote) = config.workspace.engram_personas_remote {
            if !remote.is_empty() {
                return Ok(remote.clone());
            }
        }
    }

    Ok("vincents-ai/engram-personas".to_string())
}

pub fn submit_persona<S: Storage>(
    storage: &S,
    id: &str,
    submit_type: String,
    repo: Option<String>,
    message: Option<String>,
) -> Result<(), EngramError> {
    let which_result = std::process::Command::new("which")
        .arg("gh")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if !which_result {
        return Err(EngramError::InvalidOperation(
            "gh CLI is required for persona submission. Install it from https://cli.github.com/"
                .to_string(),
        ));
    }

    let persona = resolve_persona(storage, id)?;
    let repo = resolve_repo(repo)?;
    let body = format_issue_body(&persona, message.as_deref(), &submit_type);

    let title = format!("[Persona]: {}", persona.slug);

    let mut tmp_path = std::env::temp_dir();
    tmp_path.push(format!("engram-persona-submit-{}", persona.id));
    {
        let mut tmp_file = std::fs::File::create(&tmp_path)?;
        tmp_file.write_all(body.as_bytes())?;
    }

    let output = std::process::Command::new("gh")
        .arg("issue")
        .arg("create")
        .arg("--repo")
        .arg(&repo)
        .arg("--title")
        .arg(&title)
        .arg("--body-file")
        .arg(&tmp_path)
        .arg("--label")
        .arg(&submit_type)
        .output()?;

    let _ = std::fs::remove_file(&tmp_path);

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(EngramError::InvalidOperation(format!(
            "gh issue create failed: {}",
            stderr
        )));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let url = stdout.trim();

    println!("Issue created: {}", url);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::MemoryStorage;

    fn create_test_storage() -> MemoryStorage {
        MemoryStorage::new("default")
    }

    fn create_sample_persona(storage: &mut MemoryStorage) {
        create_persona(
            storage,
            "rust-expert".to_string(),
            "Rust Expert".to_string(),
            "Expert in Rust".to_string(),
            "You are a seasoned Rust engineer.".to_string(),
            "rust".to_string(),
            None,
            None,
            None,
            vec![],
            vec![],
            vec![],
        )
        .unwrap();
    }

    #[test]
    fn test_create_persona_basic() {
        let mut storage = create_test_storage();
        create_sample_persona(&mut storage);

        let ids = storage.list_ids("persona").unwrap();
        assert_eq!(ids.len(), 1);

        let entity = storage.get(&ids[0], "persona").unwrap().unwrap();
        let persona = Persona::from_generic(entity).unwrap();
        assert_eq!(persona.slug, "rust-expert");
        assert_eq!(persona.version, "1.0.0");
    }

    #[test]
    fn test_create_persona_invalid_slug() {
        let mut storage = create_test_storage();
        let result = create_persona(
            &mut storage,
            "Rust Expert".to_string(), // invalid slug
            "Title".to_string(),
            "".to_string(),
            "instructions".to_string(),
            "".to_string(),
            None,
            None,
            None,
            vec![],
            vec![],
            vec![],
        );
        assert!(matches!(result, Err(EngramError::Validation(_))));
    }

    #[test]
    fn test_create_persona_invalid_fap_format() {
        let mut storage = create_test_storage();
        let result = create_persona(
            &mut storage,
            "valid-slug".to_string(),
            "Title".to_string(),
            "".to_string(),
            "instructions".to_string(),
            "".to_string(),
            None,
            None,
            None,
            vec![],
            vec!["INVALID_NO_EQUALS".to_string()],
            vec![],
        );
        assert!(matches!(result, Err(EngramError::Validation(_))));
    }

    #[test]
    fn test_create_persona_with_fap_valid() {
        let mut storage = create_test_storage();
        create_persona(
            &mut storage,
            "sec-expert".to_string(),
            "Security Expert".to_string(),
            "".to_string(),
            "instructions".to_string(),
            "security".to_string(),
            None,
            None,
            None,
            vec![],
            vec![
                "WHO=Security engineers".to_string(),
                "WHAT=Secure systems".to_string(),
                "WHY=Prevent breaches".to_string(),
            ],
            vec![],
        )
        .unwrap();

        let ids = storage.list_ids("persona").unwrap();
        let entity = storage.get(&ids[0], "persona").unwrap().unwrap();
        let persona = Persona::from_generic(entity).unwrap();
        assert_eq!(persona.fap_table.get("WHO").unwrap(), "Security engineers");
    }

    #[test]
    fn test_create_persona_cov_too_few() {
        let mut storage = create_test_storage();
        let result = create_persona(
            &mut storage,
            "slug-here".to_string(),
            "Title".to_string(),
            "".to_string(),
            "instructions".to_string(),
            "".to_string(),
            None,
            None,
            None,
            vec!["Q1".into(), "Q2".into()], // only 2
            vec![],
            vec![],
        );
        assert!(matches!(result, Err(EngramError::Validation(_))));
    }

    #[test]
    fn test_show_persona_by_id() {
        let mut storage = create_test_storage();
        create_sample_persona(&mut storage);
        let ids = storage.list_ids("persona").unwrap();
        assert!(show_persona(&storage, &ids[0]).is_ok());
    }

    #[test]
    fn test_show_persona_by_slug() {
        let mut storage = create_test_storage();
        create_sample_persona(&mut storage);
        assert!(show_persona(&storage, "rust-expert").is_ok());
    }

    #[test]
    fn test_show_persona_not_found() {
        let storage = create_test_storage();
        let result = show_persona(&storage, "missing-slug");
        assert!(matches!(result, Err(EngramError::NotFound(_))));
    }

    #[test]
    fn test_update_persona_title() {
        let mut storage = create_test_storage();
        create_sample_persona(&mut storage);
        let ids = storage.list_ids("persona").unwrap();
        let id = &ids[0];

        update_persona(
            &mut storage,
            id,
            Some("New Title".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();

        let entity = storage.get(id, "persona").unwrap().unwrap();
        let persona = Persona::from_generic(entity).unwrap();
        assert_eq!(persona.title, "New Title");
    }

    #[test]
    fn test_update_persona_by_slug() {
        let mut storage = create_test_storage();
        create_sample_persona(&mut storage);

        update_persona(
            &mut storage,
            "rust-expert",
            Some("Slug Updated".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();

        let p = resolve_persona(&storage, "rust-expert").unwrap();
        assert_eq!(p.title, "Slug Updated");
    }

    #[test]
    fn test_update_persona_not_found() {
        let mut storage = create_test_storage();
        let result = update_persona(
            &mut storage,
            "missing",
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );
        assert!(matches!(result, Err(EngramError::NotFound(_))));
    }

    #[test]
    fn test_delete_persona() {
        let mut storage = create_test_storage();
        create_sample_persona(&mut storage);
        let ids = storage.list_ids("persona").unwrap();
        let id = ids[0].clone();

        delete_persona(&mut storage, &id).unwrap();

        let result = storage.get(&id, "persona").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_delete_persona_not_found() {
        let mut storage = create_test_storage();
        let result = delete_persona(&mut storage, "missing-id");
        assert!(matches!(result, Err(EngramError::NotFound(_))));
    }

    #[test]
    fn test_list_personas_empty() {
        let storage = create_test_storage();
        assert!(list_personas(&storage, None, None, None, None, false, None).is_ok());
    }

    #[test]
    fn test_list_personas_domain_filter() {
        let mut storage = create_test_storage();
        create_sample_persona(&mut storage); // domain: rust
        create_persona(
            &mut storage,
            "sec-expert".to_string(),
            "Sec".to_string(),
            "".to_string(),
            "instructions".to_string(),
            "security".to_string(),
            None,
            None,
            None,
            vec![],
            vec![],
            vec![],
        )
        .unwrap();

        assert!(list_personas(
            &storage,
            None,
            Some("rust".to_string()),
            None,
            None,
            false,
            None
        )
        .is_ok());
    }

    #[test]
    fn test_create_persona_with_tags() {
        let mut storage = create_test_storage();
        create_persona(
            &mut storage,
            "tag-test".to_string(),
            "Tag Test".to_string(),
            "".to_string(),
            "instructions".to_string(),
            "".to_string(),
            None,
            None,
            Some("rust,expert,backend".to_string()),
            vec![],
            vec![],
            vec![],
        )
        .unwrap();

        let ids = storage.list_ids("persona").unwrap();
        let entity = storage.get(&ids[0], "persona").unwrap().unwrap();
        let persona = Persona::from_generic(entity).unwrap();
        assert_eq!(persona.tags.len(), 3);
    }

    #[test]
    fn test_parse_fap_entry_valid() {
        let (k, v) = parse_fap_entry("WHO=Developers").unwrap();
        assert_eq!(k, "WHO");
        assert_eq!(v, "Developers");
    }

    #[test]
    fn test_parse_fap_entry_with_equals_in_value() {
        let (k, v) = parse_fap_entry("WHAT=a=b").unwrap();
        assert_eq!(k, "WHAT");
        assert_eq!(v, "a=b");
    }

    #[test]
    fn test_parse_fap_entry_invalid() {
        assert!(parse_fap_entry("NO_EQUALS").is_err());
        assert!(parse_fap_entry("=value").is_err()); // empty key
    }
}
