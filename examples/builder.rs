use std::fs;
use tree_fs::{Settings, TreeBuilder};

fn main() {
    println!("--- Comprehensive TreeBuilder Example ---");

    // 1. Default behavior: creates in a temporary directory, auto-drops
    let default_tree = TreeBuilder::default()
        .add_file("default_file.txt", "Content in default temp dir")
        .create()
        .expect("Failed to create default tree");
    println!("Default tree created in: {}", default_tree.root.display());
    let default_path = default_tree.root.clone();
    assert!(default_path.exists());
    drop(default_tree);
    assert!(
        !default_path.exists(),
        "Default tree should be auto-deleted."
    );
    println!("Default tree auto-deleted successfully.");

    // 2. Custom root, various file types, and settings
    let custom_root_path = std::env::temp_dir().join("tree_fs_custom_example");
    // Clean up previous run if any, for idempotency of example
    if custom_root_path.exists() {
        let _ = fs::remove_dir_all(&custom_root_path);
    }

    let complex_tree = TreeBuilder::default()
        .root_folder(&custom_root_path) // Custom root
        .add_file("project/README.md", "# My Project")
        .add_empty_file("project/.gitignore")
        .add_directory("project/src")
        .add_file("project/src/main.rs", "fn main() { println!(\"Hello!\"); }")
        .add_directory("project/config")
        .add_file_with_settings(
            "project/config/prod.json",
            "{ \"api_key\": \"prod_secret\" }",
            Settings::new().readonly(true), // Read-only setting
        )
        .add_readonly_file("project/config/default.json", "{ \"timeout\": 5000 }")
        .override_file(true) // Allow overwriting if files exist (e.g. from previous run if not cleaned)
        .drop(false) // Do not auto-delete this tree
        .create()
        .expect("Failed to create complex tree");

    println!("Complex tree created at: {}", complex_tree.root.display());
    println!("  (This tree will NOT be auto-deleted)");

    // Verify read-only status
    let readonly_config_path = complex_tree.root.join("project/config/prod.json");
    match fs::metadata(&readonly_config_path) {
        Ok(metadata) => {
            assert!(
                metadata.permissions().readonly(),
                "prod.json should be read-only"
            );
            println!("Verified: {} is read-only.", readonly_config_path.display());
        }
        Err(e) => eprintln!(
            "Could not get metadata for {}: {}",
            readonly_config_path.display(),
            e
        ),
    }

    // Verify another readonly file
    let default_config_path = complex_tree.root.join("project/config/default.json");
    match fs::metadata(&default_config_path) {
        Ok(metadata) => {
            assert!(
                metadata.permissions().readonly(),
                "default.json should be read-only"
            );
            println!("Verified: {} is read-only.", default_config_path.display());
        }
        Err(e) => eprintln!(
            "Could not get metadata for {}: {}",
            default_config_path.display(),
            e
        ),
    }

    println!(
        "Example finished. To clean up, manually delete: {}",
        custom_root_path.display()
    );
}
