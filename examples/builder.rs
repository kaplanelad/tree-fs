use tree_fs::TreeBuilder;

fn main() {
    let tree_fs = TreeBuilder::default()
        .add("test/foo.txt", "bar")
        .add_empty("test/folder-a/folder-b/bar.txt")
        .create()
        .expect("create tree fs");

    println!("created successfully in {}", tree_fs.root.display());
}
