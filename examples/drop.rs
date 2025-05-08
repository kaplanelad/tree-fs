use tree_fs::TreeBuilder;

fn main() {
    let tree = TreeBuilder::default()
        .add_file("temp_data/file.txt", "temporary content")
        .add_empty_file("temp_data/another.tmp")
        .drop(true)
        .create()
        .expect("create tree fs for drop example");

    println!("Temporary tree created at: {}", tree.root.display());

    let path_to_check = tree.root.clone();
    assert!(path_to_check.exists(), "Directory should exist before drop");

    drop(tree);
    assert!(
        !path_to_check.exists(),
        "Directory should be deleted after drop"
    );
    println!("Drop example: Temporary tree auto-deleted successfully.");
}
