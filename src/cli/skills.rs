use crate::error::EngramError;
use clap::Subcommand;
use std::path::PathBuf;

/// Skills commands
#[derive(Debug, Subcommand)]
pub enum SkillsCommands {
    /// Install OpenCode skills
    Setup,
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
    std::env::var("ENGRAM_SKILLS_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let cwd_skills = PathBuf::from(".engram/skills");
            if cwd_skills.exists() {
                cwd_skills
            } else {
                PathBuf::from("./engram/skills")
            }
        })
}

/// List all skills in skills directory
pub fn list_skills(
    writer: &mut dyn std::io::Write,
    format: &str,
    verbose: bool,
    config_dir: Option<PathBuf>,
) -> Result<(), std::io::Error> {
    let skills_path = get_skills_path(config_dir);

    if verbose {
        writeln!(writer, "Looking for skills in: {:?}", skills_path)?;
    }

    if !skills_path.exists() {
        writeln!(writer, "Skills directory not found: {:?}", skills_path)?;
        writeln!(writer, "Set ENGRAM_SKILLS_PATH environment variable.")?;
        return Ok(());
    }

    let entries = std::fs::read_dir(&skills_path)?;

    match format {
        "short" | "s" => {
            writeln!(writer, "Available Skills:")?;
            writeln!(writer, "=================")?;
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    writeln!(writer, "  - {}", entry.file_name().to_string_lossy())?;
                } else if verbose {
                    writeln!(
                        writer,
                        "  (Skipping non-directory: {})",
                        entry.file_name().to_string_lossy()
                    )?;
                }
            }
        }
        "full" | "f" => {
            writeln!(writer, "Available Skills:")?;
            writeln!(writer, "=================")?;
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    let file_name = entry.file_name();
                    let name = file_name.to_string_lossy();
                    let skill_file = entry.path().join("skill.md");
                    let description = if skill_file.exists() {
                        let content = std::fs::read_to_string(&skill_file)?;
                        content.lines().next().unwrap_or("").to_string()
                    } else {
                        "(no description)".to_string()
                    };
                    writeln!(writer, "\n[{}]", name)?;
                    writeln!(writer, "  Description: {}", description)?;
                } else if verbose {
                    writeln!(
                        writer,
                        "  (Skipping non-directory: {})",
                        entry.file_name().to_string_lossy()
                    )?;
                }
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

    // Try exact match first, then case-insensitive
    let skill_path = skills_path.join(name); // Use base path first, avoid assuming name is path

    let actual_path = if skill_path.exists() && skill_path.is_dir() {
        skill_path
    } else {
        // Search for matching directory
        let name_lower = name.to_lowercase();
        // Check if skills_path exists before reading
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
            // Fallback to checking if name provided was actually a path relative to CWD,
            // but prioritize skills_dir
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

    // List skill contents
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

/// Handle setup skills command
pub fn handle_skills_command(
    writer: &mut dyn std::io::Write,
    _command: crate::cli::SkillsCommands,
) -> Result<(), EngramError> {
    use std::env;

    // Get OpenCode config directory
    let opencode_dir = env::var("HOME")
        .map(|home| PathBuf::from(home).join(".config").join("opencode"))
        .map_err(|_| EngramError::Validation("HOME environment variable not set".to_string()))?;

    let skills_dir = opencode_dir.join("skills");
    std::fs::create_dir_all(&skills_dir).map_err(EngramError::Io)?;

    // List of built-in Engram skills to install with their content
    let skills: &[(&str, &str)] = &[
        (
            "engram-use-engram-memory",
            include_str!("../../skills/meta/use-engram-memory.md"),
        ),
        (
            "engram-delegate-to-agents",
            include_str!("../../skills/meta/delegate-to-agents.md"),
        ),
        (
            "engram-audit-trail",
            include_str!("../../skills/meta/audit-trail.md"),
        ),
        (
            "engram-brainstorming",
            include_str!("../../skills/workflow/brainstorming.md"),
        ),
        (
            "engram-writing-plans",
            include_str!("../../skills/workflow/writing-plans.md"),
        ),
        (
            "engram-plan-feature",
            include_str!("../../skills/workflow/plan-feature.md"),
        ),
        (
            "engram-requesting-code-review",
            include_str!("../../skills/workflow/requesting-code-review.md"),
        ),
        (
            "engram-check-compliance",
            include_str!("../../skills/compliance/check-compliance.md"),
        ),
        (
            "engram-test-driven-development",
            include_str!("../../skills/development/test-driven-development.md"),
        ),
        (
            "engram-systematic-debugging",
            include_str!("../../skills/debugging/systematic-debugging.md"),
        ),
        (
            "engram-subagent-driven-development",
            include_str!("../../skills/development/subagent-driven-development.md"),
        ),
        (
            "engram-dispatching-parallel-agents",
            include_str!("../../skills/meta/dispatching-parallel-agents.md"),
        ),
    ];

    let mut installed_count = 0;

    for (skill_name, skill_content) in skills {
        let skill_dir = skills_dir.join(skill_name);

        // Skip if skill already exists (user skill take precedence)
        if skill_dir.exists() {
            writeln!(
                writer,
                "⚠️  Skill '{}' already exists, skipping (user skill preserved)",
                skill_name
            )
            .map_err(EngramError::Io)?;
            continue;
        }

        std::fs::create_dir_all(&skill_dir).map_err(EngramError::Io)?;

        // Create SKILL.md file with embedded content
        let skill_file = skill_dir.join("SKILL.md");
        std::fs::write(&skill_file, skill_content).map_err(EngramError::Io)?;

        writeln!(writer, "✅ Installed skill: {}", skill_name).map_err(EngramError::Io)?;
        installed_count += 1;
    }

    writeln!(writer).map_err(EngramError::Io)?;
    writeln!(writer, "🎉 Skills setup complete!").map_err(EngramError::Io)?;
    writeln!(writer, "📁 Skills installed to: {:?}", skills_dir).map_err(EngramError::Io)?;
    writeln!(writer, "📊 Total skills installed: {}", installed_count).map_err(EngramError::Io)?;
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
    fn test_list_skills_empty() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path().to_path_buf();

        // Ensure parent dir exists but is empty
        let skills_dir = root.join("engram/skills");
        fs::create_dir_all(&skills_dir).unwrap();

        // Should just print header and return Ok
        let mut buffer = Vec::new();
        let result = list_skills(&mut buffer, "short", false, Some(root));
        assert!(result.is_ok());

        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Available Skills:"));
    }

    #[test]
    fn test_list_skills_populated() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path().to_path_buf();
        let skills_dir = root.join("engram/skills");

        fs::create_dir_all(&skills_dir.join("skill-a")).unwrap();
        fs::create_dir_all(&skills_dir.join("skill-b")).unwrap();

        // Add descriptions
        fs::write(skills_dir.join("skill-a/skill.md"), "Description A").unwrap();

        // Test short listing
        let mut buffer_short = Vec::new();
        list_skills(&mut buffer_short, "short", false, Some(root.clone())).unwrap();
        let output_short = String::from_utf8(buffer_short).unwrap();
        assert!(output_short.contains("skill-a"));
        assert!(output_short.contains("skill-b"));

        // Test full listing
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

        // Test exact match
        let mut buffer = Vec::new();
        let result = show_skill(&mut buffer, "test-skill", Some(root.clone()));
        assert!(result.is_ok());
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Skill: test-skill"));
        assert!(output.contains("file1.txt"));
        assert!(output.contains("content 1"));

        // Test case insensitive
        let mut buffer_case = Vec::new();
        let result = show_skill(&mut buffer_case, "TEST-SKILL", Some(root.clone()));
        assert!(result.is_ok());
        let output_case = String::from_utf8(buffer_case).unwrap();
        assert!(output_case.contains("Skill: test-skill"));

        // Test non-existent
        let mut buffer_missing = Vec::new();
        let result = show_skill(&mut buffer_missing, "missing-skill", Some(root));
        assert!(result.is_ok()); // Returns Ok but prints error message
        let output_missing = String::from_utf8(buffer_missing).unwrap();
        assert!(output_missing.contains("Skill not found"));
    }

    #[test]
    fn test_list_skills_missing_dir() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path().to_path_buf();

        // Don't create directory

        // Should handle gracefully (print message and return Ok)
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

        // Should print unknown format message and return Ok
        let mut buffer = Vec::new();
        let result = list_skills(&mut buffer, "invalid", false, Some(root));
        assert!(result.is_ok());
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Unknown format"));
    }
}
