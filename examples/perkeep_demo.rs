//! Perkeep backup/restore demonstration
//!
//! This example demonstrates the Perkeep integration concepts for
//! backing up and restoring entity data.

fn main() {
    println!("Engram Perkeep Backup/Restore Demo");
    println!("===================================\n");

    println!("📦 Perkeep Backup Features:");
    println!("1. ✅ Content-addressable storage (SHA-256)");
    println!("2. ✅ Entity type filtering (task, context, etc.)");
    println!("3. ✅ Relationship preservation");
    println!("4. ✅ Backup metadata and versioning");
    println!("5. ✅ Selective restore by blob reference");

    // Show backup commands
    println!("\n💾 Backup Commands:");
    println!("   engram perkeep backup                    # All entities");
    println!("   engram perkeep backup --entity-type task # Specific type");
    println!("   engram perkeep backup --description 'Weekly backup'");
    println!("   engram perkeep backup --include-relationships");

    // Show restore commands
    println!("\n🔄 Restore Commands:");
    println!("   engram perkeep restore                   # Latest backup");
    println!("   engram perkeep restore --blobref 'sha256-...'");
    println!("   engram perkeep restore --dry-run         # Preview only");
    println!("   engram perkeep restore --agent default   # To specific agent");

    // Show management commands
    println!("\n🛠️ Management Commands:");
    println!("   engram perkeep list                      # List backups");
    println!("   engram perkeep list --detailed");
    println!("   engram perkeep health                    # Server health");
    println!("   engram perkeep config --server 'http://localhost:3179'");

    // Show configuration
    println!("\n⚙️ Configuration:");
    println!("   PERKEEP_SERVER=http://localhost:3179");
    println!("   PERKEEP_AUTH_TOKEN=your-token (optional)");

    // Note about Perkeep server
    println!("\n📌 Perkeep Server:");
    println!("   Perkeep is a personal data store server.");
    println!("   Install: https://perkeep.org/");
    println!("   Default port: 3179");

    // Backup process
    println!("\n🔍 Backup Process:");
    println!("   1. Connect to Perkeep server");
    println!("   2. Serialize entities to JSON");
    println!("   3. Upload as blobs (content-addressed)");
    println!("   4. Create schema object tracking blobs");
    println!("   5. Store metadata (timestamps, counts)");

    // Restore process
    println!("\n🔍 Restore Process:");
    println!("   1. Fetch backup metadata");
    println!("   2. Retrieve all entity blobs");
    println!("   3. Deserialize JSON to entities");
    println!("   4. Store in Engram storage");

    // Use cases
    println!("\n💡 Use Cases:");
    println!("   • Disaster recovery");
    println!("   • Cross-machine transfer");
    println!("   • Long-term archival");
    println!("   • Version history");

    println!("\n🎯 Perkeep Integration Benefits:");
    println!("1. ✅ Content-addressable integrity");
    println!("2. ✅ Selective backup/restore");
    println!("3. ✅ Relationship preservation");
    println!("4. ✅ Metadata tracking");
    println!("5. ✅ Server-based storage");

    println!("\n💻 Perkeep CLI Integration:");
    println!("   Note: Requires running Perkeep server");
    println!("   Example: export PERKEEP_SERVER=http://localhost:3179");
    println!("   Then use: engram perkeep backup --description 'Backup'");
}
