#![doc = include_str!("../README.md")]

#[cfg(feature = "yaml")]
mod yaml;
#[cfg(feature = "yaml")]
pub use yaml::{from_yaml_file, from_yaml_str};

mod builder;
pub use builder::TreeBuilder;

mod tree;
pub use tree::{Entry, Kind, Settings, Tree};
