//! Helper functions and structs that deal with the filesystem.

use jwalk::WalkDir;
use pantheon::shared::CharacterFile;
use shrinkwraprs::Shrinkwrap;
use std::path::{Path, PathBuf};

/// File extension for Pantheon character files.
const PANTHEON_EXTENSION: &str = "panth";

/// Uses [jwalk](https://docs.rs/jwalk/0.6.0/jwalk/) to find all files
/// ending in [PANTHEON_EXTENSION].
pub(crate) fn list_characters(prefix: &Path) -> Vec<CharacterFile> {
    WalkDir::new(prefix)
        .sort(true)
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            if !entry.file_type.is_file() {
                return None;
            }
            let path: PathBuf = entry.path();
            match path.extension() {
                Some(ext) if ext == PANTHEON_EXTENSION => Some(
                    path.strip_prefix(prefix)
                        .ok()?
                        .to_path_buf()
                        .try_into()
                        .unwrap(),
                ),
                _ => None,
            }
        })
        .collect()
}

#[derive(Clone, Shrinkwrap)]
pub(crate) struct ServeRoot(String);

impl Default for ServeRoot {
    fn default() -> ServeRoot {
        ServeRoot(format!(
            "{}/www",
            std::env::var("PANTHEON_ROOT").expect("Set PANTHEON_ROOT env variable.")
        ))
    }
}

impl std::fmt::Display for ServeRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub(crate) fn list_systems(root: &ServeRoot) -> Vec<String> {
    WalkDir::new(format!("{root}/systems/"))
        .sort(true)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            if !entry.file_type.is_dir() {
                return None;
            }
            match entry.file_name.to_str().unwrap() {
                // "template" => None,
                other => Some(other.to_string())
            }
        }).collect()
}