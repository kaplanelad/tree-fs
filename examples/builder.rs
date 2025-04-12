use tree_fs::TreeBuilder;

fn main() {
    let tree_fs = TreeBuilder::default()
        .add_text("test/foo.txt", "bar")
        .add_empty("test/folder-a/folder-b/bar.txt")
        .add_file("test_file.rs", file!())
        .create()
        .expect("create tree fs");

    println!("created successfully in {}", tree_fs.root().display());

    let path = tree_fs.root().to_path_buf();

    assert!(path.exists());

    drop(tree_fs);

    assert!(!path.exists());
}
