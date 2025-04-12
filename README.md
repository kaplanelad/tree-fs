[![crate](https://img.shields.io/crates/v/tree-fs.svg)](https://crates.io/crates/tree-fs)
[![docs](https://docs.rs/tree-fs/badge.svg)](https://docs.rs/tree-fs)

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
    .add_text("test/foo.txt", "bar")
    .add_empty("test/folder-a/folder-b/bar.txt")
    .add_file("destination.rs", file!())
    .create()
    .expect("create tree fs");
println!("created successfully in {}", tree_fs.root().display());
```
<!-- </snip> -->

### Drop folder 
When the `tree_fs` instance is dropped, the temporary folder and its contents are automatically deleted, which is particularly useful for tests that require a clean state.

<!-- <snip id="example-drop" inject_from="code" strip_prefix="///" template="rust"> -->
```rust
 use tree_fs::TreeBuilder;
 let tree_fs = TreeBuilder::default()
      .add_text("test/foo.txt", "bar")
      .add_empty("test/folder-a/folder-b/bar.txt")
      .drop(true)
      .create()
      .expect("create tree fs");

 println!("created successfully in {}", tree_fs.root().display());

 let path = tree_fs.root().to_path_buf();
 assert!(path.exists());

 drop(tree_fs);
 assert!(!path.exists());
```
<!-- </snip> -->
