use std::fs;
use tree_fs::{Settings, TreeBuilder};

#[test]
fn test_file_settings() {
    let tree = TreeBuilder::default()
        .add_file_with_settings(
            "config.json",
            "{\"readonly\": true}",
            Settings::new().readonly(true),
        )
        .add_empty_file_with_settings("data.txt", Settings::new().readonly(false))
        .create()
        .expect("Failed to create tree with settings");

    // Verify settings were applied
    let readonly_perms = fs::metadata(tree.root.join("config.json"))
        .expect("Failed to get metadata")
        .permissions();
    assert!(readonly_perms.readonly());

    let writable_perms = fs::metadata(tree.root.join("data.txt"))
        .expect("Failed to get metadata")
        .permissions();
    assert!(!writable_perms.readonly());
}

#[test]
fn test_readonly_convenience_methods() {
    let tree = TreeBuilder::default()
        .add_readonly_file("readonly.txt", "read-only content")
        .add_readonly_empty_file("empty.txt")
        .create()
        .expect("Failed to create tree with readonly methods");

    // Verify both files are readonly
    let text_perms = fs::metadata(tree.root.join("readonly.txt"))
        .expect("Failed to get metadata")
        .permissions();
    assert!(text_perms.readonly());

    let empty_perms = fs::metadata(tree.root.join("empty.txt"))
        .expect("Failed to get metadata")
        .permissions();
    assert!(empty_perms.readonly());
}
