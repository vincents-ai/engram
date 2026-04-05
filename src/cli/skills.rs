use crate::error::EngramError;
use clap::Subcommand;
use std::path::PathBuf;

/// Skills commands
#[derive(Debug, Subcommand)]
pub enum SkillsCommands {
    /// Install skills
    Setup {
        /// Overwrite skills that already exist on disk
        #[arg(long, short)]
        force: bool,
        /// Install skills to this directory instead of the default
        #[arg(long, short)]
        dir: Option<String>,
        /// Install to a well-known tool directory: opencode, claude, goose
        #[arg(long, short)]
        tool: Option<String>,
        /// Source skills directory (default: ./skills)
        #[arg(long, short)]
        source: Option<String>,
    },
    /// List all available skills
    List {
        /// Format output (short, full)
        #[arg(long, short, default_value = "short")]
        format: String,

        /// Verbose output
        #[arg(long, short)]
        verbose: bool,
    },
    /// Show skill details
    Show {
        /// Skill name or path
        #[arg(help = "Skill name or path")]
        name: String,
    },
}

/// Get skills path from environment or default
pub fn get_skills_path(config_dir: Option<PathBuf>) -> PathBuf {
    if let Some(dir) = config_dir {
        return dir.join("engram/skills");
    }

    if let Ok(path_str) = std::env::var("ENGRAM_SKILLS_PATH") {
        let path = PathBuf::from(&path_str);
        if path.exists() {
            return path;
        }
    }

    let cwd_skills = PathBuf::from(".engram/skills");
    if cwd_skills.exists() {
        return cwd_skills;
    }

    let local_skills = PathBuf::from("./engram/skills");
    if local_skills.exists() {
        return local_skills;
    }

    PathBuf::from(".engram/skills")
}

/// Resolve the source skills directory for scanning.
/// Checks ENGRAM_SKILLS_SOURCE env var, then --source flag, then defaults to ./skills
pub fn resolve_skills_source(source: Option<&str>) -> PathBuf {
    if let Ok(env_path) = std::env::var("ENGRAM_SKILLS_SOURCE") {
        let p = PathBuf::from(&env_path);
        if p.exists() {
            return p;
        }
    }

    if let Some(s) = source {
        return PathBuf::from(s);
    }

    PathBuf::from("./skills")
}

/// Scan a skills source directory and return (skill_name, content) pairs.
///
/// Convention:
/// - Walks `source_dir` recursively for `.md` files (skips `README.md`)
/// - If filename is `skill.md` or `SKILL.md`, skill name = parent directory name
/// - Otherwise, skill name = filename without `.md` extension
/// - If skill name doesn't start with `engram-` or `screenplay-`, prefix with `engram-`
pub fn scan_skills_from_dir(source_dir: &PathBuf) -> Result<Vec<(String, String)>, EngramError> {
    if !source_dir.exists() {
        return Err(EngramError::Validation(format!(
            "Skills source directory not found: {:?}\n  Set --source or ENGRAM_SKILLS_SOURCE, or run from the engram repo root",
            source_dir
        )));
    }

    let mut skills = Vec::new();

    fn walk(dir: &PathBuf, skills: &mut Vec<(String, String)>) -> Result<(), EngramError> {
        let entries = std::fs::read_dir(dir).map_err(EngramError::Io)?;
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, skills)?;
            } else if path.extension().map(|e| e == "md").unwrap_or(false) {
                let file_name = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                if file_name == "README.md" {
                    continue;
                }

                let stem = path
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                let dir_name = path
                    .parent()
                    .and_then(|p| p.file_name())
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                let skill_name = if stem.starts_with("engram-") || stem.starts_with("screenplay-") {
                    stem
                } else if file_name.eq_ignore_ascii_case("skill.md") {
                    format!("engram-{}", dir_name)
                } else if dir_name == "screenplay" {
                    format!("screenplay-{}", stem)
                } else {
                    format!("engram-{}", stem)
                };

                let content = std::fs::read_to_string(&path).map_err(EngramError::Io)?;
                skills.push((skill_name, content));
            }
        }
        Ok(())
    }

    walk(source_dir, &mut skills)?;
    skills.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(skills)
}

use crate::cli::utils::{create_table, truncate};
use prettytable::row;

