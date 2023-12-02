use std::fs;
use std::path::PathBuf;
use tree_fs::Tree;

#[test]
fn can_create_tree_from_builder() {
    let tree_res = Tree::default()
        .add("foo.txt", "foo")
        .add_empty("folder-a/folder-b/bar.txt")
        .create()
        .unwrap();

    assert!(tree_res.join("foo.txt").exists());
    assert!(tree_res
        .join("folder-a")
        .join("folder-b")
        .join("bar.txt")
        .exists());

    assert_eq!(fs::read_to_string(tree_res.join("foo.txt")).unwrap(), "foo");
    assert_eq!(
        fs::read_to_string(tree_res.join("folder-a").join("folder-b").join("bar.txt")).unwrap(),
        ""
    );
}

#[test]
fn can_create_tree_from_yaml_file() {
    let yaml_path = PathBuf::from("tests/fixtures/tree.yaml");
    let tree_res: PathBuf = tree_fs::from_yaml_file(&yaml_path).unwrap();

    assert!(tree_res.join("foo.json").exists());

    assert_eq!(
        fs::read_to_string(tree_res.join("foo.json")).unwrap(),
        "{ \"foo\": \"bar\" }\n"
    );
    assert_eq!(
        fs::read_to_string(tree_res.join("folder").join("bar.yaml")).unwrap(),
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

    assert!(tree_res.join("foo.txt").exists());
    assert!(tree_res.join("folder").join("bar.txt").exists());

    assert_eq!(fs::read_to_string(tree_res.join("foo.txt")).unwrap(), "foo");
    assert_eq!(
        fs::read_to_string(tree_res.join("folder").join("bar.txt")).unwrap(),
        "bar"
    );
}
