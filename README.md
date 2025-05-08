[![crate](https://img.shields.io/crates/v/tree-fs.svg)](https://crates.io/crates/tree-fs)
[![docs](https://docs.rs/tree-fs/badge.svg)](https://docs.rs/tree-fs)

# tree-fs

`tree-fs` is a Rust library designed to simplify the creation and management of **temporary file system structures**. Its core feature is creating directories and files programmatically, which are **automatically cleaned up** when the managing object goes out of scope. This makes it ideal for scenarios where you need a predictable, temporary workspace, such as testing, build processes, or data generation tasks.

## Why `tree-fs`?

Manually creating and cleaning up temporary file structures can be cumbersome and error-prone. You might need to:

- Create specific directory layouts.
- Populate files with predefined content.
- Ensure files have particular permissions (e.g., a config file that should be read-only).
- Reliably clean up all temporary files and directories afterwards.

`tree-fs` automates these tasks, making your code cleaner and more reliable. Consider these use cases:

- **Testing**: Set up fixtures for tests that interact with the file system, ensuring a clean state for each test run.
- **Configuration Management**: Applications that load settings from files (e.g., `config/app.json`) can use `tree-fs` to generate temporary config files for specific runs.
- **Temporary Workspaces**: Create a temporary space for data processing, intermediate file generation, or running external tools that expect a certain directory structure.
- **Build Scripts & Scaffolding**: Generate temporary project structures or configuration files needed during a build process or for code generation tasks.
- **Permissions Handling**: Create files with specific permissions (like read-only) to test or simulate real-world scenarios.

## Features

- **Fluent Builder API**: Programmatically define your file tree.
- **YAML Configuration**: Define trees using YAML files or strings (requires the `yaml` feature).
- **Temporary Directories**: Trees are typically created in a system temporary folder.
- **Automatic Cleanup**: Temporary trees are automatically deleted when the `Tree` instance goes out of scope (this can be disabled).
- **File Contents**: Easily specify text content for files.
- **Empty Files & Directories**: Create empty files or entire directory structures.
- **File Settings**: Set file attributes, such as read-only permissions.

## Installation

Add `tree-fs` to your `Cargo.toml`:

```toml
[dependencies]
tree-fs = "0.3" # Replace with the latest version
```

## Usage

### 1. Using the Builder API

The `TreeBuilder` provides a fluent interface to construct your desired file system structure.

```rust
use tree_fs::{TreeBuilder, Settings};
let tree = TreeBuilder::default()
    .add_file("config/app.conf", "host = localhost")
    .add_empty_file("logs/app.log")
    .add_directory("data/raw")
    .add_file_with_settings(
        "secrets/api.key",
        "supersecretkey",
        Settings::new().readonly(true)
    )
    .create()
    .expect("create tree fs");
println!("Created a complex tree in: {}", tree.root.display());

// You can verify the readonly status (this requires std::fs)
// let key_path = tree.root.join("secrets/api.key");
// let metadata = std::fs::metadata(key_path).unwrap();
// assert!(metadata.permissions().readonly());
```

For a more comprehensive example covering custom root directories, overriding files, and various file types, see `examples/builder.rs`.

You can disable this behavior using `.drop(false)` on the builder if you need the files to persist.

### 2. Using YAML (requires the `yaml` feature)

To use YAML, enable the `yaml` feature for `tree-fs` in your `Cargo.toml`.

#### From a YAML File

You can define your file tree in a YAML file. This is useful for complex or reusable structures.

**Example `tests/fixtures/tree.yaml`:**

```yaml
override_file: false
entries:
  - path: foo.json
    type: text_file
    content: |
      { "foo": "bar" }
  - path: folder/bar.yaml
    type: text_file
    content: |
      foo: bar
  - path: readonly_config.ini
    type: text_file
    content: |
      ; Sample read-only INI file
      [general]
      setting = value
    settings:
      readonly: true
```

**Rust code to load the YAML file:**

See the example file `examples/yaml-file.rs` for how to load this structure using `tree_fs::from_yaml_file`.

#### From a YAML String

For simpler or inline definitions, you can provide the YAML structure as a string.

See the example file `examples/yaml-str.rs` for how to load a structure from a YAML string using `tree_fs::from_yaml_str`, including defining settings like `readonly`.

## Contributing

Contributions, issues, and feature requests are welcome! Feel free to check the [issues page](https://github.com/kaplanelad/tree-fs/issues).

## License

This project is licensed under the Apache-2.0 License - see the [LICENSE](LICENSE) file for details.