/// List all skills in skills directory
pub fn list_skills(
    writer: &mut dyn std::io::Write,
    format: &str,
    verbose: bool,
    config_dir: Option<PathBuf>,
) -> Result<(), std::io::Error> {
    let skills_path = get_skills_path(config_dir);
    let abs_path = std::fs::canonicalize(&skills_path).unwrap_or_else(|_| skills_path.clone());

    if verbose {
        writeln!(writer, "🔎 Skills configuration:")?;
        writeln!(writer, "  • Target path: {:?}", skills_path)?;
        writeln!(writer, "  • Absolute path: {:?}", abs_path)?;
        if let Ok(env_path) = std::env::var("ENGRAM_SKILLS_PATH") {
            writeln!(writer, "  • ENGRAM_SKILLS_PATH: {}", env_path)?;
        } else {
            writeln!(writer, "  • ENGRAM_SKILLS_PATH: (not set)")?;
        }
    }

    if !skills_path.exists() {
        if verbose {
            writeln!(writer, "❌ Directory does not exist")?;
        }
        writeln!(writer, "Skills directory not found at: {:?}", abs_path)?;
        writeln!(
            writer,
            "Current working directory: {:?}",
            std::env::current_dir().unwrap_or_default()
        )?;
        writeln!(writer, "\nTo fix this:")?;
        writeln!(
            writer,
            "1. Run 'engram setup skills' to install default skills"
        )?;
        writeln!(
            writer,
            "2. Or set ENGRAM_SKILLS_PATH to your skills directory"
        )?;
        return Ok(());
    }

    let entries = std::fs::read_dir(&skills_path)?;
    let mut table = create_table();
    let mut found_any = false;

    match format {
        "short" | "s" => {
            table.set_titles(row!["Skill Name"]);
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    table.add_row(row![entry.file_name().to_string_lossy()]);
                    found_any = true;
                } else if verbose {
                    writeln!(
                        writer,
                        "  (Skipping non-directory: {})",
                        entry.file_name().to_string_lossy()
                    )?;
                }
            }
            if found_any {
                table.print(writer)?;
            } else if verbose {
                writeln!(writer, "No skills found in {:?}", skills_path)?;
            }
        }
        "full" | "f" => {
            table.set_titles(row!["Skill Name", "Description"]);
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    let file_name = entry.file_name();
                    let name = file_name.to_string_lossy();
                    let skill_file = entry.path().join("skill.md");
                    let description = if skill_file.exists() {
                        let content = std::fs::read_to_string(&skill_file)?;
                        content.lines().next().unwrap_or("").to_string()
                    } else {
                        let skill_file_upper = entry.path().join("SKILL.md");
                        if skill_file_upper.exists() {
                            let content = std::fs::read_to_string(&skill_file_upper)?;
                            content.lines().next().unwrap_or("").to_string()
                        } else {
                            "(no description)".to_string()
                        }
                    };

                    table.add_row(row![truncate(&name, 30), truncate(&description, 50)]);
                    found_any = true;
                } else if verbose {
                    writeln!(
                        writer,
                        "  (Skipping non-directory: {})",
                        entry.file_name().to_string_lossy()
                    )?;
                }
            }
            if found_any {
                table.print(writer)?;
            } else if verbose {
                writeln!(writer, "No skills found in {:?}", skills_path)?;
            }
        }
        _ => {
            writeln!(writer, "Unknown format: {}. Use 'short' or 'full'.", format)?;
        }
    }

    Ok(())
}

/// Show a specific skill
pub fn show_skill(
    writer: &mut dyn std::io::Write,
    name: &str,
    config_dir: Option<PathBuf>,
) -> Result<(), std::io::Error> {
    let skills_path = get_skills_path(config_dir);

    let skill_path = skills_path.join(name);

    let actual_path = if skill_path.exists() && skill_path.is_dir() {
        skill_path
    } else {
        let name_lower = name.to_lowercase();
        if !skills_path.exists() {
            writeln!(writer, "Skill not found: {}", name)?;
            writeln!(writer, "Searched in: {:?}", skills_path)?;
            return Ok(());
        }

        let entries = std::fs::read_dir(&skills_path)?;
        let found_path = entries
            .flatten()
            .filter(|e| e.path().is_dir())
            .find(|e| {
                let file_name = e.file_name();
                file_name.to_string_lossy().to_lowercase() == name_lower
            })
            .map(|e| e.path());

        if let Some(path) = found_path {
            path
        } else {
            let local_path = PathBuf::from(name);
            if local_path.exists() && local_path.is_dir() {
                local_path
            } else {
                skills_path.join(name)
            }
        }
    };

    if !actual_path.exists() {
        writeln!(writer, "Skill not found: {}", name)?;
        writeln!(writer, "Searched in: {:?}", skills_path)?;
        return Ok(());
    }

    writeln!(
        writer,
        "\nSkill: {}",
        actual_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
    )?;
    writeln!(writer, "======")?;

    let entries = std::fs::read_dir(&actual_path)?;
    for entry in entries.flatten() {
        let file_name = entry.file_name();
        let file_type = if entry.path().is_dir() {
            "[DIR]"
        } else {
            "[FILE]"
        };
        writeln!(writer, "  {} {}", file_type, file_name.to_string_lossy())?;

        if entry.path().is_file() {
            let content = std::fs::read_to_string(entry.path())?;
            let preview = String::from_iter(content.lines().take(5));
            if preview.len() > 100 {
                writeln!(writer, "       {}", &preview[..100])?;
            } else {
                writeln!(writer, "       {}", preview)?;
            }
        }
    }

    Ok(())
}

/// Resolve the target skills directory from explicit --dir, --tool shorthand, or default.
/// Returns an error if --tool is given an unrecognised value.
pub fn resolve_skills_dir(dir: Option<&str>, tool: Option<&str>) -> Result<PathBuf, EngramError> {
    use std::env;

    let home = env::var("HOME")
        .map(PathBuf::from)
        .map_err(|_| EngramError::Validation("HOME environment variable not set".to_string()))?;

    if let Some(explicit) = dir {
        let expanded = if explicit.starts_with("~/") {
            home.join(&explicit[2..])
        } else if explicit == "~" {
            home.clone()
        } else {
            PathBuf::from(explicit)
        };
        return Ok(expanded);
    }

    match tool {
        Some("opencode") | None => Ok(home.join(".config").join("opencode").join("skills")),
        Some("claude") => Ok(home.join(".claude").join("skills")),
        Some("goose") => Ok(home.join(".config").join("goose").join("skills")),
        Some(other) => Err(EngramError::Validation(format!(
            "Unknown tool '{}'. Supported values: opencode, claude, goose. \
             Use --dir to specify a custom path.",
            other
        ))),
    }
}

