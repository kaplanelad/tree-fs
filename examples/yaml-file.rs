use std::fs;
use std::path::PathBuf;

fn main() {
    // Make sure to use the correct relative path to find the YAML file
    let yaml_path = PathBuf::from("tests/fixtures/tree.yaml");

    let tree = tree_fs::from_yaml_file(&yaml_path).expect("Failed to create tree from YAML file");

    println!("--- Tree from YAML File Example ---");
    println!("Tree created successfully from: {}", yaml_path.display());
    println!("Root directory: {}", tree.root.display());
    println!("Files created (check tests/fixtures/tree.yaml for full structure including a readonly file):");
    println!("  - {}", tree.root.join("foo.json").display());
    println!("  - {}", tree.root.join("folder/bar.yaml").display());
    println!("  - {}", tree.root.join("readonly_config.ini").display());

    // Optional: Verify read-only status if desired in an example
    let readonly_path = tree.root.join("readonly_config.ini");
    if readonly_path.exists() {
        match fs::metadata(&readonly_path) {
            Ok(metadata) => {
                if metadata.permissions().readonly() {
                    println!("Verified: {} is read-only.", readonly_path.display());
                } else {
                    println!(
                        "Note: {} was expected to be read-only but isn't.",
                        readonly_path.display()
                    );
                }
            }
            Err(e) => eprintln!(
                "Could not get metadata for {}: {}",
                readonly_path.display(),
                e
            ),
        }
    } else {
        eprintln!("File not found: {}", readonly_path.display());
    }
}
