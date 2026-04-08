//! Doc command implementations — mdBook assembler + refs file search

use crate::cli::utils::{create_table, truncate};
use crate::entities::doc_fragment::{check_staleness, StalenessReport};
use crate::entities::{DocFragment, Entity};
use crate::error::EngramError;
use crate::storage::Storage;
use clap::Subcommand;
use prettytable::row;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::io::Read as _;
use std::path::{Path, PathBuf};

const VALID_TOPICS: &[&str] = &[
    "adrs",
    "decisions",
    "tasks",
    "knowledge",
    "theories",
    "workflows",
    "sessions",
    "reasoning",
    "standards",
    "overview",
];

const TOPIC_DESCRIPTIONS: &[(&str, &str, &str)] = &[
    ("adrs", "adr", "Architectural Decision Records"),
    ("decisions", "reasoning", "Decision chains and rationale"),
    ("tasks", "task", "Task descriptions and outcomes"),
    ("knowledge", "knowledge", "Knowledge base entries"),
    ("theories", "theory", "Domain theories and mental models"),
    (
        "workflows",
        "workflow",
        "Workflow definitions and state machines",
    ),
    ("sessions", "session", "Coding session records"),
    ("reasoning", "reasoning", "Reasoning chains and analysis"),
    ("standards", "standard", "Coding standards and requirements"),
    ("overview", "(auto)", "Auto-aggregated project overview"),
];