/// Produce a compact unified diff between `old` and `new` text.
/// Returns an empty string if the contents are identical.
fn unified_diff(skill_name: &str, old: &str, new: &str) -> String {
    use similar::{ChangeTag, TextDiff};
    if old == new {
        return String::new();
    }
    let diff = TextDiff::from_lines::<str>(old, new);
    let mut out = String::new();
    let mut header_written = false;
    for (idx, group) in diff.grouped_ops(3).iter().enumerate() {
        if idx > 0 {
            out.push_str("---\n");
        }
        for op in group {
            for change in diff.iter_changes(op) {
                let prefix = match change.tag() {
                    ChangeTag::Delete => "-",
                    ChangeTag::Insert => "+",
                    ChangeTag::Equal => " ",
                };
                if !header_written && change.tag() != ChangeTag::Equal {
                    out.push_str(&format!("--- {}/SKILL.md (on disk)\n", skill_name));
                    out.push_str(&format!("+++ {}/SKILL.md (source)\n", skill_name));
                    header_written = true;
                }
                out.push_str(prefix);
                out.push_str(change.value());
                if change.missing_newline() {
                    out.push('\n');
                }
            }
        }
    }
    out
}

/// Install scanned skills to a target directory.
/// Returns (installed, updated, skipped) counts.
pub fn install_skills(
    writer: &mut dyn std::io::Write,
    skills: &[(String, String)],
    skills_dir: &PathBuf,
    force: bool,
) -> Result<(usize, usize, usize), EngramError> {
    std::fs::create_dir_all(skills_dir).map_err(EngramError::Io)?;

    let mut installed_count = 0;
    let mut skipped_count = 0;
    let mut updated_count = 0;

    for (skill_name, skill_content) in skills {
        let skill_dir = skills_dir.join(skill_name);
        let skill_file = skill_dir.join("SKILL.md");

        if skill_dir.exists() {
            let on_disk = std::fs::read_to_string(&skill_file).unwrap_or_default();

            if on_disk == *skill_content {
                writeln!(writer, "✅ Skill '{}' is up to date", skill_name)
                    .map_err(EngramError::Io)?;
                skipped_count += 1;
                continue;
            }

            let diff = unified_diff(skill_name, &on_disk, skill_content);
            writeln!(writer, "📝 Skill '{}' differs from source:", skill_name)
                .map_err(EngramError::Io)?;
            writeln!(writer, "{}", diff).map_err(EngramError::Io)?;

            if force {
                std::fs::write(&skill_file, skill_content).map_err(EngramError::Io)?;
                writeln!(writer, "🔄 Updated skill: {}", skill_name).map_err(EngramError::Io)?;
                updated_count += 1;
            } else {
                writeln!(
                    writer,
                    "⚠️  Skipping '{}' — run with --force to overwrite",
                    skill_name
                )
                .map_err(EngramError::Io)?;
                skipped_count += 1;
            }
            continue;
        }

        std::fs::create_dir_all(&skill_dir).map_err(EngramError::Io)?;
        std::fs::write(&skill_file, skill_content).map_err(EngramError::Io)?;
        writeln!(writer, "✅ Installed skill: {}", skill_name).map_err(EngramError::Io)?;
        installed_count += 1;
    }

    Ok((installed_count, updated_count, skipped_count))
}

