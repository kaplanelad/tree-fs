use std::fs;
use tree_fs::TreeBuilder;

#[test]
fn test_default_builder() {
    let builder = TreeBuilder::default();

    // Default builder should use a temp directory
    assert!(builder.root.starts_with(std::env::temp_dir()));

    // Build an empty tree
    let tree = builder.create().expect("Failed to create default tree");
    assert!(tree.root.exists());
    assert!(tree.root.is_dir());
}

#[test]
fn test_root_folder() {
    // Create a custom root in the temp directory
    let custom_root = std::env::temp_dir().join("tree-fs-test-custom-root");
    let _ = fs::create_dir_all(&custom_root);

    let tree = TreeBuilder::default()
        .root_folder(&custom_root)
        .create()
        .expect("Failed to create tree with custom root folder");

    assert_eq!(tree.root, custom_root);
    assert!(tree.root.exists());
    assert!(tree.root.is_dir());
}

#[test]
fn test_add_file_methods() {
    let tree = TreeBuilder::default()
        .add("file1.txt", "content1") // Using add
        .add_file("file2.txt", "content2") // Using add_file alias
        .create()
        .expect("Failed to create tree with files");

    // Verify both files exist with correct content
    assert!(tree.root.join("file1.txt").exists());
    assert!(tree.root.join("file2.txt").exists());

    assert_eq!(
        fs::read_to_string(tree.root.join("file1.txt")).expect("Failed to read file1.txt"),
        "content1"
    );
    assert_eq!(
        fs::read_to_string(tree.root.join("file2.txt")).expect("Failed to read file2.txt"),
        "content2"
    );
}

#[test]
fn test_add_empty_file_methods() {
    let tree = TreeBuilder::default()
        .add_empty_file("empty1.txt") // Using add_empty_file
        .add_empty("empty2.txt") // Using add_empty alias
        .create()
        .expect("Failed to create tree with empty files");

    // Verify both files exist and are empty
    assert!(tree.root.join("empty1.txt").exists());
    assert!(tree.root.join("empty2.txt").exists());

    assert_eq!(
        fs::read_to_string(tree.root.join("empty1.txt")).expect("Failed to read empty1.txt"),
        ""
    );
    assert_eq!(
        fs::read_to_string(tree.root.join("empty2.txt")).expect("Failed to read empty2.txt"),
        ""
    );
}

#[test]
fn test_add_directory() {
    let tree = TreeBuilder::default()
        .add_directory("dir1")
        .add_directory("dir1/subdir")
        .add_file("dir1/file.txt", "content")
        .create()
        .expect("Failed to create tree with directories");

    // Verify directories exist
    assert!(tree.root.join("dir1").exists());
    assert!(tree.root.join("dir1").is_dir());
    assert!(tree.root.join("dir1/subdir").exists());
    assert!(tree.root.join("dir1/subdir").is_dir());

    // Verify file in directory
    assert!(tree.root.join("dir1/file.txt").exists());
    assert_eq!(
        fs::read_to_string(tree.root.join("dir1/file.txt"))
            .expect("Failed to read file in directory"),
        "content"
    );
}

#[test]
fn test_nested_structure() {
    let tree = TreeBuilder::default()
        .add_directory("deeply/nested/directory/structure")
        .add_file("deeply/nested/file.txt", "nested file")
        .add_empty_file("deeply/nested/directory/empty.txt")
        .create()
        .expect("Failed to create tree with nested structure");

    // Verify nested structures
    assert!(tree.root.join("deeply/nested/directory/structure").exists());
    assert!(tree.root.join("deeply/nested/directory/structure").is_dir());
    assert!(tree.root.join("deeply/nested/file.txt").exists());
    assert!(tree.root.join("deeply/nested/directory/empty.txt").exists());

    assert_eq!(
        fs::read_to_string(tree.root.join("deeply/nested/file.txt"))
            .expect("Failed to read nested file"),
        "nested file"
    );
}

#[test]
fn test_override_file() {
    // For this test, we need a consistent root directory across multiple builders
    let tree = TreeBuilder::default()
        .root_folder("override-test-dir")
        .add_file("test.txt", "original content")
        .create()
        .expect("Failed to create initial tree for override test");

    let root_path = tree.root.clone();

    // Verify original content
    assert_eq!(
        fs::read_to_string(root_path.join("test.txt"))
            .expect("Failed to read original file content"),
        "original content"
    );

    // Create a new tree with the same root but override_file set to false (default)
    let _tree2 = TreeBuilder::default()
        .root_folder(&root_path)
        .add_file("test.txt", "new content")
        .create()
        .expect("Failed to create second tree for override test");

    // File should still have original content since override is false by default
    assert_eq!(
        fs::read_to_string(root_path.join("test.txt"))
            .expect("Failed to read file with override=false"),
        "original content"
    );

    // Create a new tree with the same root but override_file set to true
    let _tree3 = TreeBuilder::default()
        .root_folder(&root_path)
        .override_file(true)
        .add_file("test.txt", "new content")
        .create()
        .expect("Failed to create third tree with override=true");

    // File should have new content since override is true
    assert_eq!(
        fs::read_to_string(root_path.join("test.txt"))
            .expect("Failed to read file after override=true"),
        "new content"
    );
}

#[test]
fn test_drop_flag() {
    // Create a tree with drop = true (default)
    let tree = TreeBuilder::default()
        .add_file("file.txt", "content")
        .create()
        .expect("Failed to create tree with default drop=true");

    let root_path = tree.root.clone();
    assert!(root_path.exists());

    // Drop the tree, root should be deleted
    drop(tree);
    assert!(!root_path.exists());

    // Create a tree with drop = false
    let tree = TreeBuilder::default()
        .root_folder("no-drop-test-dir")
        .drop(false)
        .add_file("file.txt", "content")
        .create()
        .expect("Failed to create tree with drop=false");

    let root_path = tree.root.clone();
    assert!(root_path.exists());

    // Drop the tree, root should still exist
    drop(tree);
    assert!(root_path.exists());

    // Clean up
    let _ = fs::remove_dir_all(root_path);
}
