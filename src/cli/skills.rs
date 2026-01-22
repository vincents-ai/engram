//! Skills management commands
//!
//! Provides commands for listing and showing skills from ENGRAM_SKILLS_PATH.

use clap::Subcommand;
use std::fs;
use std::path::PathBuf;

/// Skills commands
#[derive(Debug, Subcommand)]
pub enum SkillsCommands {
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

/// List all skills in the skills directory
pub fn list_skills(format: &str) -> Result<(), std::io::Error> {
    let skills_path = get_skills_path();

    if !skills_path.exists() {
        println!("Skills directory not found: {:?}", skills_path);
        println!("Set ENGRAM_SKILLS_PATH environment variable.");
        return Ok(());
    }

    let entries = fs::read_dir(&skills_path)?;

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
                        let content = fs::read_to_string(&skill_file)?;
                        content.lines().next().unwrap_or("").to_string()
                    } else {
                        "(no description)".to_string()
                    };
                    println!("\n  [{}]", name);
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
        let entries = fs::read_dir(&skills_path)?;
        let found_path: Option<PathBuf> = entries
            .flatten()
            .filter(|e| e.path().is_dir())
            .find(|e| e.file_name().to_string_lossy().to_lowercase() == name.to_lowercase())
            .map(|e| e.path());

        if let Some(path) = found_path {
            println!("Found skill: {} (matched {})", path.file_name().unwrap_or_default().to_string_lossy(), name);
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
    println!("\nSkill: {}", actual_path.file_name().unwrap_or_default().to_string_lossy());
    println!("======");

    let entries = fs::read_dir(&actual_path)?;
    for entry in entries.flatten() {
        let file_name = entry.file_name().to_string_lossy().into_owned();
        let file_type = if entry.path().is_dir() { "[DIR]" } else { "[FILE]" };
        println!("  {} {}", file_type, file_name);

        if entry.path().is_file() {
            let content = fs::read_to_string(entry.path())?;
            let preview: String = content.lines().take(5).collect();
            if preview.len() > 100 {
                println!("       {}", &preview[..100]);
            } else {
                println!("       {}", preview);
            }
        }
    }

    Ok(())
}
