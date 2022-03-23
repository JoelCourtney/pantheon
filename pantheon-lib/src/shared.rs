use anyhow::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, Debug)]
pub struct CharacterFile {
    pub system: String,
    pub prefix: String,
    pub name: String,
}

impl TryFrom<PathBuf> for CharacterFile {
    type Error = anyhow::Error;
    fn try_from(path: PathBuf) -> Result<Self> {
        let full_path = path.to_str().ok_or(anyhow!("couldn't covert full path"))?;
        let file_name = path
            .file_name()
            .ok_or(anyhow!("no file name"))?
            .to_str()
            .ok_or(anyhow!("could not convert OS string"))?
            .to_owned();
        let prefix_end = full_path.find(&file_name).unwrap();
        let first_dot = file_name.find(".").ok_or(anyhow!("no first dot"))?;
        let second_dot = &file_name[first_dot + 1..]
            .find(".")
            .ok_or(anyhow!("no second dot"))?
            + first_dot
            + 1;
        Ok(CharacterFile {
            system: file_name[first_dot + 1..second_dot].to_string(),
            prefix: full_path[..prefix_end].to_string(),
            name: file_name[..first_dot].to_string(),
        })
    }
}
