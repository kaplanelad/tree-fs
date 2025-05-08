use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Yaml(#[from] serde_yaml::Error),
    #[error(transparent)]
    IO(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

/// Creates a file tree based on the content of a YAML file.
///
/// # Errors
///
/// Returns a `Result` containing the path to the root folder of the generated file tree on success,
/// or an error if the operation fails.
pub fn from_yaml_file(path: &PathBuf) -> Result<crate::Tree> {
    let f = std::fs::File::open(path)?;
    let tree_builder: crate::TreeBuilder = serde_yaml::from_reader(f)?;
    Ok(tree_builder.create()?)
}

/// Creates a file tree based on a YAML-formatted string.
///
/// # Errors
/// Returns a `Result` containing the path to the root folder of the generated file tree on success,
/// or an error if the operation fails.
pub fn from_yaml_str(content: &str) -> Result<crate::Tree> {
    let tree_builder: crate::TreeBuilder = serde_yaml::from_str(content)?;
    Ok(tree_builder.create()?)
}

/// Default is to drop the directory when the Tree is dropped
pub const fn default_drop() -> bool {
    true
}
