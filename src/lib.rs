#![doc = include_str!("../README.md")]

use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use thiserror::Error;

#[cfg(feature = "yaml")]
#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    YAML(#[from] serde_yaml::Error),
    #[error(transparent)]
    IO(#[from] std::io::Error),
}
#[cfg(feature = "yaml")]
pub type Result<T> = std::result::Result<T, Error>;

/// Represents a file tree structure
#[derive(Debug, Deserialize)]
pub struct Tree {
    /// Root folder where the tree will be created.
    #[serde(default = "temp_dir")]
    pub root: PathBuf,
    #[serde(default)]
    drop: bool,
}

/// Represents a file tree structure
///
/// # Examples
///
/// ```rust
// <snip id="example-builder">
/// use tree_fs::TreeBuilder;
/// let tree_fs = TreeBuilder::default()
///     .add("test/foo.txt", "bar")
///     .add_empty("test/folder-a/folder-b/bar.txt")
///     .create()
///     .expect("create tree fs");
/// println!("created successfully in {}", tree_fs.root.display());
// </snip>
/// ```
///
/// ```rust
// <snip id="example-drop">
/// use tree_fs::TreeBuilder;
/// let tree_fs = TreeBuilder::default()
///      .add("test/foo.txt", "bar")
///      .add_empty("test/folder-a/folder-b/bar.txt")
///      .drop(true)
///      .create()
///      .expect("create tree fs");
///
/// println!("created successfully in {}", tree_fs.root.display());
///
/// let path = tree_fs.root.clone();
/// assert!(path.exists());
///
/// drop(tree_fs);
/// assert!(!path.exists());
// </snip>
/// ```
#[derive(Debug, Deserialize)]
pub struct TreeBuilder {
    /// Root folder where the tree will be created.
    #[serde(default = "temp_dir")]
    pub root: PathBuf,
    /// Flag indicating whether existing files should be overridden.
    #[serde(default)]
    override_file: bool,
    /// List of file metadata entries in the tree.
    files: Vec<FileMetadata>,
    #[serde(default)]
    drop: bool,
}

/// Represents metadata for a file in the tree.
#[derive(Debug, Deserialize)]
pub struct FileMetadata {
    /// Path of the file relative to the root folder.
    pub path: PathBuf,
    /// Optional content to be written to the file.
    pub content: Option<String>,
}

impl Default for TreeBuilder {
    /// Creates a default `Tree` instance with an empty file list,
    fn default() -> Self {
        Self {
            files: vec![],
            override_file: false,
            root: temp_dir(),
            drop: false,
        }
    }
}

impl Drop for Tree {
    fn drop(&mut self) {
        if self.drop {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }
}

impl TreeBuilder {
    /// Sets the root folder where the tree will be created.
    #[must_use]
    pub fn root_folder<P: AsRef<Path>>(mut self, dir: P) -> Self {
        self.root = dir.as_ref().to_path_buf();
        self
    }

    /// Sets the `drop` flag, indicating whether to automatically delete the temporary folder when the `tree_fs` instance is dropped
    #[must_use]
    pub const fn drop(mut self, yes: bool) -> Self {
        self.drop = yes;
        self
    }

    /// Sets the `override_file` flag, indicating whether existing files should be overridden.
    #[must_use]
    pub const fn override_file(mut self, yes: bool) -> Self {
        self.override_file = yes;
        self
    }

    /// Adds a file with content to the tree.
    #[must_use]
    pub fn add<P: AsRef<Path>>(mut self, path: P, content: &str) -> Self {
        self.files.push(FileMetadata {
            path: path.as_ref().to_path_buf(),
            content: Some(content.to_string()),
        });
        self
    }

    /// Adds a file with a empty content.
    #[must_use]
    pub fn add_empty<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.files.push(FileMetadata {
            path: path.as_ref().to_path_buf(),
            content: None,
        });
        self
    }

    /// Creates the file tree by generating files and directories based on the specified metadata.
    ///
    /// # Errors
    ///
    /// Returns an `std::io::Result` indicating success or failure in creating the file tree.
    pub fn create(&self) -> std::io::Result<Tree> {
        if !self.root.exists() {
            std::fs::create_dir_all(&self.root)?;
        }
        for file in &self.files {
            let dest_file = self.root.join(&file.path);
            if !self.override_file && dest_file.exists() {
                continue;
            }

            if let Some(parent_dir) = Path::new(&dest_file).parent() {
                std::fs::create_dir_all(parent_dir)?;
            }

            let mut new_file = File::create(&dest_file)?;
            if let Some(content) = &file.content {
                new_file.write_all(content.as_bytes())?;
            }
        }

        Ok(Tree {
            root: self.root.clone(),
            drop: self.drop,
        })
    }
}

#[cfg(feature = "yaml")]
/// Creates a file tree based on the content of a YAML file.
///
/// # Examples
///
/// ```rust
// <snip id="example-from-yaml-file">
/// use std::path::PathBuf;
/// let yaml_path = PathBuf::from("tests/fixtures/tree.yaml");
/// let tree_fs = tree_fs::from_yaml_file(&yaml_path).expect("create tree fs");
/// assert!(tree_fs.root.exists())
// </snip>
/// ```
///
/// # Errors
///
/// Returns a `Result` containing the path to the root folder of the generated file tree on success,
/// or an error if the operation fails.
pub fn from_yaml_file(path: &PathBuf) -> Result<Tree> {
    let f = std::fs::File::open(path)?;
    let tree_builder: TreeBuilder = serde_yaml::from_reader(f)?;
    Ok(tree_builder.create()?)
}

#[cfg(feature = "yaml")]
/// Creates a file tree based on a YAML-formatted string.
///
/// # Examples
///
/// ```rust
// <snip id="example-from-yaml-str">
/// let content = r#"
/// override_file: false
/// files:
///   - path: foo.json
///     content: |
///       { "foo;": "bar" }
///   - path: folder/bar.yaml
///     content: |
///       foo: bar
///     "#;
///
/// let tree_fs = tree_fs::from_yaml_str(content).expect("create tree fs");
/// assert!(tree_fs.root.exists())
// </snip>
///
/// ```
///
/// # Errors
/// Returns a `Result` containing the path to the root folder of the generated file tree on success,
/// or an error if the operation fails.
pub fn from_yaml_str(content: &str) -> Result<Tree> {
    let tree_builder: TreeBuilder = serde_yaml::from_str(content)?;
    Ok(tree_builder.create()?)
}

fn temp_dir() -> PathBuf {
    let random_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(5)
        .map(char::from)
        .collect();

    env::temp_dir().join(random_string)
}
