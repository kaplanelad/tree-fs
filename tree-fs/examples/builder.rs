use tree_fs::Tree;

fn main() {
    let res = Tree::default()
        .add("test/foo.txt", "bar")
        .add_empty("test/folder-a/folder-b/bar.txt")
        .create();

    match res {
        Ok(res) => {
            println!("created successfully in {}", res.display());
        }
        Err(err) => {
            println!("creating tree files finish with errors: {err}");
        }
    }
}
