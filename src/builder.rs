use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

#[cfg(feature = "yaml")]
use serde::Deserialize;

/// Represents a file tree structure
///
/// # Examples
///
/// ```rust
/// use tree_fs::{TreeBuilder, Settings};
/// let tree = TreeBuilder::default()
///     .add_file("config/app.conf", "host = localhost")
///     .add_empty_file("logs/app.log")
///     .add_directory("data/raw")
///     .add_file_with_settings(
///         "secrets/api.key",
///         "supersecretkey",
///         Settings::new().readonly(true)
///     )
///     .create()
///     .expect("create tree fs");
/// println!("Created a complex tree in: {}", tree.root.display());
///
/// // You can verify the readonly status (this requires std::fs)
/// // let key_path = tree.root.join("secrets/api.key");
/// // let metadata = std::fs::metadata(key_path).unwrap();
/// // assert!(metadata.permissions().readonly());
/// ```
///
/// ```rust
/// use tree_fs::TreeBuilder;
/// let tree = TreeBuilder::default()
///      .add_file("temp_data/file.txt", "temporary content")
///      .add_empty_file("temp_data/another.tmp")
///      .drop(true) // This is the default, but explicitly shown here
///      .create()
///      .expect("create tree fs for drop example");
///
/// println!("Temporary tree created at: {}", tree.root.display());
///
/// let path_to_check = tree.root.clone();
/// assert!(path_to_check.exists(), "Directory should exist before drop");
///
/// drop(tree); // tree_fs instance goes out of scope
/// assert!(!path_to_check.exists(), "Directory should be deleted after drop");
/// ```
#[derive(Debug)]
#[cfg_attr(feature = "yaml", derive(Deserialize))]
pub struct TreeBuilder {
    /// Root folder where the tree will be created.
    #[cfg_attr(feature = "yaml", serde(default = "crate::tree::temp_dir"))]
    pub root: PathBuf,
    /// Flag indicating whether existing files should be overridden.
    #[cfg_attr(feature = "yaml", serde(default))]
    override_file: bool,
    /// List of entries in the tree.
    entries: Vec<crate::Entry>,
    /// Whether to automatically delete the temporary folder when Tree is dropped
    #[cfg_attr(feature = "yaml", serde(default = "crate::yaml::default_drop"))]
    drop: bool,
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
        self.entries.push(crate::Entry {
            path: path.as_ref().to_path_buf(),
            kind: crate::Kind::TextFile {
                content: content.to_string(),
            },
            settings: None,
        });
        self
    }

    /// Adds a file with content to the tree.
    ///
    /// This is an alias for `add`.
    #[must_use]
    pub fn add_file<P: AsRef<Path>>(self, path: P, content: &str) -> Self {
        self.add(path, content)
    }

    /// Adds a file with a empty content.
    #[must_use]
    pub fn add_empty<P: AsRef<Path>>(self, path: P) -> Self {
        self.add_empty_file(path)
    }

    /// Adds a file with a empty content.
    #[must_use]
    pub fn add_empty_file<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.entries.push(crate::Entry {
            path: path.as_ref().to_path_buf(),
            kind: crate::Kind::EmptyFile,
            settings: None,
        });
        self
    }

    /// Adds a directory to the tree.
    #[must_use]
    pub fn add_directory<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.entries.push(crate::Entry {
            path: path.as_ref().to_path_buf(),
            kind: crate::Kind::Directory,
            settings: None,
        });
        self
    }

    /// Adds a file with content and custom settings to the tree.
    #[must_use]
    pub fn add_file_with_settings<P: AsRef<Path>>(
        mut self,
        path: P,
        content: &str,
        settings: crate::tree::Settings,
    ) -> Self {
        self.entries.push(crate::Entry {
            path: path.as_ref().to_path_buf(),
            kind: crate::Kind::TextFile {
                content: content.to_string(),
            },
            settings: Some(settings),
        });
        self
    }

    /// Adds an empty file with custom settings to the tree.
    #[must_use]
    pub fn add_empty_file_with_settings<P: AsRef<Path>>(
        mut self,
        path: P,
        settings: crate::tree::Settings,
    ) -> Self {
        self.entries.push(crate::Entry {
            path: path.as_ref().to_path_buf(),
            kind: crate::Kind::EmptyFile,
            settings: Some(settings),
        });
        self
    }

    /// Adds a directory with custom settings to the tree.
    #[must_use]
    pub fn add_directory_with_settings<P: AsRef<Path>>(
        mut self,
        path: P,
        settings: crate::tree::Settings,
    ) -> Self {
        self.entries.push(crate::Entry {
            path: path.as_ref().to_path_buf(),
            kind: crate::Kind::Directory,
            settings: Some(settings),
        });
        self
    }

    /// Convenience method for adding a read-only file.
    #[must_use]
    pub fn add_readonly_file<P: AsRef<Path>>(self, path: P, content: &str) -> Self {
        self.add_file_with_settings(path, content, crate::tree::Settings::new().readonly(true))
    }

    /// Convenience method for adding a read-only empty file.
    #[must_use]
    pub fn add_readonly_empty_file<P: AsRef<Path>>(self, path: P) -> Self {
        self.add_empty_file_with_settings(path, crate::tree::Settings::new().readonly(true))
    }

    /// Creates the file tree by generating files and directories based on the specified metadata.
    ///
    /// # Errors
    ///
    /// Returns an `std::io::Result` indicating success or failure in creating the file tree.
    pub fn create(&self) -> std::io::Result<crate::Tree> {
        if !self.root.exists() {
            std::fs::create_dir_all(&self.root)?;
        }

        // Process entries
        for entry in &self.entries {
            let dest_path = self.root.join(&entry.path);
            if !self.override_file && dest_path.exists() {
                continue;
            }

            match &entry.kind {
                crate::Kind::Directory => {
                    std::fs::create_dir_all(&dest_path)?;
                }
                crate::Kind::EmptyFile => {
                    if let Some(parent_dir) = Path::new(&dest_path).parent() {
                        std::fs::create_dir_all(parent_dir)?;
                    }
                    File::create(&dest_path)?;
                }
                crate::Kind::TextFile { content } => {
                    if let Some(parent_dir) = Path::new(&dest_path).parent() {
                        std::fs::create_dir_all(parent_dir)?;
                    }
                    let mut file = File::create(&dest_path)?;
                    file.write_all(content.as_bytes())?;
                }
            }

            if let Some(settings) = &entry.settings {
                if matches!(entry.kind, crate::Kind::Directory) {
                    continue;
                }

                let dest_path_for_perms = self.root.join(&entry.path);
                if settings.readonly {
                    let mut permissions = std::fs::metadata(&dest_path_for_perms)?.permissions();
                    permissions.set_readonly(true);
                    std::fs::set_permissions(&dest_path_for_perms, permissions)?;
                }
            }
        }

        Ok(crate::Tree {
            root: self.root.clone(),
            drop: self.drop,
        })
    }
}

impl Default for TreeBuilder {
    /// Creates a default `Tree` instance with an empty file list,
    fn default() -> Self {
        Self {
            entries: vec![],
            override_file: false,
            root: crate::tree::temp_dir(),
            drop: true,
        }
    }
}
