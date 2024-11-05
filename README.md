# tree-fs

Oftentimes, testing scenarios involve interactions with the file system. `tree-fs` provides a convenient solution for creating file system trees tailored to the needs of your tests. This library offers:

- An easy way to generate a tree with recursive paths.
- Tree creation within a temporary folder.
- The ability to create a tree using either YAML or a builder.

## Usage

### From builder
With the builder API, you can define file paths and contents in a structured way. Here’s how to create a tree with the builder:

<!-- <snip id="example-builder" inject_from="code" strip_prefix="/// " template="rust"> -->
```rust
use tree_fs::TreeBuilder;
let tree_fs = TreeBuilder::default()
    .add("test/foo.txt", "bar")
    .add_empty("test/folder-a/folder-b/bar.txt")
    .create()
    .expect("create tree fs");
println!("created successfully in {}", tree_fs.root.display());
```
<!-- </snip> -->

### Drop folder 
When the `tree_fs` instance is dropped, the temporary folder and its contents are automatically deleted, which is particularly useful for tests that require a clean state.

<!-- <snip id="example-drop" inject_from="code" strip_prefix="///" template="rust"> -->
```rust
 use tree_fs::TreeBuilder;
 let tree_fs = TreeBuilder::default()
      .add("test/foo.txt", "bar")
      .add_empty("test/folder-a/folder-b/bar.txt")
      .drop(true)
      .create()
      .expect("create tree fs");

 println!("created successfully in {}", tree_fs.root.display());

 let path = tree_fs.root.clone();
 assert!(path.exists());

 drop(tree_fs);
 assert!(!path.exists());
```
<!-- </snip> -->

### Using a YAML File
You can define your file tree structure in a YAML file, which is then loaded and created by `tree-fs`. This method is great for more complex, predefined directory structures.


<!-- <snip id="example-from-yaml-file" inject_from="code" strip_prefix="/// " template="rust"> -->
```rust
use std::path::PathBuf;
let yaml_path = PathBuf::from("tests/fixtures/tree.yaml");
let tree_fs = tree_fs::from_yaml_file(&yaml_path).expect("create tree fs");
assert!(tree_fs.root.exists())
```
<!-- </snip> -->

### Using a YAML String
Alternatively, you can provide the YAML content as a string, which is particularly useful for inline definitions within test code.

<!-- <snip id="example-from-yaml-str" inject_from="code" strip_prefix="/// " template="rust"> -->
```rust
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
///
let tree_fs = tree_fs::from_yaml_str(content).expect("create tree fs");
assert!(tree_fs.root.exists())
```
<!-- </snip> -->
