#![doc = include_str!("../README.md")]

use std::{
    env,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use rand::{distributions::Alphanumeric, thread_rng, Rng};

/// Represents a file tree structure
#[derive(Debug)]
pub struct Tree {
    /// Root folder where the tree will be created.
    root: PathBuf,
    drop: bool,
}

impl Tree {
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn root(&self) -> &Path {
        &self.root
    }
}
/// Represents a file tree structure
///
/// # Examples
///
/// ```rust
// <snip id="example-builder">
/// use tree_fs::TreeBuilder;
/// let tree_fs = TreeBuilder::default()
///     .add_text("test/foo.txt", "bar")
///     .add_empty("test/folder-a/folder-b/bar.txt")
///     .add_file("test/file.rs", file!())
///     .create()
///     .expect("create tree fs");
/// println!("created successfully in {}", tree_fs.root().display());
// </snip>
/// ```
#[derive(Debug)]
pub struct TreeBuilder {
    /// Root folder where the tree will be created.
    root: PathBuf,
    /// Flag indicating whether existing files should be overridden.
    override_file: bool,
    /// List of file metadata entries in the tree.
    files: Vec<FileMetadata>,
    drop: bool,
}

#[derive(Debug)]
enum Content {
    Empty,
    Text(String),
    File(PathBuf),
}

/// Represents metadata for a file in the tree.
#[derive(Debug)]
pub struct FileMetadata {
    /// Path of the file relative to the root folder.
    path: PathBuf,
    /// Optional content to be written to the file.
    content: Content,
}

impl Default for TreeBuilder {
    /// Creates a default `Tree` instance with an empty file list,
    fn default() -> Self {
        Self {
            files: vec![],
            override_file: false,
            root: random_temp_directory(),
            drop: true,
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

    /// Sets the `drop` flag, indicating whether to automatically delete the
    /// temporary folder when the `tree_fs` instance is dropped
    #[must_use]
    pub const fn drop(mut self, yes: bool) -> Self {
        self.drop = yes;
        self
    }

    /// Sets the `override_file` flag, indicating whether existing files should
    /// be overridden.
    #[must_use]
    pub const fn override_file(mut self, yes: bool) -> Self {
        self.override_file = yes;
        self
    }

    /// Adds a file with content to the tree.
    #[must_use]
    fn add<P: AsRef<Path>>(mut self, path: P, content: Content) -> Self {
        self.files.push(FileMetadata {
            path: path.as_ref().to_path_buf(),
            content,
        });
        self
    }

    /// Adds a file with a empty content.
    #[must_use]
    pub fn add_empty<P: AsRef<Path>>(self, path: P) -> Self {
        self.add(path, Content::Empty)
    }

    /// Adds a file specifying a text content.
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn add_text<P: AsRef<Path>>(self, path: P, text: impl ToString) -> Self {
        self.add(path, Content::Text(text.to_string()))
    }

    /// Adds a file specifying a source file to be copied.
    #[must_use]
    pub fn add_file<P: AsRef<Path>>(self, path: P, file: P) -> Self {
        self.add(path, Content::File(file.as_ref().to_path_buf()))
    }

    /// Creates the file tree by generating files and directories based on the
    /// specified metadata.
    ///
    /// # Errors
    ///
    /// Returns an `std::io::Result` indicating success or failure in creating
    /// the file tree.
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

            match &file.content {
                Content::Empty => {
                    File::create(&dest_file)?;
                }
                Content::Text(text) => {
                    let mut new_file = File::create(&dest_file)?;

                    new_file.write_all(text.as_bytes())?;
                }
                Content::File(source_path) => {
                    std::fs::copy(source_path, &dest_file)?;
                }
            }
        }

        Ok(Tree {
            root: self.root.clone(),
            drop: self.drop,
        })
    }
}

fn random_temp_directory() -> PathBuf {
    loop {
        let random_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(5)
            .map(char::from)
            .collect();

        let path = env::temp_dir().join(random_string);

        if !path.exists() {
            return path;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn can_create_tree_from_builder() {
        let tree_res = TreeBuilder::default()
            .add_text("foo.txt", "foo")
            .add_empty("folder-a/folder-b/bar.txt")
            .add_file("tree.rs", file!())
            .create()
            .unwrap();

        assert!(tree_res.root.join("foo.txt").exists());
        assert!(tree_res
            .root
            .join("folder-a")
            .join("folder-b")
            .join("bar.txt")
            .exists());

        assert_eq!(
            fs::read_to_string(tree_res.root.join("foo.txt")).unwrap(),
            "foo"
        );
        assert_eq!(
            fs::read_to_string(
                tree_res
                    .root
                    .join("folder-a")
                    .join("folder-b")
                    .join("bar.txt")
            )
            .unwrap(),
            ""
        );

        assert_eq!(
            fs::read_to_string(tree_res.root.join("tree.rs")).unwrap(),
            fs::read_to_string(file!()).unwrap(),
        )
    }

    #[test]
    fn can_create_build_tree_with_drop() {
        let tree_fs = TreeBuilder::default()
            .add_text("foo.txt", "foo")
            .add_empty("folder-a/folder-b/bar.txt")
            .drop(true)
            .create()
            .unwrap();

        assert!(tree_fs.root.exists());

        let root_path = tree_fs.root().to_path_buf();
        drop(tree_fs);
        assert!(!root_path.exists());
    }
}
