use std::path::PathBuf;
use jwalk::WalkDir;
use anyhow::Result;
use shrinkwraprs::Shrinkwrap;

pub(crate) fn list_characters(prefix: &PathBuf) -> Vec<PathBuf> {
    WalkDir::new(dbg!(prefix))
        .sort(true)
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            if !entry.file_type.is_file() {
                return None;
            }
            let path: PathBuf = entry.path();
            match path.extension() {
                Some(ext) if ext == "dndc" => Some(path.strip_prefix(prefix).ok()?.to_path_buf()),
                _ => None,
            }
        })
        .collect()
}

pub(crate) fn starpg_root() -> Result<StarpgRoot> {
    Ok(StarpgRoot(std::env::var("STARPG_ROOT")?))
}

#[derive(Clone, Shrinkwrap)]
pub(crate) struct StarpgRoot(String);

impl std::fmt::Display for StarpgRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}