use std::path::PathBuf;
use jwalk::WalkDir;
use anyhow::Result;
use shrinkwraprs::Shrinkwrap;

pub(crate) fn list_characters(prefix: &PathBuf) -> Vec<PathBuf> {
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
                Some(ext) if ext == "panth" => Some(path.strip_prefix(prefix).ok()?.to_path_buf()),
                _ => None,
            }
        })
        .collect()
}

pub(crate) fn pantheon_root() -> Result<PantheonRoot> {
    Ok(PantheonRoot(std::env::var("PANTHEON_ROOT")?))
}

#[derive(Clone, Shrinkwrap)]
pub(crate) struct PantheonRoot(String);

impl std::fmt::Display for PantheonRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}