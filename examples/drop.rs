use tree_fs::TreeBuilder;

fn main() {
    let tree_fs = TreeBuilder::default()
        .add_text("test/foo.txt", "bar")
        .add_empty("test/folder-a/folder-b/bar.txt")
        .drop(true)
        .create()
        .expect("create tree fs");

    println!("created successfully in {}", tree_fs.root.display());

    let path = tree_fs.root.clone();
    assert!(path.exists());

    drop(tree_fs);
    assert!(!path.exists());
}
