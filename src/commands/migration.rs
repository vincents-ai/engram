//! Migration command handlers

use engram::cli;
use engram::error::EngramError;
use engram::migration::Migration;
use engram::storage::GitRefsStorage;

/// Handle migration command
pub fn handle_migration_command() -> Result<(), EngramError> {
    let args: Vec<String> = std::env::args().collect();
    let dry_run = args.contains(&String::from("--dry-run"));
    let backup_only = args.contains(&String::from("--backup-only"));
    let flatten_refs = args.contains(&String::from("--flatten-refs"));

    // --flatten-refs: scan git refs for triple-nested data blobs and flatten them.
    if flatten_refs {
        println!("Scanning refs for triple-nested data blobs...");
        if dry_run {
            println!("DRY RUN: no changes will be written");
        }

        let storage = GitRefsStorage::new(".", "default")?;
        match storage.flatten_nested_refs(dry_run) {
            Ok(stats) => {
                println!("\nFlatten Refs Summary:");
                println!("  Refs scanned:   {}", stats.refs_scanned);
                println!("  Nested found:   {}", stats.nested_found);
                println!("  Refs rewritten: {}", stats.refs_rewritten);
                if dry_run && stats.nested_found > 0 {
                    println!("\n(dry-run) Run without --dry-run to apply fixes");
                } else if stats.nested_found == 0 {
                    println!("\nAll refs are already flat. Nothing to do.");
                } else {
                    println!("\nDone. {} ref(s) flattened.", stats.refs_rewritten);
                }
            }
            Err(e) => {
                eprintln!("Failed to flatten refs: {}", e);
                return Err(e);
            }
        }
        return Ok(());
    }

    if backup_only {
        println!("Creating backup of .engram directory...");
        let migration = Migration::new(".", "default", true, backup_only)?;
        migration.create_backup()?;
        println!("Backup completed successfully");
        return Ok(());
    }

    let mut migration = Migration::new(".", "default", dry_run, false)?;

    // Pre-flight validation
    if let Err(e) = Migration::validate_migration_readiness(".") {
        eprintln!("Migration pre-check failed: {}", e);
        return Err(e);
    }

    println!("Starting migration from dual-repository to Git refs storage");
    if dry_run {
        println!("DRY RUN: No changes will be made");
    } else {
        println!("MIGRATION: Converting data to Git refs storage");
    }

    match migration.execute() {
        Ok(stats) => {
            println!("\nMigration Summary:");
            println!("  Total processed: {}", stats.entities_processed);
            println!("  Successfully migrated: {}", stats.entities_migrated);
            if stats.entities_failed > 0 {
                println!("  Failed: {}", stats.entities_failed);
            }
            if !dry_run && stats.entities_migrated > 0 {
                println!("\nBackup created at: .engram_backup_<timestamp>");
            }
            println!("\nMigration completed successfully!");
        }
        Err(e) => {
            eprintln!("Migration failed: {}", e);
        }
    }

    Ok(())
}

/// Handle the `migrate` subcommand family.
pub fn handle_migrate_command(command: cli::MigrateCommands) -> Result<(), EngramError> {
    match command {
        cli::MigrateCommands::TripleNesting { dry_run } => {
            if dry_run {
                println!("DRY RUN: scanning for triple-nested blobs (no writes)");
            } else {
                println!("Scanning refs for triple-nested data blobs...");
            }

            let storage = GitRefsStorage::new(".", "default")?;
            let report = engram::migration::migrate_triple_nesting(&storage, dry_run)?;

            println!("\nMigrate triple-nesting summary:");
            println!("  Refs scanned:   {}", report.refs_scanned);
            println!("  Nested found:   {}", report.nested_found);
            println!("  Refs rewritten: {}", report.refs_rewritten);

            if report.nested_found == 0 {
                println!("\nAll refs are already flat. Nothing to do.");
            } else if dry_run {
                println!(
                    "\n(dry-run) Run without --dry-run to apply fixes ({} ref(s) would be rewritten)",
                    report.nested_found
                );
            } else {
                println!("\nDone. {} ref(s) flattened.", report.refs_rewritten);
            }
        }
    }
    Ok(())
}