/// Handle setup skills command
pub fn handle_skills_command(
    writer: &mut dyn std::io::Write,
    force: bool,
    dir: Option<&str>,
    tool: Option<&str>,
    source: Option<&str>,
) -> Result<(), EngramError> {
    let skills_dir = resolve_skills_dir(dir, tool)?;
    let source_dir = resolve_skills_source(source);

    writeln!(writer, "📂 Scanning skills from: {:?}", source_dir).map_err(EngramError::Io)?;
    let skills = scan_skills_from_dir(&source_dir)?;

    if skills.is_empty() {
        writeln!(writer, "⚠️  No skill files found in {:?}", source_dir)
            .map_err(EngramError::Io)?;
        return Ok(());
    }

    writeln!(writer, "📦 Found {} skills", skills.len()).map_err(EngramError::Io)?;
    writeln!(writer).map_err(EngramError::Io)?;

    let (installed, updated, skipped) = install_skills(writer, &skills, &skills_dir, force)?;

    writeln!(writer).map_err(EngramError::Io)?;
    writeln!(writer, "🎉 Skills setup complete!").map_err(EngramError::Io)?;
    writeln!(writer, "📁 Skills installed to: {:?}", skills_dir).map_err(EngramError::Io)?;
    writeln!(
        writer,
        "📊 Installed: {}  Updated: {}  Skipped: {}",
        installed, updated, skipped
    )
    .map_err(EngramError::Io)?;
    writeln!(writer).map_err(EngramError::Io)?;
    writeln!(writer, "💡 Skills are now available with 'engram:' prefix")
        .map_err(EngramError::Io)?;
    writeln!(
        writer,
        "   Example: skill() tool with 'engram:use-engram-memory'"
    )
    .map_err(EngramError::Io)?;
    writeln!(writer).map_err(EngramError::Io)?;
    writeln!(writer, "📖 To use skills:").map_err(EngramError::Io)?;
    writeln!(writer, "   1. Restart your agent session to reload skills")
        .map_err(EngramError::Io)?;
    writeln!(writer, "   2. Use skill() tool with skill name").map_err(EngramError::Io)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_scan_skills_from_dir_basic() {
        let temp = TempDir::new().unwrap();
        let source = temp.path().to_path_buf();

        fs::create_dir_all(source.join("meta")).unwrap();
        fs::write(
            source.join("meta/use-engram-memory.md"),
            "# Use Engram Memory",
        )
        .unwrap();
        fs::create_dir_all(source.join("screenplay")).unwrap();
        fs::write(
            source.join("screenplay/session-start.md"),
            "# Session Start",
        )
        .unwrap();

        let skills = scan_skills_from_dir(&source).unwrap();
        assert_eq!(skills.len(), 2);

        let names: Vec<&str> = skills.iter().map(|(n, _)| n.as_str()).collect();
        assert!(names.contains(&"engram-use-engram-memory"));
        assert!(names.contains(&"screenplay-session-start"));
    }

    #[test]
    fn test_scan_skills_skill_md_uses_dir_name() {
        let temp = TempDir::new().unwrap();
        let source = temp.path().to_path_buf();

        fs::create_dir_all(source.join("testing")).unwrap();
        fs::write(source.join("testing/skill.md"), "# Testing").unwrap();

        let skills = scan_skills_from_dir(&source).unwrap();
        assert_eq!(skills.len(), 1);
        assert_eq!(skills[0].0, "engram-testing");
    }

    #[test]
    fn test_scan_skills_skips_readme() {
        let temp = TempDir::new().unwrap();
        let source = temp.path().to_path_buf();

        fs::create_dir_all(source.join("meta")).unwrap();
        fs::write(source.join("meta/README.md"), "# README").unwrap();
        fs::write(source.join("meta/delegate-to-agents.md"), "# Delegate").unwrap();

        let skills = scan_skills_from_dir(&source).unwrap();
        assert_eq!(skills.len(), 1);
        assert_eq!(skills[0].0, "engram-delegate-to-agents");
    }

    #[test]
    fn test_scan_skills_auto_prefix() {
        let temp = TempDir::new().unwrap();
        let source = temp.path().to_path_buf();

        fs::create_dir_all(source.join("planning")).unwrap();
        fs::write(
            source.join("planning/risk-assessment.md"),
            "# Risk Assessment",
        )
        .unwrap();

        let skills = scan_skills_from_dir(&source).unwrap();
        assert_eq!(skills[0].0, "engram-risk-assessment");
    }

    #[test]
    fn test_scan_skills_missing_dir() {
        let result = scan_skills_from_dir(&PathBuf::from("/nonexistent/path"));
        assert!(result.is_err());
    }

    #[test]
    fn test_scan_skills_sorted() {
        let temp = TempDir::new().unwrap();
        let source = temp.path().to_path_buf();

        fs::create_dir_all(source.join("meta")).unwrap();
        fs::write(source.join("meta/zebra.md"), "# Z").unwrap();
        fs::write(source.join("meta/alpha.md"), "# A").unwrap();

        let skills = scan_skills_from_dir(&source).unwrap();
        assert_eq!(skills[0].0, "engram-alpha");
        assert_eq!(skills[1].0, "engram-zebra");
    }

    #[test]
    fn test_list_skills_empty() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path().to_path_buf();

        let skills_dir = root.join("engram/skills");
        fs::create_dir_all(&skills_dir).unwrap();

        let mut buffer = Vec::new();
        let result = list_skills(&mut buffer, "short", false, Some(root));
        assert!(result.is_ok());

        let output = String::from_utf8(buffer).unwrap();
        assert!(!output.contains("Skill Name"));
    }

    #[test]
    fn test_list_skills_populated() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path().to_path_buf();
        let skills_dir = root.join("engram/skills");

        fs::create_dir_all(&skills_dir.join("skill-a")).unwrap();
        fs::create_dir_all(&skills_dir.join("skill-b")).unwrap();

        fs::write(skills_dir.join("skill-a/skill.md"), "Description A").unwrap();

        let mut buffer_short = Vec::new();
        list_skills(&mut buffer_short, "short", false, Some(root.clone())).unwrap();
        let output_short = String::from_utf8(buffer_short).unwrap();

        assert!(output_short.contains("skill-a"));
        assert!(output_short.contains("skill-b"));

        let mut buffer_full = Vec::new();
        list_skills(&mut buffer_full, "full", false, Some(root)).unwrap();
        let output_full = String::from_utf8(buffer_full).unwrap();

        assert!(output_full.contains("skill-a"));
        assert!(output_full.contains("Description A"));
    }

    #[test]
    fn test_show_skill() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path().to_path_buf();
        let skills_dir = root.join("engram/skills");

        let skill_path = skills_dir.join("test-skill");
        fs::create_dir_all(&skill_path).unwrap();

        fs::write(skill_path.join("file1.txt"), "content 1").unwrap();
        fs::write(skill_path.join("file2.rs"), "fn main() {}").unwrap();

        let mut buffer = Vec::new();
        let result = show_skill(&mut buffer, "test-skill", Some(root.clone()));
        assert!(result.is_ok());
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Skill: test-skill"));
        assert!(output.contains("file1.txt"));
        assert!(output.contains("content 1"));

        let mut buffer_case = Vec::new();
        let result = show_skill(&mut buffer_case, "TEST-SKILL", Some(root.clone()));
        assert!(result.is_ok());
        let output_case = String::from_utf8(buffer_case).unwrap();
        assert!(output_case.contains("Skill: test-skill"));

        let mut buffer_missing = Vec::new();
        let result = show_skill(&mut buffer_missing, "missing-skill", Some(root));
        assert!(result.is_ok());
        let output_missing = String::from_utf8(buffer_missing).unwrap();
        assert!(output_missing.contains("Skill not found"));
    }

    #[test]
    fn test_list_skills_missing_dir() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path().to_path_buf();

        let mut buffer = Vec::new();
        let result = list_skills(&mut buffer, "short", false, Some(root));
        assert!(result.is_ok());
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Skills directory not found"));
    }

    #[test]
    fn test_list_skills_invalid_format() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path().to_path_buf();
        let skills_dir = root.join("engram/skills");
        fs::create_dir_all(&skills_dir).unwrap();

        let mut buffer = Vec::new();
        let result = list_skills(&mut buffer, "invalid", false, Some(root));
        assert!(result.is_ok());
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Unknown format"));
    }

    #[test]
    fn test_install_skills_new() {
        let temp = TempDir::new().unwrap();
        let target = temp.path().join("skills");

        let skills = vec![(
            "engram-test-skill".to_string(),
            "# Test\nContent".to_string(),
        )];

        let mut buf = Vec::new();
        let (installed, updated, skipped) =
            install_skills(&mut buf, &skills, &target, false).unwrap();

        assert_eq!(installed, 1);
        assert_eq!(updated, 0);
        assert_eq!(skipped, 0);
        assert!(target.join("engram-test-skill/SKILL.md").exists());
    }

    #[test]
    fn test_install_skills_skip_without_force() {
        let temp = TempDir::new().unwrap();
        let target = temp.path().join("skills");

        let skills = vec![("engram-existing".to_string(), "# Original".to_string())];

        let mut buf = Vec::new();
        install_skills(&mut buf, &skills, &target, false).unwrap();

        let modified = vec![("engram-existing".to_string(), "# Modified".to_string())];

        let mut buf2 = Vec::new();
        let (installed, updated, skipped) =
            install_skills(&mut buf2, &modified, &target, false).unwrap();

        assert_eq!(installed, 0);
        assert_eq!(updated, 0);
        assert_eq!(skipped, 1);

        let content = fs::read_to_string(target.join("engram-existing/SKILL.md")).unwrap();
        assert_eq!(content, "# Original");
    }

    #[test]
    fn test_install_skills_update_with_force() {
        let temp = TempDir::new().unwrap();
        let target = temp.path().join("skills");

        let skills = vec![("engram-existing".to_string(), "# Original".to_string())];

        let mut buf = Vec::new();
        install_skills(&mut buf, &skills, &target, false).unwrap();

        let modified = vec![("engram-existing".to_string(), "# Modified".to_string())];

        let mut buf2 = Vec::new();
        let (installed, updated, skipped) =
            install_skills(&mut buf2, &modified, &target, true).unwrap();

        assert_eq!(installed, 0);
        assert_eq!(updated, 1);
        assert_eq!(skipped, 0);

        let content = fs::read_to_string(target.join("engram-existing/SKILL.md")).unwrap();
        assert_eq!(content, "# Modified");
    }

    #[test]
    fn test_list_skills_verbose() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path().to_path_buf();
        let skills_dir = root.join("engram/skills");

        fs::create_dir_all(&skills_dir.join("my-skill")).unwrap();

        let mut buffer = Vec::new();
        list_skills(&mut buffer, "short", true, Some(root)).unwrap();
        let output = String::from_utf8(buffer).unwrap();

        assert!(output.contains("🔎 Skills configuration:"));
        assert!(output.contains("Target path:"));
        assert!(output.contains("Absolute path:"));
    }

    #[test]
    fn test_list_skills_verbose_env_set() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path().to_path_buf();
        let skills_dir = root.join("engram/skills");
        fs::create_dir_all(&skills_dir).unwrap();

        let env_value = skills_dir.to_string_lossy().to_string();
        std::env::set_var("ENGRAM_SKILLS_PATH", &env_value);

        let mut buffer = Vec::new();
        let result = list_skills(&mut buffer, "short", true, Some(root));
        std::env::remove_var("ENGRAM_SKILLS_PATH");

        assert!(result.is_ok());
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains(&env_value));
    }

    #[test]
    fn test_list_skills_full_format_with_uppercase_skill_md() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path().to_path_buf();
        let skills_dir = root.join("engram/skills");

        let skill_dir = skills_dir.join("upper-skill");
        fs::create_dir_all(&skill_dir).unwrap();
        fs::write(
            skill_dir.join("SKILL.md"),
            "This is the uppercase description",
        )
        .unwrap();

        let mut buffer = Vec::new();
        list_skills(&mut buffer, "full", false, Some(root)).unwrap();
        let output = String::from_utf8(buffer).unwrap();

        assert!(output.contains("This is the uppercase description"));
    }

    #[test]
    fn test_list_skills_full_format_no_description() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path().to_path_buf();
        let skills_dir = root.join("engram/skills");

        let skill_dir = skills_dir.join("no-desc-skill");
        fs::create_dir_all(&skill_dir).unwrap();

        let mut buffer = Vec::new();
        list_skills(&mut buffer, "full", false, Some(root)).unwrap();
        let output = String::from_utf8(buffer).unwrap();

        assert!(output.contains("(no description)"));
    }

    #[test]
    fn test_list_skills_verbose_skips_non_directories() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path().to_path_buf();
        let skills_dir = root.join("engram/skills");

        fs::create_dir_all(&skills_dir).unwrap();
        fs::write(skills_dir.join("not_a_skill.txt"), "junk").unwrap();

        let mut buffer = Vec::new();
        list_skills(&mut buffer, "short", true, Some(root)).unwrap();
        let output = String::from_utf8(buffer).unwrap();

        assert!(output.contains("Skipping non-directory"));
    }

    #[test]
    fn test_list_skills_verbose_missing_dir() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path().to_path_buf();

        let mut buffer = Vec::new();
        list_skills(&mut buffer, "short", true, Some(root)).unwrap();
        let output = String::from_utf8(buffer).unwrap();

        assert!(output.contains("❌ Directory does not exist"));
        assert!(output.contains("Skills directory not found"));
    }

    #[test]
    fn test_list_skills_short_skips_non_directories() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path().to_path_buf();
        let skills_dir = root.join("engram/skills");

        fs::create_dir_all(&skills_dir.join("real-skill")).unwrap();
        fs::write(skills_dir.join("junkfile.md"), "not a dir").unwrap();

        let mut buffer = Vec::new();
        list_skills(&mut buffer, "short", false, Some(root)).unwrap();
        let output = String::from_utf8(buffer).unwrap();

        assert!(output.contains("real-skill"));
        assert!(!output.contains("junkfile.md"));
    }

    #[test]
    fn test_show_skill_with_subdirectory() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path().to_path_buf();
        let skills_dir = root.join("engram/skills");

        let skill_path = skills_dir.join("subdir-skill");
        fs::create_dir_all(skill_path.join("examples")).unwrap();
        fs::write(skill_path.join("README.md"), "top level").unwrap();

        let mut buffer = Vec::new();
        show_skill(&mut buffer, "subdir-skill", Some(root)).unwrap();
        let output = String::from_utf8(buffer).unwrap();

        assert!(output.contains("[DIR]"));
    }

    #[test]
    fn test_show_skill_file_preview_truncation() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path().to_path_buf();
        let skills_dir = root.join("engram/skills");

        let skill_path = skills_dir.join("trunc-skill");
        fs::create_dir_all(&skill_path).unwrap();

        let long_content = "A".repeat(200);
        fs::write(skill_path.join("long.txt"), &long_content).unwrap();

        let mut buffer = Vec::new();
        show_skill(&mut buffer, "trunc-skill", Some(root)).unwrap();
        let output = String::from_utf8(buffer).unwrap();

        assert!(!output.contains(&long_content));
        assert!(output.contains(&"A".repeat(100)));
    }

    #[test]
    fn test_show_skill_fallback_to_local_path() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path().to_path_buf();
        let skills_dir = root.join("engram/skills");
        fs::create_dir_all(&skills_dir).unwrap();

        let skill_path = skills_dir.join("fallback-skill");
        fs::create_dir_all(&skill_path).unwrap();
        fs::write(skill_path.join("info.md"), "fallback content").unwrap();

        let mut buffer = Vec::new();
        show_skill(&mut buffer, "fallback-skill", Some(root)).unwrap();
        let output = String::from_utf8(buffer).unwrap();

        assert!(output.contains("Skill: fallback-skill"));
        assert!(output.contains("info.md"));
    }

    // --- get_skills_path tests ---

    #[test]
    fn test_get_skills_path_uses_config_dir() {
        let temp = TempDir::new().unwrap();
        let config = temp.path().to_path_buf();
        let result = get_skills_path(Some(config.clone()));
        assert_eq!(result, config.join("engram/skills"));
    }

    #[test]
    fn test_get_skills_path_uses_env_var() {
        let temp = TempDir::new().unwrap();
        let prev = std::env::var("ENGRAM_SKILLS_PATH").ok();
        std::env::set_var("ENGRAM_SKILLS_PATH", temp.path());
        let result = get_skills_path(None);
        assert_eq!(result, temp.path().to_path_buf());
        match prev {
            Some(v) => std::env::set_var("ENGRAM_SKILLS_PATH", v),
            None => std::env::remove_var("ENGRAM_SKILLS_PATH"),
        }
    }

    #[test]
    fn test_get_skills_path_uses_dot_engram_skills_in_cwd() {
        let temp = TempDir::new().unwrap();
        let prev_env = std::env::var("ENGRAM_SKILLS_PATH").ok();
        std::env::remove_var("ENGRAM_SKILLS_PATH");
        fs::create_dir_all(temp.path().join(".engram/skills")).unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp.path()).unwrap();
        let result = get_skills_path(None);
        std::env::set_current_dir(&original_dir).unwrap();
        assert!(result.ends_with(".engram/skills"));
        match prev_env {
            Some(v) => std::env::set_var("ENGRAM_SKILLS_PATH", v),
            None => std::env::remove_var("ENGRAM_SKILLS_PATH"),
        }
    }

    #[test]
    fn test_get_skills_path_uses_engram_skills_in_cwd() {
        let temp = TempDir::new().unwrap();
        let prev_env = std::env::var("ENGRAM_SKILLS_PATH").ok();
        std::env::remove_var("ENGRAM_SKILLS_PATH");
        fs::create_dir_all(temp.path().join("engram/skills")).unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp.path()).unwrap();
        let result = get_skills_path(None);
        std::env::set_current_dir(&original_dir).unwrap();
        assert_eq!(result, PathBuf::from("./engram/skills"));
        match prev_env {
            Some(v) => std::env::set_var("ENGRAM_SKILLS_PATH", v),
            None => std::env::remove_var("ENGRAM_SKILLS_PATH"),
        }
    }

    #[test]
    fn test_get_skills_path_fallback() {
        let temp = TempDir::new().unwrap();
        let prev_env = std::env::var("ENGRAM_SKILLS_PATH").ok();
        std::env::remove_var("ENGRAM_SKILLS_PATH");
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp.path()).unwrap();
        let result = get_skills_path(None);
        std::env::set_current_dir(&original_dir).unwrap();
        assert_eq!(result, PathBuf::from(".engram/skills"));
        match prev_env {
            Some(v) => std::env::set_var("ENGRAM_SKILLS_PATH", v),
            None => std::env::remove_var("ENGRAM_SKILLS_PATH"),
        }
    }

    // --- resolve_skills_source tests ---

    #[test]
    fn test_resolve_skills_source_uses_env_var() {
        let temp = TempDir::new().unwrap();
        let prev = std::env::var("ENGRAM_SKILLS_SOURCE").ok();
        std::env::set_var("ENGRAM_SKILLS_SOURCE", temp.path());
        let result = resolve_skills_source(None);
        assert_eq!(result, temp.path().to_path_buf());
        match prev {
            Some(v) => std::env::set_var("ENGRAM_SKILLS_SOURCE", v),
            None => std::env::remove_var("ENGRAM_SKILLS_SOURCE"),
        }
    }

    #[test]
    fn test_resolve_skills_source_uses_arg() {
        let result = resolve_skills_source(Some("/custom/source"));
        assert_eq!(result, PathBuf::from("/custom/source"));
    }

    #[test]
    fn test_resolve_skills_source_default() {
        let prev = std::env::var("ENGRAM_SKILLS_SOURCE").ok();
        std::env::remove_var("ENGRAM_SKILLS_SOURCE");
        let result = resolve_skills_source(None);
        assert_eq!(result, PathBuf::from("./skills"));
        match prev {
            Some(v) => std::env::set_var("ENGRAM_SKILLS_SOURCE", v),
            None => std::env::remove_var("ENGRAM_SKILLS_SOURCE"),
        }
    }

    // --- resolve_skills_dir tests ---

    #[test]
    fn test_resolve_skills_dir_default() {
        let home = std::env::var("HOME").unwrap();
        let result = resolve_skills_dir(None, None).unwrap();
        assert_eq!(result, PathBuf::from(home).join(".config/opencode/skills"));
    }

    #[test]
    fn test_resolve_skills_dir_tool_claude() {
        let home = std::env::var("HOME").unwrap();
        let result = resolve_skills_dir(None, Some("claude")).unwrap();
        assert_eq!(result, PathBuf::from(home).join(".claude/skills"));
    }

    #[test]
    fn test_resolve_skills_dir_tool_goose() {
        let home = std::env::var("HOME").unwrap();
        let result = resolve_skills_dir(None, Some("goose")).unwrap();
        assert_eq!(result, PathBuf::from(home).join(".config/goose/skills"));
    }

    #[test]
    fn test_resolve_skills_dir_expands_tilde_slash() {
        let home = std::env::var("HOME").unwrap();
        let result = resolve_skills_dir(Some("~/foo"), None).unwrap();
        assert_eq!(result, PathBuf::from(home).join("foo"));
    }

    #[test]
    fn test_resolve_skills_dir_expands_bare_tilde() {
        let home = std::env::var("HOME").unwrap();
        let result = resolve_skills_dir(Some("~"), None).unwrap();
        assert_eq!(result, PathBuf::from(&home));
    }

    #[test]
    fn test_resolve_skills_dir_absolute_path() {
        let result = resolve_skills_dir(Some("/tmp/custom/skills"), None).unwrap();
        assert_eq!(result, PathBuf::from("/tmp/custom/skills"));
    }

    #[test]
    fn test_resolve_skills_dir_unknown_tool() {
        let result = resolve_skills_dir(None, Some("unknown"));
        assert!(result.is_err());
    }

    // --- unified_diff tests ---

    #[test]
    fn test_unified_diff_identical() {
        let result = unified_diff("test-skill", "same content\n", "same content\n");
        assert_eq!(result, "");
    }

    #[test]
    fn test_unified_diff_has_headers() {
        let old = "line one\n";
        let new = "line two\n";
        let result = unified_diff("my-skill", old, new);
        assert!(result.contains("--- my-skill/SKILL.md (on disk)"));
        assert!(result.contains("+++ my-skill/SKILL.md (source)"));
    }

    #[test]
    fn test_unified_diff_single_line_addition() {
        let old = "existing line\n";
        let new = "existing line\nnew line\n";
        let result = unified_diff("add-test", old, new);
        assert!(result.contains("+new line"));
    }

    #[test]
    fn test_unified_diff_single_line_deletion() {
        let old = "line to delete\nremaining\n";
        let new = "remaining\n";
        let result = unified_diff("del-test", old, new);
        assert!(result.contains("-line to delete"));
    }

    #[test]
    fn test_unified_diff_multi_line_changes() {
        let old = "alpha\nbeta\ngamma\n";
        let new = "alpha\nbravo\ndelta\n";
        let result = unified_diff("multi-test", old, new);
        assert!(result.contains("-beta"));
        assert!(result.contains("-gamma"));
        assert!(result.contains("+bravo"));
        assert!(result.contains("+delta"));
        assert!(result.contains(" alpha"));
    }

    // --- handle_skills_command integration tests ---

    #[test]
    fn test_handle_skills_command_happy_path() {
        let source_temp = TempDir::new().unwrap();
        let target_temp = TempDir::new().unwrap();

        fs::write(
            source_temp.path().join("my-skill.md"),
            "# My Skill\nSome content",
        )
        .unwrap();

        let target_path = target_temp.path().to_string_lossy().to_string();
        let source_path = source_temp.path().to_string_lossy().to_string();

        let mut buf: Vec<u8> = Vec::new();
        let result = handle_skills_command(
            &mut buf,
            false,
            Some(&target_path),
            None,
            Some(&source_path),
        );
        assert!(result.is_ok());

        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("Found 1 skills"));
        assert!(output.contains("Installed: 1"));
        assert!(output.contains("Skills setup complete"));

        let skill_dir = target_temp.path().join("engram-my-skill");
        assert!(skill_dir.exists());
        assert!(skill_dir.join("SKILL.md").exists());
    }

    #[test]
    fn test_handle_skills_command_empty_source() {
        let source_temp = TempDir::new().unwrap();
        let target_temp = TempDir::new().unwrap();

        let target_path = target_temp.path().to_string_lossy().to_string();
        let source_path = source_temp.path().to_string_lossy().to_string();

        let mut buf: Vec<u8> = Vec::new();
        let result = handle_skills_command(
            &mut buf,
            false,
            Some(&target_path),
            None,
            Some(&source_path),
        );
        assert!(result.is_ok());

        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("No skill files found"));

        assert!(!target_temp.path().join("engram-my-skill").exists());
    }

    #[test]
    fn test_handle_skills_command_missing_source() {
        let target_temp = TempDir::new().unwrap();
        let target_path = target_temp.path().to_string_lossy().to_string();

        let mut buf: Vec<u8> = Vec::new();
        let result = handle_skills_command(
            &mut buf,
            false,
            Some(&target_path),
            None,
            Some("/nonexistent/path"),
        );
        assert!(matches!(result, Err(EngramError::Validation(_))));
    }

    #[test]
    fn test_handle_skills_command_update_with_force() {
        let source_temp = TempDir::new().unwrap();
        let target_temp = TempDir::new().unwrap();

        fs::write(
            source_temp.path().join("update-skill.md"),
            "# Original Content",
        )
        .unwrap();

        let target_path = target_temp.path().to_string_lossy().to_string();
        let source_path = source_temp.path().to_string_lossy().to_string();

        let mut buf1: Vec<u8> = Vec::new();
        handle_skills_command(
            &mut buf1,
            false,
            Some(&target_path),
            None,
            Some(&source_path),
        )
        .unwrap();

        fs::write(
            source_temp.path().join("update-skill.md"),
            "# Updated Content\nNew line",
        )
        .unwrap();

        let mut buf2: Vec<u8> = Vec::new();
        let result = handle_skills_command(
            &mut buf2,
            true,
            Some(&target_path),
            None,
            Some(&source_path),
        );
        assert!(result.is_ok());

        let output = String::from_utf8(buf2).unwrap();
        assert!(output.contains("Updated: 1"));
    }

    #[test]
    fn test_handle_skills_command_skip_without_force() {
        let source_temp = TempDir::new().unwrap();
        let target_temp = TempDir::new().unwrap();

        fs::write(
            source_temp.path().join("skip-skill.md"),
            "# Original Content",
        )
        .unwrap();

        let target_path = target_temp.path().to_string_lossy().to_string();
        let source_path = source_temp.path().to_string_lossy().to_string();

        let mut buf1: Vec<u8> = Vec::new();
        handle_skills_command(
            &mut buf1,
            false,
            Some(&target_path),
            None,
            Some(&source_path),
        )
        .unwrap();

        fs::write(
            source_temp.path().join("skip-skill.md"),
            "# Modified Content",
        )
        .unwrap();

        let mut buf2: Vec<u8> = Vec::new();
        let result = handle_skills_command(
            &mut buf2,
            false,
            Some(&target_path),
            None,
            Some(&source_path),
        );
        assert!(result.is_ok());

        let output = String::from_utf8(buf2).unwrap();
        assert!(output.contains("Skipping"));

        let on_disk =
            fs::read_to_string(target_temp.path().join("engram-skip-skill/SKILL.md")).unwrap();
        assert_eq!(on_disk, "# Original Content");
    }

    #[test]
    fn test_handle_skills_command_unknown_tool() {
        let mut buf: Vec<u8> = Vec::new();
        let result = handle_skills_command(&mut buf, false, None, Some("unknown_tool"), None);
        assert!(matches!(result, Err(EngramError::Validation(_))));
    }

    #[test]
    fn test_handle_skills_command_multiple_skills() {
        let source_temp = TempDir::new().unwrap();
        let target_temp = TempDir::new().unwrap();

        fs::create_dir_all(source_temp.path().join("alpha")).unwrap();
        fs::write(source_temp.path().join("alpha/skill.md"), "# Alpha Skill").unwrap();

        fs::create_dir_all(source_temp.path().join("beta")).unwrap();
        fs::write(source_temp.path().join("beta/skill.md"), "# Beta Skill").unwrap();

        fs::create_dir_all(source_temp.path().join("gamma/deep")).unwrap();
        fs::write(
            source_temp.path().join("gamma/deep/nested.md"),
            "# Gamma Nested",
        )
        .unwrap();

        let target_path = target_temp.path().to_string_lossy().to_string();
        let source_path = source_temp.path().to_string_lossy().to_string();

        let mut buf: Vec<u8> = Vec::new();
        let result =
            handle_skills_command(&mut buf, true, Some(&target_path), None, Some(&source_path));
        assert!(result.is_ok());

        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("Found 3 skills"));
        assert!(output.contains("Installed: 3"));
    }
}
