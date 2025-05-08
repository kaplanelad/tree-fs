#![cfg(feature = "yaml")]

use std::fs;
use std::path::PathBuf;

#[test]
fn test_from_yaml_file() {
    let yaml_path = PathBuf::from("tests/fixtures/tree.yaml");
    let tree = tree_fs::from_yaml_file(&yaml_path).expect("Failed to create tree from YAML file");

    // Verify that files from YAML exist
    assert!(tree.root.join("foo.json").exists());
    assert!(tree.root.join("folder/bar.yaml").exists());

    // Verify content matches what's defined in the YAML
    assert_eq!(
        fs::read_to_string(tree.root.join("foo.json")).expect("Failed to read foo.json content"),
        "{ \"foo\": \"bar\" }\n"
    );
    assert_eq!(
        fs::read_to_string(tree.root.join("folder").join("bar.yaml"))
            .expect("Failed to read bar.yaml content"),
        "foo: bar\n"
    );
}

#[test]
fn test_from_yaml_str() {
    let yaml_content = r"
        override_file: false
        entries:
        - path: foo.txt
          type: text_file
          content: foo
        - path: folder/bar.txt
          type: text_file
          content: bar
    ";

    let tree =
        tree_fs::from_yaml_str(yaml_content).expect("Failed to create tree from YAML string");

    // Verify that files specified in the YAML string exist
    assert!(tree.root.join("foo.txt").exists());
    assert!(tree.root.join("folder/bar.txt").exists());

    // Verify content matches YAML definition
    assert_eq!(
        fs::read_to_string(tree.root.join("foo.txt")).expect("Failed to read foo.txt content"),
        "foo"
    );
    assert_eq!(
        fs::read_to_string(tree.root.join("folder/bar.txt"))
            .expect("Failed to read bar.txt content"),
        "bar"
    );
}

#[test]
fn test_yaml_with_empty_files() {
    let yaml_content = r"
        entries:
        - path: empty.txt
          type: empty_file
        - path: nested/also-empty.txt
          type: empty_file
    ";

    let tree = tree_fs::from_yaml_str(yaml_content)
        .expect("Failed to create tree with empty files from YAML");

    // Verify empty files exist
    assert!(tree.root.join("empty.txt").exists());
    assert!(tree.root.join("nested/also-empty.txt").exists());

    // Verify they are indeed empty
    assert_eq!(
        fs::read_to_string(tree.root.join("empty.txt")).expect("Failed to read empty.txt"),
        ""
    );
    assert_eq!(
        fs::read_to_string(tree.root.join("nested/also-empty.txt"))
            .expect("Failed to read also-empty.txt"),
        ""
    );
}

#[test]
fn test_yaml_with_directories() {
    let yaml_content = r"
        entries:
        - path: empty-dir
          type: directory
        - path: nested/dir/structure
          type: directory
    ";

    let tree = tree_fs::from_yaml_str(yaml_content)
        .expect("Failed to create tree with directories from YAML");

    // Verify directories exist
    assert!(tree.root.join("empty-dir").exists());
    assert!(tree.root.join("empty-dir").is_dir());
    assert!(tree.root.join("nested/dir/structure").exists());
    assert!(tree.root.join("nested/dir/structure").is_dir());
}

#[test]
fn test_yaml_drop_behavior() {
    // Test with drop: true (default)
    let yaml_content = r"
        entries:
        - path: file.txt
          type: text_file
          content: test content
    ";

    let tree =
        tree_fs::from_yaml_str(yaml_content).expect("Failed to create tree with default drop=true");

    let root_path = tree.root.clone();
    assert!(root_path.exists());

    // Drop the tree, root should be deleted
    drop(tree);
    assert!(!root_path.exists());

    // Test with drop: false
    let yaml_content = r"
        drop: false
        entries:
        - path: file.txt
          type: text_file
          content: test content
    ";

    let tree = tree_fs::from_yaml_str(yaml_content).expect("Failed to create tree with drop=false");

    let root_path = tree.root.clone();
    assert!(root_path.exists());

    // Drop the tree, root should still exist
    drop(tree);
    assert!(root_path.exists());

    // Clean up
    let _ = fs::remove_dir_all(root_path);
}

#[test]
fn test_yaml_custom_root() {
    // Create a custom root in the temp directory
    let custom_root = std::env::temp_dir().join("tree-fs-yaml-test-root");
    let _ = fs::create_dir_all(&custom_root);

    let yaml_content = format!(
        r"
        root: {}
        entries:
        - path: custom-root-file.txt
          type: text_file
          content: in custom root
    ",
        custom_root.to_string_lossy()
    );

    let tree =
        tree_fs::from_yaml_str(&yaml_content).expect("Failed to create tree with custom root");

    assert_eq!(tree.root, custom_root);
    assert!(tree.root.join("custom-root-file.txt").exists());
    assert_eq!(
        fs::read_to_string(tree.root.join("custom-root-file.txt"))
            .expect("Failed to read file in custom root"),
        "in custom root"
    );

    // Clean up
    let _ = fs::remove_dir_all(&custom_root);
}
