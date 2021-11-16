use std::path::PathBuf;
use walkdir::WalkDir;

pub(crate) fn list_characters(prefix: &PathBuf) -> Vec<PathBuf> {
    WalkDir::new(prefix)
        .into_iter()
        .filter_entry(|entry| {
            !entry
                .file_name()
                .to_str()
                .map(|s| s.starts_with("."))
                .unwrap_or(false)
        })
        .filter_map(|entry| {
            let path = entry.ok()?.into_path();
            match path.extension() {
                Some(ext) if ext == "dndc" => Some(path.strip_prefix(prefix).ok()?.to_path_buf()),
                _ => None,
            }
        })
        .collect()
}
