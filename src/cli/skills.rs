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
    },
    /// Show skill details
    Show {
        /// Skill name or path
        #[arg(help = "Skill name or path")]
        name: String,
    },
}

/// Get skills path from environment or default
pub fn get_skills_path() -> PathBuf {
    std::env::var("ENGRAM_SKILLS_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("./engram/skills"))
}

/// List all skills in skills directory
pub fn list_skills(format: &str) -> Result<(), std::io::Error> {
    let skills_path = get_skills_path();

    if !skills_path.exists() {
        println!("Skills directory not found: {:?}", skills_path);
        println!("Set ENGRAM_SKILLS_PATH environment variable.");
        return Ok(());
    }

    let entries = std::fs::read_dir(&skills_path)?;

    match format {
        "short" | "s" => {
            println!("Available Skills:");
            println!("=================");
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    println!("  - {}", entry.file_name().to_string_lossy());
                }
            }
        }
        "full" | "f" => {
            println!("Available Skills:");
            println!("=================");
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
                    println!("\n[{}]", name);
                    println!("  Description: {}", description);
                }
            }
        }
        _ => {
            println!("Unknown format: {}. Use 'short' or 'full'.", format);
        }
    }

    Ok(())
}

/// Show a specific skill
pub fn show_skill(name: &str) -> Result<(), std::io::Error> {
    let skills_path = get_skills_path();

    // Try exact match first, then case-insensitive
    let skill_path = PathBuf::from(name);

    let actual_path = if skill_path.exists() && skill_path.is_dir() {
        skill_path
    } else {
        // Search for matching directory
        let name_lower = name.to_lowercase();
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
            skills_path.join(name)
        }
    };

    if !actual_path.exists() {
        println!("Skill not found: {}", name);
        println!("Searched in: {:?}", skills_path);
        return Ok(());
    }

    // List skill contents
    println!(
        "\nSkill: {}",
        actual_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
    );
    println!("======");

    let entries = std::fs::read_dir(&actual_path)?;
    for entry in entries.flatten() {
        let file_name = entry.file_name();
        let file_type = if entry.path().is_dir() {
            "[DIR]"
        } else {
            "[FILE]"
        };
        println!("  {} {}", file_type, file_name.to_string_lossy());

        if entry.path().is_file() {
            let content = std::fs::read_to_string(entry.path())?;
            let preview = String::from_iter(content.lines().take(5));
            if preview.len() > 100 {
                println!("       {}", &preview[..100]);
            } else {
                println!("       {}", preview);
            }
        }
    }

    Ok(())
}

/// Handle setup skills command
pub fn handle_skills_command(_command: crate::cli::SkillsCommands) -> Result<(), EngramError> {
    use std::env;

    // Get OpenCode config directory
    let opencode_dir = env::var("HOME")
        .map(|home| PathBuf::from(home).join(".config").join("opencode"))
        .map_err(|_| EngramError::Validation("HOME environment variable not set".to_string()))?;

    let skills_dir = opencode_dir.join("skills");
    std::fs::create_dir_all(&skills_dir).map_err(EngramError::Io)?;

    // List of built-in Engram skills to install
    let skills = [
        "engram-use-engram-memory",
        "engram-delegate-to-agents",
        "engram-audit-trail",
        "engram-brainstorming",
        "engram-writing-plans",
        "engram-plan-feature",
        "engram-requesting-code-review",
        "engram-check-compliance",
        "engram-test-driven-development",
        "engram-systematic-debugging",
        "engram-subagent-driven-development",
        "engram-dispatching-parallel-agents",
    ];

    let mut installed_count = 0;

    for skill_name in skills {
        let skill_dir = skills_dir.join(skill_name);

        // Skip if skill already exists (user skill take precedence)
        if skill_dir.exists() {
            println!(
                "⚠️  Skill '{}' already exists, skipping (user skill preserved)",
                skill_name
            );
            continue;
        }

        std::fs::create_dir_all(&skill_dir).map_err(EngramError::Io)?;

        // Create a basic SKILL.md file for each skill
        let skill_content = format!(
            include_str!("../skill_templates/basic_skill.md"),
            skill_name, skill_name
        );
        let skill_file = skill_dir.join("SKILL.md");
        std::fs::write(&skill_file, skill_content).map_err(EngramError::Io)?;

        println!("✅ Installed skill: {}", skill_name);
        installed_count += 1;
    }

    println!();
    println!("🎉 Skills setup complete!");
    println!("📁 Skills installed to: {:?}", skills_dir);
    println!("📊 Total skills installed: {}", installed_count);
    println!();
    println!("💡 Skills are now available with 'engram:' prefix");
    println!("   Example: skill() tool with 'engram:use-engram-memory'");
    println!();
    println!("📖 To use skills:");
    println!("   1. Restart your agent session to reload skills");
    println!("   2. Use skill() tool with skill name");

    Ok(())
}
