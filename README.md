# tree-fs

Oftentimes, testing scenarios involve interactions with the file system. `tree-fs` provides a convenient solution for creating file system trees tailored to the needs of your tests. This library offers:

- An easy way to generate a tree with recursive paths.
- Tree creation within a temporary folder.
- The ability to create a tree using either YAML or a builder.

# Usage

See usage example (here)[./examples]

### From builder

```rust
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

```

### Using a YAML File

```rust
use std::path::PathBuf;

fn main() {
    let yaml_path = PathBuf::from("tree-create/tests/fixtures/tree.yaml");

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

```

### Using a YAML String

```rust
use std::fs;

fn main() {
    let content =
        fs::read_to_string("tree-create/tests/fixtures/tree.yaml").expect("Unable to read file");

    let res = tree_fs::from_yaml_str(&content);

    match res {
        Ok(res) => {
            println!("created successfully in {}", res.display());
        }
        Err(err) => {
            println!("creating tree files finish with errors: {err}");
        }
    }
}
```
