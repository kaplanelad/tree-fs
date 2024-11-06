use std::path::PathBuf;

fn main() {
    let yaml_path = PathBuf::from("tree-fs/tests/fixtures/tree.yaml");
    let tree_fs = tree_fs::from_yaml_file(&yaml_path).expect("create tree fs");
    println!("created successfully in {}", tree_fs.root.display());
}
