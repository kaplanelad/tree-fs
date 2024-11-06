fn main() {
    let content = r#"
        override_file: false
        files:
          - path: foo.json
            content: |
              { "foo;": "bar" }
          - path: folder/bar.yaml
            content: |
              foo: bar
    "#;

    let tree_fs = tree_fs::from_yaml_str(content).expect("create tree fs");

    println!("created successfully in {}", tree_fs.root.display());
}
