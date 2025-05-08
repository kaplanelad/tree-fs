use std::fs;

fn main() {
    println!("--- Tree from YAML String Example ---");
    let comprehensive_yaml = r#"
        root: "custom_yaml_str_root" # Example of setting a custom root via YAML
        drop: false                 # Don't auto-delete when dropped
        override_file: true         # Override existing files
        entries:
        - path: text_file.txt       # Text file
          type: text_file
          content: Content from YAML string
        - path: empty_file.txt      # Empty file
          type: empty_file
        - path: directory           # Directory
          type: directory
        - path: directory/nested.txt
          type: text_file
          content: Nested in directory from string
        - path: secrets.conf
          type: text_file
          content: "api_key=verysecret"
          settings:
            readonly: true # This file will be read-only
    "#;

    let tree = tree_fs::from_yaml_str(comprehensive_yaml)
        .expect("Failed to create comprehensive tree from YAML string");

    println!("Comprehensive tree created at: {}", tree.root.display());
    println!("  (This tree will NOT be auto-deleted and uses a custom root name if not in temp)");

    let readonly_secret_path = tree.root.join("secrets.conf");
    match fs::metadata(&readonly_secret_path) {
        Ok(metadata) => {
            assert!(
                metadata.permissions().readonly(),
                "secrets.conf should be read-only"
            );
            println!("Verified: {} is read-only.", readonly_secret_path.display());
        }
        Err(e) => eprintln!(
            "Could not get metadata for {}: {}",
            readonly_secret_path.display(),
            e
        ),
    }

    println!(
        "Example finished. If root was not in temp, manually delete: {}",
        tree.root.display()
    );
}
