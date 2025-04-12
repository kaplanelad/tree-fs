use std::fs;
use std::path::PathBuf;
use tree_fs::{Tree, TreeBuilder};

#[test]
fn can_create_tree_from_builder() {
    let tree_res = TreeBuilder::default()
        .add_text("foo.txt", "foo")
        .add_empty("folder-a/folder-b/bar.txt")
        .create()
        .unwrap();

    assert!(tree_res.root.join("foo.txt").exists());
    assert!(tree_res
        .root
        .join("folder-a")
        .join("folder-b")
        .join("bar.txt")
        .exists());

    assert_eq!(
        fs::read_to_string(tree_res.root.join("foo.txt")).unwrap(),
        "foo"
    );
    assert_eq!(
        fs::read_to_string(
            tree_res
                .root
                .join("folder-a")
                .join("folder-b")
                .join("bar.txt")
        )
        .unwrap(),
        ""
    );
}

#[test]
fn can_create_tree_from_yaml_file() {
    let yaml_path = PathBuf::from("tests/fixtures/tree.yaml");
    let tree_res: Tree = tree_fs::from_yaml_file(&yaml_path).unwrap();

    assert!(tree_res.root.join("foo.json").exists());

    assert_eq!(
        fs::read_to_string(tree_res.root.join("foo.json")).unwrap(),
        "{ \"foo\": \"bar\" }\n"
    );
    assert_eq!(
        fs::read_to_string(tree_res.root.join("folder").join("bar.yaml")).unwrap(),
        "foo: bar\n"
    );
}

#[test]
fn can_create_tree_from_yaml_str() {
    let yaml_content = r"
        override_file: false
        files:
        - path: foo.txt
          content: foo
        - path: folder/bar.txt
          content: bar

    ";

    let tree_res = tree_fs::from_yaml_str(yaml_content).unwrap();

    assert!(tree_res.root.join("foo.txt").exists());
    assert!(tree_res.root.join("folder").join("bar.txt").exists());

    assert_eq!(
        fs::read_to_string(tree_res.root.join("foo.txt")).unwrap(),
        "foo"
    );
    assert_eq!(
        fs::read_to_string(tree_res.root.join("folder").join("bar.txt")).unwrap(),
        "bar"
    );
}

#[test]
fn can_create_build_tree_with_drop() {
    let tree_fs = TreeBuilder::default()
        .add_text("foo.txt", "foo")
        .add_empty("folder-a/folder-b/bar.txt")
        .drop(true)
        .create()
        .unwrap();

    assert!(tree_fs.root.exists());

    let root_path = tree_fs.root.clone();
    drop(tree_fs);
    assert!(!root_path.exists());
}

#[test]
fn can_create_tree_from_yaml_str_with_drop() {
    let yaml_content = r"
        drop: true
        override_file: false
        files:
        - path: foo.txt
          content: foo
        - path: folder/bar.txt
          content: bar

    ";

    let tree_fs = tree_fs::from_yaml_str(yaml_content).unwrap();

    assert!(tree_fs.root.exists());

    let root_path = tree_fs.root.clone();
    drop(tree_fs);
    assert!(!root_path.exists());
}
