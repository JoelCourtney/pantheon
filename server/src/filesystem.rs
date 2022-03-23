use std::path::PathBuf;
use jwalk::WalkDir;
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

#[derive(Clone, Shrinkwrap)]
pub(crate) struct ServeRoot(String);

impl Default for ServeRoot {
    fn default() -> ServeRoot {
        ServeRoot(format!("{}/www", std::env::var("PANTHEON_ROOT")
            .expect("Set PANTHEON_ROOT env variable.")))
    }
}

impl std::fmt::Display for ServeRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}