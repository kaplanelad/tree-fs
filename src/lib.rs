#![doc = include_str!("../README.md")]

use std::{
    env,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use rand::{distributions::Alphanumeric, thread_rng, Rng};

/// Represents a temporary directory.  
/// By default this temporary directory is deleted when this struct is dropped.
#[derive(Debug)]
pub struct TempDirectory {
    path: PathBuf,
    delete_on_drop: bool,
}

impl TempDirectory {
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn path(&self) -> &Path {
        &self.path
    }
}

/// Error happening when creating the directory tree.
#[derive(Debug, thiserror::Error)]
pub enum CreateError {
    #[error("Failed to create the root directory '{0}': {1}")]
    FailedToCreateRootDirectory(PathBuf, std::io::Error),
    #[error("Failed to create directory '{0}': {1}")]
    FailedToCreateDirectory(PathBuf, std::io::Error),
    #[error("Failed to delete directory '{0}': {1}")]
    FailedToDeleteDirectory(PathBuf, std::io::Error),
    #[error("Failed to create file '{0}': {1}")]
    FailedToCreateFile(PathBuf, std::io::Error),
    #[error("Failed to read source file '{0}': {1}")]
    FailedToCopyFile(PathBuf, std::io::Error),
    #[error("Failed to write file '{0}': {1}")]
    FailedToWriteFile(PathBuf, std::io::Error),
    #[error("The entry '{0}' is outside the temporary directory")]
    EntryOutsideDirectory(PathBuf),
    #[error("The entry '{0}' is already existing")]
    DuplicateEntry(PathBuf),
}

/// A temporary directory builder that contains a list of entries to be created.
///
/// # Examples
///
/// ```rust
// <snip id="example-builder">
/// use temp_dir_builder::TempDirectoryBuilder;
/// let temp_dir = TempDirectoryBuilder::default()
///     .add_text_file("test/foo.txt", "bar")
///     .add_empty_file("test/folder-a/folder-b/bar.txt")
///     .add_file("test/file.rs", file!())
///     .add_directory("test/dir")
///     .create()
///     .expect("create temp dir");
/// println!("created successfully in {}", temp_dir.path().display());
// </snip>
/// ```
#[derive(Debug)]
pub struct TempDirectoryBuilder {
    /// Root folder where the tree will be created.
    root: PathBuf,
    /// List of file metadata entries in the tree.
    entries: Vec<Entry>,
    /// Flag indicating whether the temporary directory created must be deleted when the instance is dropped.
    delete_on_drop: bool,
}

impl Default for TempDirectoryBuilder {
    /// Creates a default `TempDirectoryBuilder` instance with an empty file list,
    fn default() -> Self {
        Self {
            entries: vec![],
            root: random_temp_directory(),
            delete_on_drop: true,
        }
    }
}

impl Drop for TempDirectory {
    fn drop(&mut self) {
        if self.delete_on_drop {
            let _ = std::fs::remove_dir_all(&self.path);
        }
    }
}

impl TempDirectoryBuilder {
    /// Sets the root folder where the tree will be created.  
    /// By default this is the temporary directory path returned by `std::env::temp_dir()`.
    #[must_use]
    pub fn root_folder<P: AsRef<Path>>(mut self, dir: P) -> Self {
        self.root = dir.as_ref().to_path_buf();
        self
    }

    /// Specifies whether to automatically delete the temporary folder when the `TempDirectory` instance is dropped.  
    /// By default this is value is set to `true`.
    #[must_use]
    pub const fn delete_on_drop(mut self, delete_on_drop: bool) -> Self {
        self.delete_on_drop = delete_on_drop;
        self
    }

    /// Adds an entry.
    #[must_use]
    fn add(mut self, path: impl AsRef<Path>, kind: Kind) -> Self {
        self.entries.push(Entry {
            path: path.as_ref().to_path_buf(),
            kind,
        });
        self
    }

    /// Adds an empty file.
    #[must_use]
    pub fn add_empty_file<P: AsRef<Path>>(self, path: P) -> Self {
        self.add(path, Kind::EmptyFile)
    }

    /// Adds a directory.
    #[must_use]
    pub fn add_directory(self, path: impl AsRef<Path>) -> Self {
        self.add(path, Kind::Directory)
    }

    /// Adds a text file specifying the content.
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn add_text_file(self, path: impl AsRef<Path>, text: impl ToString) -> Self {
        self.add(path, Kind::TextFile(text.to_string()))
    }

    /// Adds a file specifying a source file to be copied.
    #[must_use]
    pub fn add_file(self, path: impl AsRef<Path>, file: impl AsRef<Path>) -> Self {
        self.add(path, Kind::FileToCopy(file.as_ref().to_path_buf()))
    }

    /// Creates the file tree by generating files and directories based on the
    /// list of `Entry`s.
    ///
    /// # Errors
    ///
    /// Returns an `std::io::Result` indicating success or failure in creating
    /// the file tree.
    pub fn create(&self) -> Result<TempDirectory, CreateError> {
        if !self.root.exists() {
            std::fs::create_dir_all(&self.root)
                .map_err(|err| CreateError::FailedToCreateRootDirectory(self.root.clone(), err))?;
        }

        for entry in &self.entries {
            if entry.path.is_absolute() && !entry.path.starts_with(&self.root) {
                return Err(CreateError::EntryOutsideDirectory(entry.path.clone()));
            }

            let entry_path = self.root.join(&entry.path);

            if entry_path.exists() {
                return Err(CreateError::DuplicateEntry(entry_path));
            }

            if let Some(parent_dir) = Path::new(&entry_path).parent() {
                std::fs::create_dir_all(parent_dir).map_err(|err| {
                    CreateError::FailedToCreateDirectory(parent_dir.to_path_buf(), err)
                })?;
            }

            match &entry.kind {
                Kind::Directory => {
                    std::fs::create_dir(&entry_path)
                        .map_err(|err| CreateError::FailedToCreateDirectory(entry_path, err))?;
                }
                Kind::EmptyFile => {
                    File::create(&entry_path)
                        .map_err(|err| CreateError::FailedToCreateFile(entry_path, err))?;
                }
                Kind::TextFile(text) => {
                    let mut new_file = File::create(&entry_path)
                        .map_err(|err| CreateError::FailedToCreateFile(entry_path.clone(), err))?;

                    new_file
                        .write_all(text.as_bytes())
                        .map_err(|err| CreateError::FailedToWriteFile(entry_path, err))?;
                }
                Kind::FileToCopy(source_path) => {
                    std::fs::copy(source_path, &entry_path)
                        .map_err(|err| CreateError::FailedToCopyFile(source_path.clone(), err))?;
                }
            }
        }

        Ok(TempDirectory {
            path: self.root.clone(),
            delete_on_drop: self.delete_on_drop,
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

#[derive(Debug)]
enum Kind {
    Directory,
    EmptyFile,
    TextFile(String),
    FileToCopy(PathBuf),
}

/// Represents an entry, file or directory, to be created.
#[derive(Debug)]
struct Entry {
    /// Path of the entry relative to the root folder.
    path: PathBuf,
    /// The kind of the entry
    kind: Kind,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temp_dir() {
        let temp_dir = TempDirectoryBuilder::default().create().unwrap();

        assert!(temp_dir.path().exists());
        assert!(temp_dir.path().is_dir());
    }

    #[test]
    fn test_add_text_file() {
        let expected_content = "bar";
        let entry_name = "foo.txt";
        let temp_dir = TempDirectoryBuilder::default()
            .add_text_file(entry_name, expected_content)
            .create()
            .unwrap();
        let entry_path = temp_dir.path().join(entry_name);

        assert!(entry_path.exists());

        let content = std::fs::read_to_string(entry_path).expect("read text in foo.txt");

        assert_eq!(content, expected_content);
    }

    #[test]
    fn test_add_empty_file() {
        let entry_name = "empty_file.txt";
        let temp_dir = TempDirectoryBuilder::default()
            .add_empty_file(entry_name)
            .create()
            .unwrap();
        let entry_path = temp_dir.path().join(entry_name);

        assert!(entry_path.exists());

        let created_entry_metadata = std::fs::metadata(entry_path).expect("get entry metadata");

        assert_eq!(created_entry_metadata.len(), 0);
    }

    #[test]
    fn test_add_directory() {
        let entry_name = "empty_directory";
        let temp_dir = TempDirectoryBuilder::default()
            .add_directory(entry_name)
            .create()
            .unwrap();
        let entry_path = temp_dir.path().join(entry_name);

        assert!(entry_path.exists());
        assert!(entry_path.is_dir());
    }

    #[test]
    fn test_add_file() {
        let entry_name = "test.rs";
        let source_file_path = file!();
        let temp_dir = TempDirectoryBuilder::default()
            .add_file(entry_name, source_file_path)
            .create()
            .unwrap();
        let entry_path = temp_dir.path().join(entry_name);

        assert!(entry_path.exists());
        assert!(entry_path.is_file());

        let entry_content = std::fs::read_to_string(entry_path).unwrap();
        let source_content = std::fs::read_to_string(source_file_path).unwrap();

        assert_eq!(entry_content, source_content);
    }

    #[test]
    fn test_temp_dir_is_dropped() {
        let temp_dir = TempDirectoryBuilder::default().create().unwrap();

        let temp_dir_path = temp_dir.path().to_path_buf();

        assert!(temp_dir_path.exists());
        assert!(temp_dir_path.is_dir());

        drop(temp_dir);

        assert!(!temp_dir_path.exists())
    }

    #[test]
    fn test_entry_outside_temp_dir() {
        let path_outside_temp_dir = std::env::temp_dir().join("outside");
        let builder = TempDirectoryBuilder::default().add_empty_file(path_outside_temp_dir);
        let error = builder.create().unwrap_err();

        assert!(matches!(error, CreateError::EntryOutsideDirectory(_)));
    }

    #[test]
    fn test_source_file_does_not_exists() {
        let source_file_path = std::env::temp_dir().join("not existing file");
        let builder = TempDirectoryBuilder::default().add_file("foo", source_file_path);
        let error = builder.create().unwrap_err();

        assert!(matches!(error, CreateError::FailedToCopyFile(..)));
    }

    #[test]
    fn test_duplicated_entries() {
        let builder = TempDirectoryBuilder::default()
            .add_empty_file("foo")
            .add_empty_file("foo");
        let error = builder.create().unwrap_err();

        assert!(matches!(error, CreateError::DuplicateEntry(..)));
    }
}
