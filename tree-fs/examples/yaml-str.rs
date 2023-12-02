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

    let res = tree_fs::from_yaml_str(content);

    match res {
        Ok(res) => {
            println!("created successfully in {}", res.display());
        }
        Err(err) => {
            println!("creating tree files finish with errors: {err}");
        }
    }
}
