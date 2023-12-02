//! # tree-fs
//!
//! Oftentimes, testing scenarios involve interactions with the file system. `tree-fs` provides a convenient solution for creating file system trees tailored to the needs of your tests. This library offers:
//!
//! - An easy way to generate a tree with recursive paths.
//! - Tree creation within a temporary folder.
//! - The ability to create a tree using either YAML or a builder.
//!
//! ## Example
//!
//! From builder
//! ```rust
#![doc = include_str!("../examples/builder.rs")]
//! ```
//!
//!
//! Using a YAML File
//! ```rust
#![doc = include_str!("../examples/yaml-file.rs")]
//! ```
//!
//!
//! Using a YAML String
//! ```rust
#![doc = include_str!("../examples/yaml-str.rs")]
//! ```
//!

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
    pub root_folder: PathBuf,
    /// Flag indicating whether existing files should be overridden.
    #[serde(default)]
    pub override_file: bool,
    /// List of file metadata entries in the tree.
    pub files: Vec<FileMetadata>,
}

/// Represents metadata for a file in the tree.
#[derive(Debug, Deserialize)]
pub struct FileMetadata {
    /// Path of the file relative to the root folder.
    pub path: PathBuf,
    /// Optional content to be written to the file.
    pub content: Option<String>,
}

impl Default for Tree {
    /// Creates a default `Tree` instance with an empty file list,
    fn default() -> Self {
        Self {
            files: vec![],
            override_file: false,
            root_folder: temp_dir(),
        }
    }
}

impl Tree {
    /// Sets the root folder where the tree will be created.
    #[must_use]
    pub fn root_folder<P: AsRef<Path>>(mut self, dir: P) -> Self {
        self.root_folder = dir.as_ref().to_path_buf();
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
    pub fn create(&self) -> std::io::Result<PathBuf> {
        for file in &self.files {
            let dest_file = self.root_folder.join(&file.path);
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
        Ok(self.root_folder.clone())
    }
}

#[cfg(feature = "yaml")]
/// Creates a file tree based on the content of a YAML file.
///
/// # Errors
///
/// Returns a `Result` containing the path to the root folder of the generated file tree on success,
/// or an error if the operation fails.
pub fn from_yaml_file(path: &PathBuf) -> Result<PathBuf> {
    let f = std::fs::File::open(path)?;
    let tree: Tree = serde_yaml::from_reader(f)?;
    Ok(tree.create()?)
}

#[cfg(feature = "yaml")]
/// Creates a file tree based on a YAML-formatted string.
///
/// # Errors
/// Returns a `Result` containing the path to the root folder of the generated file tree on success,
/// or an error if the operation fails.
pub fn from_yaml_str(content: &str) -> Result<PathBuf> {
    let tree: Tree = serde_yaml::from_str(content)?;
    Ok(tree.create()?)
}

fn temp_dir() -> PathBuf {
    let random_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(5)
        .map(char::from)
        .collect();

    env::temp_dir().join(random_string)
}