#[derive(Debug, Subcommand)]
pub enum DocCommands {
    /// Assemble mdBook source files from stored DocFragments
    Build {
        /// Output directory for mdBook source (default: docs/)
        #[arg(long, short, default_value = "docs")]
        output: String,
    },
    /// List available documentation topics
    Topics {
        #[command(subcommand)]
        command: TopicsCommands,
    },
    /// Manage chunks within a topic
    Chunk {
        #[command(subcommand)]
        command: ChunkCommands,
    },
    /// Write a documentation chunk
    Write {
        /// Topic name
        topic: String,
        /// Chunk identifier
        chunk_id: String,
        /// Chunk title
        #[arg(long, short)]
        title: String,
        /// Ordering position
        #[arg(long, short, default_value = "0")]
        order: u32,
        /// Read content from stdin
        #[arg(long)]
        stdin: bool,
        /// Read content from file
        #[arg(long)]
        file: Option<String>,
        /// Content string
        #[arg(long, conflicts_with_all = ["stdin", "file"])]
        content: Option<String>,

        /// Agent name
        #[arg(long, default_value = "default")]
        agent: String,
    },
    /// Show build status — which topics have chunks, staleness
    Status {
        /// Output directory to check
        #[arg(long, short, default_value = "docs")]
        output: String,
    },
    /// Search project files for references matching a query
    Refs {
        /// Search query (literal string or regex if --regex)
        query: String,

        /// Directory to search (default: current project root)
        #[arg(long, short)]
        dir: Option<String>,

        /// Treat query as a regular expression
        #[arg(long)]
        regex: bool,

        /// Case-insensitive search
        #[arg(long, short)]
        ignore_case: bool,

        /// Maximum number of results per file (default: 3)
        #[arg(long, short, default_value = "3")]
        max_per_file: usize,

        /// Maximum total results (default: 50)
        #[arg(long, short, default_value = "50")]
        max_results: usize,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Fetch all entities for a topic, formatted for LLM consumption
    Fetch {
        /// Topic to fetch (adrs, decisions, tasks, knowledge, etc.)
        topic: String,

        /// Output format: json or md
        #[arg(long, default_value = "json")]
        format: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum TopicsCommands {
    /// List all available documentation topics
    List,
}

#[derive(Debug, Subcommand)]
pub enum ChunkCommands {
    /// List chunks in a topic
    List {
        /// Topic name
        topic: String,
    },
    /// Delete a chunk
    Delete {
        /// Topic name
        topic: String,
        /// Chunk identifier
        chunk_id: String,
    },
}

pub fn handle_doc_command<S: Storage>(
    command: DocCommands,
    storage: &mut S,
) -> Result<(), EngramError> {
    match command {
        DocCommands::Build { output } => build_doc(&output, storage),
        DocCommands::Topics { command } => handle_topics_command(command, storage),
        DocCommands::Chunk { command } => handle_chunk_command(command, storage),
        DocCommands::Write {
            topic,
            chunk_id,
            title,
            order,
            stdin,
            file,
            content,
            agent,
        } => write_chunk(
            storage,
            &topic,
            &chunk_id,
            &title,
            order,
            stdin,
            file.as_ref(),
            content,
            &agent,
        ),
        DocCommands::Status { output } => status_doc(&output, storage),
        DocCommands::Refs {
            query,
            dir,
            regex,
            ignore_case,
            max_per_file,
            max_results,
            json,
        } => refs_command(
            &query,
            dir,
            regex,
            ignore_case,
            max_per_file,
            max_results,
            json,
        ),
        DocCommands::Fetch { topic, format } => handle_fetch_command(storage, &topic, &format),
    }
}

fn build_doc<S: Storage>(output_dir: &str, storage: &S) -> Result<(), EngramError> {
    let all_fragments = storage.get_all("doc_fragment")?;

    let mut fragments: Vec<DocFragment> = Vec::new();
    for entity in &all_fragments {
        match DocFragment::from_generic(entity.clone()) {
            Ok(f) => fragments.push(f),
            Err(e) => {
                eprintln!("Warning: skipping malformed DocFragment: {}", e);
            }
        }
    }

    if fragments.is_empty() {
        println!("No DocFragments found. Write chunks first with `engram doc write`.");
        return Ok(());
    }

    let out = Path::new(output_dir);
    fs::create_dir_all(out)?;

    let src_dir = out.join("src");
    fs::create_dir_all(&src_dir)?;

    group_and_write(&fragments, out, &src_dir)?;

    println!(
        "Build complete: {} fragments across {} topics written to {}",
        fragments.len(),
        count_topics(&fragments),
        output_dir
    );

    Ok(())
}

fn group_and_write(
    fragments: &[DocFragment],
    out: &Path,
    src_dir: &Path,
) -> Result<(), EngramError> {
    let mut topics: BTreeMap<String, Vec<&DocFragment>> = BTreeMap::new();
    for f in fragments {
        topics.entry(f.topic.clone()).or_default().push(f);
    }

    let book_toml_path = out.join("book.toml");
    if !book_toml_path.exists() {
        let book_toml = "[book]\ntitle = \"Project Documentation\"\n";
        fs::write(&book_toml_path, book_toml)?;
        println!("Created book.toml");
    }

    let mut summary_lines: Vec<String> = Vec::new();
    summary_lines.push("# Summary".to_string());
    summary_lines.push(String::new());

    let mut topic_dirs: Vec<(&String, &Vec<&DocFragment>)> = topics.iter().collect();

    let overview_idx = topic_dirs.iter().position(|(t, _)| *t == "overview");
    if let Some(idx) = overview_idx {
        let overview = topic_dirs.remove(idx);
        topic_dirs.insert(0, overview);
    }

    for (topic_name, topic_fragments) in &topic_dirs {
        let topic_dir_name = slugify(topic_name);
        let topic_path = src_dir.join(&topic_dir_name);
        fs::create_dir_all(&topic_path)?;

        let display_name = title_case(topic_name);
        summary_lines.push(format!(
            "- [{}](./{}/README.md)",
            display_name, topic_dir_name
        ));

        let mut sorted: Vec<&&DocFragment> = topic_fragments.iter().collect();
        sorted.sort_by_key(|f| f.order);

        let mut readme_parts: Vec<String> = Vec::new();
        readme_parts.push(format!("# {}", display_name));
        readme_parts.push(String::new());

        for frag in sorted {
            let chunk_file_name = format!("{}.md", slugify(&frag.chunk_id));
            let chunk_path = topic_path.join(&chunk_file_name);

            let existing_written_at = read_existing_written_at(&chunk_path);

            if existing_written_at.is_none()
                || existing_written_at.unwrap() != frag.written_at.timestamp()
            {
                fs::write(&chunk_path, &frag.content)?;
            }

            summary_lines.push(format!(
                "    - [{}](./{}/{})",
                frag.title, topic_dir_name, chunk_file_name
            ));

            readme_parts.push(format!("- [{}](./{})", frag.title, chunk_file_name));
        }

        readme_parts.push(String::new());
        fs::write(topic_path.join("README.md"), readme_parts.join("\n"))?;
    }

    fs::write(src_dir.join("SUMMARY.md"), summary_lines.join("\n") + "\n")?;

    Ok(())
}

fn read_existing_written_at(path: &Path) -> Option<i64> {
    let content = fs::read_to_string(path).ok()?;
    let lines: Vec<&str> = content.lines().collect();
    if lines.len() >= 2 {
        let marker = lines[lines.len() - 2].trim();
        if marker.starts_with("<!-- written_at:") && marker.ends_with(" -->") {
            let ts_str = marker
                .trim_start_matches("<!-- written_at:")
                .trim_end_matches(" -->")
                .trim();
            return ts_str.parse::<i64>().ok();
        }
    }
    None
}

fn slugify(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c.to_ascii_lowercase()
            } else if c.is_whitespace() {
                '-'
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

fn title_case(s: &str) -> String {
    s.split(|c: char| c == '-' || c == '_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn count_topics(fragments: &[DocFragment]) -> usize {
    let mut topics = std::collections::HashSet::new();
    for f in fragments {
        topics.insert(&f.topic);
    }
    topics.len()
}

fn validate_topic(topic: &str) -> Result<(), EngramError> {
    if !VALID_TOPICS.contains(&topic) {
        return Err(EngramError::Validation(format!(
            "Invalid topic '{}'. Valid topics: {}",
            topic,
            VALID_TOPICS.join(", ")
        )));
    }
    Ok(())
}

fn topic_entity_type(topic: &str) -> &'static str {
    match topic {
        "adrs" => "adr",
        "decisions" => "reasoning",
        "tasks" => "task",
        "knowledge" => "knowledge",
        "theories" => "theory",
        "workflows" => "workflow",
        "sessions" => "session",
        "reasoning" => "reasoning",
        "standards" => "standard",
        "overview" => "overview",
        _ => "context",
    }
}

fn handle_fetch_command<S: Storage>(
    storage: &S,
    topic: &str,
    format: &str,
) -> Result<(), EngramError> {
    validate_topic(topic)?;

    if topic == "overview" {
        return fetch_overview(storage, format);
    }

    let entity_type = topic_entity_type(topic);
    let entities = storage.get_all(entity_type)?;

    if entities.is_empty() {
        return Err(EngramError::NotFound(format!(
            "No {} entities found",
            topic
        )));
    }

    match format {
        "json" => {
            let output = serde_json::json!({
                "topic": topic,
                "entity_type": entity_type,
                "count": entities.len(),
                "entities": entities,
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        "md" => {
            println!("# {}\n", topic.to_uppercase());
            for entity in &entities {
                let title = entity
                    .data
                    .get("title")
                    .and_then(|v| v.as_str())
                    .unwrap_or("(untitled)");
                println!("## {}\n", title);
                println!("**ID:** {}", entity.id);
                println!(
                    "**Created:** {}",
                    entity.timestamp.format("%Y-%m-%d %H:%M UTC")
                );
                if let Some(agent) = entity.data.get("agent").and_then(|v| v.as_str()) {
                    println!("**Agent:** {}", agent);
                }
                if let Some(content) = entity.data.get("content").and_then(|v| v.as_str()) {
                    println!("\n{}\n", content);
                }
                if let Some(description) = entity.data.get("description").and_then(|v| v.as_str()) {
                    println!("\n{}\n", description);
                }
                println!("---\n");
            }
        }
        _ => {
            return Err(EngramError::Validation(format!(
                "Invalid format '{}'. Use json or md.",
                format
            )));
        }
    }

    Ok(())
}

fn fetch_overview<S: Storage>(storage: &S, format: &str) -> Result<(), EngramError> {
    let mut summary = HashMap::new();
    for &(topic, _, _) in TOPIC_DESCRIPTIONS {
        if topic == "overview" {
            continue;
        }
        let entity_type = topic_entity_type(topic);
        match storage.count(&crate::storage::QueryFilter {
            entity_type: Some(entity_type.to_string()),
            ..Default::default()
        }) {
            Ok(count) => {
                summary.insert(topic, count);
            }
            Err(_) => {
                summary.insert(topic, 0);
            }
        }
    }

    let frag_count = storage
        .count(&crate::storage::QueryFilter {
            entity_type: Some("doc_fragment".to_string()),
            ..Default::default()
        })
        .unwrap_or(0);

    match format {
        "json" => {
            let output = serde_json::json!({
                "topic": "overview",
                "entity_counts": summary,
                "total_fragments": frag_count,
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        "md" => {
            println!("# Project Overview\n");
            println!("## Entity Counts\n");
            let mut table = create_table();
            table.set_titles(row!["Topic", "Count"]);
            for &(topic, _, _) in TOPIC_DESCRIPTIONS {
                if topic == "overview" {
                    continue;
                }
                let count = summary.get(topic).unwrap_or(&0);
                table.add_row(row![topic, count]);
            }
            table.printstd();
            println!();
            println!("**Doc fragments written:** {}", frag_count);
        }
        _ => {
            return Err(EngramError::Validation(format!(
                "Invalid format '{}'. Use json or md.",
                format
            )));
        }
    }

    Ok(())
}

fn handle_topics_command<S: Storage>(
    command: TopicsCommands,
    storage: &S,
) -> Result<(), EngramError> {
    match command {
        TopicsCommands::List => {
            let all = storage.get_all("doc_fragment")?;
            let mut chunk_counts: BTreeMap<String, usize> = BTreeMap::new();
            for entity in &all {
                if let Ok(f) = DocFragment::from_generic(entity.clone()) {
                    *chunk_counts.entry(f.topic.clone()).or_default() += 1;
                }
            }

            println!("Available documentation topics:\n");
            let mut table = create_table();
            table.set_titles(row!["Topic", "Entity Source", "Description", "Chunks"]);
            for (topic, entity_type, desc) in TOPIC_DESCRIPTIONS {
                let count = chunk_counts.get(*topic).unwrap_or(&0);
                table.add_row(row![topic, entity_type, desc, count]);
            }
            table.printstd();
        }
    }
    Ok(())
}

fn handle_chunk_command<S: Storage>(
    command: ChunkCommands,
    storage: &mut S,
) -> Result<(), EngramError> {
    match command {
        ChunkCommands::List { topic } => {
            validate_topic(&topic)?;

            let all = storage.get_all("doc_fragment")?;
            let mut chunks: Vec<DocFragment> = Vec::new();
            for entity in &all {
                if let Ok(f) = DocFragment::from_generic(entity.clone()) {
                    if f.topic == topic {
                        chunks.push(f);
                    }
                }
            }

            if chunks.is_empty() {
                println!("No chunks found for topic '{}'.", topic);
                return Ok(());
            }

            chunks.sort_by_key(|f| f.order);

            println!("Chunks for topic '{}':\n", topic);
            let mut table = create_table();
            table.set_titles(row!["Chunk ID", "Title", "Order", "Stale", "Written"]);

            for chunk in &chunks {
                table.add_row(row![
                    truncate(&chunk.chunk_id, 30),
                    truncate(&chunk.title, 40),
                    chunk.order,
                    if chunk.stale { "YES" } else { "no" },
                    chunk.written_at.format("%Y-%m-%d %H:%M")
                ]);
            }

            table.printstd();
            println!("\nTotal: {} chunks", chunks.len());
        }
        ChunkCommands::Delete { topic, chunk_id } => {
            validate_topic(&topic)?;

            let all = storage.get_all("doc_fragment")?;
            for entity in &all {
                if let Ok(f) = DocFragment::from_generic(entity.clone()) {
                    if f.topic == topic && f.chunk_id == chunk_id {
                        storage.delete(&f.id, "doc_fragment")?;
                        println!(
                            "Deleted chunk '{}' from topic '{}' (id: {}).",
                            chunk_id, topic, f.id
                        );
                        return Ok(());
                    }
                }
            }
            return Err(EngramError::NotFound(format!(
                "Chunk '{}' not found in topic '{}'.",
                chunk_id, topic
            )));
        }
    }
    Ok(())
}

fn write_chunk<S: Storage>(
    storage: &mut S,
    topic: &str,
    chunk_id: &str,
    title: &str,
    order: u32,
    stdin: bool,
    file: Option<&String>,
    content: Option<String>,
    agent: &str,
) -> Result<(), EngramError> {
    validate_topic(topic)?;

    let body = if stdin {
        let mut buf = String::new();
        std::io::stdin()
            .read_to_string(&mut buf)
            .map_err(EngramError::Io)?;
        buf.trim().to_string()
    } else if let Some(ref path) = file {
        fs::read_to_string(path).map_err(EngramError::Io)?
    } else if let Some(c) = content {
        c
    } else {
        return Err(EngramError::Validation(
            "Provide content via --stdin, --file, or --content".to_string(),
        ));
    };

    if body.is_empty() {
        return Err(EngramError::Validation(
            "Content cannot be empty".to_string(),
        ));
    }

    let now = chrono::Utc::now();
    let stamped = format!("{}\n\n<!-- written_at: {} -->", body, now.timestamp());

    let all = storage.get_all("doc_fragment")?;
    for entity in &all {
        if let Ok(mut f) = DocFragment::from_generic(entity.clone()) {
            if f.topic == topic && f.chunk_id == chunk_id {
                f.title = title.to_string();
                f.content = stamped;
                f.order = order;
                f.written_at = now;
                f.agent = agent.to_string();
                f.stale = false;
                let generic = f.to_generic();
                storage.store(&generic)?;
                println!(
                    "Updated chunk '{}' in topic '{}' (id: {})",
                    chunk_id, topic, f.id
                );
                return Ok(());
            }
        }
    }

    let fragment = DocFragment::new(
        topic.to_string(),
        chunk_id.to_string(),
        title.to_string(),
        stamped,
        order,
        agent.to_string(),
    );
    let generic = fragment.to_generic();
    storage.store(&generic)?;
    println!(
        "Created chunk '{}' in topic '{}' (id: {})",
        chunk_id, topic, fragment.id
    );

    Ok(())
}

fn status_doc<S: Storage>(output_dir: &str, storage: &S) -> Result<(), EngramError> {
    let all = storage.get_all("doc_fragment")?;
    let mut by_topic: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    for entity in &all {
        if let Ok(f) = DocFragment::from_generic(entity.clone()) {
            let entry = by_topic.entry(f.topic.clone()).or_default();
            entry.0 += 1;
            if f.stale {
                entry.1 += 1;
            }
        }
    }

    let out_path = Path::new(output_dir);
    let summary_exists = out_path.join("src").join("SUMMARY.md").exists();

    println!("Documentation Status\n");
    println!("  Output dir: {}", output_dir);
    println!(
        "  SUMMARY.md: {}",
        if summary_exists { "exists" } else { "missing" }
    );
    println!("  Total fragments: {}", all.len());
    println!();

    if all.is_empty() {
        println!("No documentation fragments found.");
        return Ok(());
    }

    let mut table = create_table();
    table.set_titles(row!["Topic", "Chunks", "Stale", "Latest Written"]);

    let mut total_chunks: usize = 0;
    let mut total_stale: usize = 0;

    for &topic in VALID_TOPICS {
        let (count, stale) = by_topic.get(topic).copied().unwrap_or((0, 0));
        total_chunks += count;
        total_stale += stale;

        let latest = format_latest_written(storage, topic, count);

        table.add_row(row![
            topic,
            count,
            if stale > 0 {
                stale.to_string()
            } else {
                "0".to_string()
            },
            latest,
        ]);
    }

    table.printstd();
    println!();
    println!("Total: {} chunks, {} stale", total_chunks, total_stale);

    let report = check_staleness(storage)?;
    print_staleness_report(&report);

    Ok(())
}

fn format_latest_written<S: Storage>(storage: &S, topic: &str, count: usize) -> String {
    if count == 0 {
        return "—".to_string();
    }
    let all = match storage.get_all("doc_fragment") {
        Ok(a) => a,
        Err(_) => return "—".to_string(),
    };
    all.iter()
        .filter_map(|e| DocFragment::from_generic(e.clone()).ok())
        .filter(|f| f.topic == topic)
        .max_by_key(|f| f.written_at)
        .map(|f| f.written_at.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_else(|| "—".to_string())
}

fn print_staleness_report(report: &StalenessReport) {
    if report.total_fragments == 0 {
        return;
    }

    println!();
    println!(
        "  Staleness check ({})",
        report.checked_at.format("%Y-%m-%d %H:%M UTC")
    );
    println!(
        "    {} total, {} fresh, {} stale",
        report.total_fragments, report.fresh_count, report.stale_count
    );

    if report.has_stale() {
        println!();
        println!("    Stale chunks:");
        for sc in &report.stale_chunks {
            println!("      {}/{} — outdated by:", sc.topic, sc.chunk_id);
            for src in &sc.outdated_by {
                println!(
                    "        {} ({}) updated {}",
                    src.entity_id,
                    src.entity_type,
                    src.source_timestamp.format("%Y-%m-%d %H:%M UTC")
                );
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefMatch {
    pub path: PathBuf,
    pub line_number: usize,
    pub snippet: String,
    pub language: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefsResult {
    pub query: String,
    pub project_type: ProjectType,
    pub matches: Vec<RefMatch>,
    pub files_searched: usize,
    pub total_matches: usize,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ProjectType {
    Code,
    NonCode,
}

const CODE_INDICATORS: &[&str] = &[
    "src/",
    "Cargo.toml",
    "package.json",
    "pyproject.toml",
    "go.mod",
    "build.gradle",
    "pom.xml",
    "Gemfile",
    "Makefile",
    "CMakeLists.txt",
    "lib/",
    "include/",
    "tsconfig.json",
    ".csproj",
    ".sln",
    "pubspec.yaml",
    "mix.exs",
];

const CODE_EXTENSIONS: &[&str] = &[
    "rs", "py", "js", "ts", "go", "java", "c", "cpp", "h", "hpp", "cs", "rb", "php", "swift", "kt",
    "scala", "r", "lua", "vim", "el", "clj", "cljs", "hs", "ml", "ex", "exs", "erl", "hrl", "zig",
    "nim", "dart", "jsx", "tsx", "vue", "svelte", "mjs", "cjs", "sh", "bash", "zsh", "fish",
];

const BINARY_EXTENSIONS: &[&str] = &[
    "png", "jpg", "jpeg", "gif", "bmp", "ico", "svg", "webp", "tiff", "tif", "mp3", "mp4", "avi",
    "mov", "mkv", "wav", "flac", "ogg", "zip", "tar", "gz", "bz2", "xz", "7z", "rar", "pdf", "doc",
    "docx", "xls", "xlsx", "ppt", "pptx", "exe", "dll", "so", "dylib", "a", "o", "obj", "wasm",
    "bin", "dat", "db", "sqlite", "sqlite3", "ttf", "otf", "woff", "woff2", "eot", "lock", "snap",
];

const IGNORE_DIRS: &[&str] = &[
    ".git",
    ".engram",
    "node_modules",
    "target",
    "build",
    "dist",
    "__pycache__",
    ".next",
    ".nuxt",
    "vendor",
    ".venv",
    "venv",
    ".tox",
    ".mypy_cache",
    ".pytest_cache",
    "coverage",
    ".cache",
    ".gradle",
    ".idea",
    ".vscode",
];

fn detect_project_type(root: &Path) -> ProjectType {
    for indicator in CODE_INDICATORS {
        if root.join(indicator).exists() {
            return ProjectType::Code;
        }
    }
    ProjectType::NonCode
}

fn load_gitignore(root: &Path) -> HashSet<String> {
    let mut patterns = HashSet::new();
    let gitignore_path = root.join(".gitignore");
    if let Ok(content) = fs::read_to_string(&gitignore_path) {
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            if let Some(stripped) = trimmed.strip_prefix('!') {
                patterns.remove(stripped);
            } else {
                patterns.insert(trimmed.to_string());
            }
        }
    }
    patterns
}

fn is_ignored(path: &Path, root: &Path, gitignore_patterns: &HashSet<String>) -> bool {
    let relative = match path.strip_prefix(root) {
        Ok(r) => r,
        Err(_) => return false,
    };

    let path_str = relative.to_string_lossy();

    for component in relative.components() {
        let comp_str = component.as_os_str().to_string_lossy();
        if IGNORE_DIRS.contains(&comp_str.as_ref()) {
            return true;
        }
    }

    for pattern in gitignore_patterns {
        if pattern.ends_with('/') {
            let dir_name = pattern.trim_end_matches('/');
            if path_str.contains(dir_name) {
                return true;
            }
        } else {
            let base_name = relative
                .file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_default();
            if base_name == *pattern || path_str.contains(pattern.as_str()) {
                return true;
            }
        }
    }

    false
}

fn is_binary_extension(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| BINARY_EXTENSIONS.contains(&e.to_lowercase().as_str()))
        .unwrap_or(false)
}

fn detect_language(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|e| e.to_str())
        .and_then(|e| match e.to_lowercase().as_str() {
            "rs" => Some("rust"),
            "py" => Some("python"),
            "js" | "mjs" | "cjs" => Some("javascript"),
            "ts" => Some("typescript"),
            "tsx" | "jsx" => Some("typescript"),
            "go" => Some("go"),
            "java" => Some("java"),
            "c" | "h" => Some("c"),
            "cpp" | "hpp" | "cc" | "cxx" => Some("cpp"),
            "rb" => Some("ruby"),
            "swift" => Some("swift"),
            "kt" | "kts" => Some("kotlin"),
            "scala" => Some("scala"),
            "hs" => Some("haskell"),
            "ex" | "exs" => Some("elixir"),
            "zig" => Some("zig"),
            "nim" => Some("nim"),
            "dart" => Some("dart"),
            "vue" => Some("vue"),
            "svelte" => Some("svelte"),
            "sh" | "bash" | "zsh" | "fish" => Some("shell"),
            _ => None,
        })
        .map(|s| s.to_string())
}

fn is_source_file(path: &Path, project_type: ProjectType) -> bool {
    match project_type {
        ProjectType::Code => {
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            let is_code_ext = CODE_EXTENSIONS.contains(&ext);
            let is_text_file = !is_binary_extension(path);
            let is_common_text = matches!(
                ext,
                "toml"
                    | "yaml"
                    | "yml"
                    | "json"
                    | "md"
                    | "txt"
                    | "cfg"
                    | "ini"
                    | "xml"
                    | "html"
                    | "css"
                    | "scss"
                    | "less"
            );
            is_code_ext || is_common_text || (is_text_file && !ext.is_empty())
        }
        ProjectType::NonCode => !is_binary_extension(path),
    }
}

fn extract_code_context(lines: &[&str], match_line: usize, language: &str) -> String {
    let (start, end) = match language {
        "rust" => (
            match_line.saturating_sub(3),
            (match_line + 4).min(lines.len()),
        ),
        _ => (
            match_line.saturating_sub(2),
            (match_line + 3).min(lines.len()),
        ),
    };
    lines[start..end].join("\n")
}

fn file_priority(path: &Path, project_type: ProjectType) -> i32 {
    match project_type {
        ProjectType::Code => {
            let path_str = path.to_string_lossy();
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

            if CODE_EXTENSIONS.contains(&ext) {
                if path_str.contains("/src/") {
                    30
                } else if path_str.contains("/lib/") {
                    25
                } else {
                    20
                }
            } else {
                10
            }
        }
        ProjectType::NonCode => 10,
    }
}

pub fn search_refs(
    query: &str,
    dir: Option<&str>,
    is_regex: bool,
    ignore_case: bool,
    max_per_file: usize,
    max_results: usize,
) -> Result<RefsResult, EngramError> {
    let root = match dir {
        Some(d) => PathBuf::from(d),
        None => std::env::current_dir().map_err(EngramError::Io)?,
    };

    if !root.exists() {
        return Err(EngramError::NotFound(format!(
            "Directory not found: {}",
            root.display()
        )));
    }

    let project_type = detect_project_type(&root);
    let gitignore_patterns = load_gitignore(&root);

    let pattern = if is_regex {
        query.to_string()
    } else {
        regex::escape(query)
    };

    let search_re = if ignore_case {
        Regex::new(&format!("(?i){}", pattern))
            .map_err(|e| EngramError::Validation(format!("Invalid regex pattern: {}", e)))?
    } else {
        Regex::new(&pattern)
            .map_err(|e| EngramError::Validation(format!("Invalid regex pattern: {}", e)))?
    };

    let mut all_matches: Vec<(i32, RefMatch)> = Vec::new();
    let mut files_searched: usize = 0;

    for entry in walkdir::WalkDir::new(&root).into_iter().filter_entry(|e| {
        if e.path() == root {
            return true;
        }
        !is_ignored(e.path(), &root, &gitignore_patterns)
    }) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();
        if !is_source_file(path, project_type) {
            continue;
        }

        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        files_searched += 1;

        let lines: Vec<&str> = content.lines().collect();
        let mut file_match_count = 0;

        for (i, line) in lines.iter().enumerate() {
            if search_re.is_match(line) {
                let language = detect_language(path);
                let priority = file_priority(path, project_type);

                let context_snippet = if let Some(ref lang) = language {
                    if lang == "rust" || lang == "python" || lang == "go" || lang == "java" {
                        extract_code_context(&lines, i, lang)
                    } else {
                        line.to_string()
                    }
                } else {
                    line.to_string()
                };

                all_matches.push((
                    priority,
                    RefMatch {
                        path: path.to_path_buf(),
                        line_number: i + 1,
                        snippet: context_snippet,
                        language,
                    },
                ));
                file_match_count += 1;

                if file_match_count >= max_per_file {
                    break;
                }
            }

            if all_matches.len() >= max_results * 2 {
                break;
            }
        }

        if all_matches.len() >= max_results * 2 {
            break;
        }
    }

    all_matches.sort_by(|a, b| b.0.cmp(&a.0));
    all_matches.truncate(max_results);

    let matches: Vec<RefMatch> = all_matches.into_iter().map(|(_, m)| m).collect();
    let total_matches = matches.len();

    Ok(RefsResult {
        query: query.to_string(),
        project_type,
        matches,
        files_searched,
        total_matches,
    })
}

fn refs_command(
    query: &str,
    dir: Option<String>,
    is_regex: bool,
    ignore_case: bool,
    max_per_file: usize,
    max_results: usize,
    as_json: bool,
) -> Result<(), EngramError> {
    let result = search_refs(
        query,
        dir.as_deref(),
        is_regex,
        ignore_case,
        max_per_file,
        max_results,
    )?;

    if as_json {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        let type_label = match result.project_type {
            ProjectType::Code => "code",
            ProjectType::NonCode => "non-code",
        };
        println!(
            "Refs for \"{}\" (project: {}, files searched: {}, matches: {})",
            result.query, type_label, result.files_searched, result.total_matches
        );
        println!();

        if result.matches.is_empty() {
            println!("No matches found.");
            return Ok(());
        }

        let mut table = create_table();
        table.set_titles(row!["Line", "File", "Language", "Snippet"]);

        for m in &result.matches {
            let lang = m.language.as_deref().unwrap_or("-");
            let display_path = m
                .path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| m.path.display().to_string());
            let snippet = truncate(&m.snippet, 80);
            table.add_row(row![m.line_number, display_path, lang, snippet]);
        }

        table.printstd();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::MemoryStorage;
    use tempfile::TempDir;

    fn setup_storage() -> MemoryStorage {
        MemoryStorage::new("default")
    }

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("ADRs"), "adrs");
        assert_eq!(slugify("my-topic"), "my-topic");
        assert_eq!(slugify("Some Topic Name"), "some-topic-name");
        assert_eq!(slugify("knowledge"), "knowledge");
    }

    #[test]
    fn test_title_case() {
        assert_eq!(title_case("adrs"), "Adrs");
        assert_eq!(title_case("my-topic"), "My Topic");
        assert_eq!(title_case("overview"), "Overview");
    }

    #[test]
    fn test_build_empty_storage() {
        let storage = setup_storage();
        let tmp = std::env::temp_dir().join("engram_test_build_empty");
        let _ = fs::remove_dir_all(&tmp);
        let result = build_doc(tmp.to_str().unwrap(), &storage);
        assert!(result.is_ok());
        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn test_build_with_fragments() {
        let mut storage = setup_storage();
        let tmp = std::env::temp_dir().join("engram_test_build_fragments");
        let _ = fs::remove_dir_all(&tmp);

        let now = chrono::Utc::now();
        let stamped = format!("Content A\n\n<!-- written_at: {} -->", now.timestamp());
        let f1 = DocFragment::new(
            "adrs".to_string(),
            "database".to_string(),
            "Database Choice".to_string(),
            stamped,
            1,
            "agent".to_string(),
        );
        storage.store(&f1.to_generic()).unwrap();

        let stamped2 = format!("Content B\n\n<!-- written_at: {} -->", now.timestamp());
        let f2 = DocFragment::new(
            "knowledge".to_string(),
            "auth".to_string(),
            "Auth Flow".to_string(),
            stamped2,
            1,
            "agent".to_string(),
        );
        storage.store(&f2.to_generic()).unwrap();

        build_doc(tmp.to_str().unwrap(), &storage).unwrap();

        assert!(tmp.join("book.toml").exists());
        assert!(tmp.join("src").join("SUMMARY.md").exists());
        assert!(tmp.join("src").join("adrs").join("database.md").exists());
        assert!(tmp.join("src").join("knowledge").join("auth.md").exists());

        let summary = fs::read_to_string(tmp.join("src").join("SUMMARY.md")).unwrap();
        assert!(summary.contains("Adrs"));
        assert!(summary.contains("Knowledge"));

        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn test_build_overview_first() {
        let mut storage = setup_storage();
        let tmp = std::env::temp_dir().join("engram_test_build_overview");
        let _ = fs::remove_dir_all(&tmp);

        let now = chrono::Utc::now();
        let stamped = format!(
            "Overview content\n\n<!-- written_at: {} -->",
            now.timestamp()
        );
        let f1 = DocFragment::new(
            "overview".to_string(),
            "summary".to_string(),
            "Summary".to_string(),
            stamped,
            0,
            "agent".to_string(),
        );
        storage.store(&f1.to_generic()).unwrap();

        let stamped2 = format!("ADR content\n\n<!-- written_at: {} -->", now.timestamp());
        let f2 = DocFragment::new(
            "adrs".to_string(),
            "db".to_string(),
            "DB Choice".to_string(),
            stamped2,
            1,
            "agent".to_string(),
        );
        storage.store(&f2.to_generic()).unwrap();

        build_doc(tmp.to_str().unwrap(), &storage).unwrap();

        let summary = fs::read_to_string(tmp.join("src").join("SUMMARY.md")).unwrap();
        let overview_pos = summary.find("Overview").unwrap();
        let adrs_pos = summary.find("Adrs").unwrap();
        assert!(overview_pos < adrs_pos);

        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn test_write_and_delete_chunk() {
        let mut storage = setup_storage();
        write_chunk(
            &mut storage,
            "adrs",
            "test-chunk",
            "Test Title",
            1,
            false,
            None,
            Some("Test body".to_string()),
            "default",
        )
        .unwrap();

        let all = storage.get_all("doc_fragment").unwrap();
        assert_eq!(all.len(), 1);

        handle_chunk_command(
            ChunkCommands::Delete {
                topic: "adrs".to_string(),
                chunk_id: "test-chunk".to_string(),
            },
            &mut storage,
        )
        .unwrap();

        let all = storage.get_all("doc_fragment").unwrap();
        assert_eq!(all.len(), 0);
    }

    #[test]
    fn test_incremental_build_skips_unchanged() {
        let mut storage = setup_storage();
        let tmp = std::env::temp_dir().join("engram_test_build_incremental");
        let _ = fs::remove_dir_all(&tmp);

        let now = chrono::Utc::now();
        let ts = now.timestamp();
        let stamped = format!("Original\n\n<!-- written_at: {} -->", ts);

        let mut f = DocFragment::new(
            "adrs".to_string(),
            "db".to_string(),
            "DB".to_string(),
            stamped.clone(),
            1,
            "agent".to_string(),
        );
        f.written_at = now;
        storage.store(&f.to_generic()).unwrap();

        build_doc(tmp.to_str().unwrap(), &storage).unwrap();

        let chunk_path = tmp.join("src").join("adrs").join("db.md");
        let content_before = fs::read_to_string(&chunk_path).unwrap();
        assert_eq!(content_before, stamped);

        build_doc(tmp.to_str().unwrap(), &storage).unwrap();
        let content_after = fs::read_to_string(&chunk_path).unwrap();
        assert_eq!(content_after, stamped);

        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn test_topics_list() {
        let mut storage = setup_storage();
        let now = chrono::Utc::now();
        let stamped = format!("Content\n\n<!-- written_at: {} -->", now.timestamp());
        let f = DocFragment::new(
            "adrs".to_string(),
            "c1".to_string(),
            "C1".to_string(),
            stamped,
            1,
            "agent".to_string(),
        );
        storage.store(&f.to_generic()).unwrap();

        handle_topics_command(TopicsCommands::List, &storage).unwrap();
    }

    fn create_code_project(dir: &Path) {
        fs::create_dir_all(dir.join("src")).unwrap();
        fs::write(
            dir.join("Cargo.toml"),
            "[package]\nname = \"test\"\nversion = \"0.1.0\"\n",
        )
        .unwrap();
        fs::write(
            dir.join("src/main.rs"),
            "fn main() {\n    println!(\"hello\");\n    let x = search_refs();\n}\n",
        )
        .unwrap();
        fs::write(
            dir.join("README.md"),
            "# Test Project\nThis uses search_refs.\n",
        )
        .unwrap();
    }

    fn create_noncode_project(dir: &Path) {
        fs::write(
            dir.join("notes.md"),
            "# Notes\nSome important reference here.\nAnother line with reference.\n",
        )
        .unwrap();
        fs::write(
            dir.join("todo.txt"),
            "- Write documentation\n- Fix reference bug\n",
        )
        .unwrap();
    }

    #[test]
    fn test_detect_project_type_code() {
        let tmp = TempDir::new().unwrap();
        create_code_project(tmp.path());
        assert_eq!(detect_project_type(tmp.path()), ProjectType::Code);
    }

    #[test]
    fn test_detect_project_type_noncode() {
        let tmp = TempDir::new().unwrap();
        create_noncode_project(tmp.path());
        assert_eq!(detect_project_type(tmp.path()), ProjectType::NonCode);
    }

    #[test]
    fn test_search_refs_literal() {
        let tmp = TempDir::new().unwrap();
        create_code_project(tmp.path());
        let result = search_refs("search_refs", tmp.path().to_str(), false, false, 3, 50).unwrap();
        assert_eq!(result.project_type, ProjectType::Code);
        assert!(result.total_matches >= 1);
        assert!(result.files_searched >= 2);
    }

    #[test]
    fn test_search_refs_regex() {
        let tmp = TempDir::new().unwrap();
        create_code_project(tmp.path());
        let result = search_refs(r"println!\(", tmp.path().to_str(), true, false, 3, 50).unwrap();
        assert!(result.total_matches >= 1);
    }

    #[test]
    fn test_search_refs_noncode() {
        let tmp = TempDir::new().unwrap();
        create_noncode_project(tmp.path());
        let result = search_refs("reference", tmp.path().to_str(), false, true, 3, 50).unwrap();
        assert_eq!(result.project_type, ProjectType::NonCode);
        assert!(result.total_matches >= 2);
    }

    #[test]
    fn test_search_refs_max_per_file() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("src")).unwrap();
        fs::write(tmp.path().join("Cargo.toml"), "").unwrap();
        let content = "match here\nmatch here\nmatch here\nmatch here\nmatch here\n";
        fs::write(tmp.path().join("src/main.rs"), content).unwrap();

        let result = search_refs("match", tmp.path().to_str(), false, false, 2, 50).unwrap();
        let rs_matches: Vec<_> = result
            .matches
            .iter()
            .filter(|m| m.path.file_name().map(|n| n == "main.rs").unwrap_or(false))
            .collect();
        assert!(rs_matches.len() <= 2);
    }

    #[test]
    fn test_search_refs_max_results() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("src")).unwrap();
        fs::write(tmp.path().join("Cargo.toml"), "").unwrap();
        let content = "x\n".repeat(100);
        fs::write(tmp.path().join("src/main.rs"), &content).unwrap();
        fs::write(tmp.path().join("README.md"), &content).unwrap();

        let result = search_refs("x", tmp.path().to_str(), false, false, 100, 5).unwrap();
        assert!(result.total_matches <= 5);
    }

    #[test]
    fn test_search_refs_invalid_regex() {
        let tmp = TempDir::new().unwrap();
        let result = search_refs("(unclosed", tmp.path().to_str(), true, false, 3, 50);
        assert!(result.is_err());
    }

    #[test]
    fn test_search_refs_nonexistent_dir() {
        let result = search_refs("test", Some("/nonexistent/path"), false, false, 3, 50);
        assert!(matches!(result, Err(EngramError::NotFound(_))));
    }

    #[test]
    fn test_detect_language() {
        assert_eq!(
            detect_language(Path::new("main.rs")),
            Some("rust".to_string())
        );
        assert_eq!(
            detect_language(Path::new("app.py")),
            Some("python".to_string())
        );
        assert_eq!(
            detect_language(Path::new("index.ts")),
            Some("typescript".to_string())
        );
        assert_eq!(detect_language(Path::new("go.mod")), None);
    }

    #[test]
    fn test_is_binary_extension() {
        assert!(is_binary_extension(Path::new("image.png")));
        assert!(is_binary_extension(Path::new("archive.tar.gz")));
        assert!(!is_binary_extension(Path::new("main.rs")));
        assert!(!is_binary_extension(Path::new("README.md")));
    }

    #[test]
    fn test_is_ignored() {
        let root = Path::new("/project");
        assert!(is_ignored(
            Path::new("/project/target/debug/main.rs"),
            root,
            &HashSet::new()
        ));
        assert!(is_ignored(
            Path::new("/project/.git/config"),
            root,
            &HashSet::new()
        ));
        assert!(!is_ignored(
            Path::new("/project/src/main.rs"),
            root,
            &HashSet::new()
        ));
    }

    #[test]
    fn test_refs_result_serialization() {
        let result = RefsResult {
            query: "test".to_string(),
            project_type: ProjectType::Code,
            matches: vec![RefMatch {
                path: PathBuf::from("src/main.rs"),
                line_number: 42,
                snippet: "let test = 1;".to_string(),
                language: Some("rust".to_string()),
            }],
            files_searched: 10,
            total_matches: 1,
        };
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"project_type\":\"code\""));
    }

    #[test]
    fn test_validate_topic_valid() {
        assert!(validate_topic("adrs").is_ok());
        assert!(validate_topic("knowledge").is_ok());
        assert!(validate_topic("overview").is_ok());
    }

    #[test]
    fn test_validate_topic_invalid() {
        assert!(validate_topic("nonexistent").is_err());
        assert!(validate_topic("").is_err());
    }

    #[test]
    fn test_topic_entity_type() {
        assert_eq!(topic_entity_type("adrs"), "adr");
        assert_eq!(topic_entity_type("tasks"), "task");
        assert_eq!(topic_entity_type("theories"), "theory");
        assert_eq!(topic_entity_type("decisions"), "reasoning");
        assert_eq!(topic_entity_type("overview"), "overview");
    }

    #[test]
    fn test_fetch_invalid_topic() {
        let storage = setup_storage();
        let result = handle_fetch_command(&storage, "invalid", "json");
        assert!(result.is_err());
    }

    #[test]
    fn test_fetch_overview() {
        let storage = setup_storage();
        handle_fetch_command(&storage, "overview", "json").unwrap();
        handle_fetch_command(&storage, "overview", "md").unwrap();
    }

    #[test]
    fn test_fetch_invalid_format() {
        let storage = setup_storage();
        let result = handle_fetch_command(&storage, "adrs", "yaml");
        assert!(result.is_err());
    }

    #[test]
    fn test_fetch_empty_topic() {
        let storage = setup_storage();
        let result = handle_fetch_command(&storage, "adrs", "json");
        assert!(result.is_err());
    }

    #[test]
    fn test_write_chunk_invalid_topic() {
        let mut storage = setup_storage();
        let result = write_chunk(
            &mut storage,
            "bogus",
            "chunk",
            "Title",
            0,
            false,
            None,
            Some("body".to_string()),
            "default",
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_write_chunk_empty_content() {
        let mut storage = setup_storage();
        let result = write_chunk(
            &mut storage,
            "adrs",
            "chunk",
            "Title",
            0,
            false,
            None,
            Some("".to_string()),
            "default",
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_write_chunk_no_content() {
        let mut storage = setup_storage();
        let result = write_chunk(
            &mut storage,
            "adrs",
            "chunk",
            "Title",
            0,
            false,
            None,
            None,
            "default",
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_chunk_list_invalid_topic() {
        let mut storage = setup_storage();
        let result = handle_chunk_command(
            ChunkCommands::List {
                topic: "bogus".to_string(),
            },
            &mut storage,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_status_empty() {
        let storage = setup_storage();
        status_doc("docs", &storage).unwrap();
    }
}
