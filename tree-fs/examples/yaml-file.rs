use std::path::PathBuf;

fn main() {
    let yaml_path = PathBuf::from("tree-fs/tests/fixtures/tree.yaml");

    let res = tree_fs::from_yaml_file(&yaml_path);

    match res {
        Ok(res) => {
            println!("created successfully in {}", res.display());
        }
        Err(err) => {
            println!("creating tree files finish with errors: {err}");
        }
    }
}
