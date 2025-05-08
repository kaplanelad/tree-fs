use rand::{distr::Alphanumeric, rng, Rng};

use std::env;
use std::path::PathBuf;

#[cfg(feature = "yaml")]
use serde::Deserialize;
#[cfg(feature = "yaml")]
use serde::Serialize;

/// Represents a file tree structure
#[derive(Debug)]
pub struct Tree {
    /// Root folder where the tree will be created.
    pub root: PathBuf,
    /// Whether to automatically delete the temporary folder when dropped
    pub(crate) drop: bool,
}

impl Drop for Tree {
    fn drop(&mut self) {
        if self.drop {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }
}

/// Settings for entries in the tree.
/// Currently supports read-only flag, but can be extended with additional settings.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "yaml", derive(Deserialize, Serialize))]
#[derive(Default)]
pub struct Settings {
    /// Whether the file is read-only.
    #[cfg_attr(
        feature = "yaml",
        serde(default, skip_serializing_if = "std::ops::Not::not")
    )]
    pub readonly: bool,
    // Future settings could be added here:
    // pub timestamp: Option<SystemTime>,
    // pub owner: Option<String>,
    // etc.
}

// Builder pattern for Settings
impl Settings {
    /// Creates a new Settings with default values.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets whether the file is read-only.
    #[must_use]
    pub const fn readonly(mut self, value: bool) -> Self {
        self.readonly = value;
        self
    }
}

/// Describes what kind of entry to create
#[derive(Debug, Clone)]
#[cfg_attr(feature = "yaml", derive(Deserialize))]
#[cfg_attr(feature = "yaml", serde(tag = "type"))]
pub enum Kind {
    /// A directory
    #[cfg_attr(feature = "yaml", serde(rename = "directory"))]
    Directory,
    /// An empty file
    #[cfg_attr(feature = "yaml", serde(rename = "empty_file"))]
    EmptyFile,
    /// A file with text content
    #[cfg_attr(feature = "yaml", serde(rename = "text_file"))]
    TextFile { content: String },
}

/// Represents an entry, file or directory, to be created.
#[derive(Debug)]
#[cfg_attr(feature = "yaml", derive(Deserialize))]
pub struct Entry {
    /// Path of the entry relative to the root folder.
    pub path: PathBuf,
    /// The kind of the entry
    #[cfg_attr(feature = "yaml", serde(flatten))]
    pub kind: Kind,
    /// Optional settings for the entry
    #[cfg_attr(
        feature = "yaml",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub settings: Option<Settings>,
}

/// Creates a temporary directory with a random name
pub fn temp_dir() -> PathBuf {
    let random_string: String = rng()
        .sample_iter(&Alphanumeric)
        .take(5)
        .map(char::from)
        .collect();

    env::temp_dir().join(random_string)
}
