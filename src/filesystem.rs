use std::path::PathBuf;
use serde::Serialize;

#[derive(Serialize)]
pub struct FilesList {
    characters: Vec<PathBuf>,
    files: Vec<PathBuf>
}

pub async fn get_lists() -> std::io::Result<FilesList> {
    Ok(FilesList {
        characters: vec!["hey".into()],
        files: vec![
            "spell:Leomund's Tiny Hut".into(),
            "spell:Fireball".into(),
            "spell:Otto's Irresistable Dance".into(),
            "spell:Fly".into(),
            "spell:Aganazzar's Scorcher".into(),
            "spell:Antipathy/Sympathy".into(),
            "spell:Command".into(),
            "spell:Comprehend Languages".into(),
            "spell:Conjure Woodland Beings".into(),
            "spell:Power Word Kill".into(),
            "spell:Power Word Stun".into(),
            "spell:Power Word Heal".into(),
        ]
    })
}