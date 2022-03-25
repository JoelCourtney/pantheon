use thiserror::Error;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub enum Query {
    ListCharacters,
    ListSystems,
    ReadCharacter(PathBuf),
    WriteCharacter(PathBuf, Vec<u8>)
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, Debug, Clone)]
pub struct CharacterFile {
    pub system: String,
    pub prefix: String,
    pub name: String,
}

impl TryFrom<PathBuf> for CharacterFile {
    type Error = CharacterFileError;
    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        use CharacterFileError::*;

        let full_path = path.to_str().ok_or(FullPath)?;
        let file_name = path
            .file_name()
            .ok_or(NoFileName)?
            .to_str()
            .ok_or(OsString)?
            .to_owned();
        let prefix_end = full_path.find(&file_name).ok_or(CouldntFindFileName)?;
        let first_dot = file_name.find('.').ok_or(NoFirstDot)?;
        let second_dot = file_name[first_dot + 1..]
            .find('.')
            .ok_or(NoSecondDot)?
            + first_dot
            + 1;
        Ok(CharacterFile {
            system: file_name[first_dot + 1..second_dot].to_string(),
            prefix: full_path[..prefix_end].to_string(),
            name: file_name[..first_dot].to_string(),
        })
    }
}

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum CharacterFileError {
    #[error("couldn't convert full path to str")] FullPath,
    #[error("path didn't have a file name")] NoFileName,
    #[error("couldn't convert OsStr to str")] OsString,
    #[error("file name wasn't in full path")] CouldntFindFileName,
    #[error("file name has no dots")] NoFirstDot,
    #[error("file name has only one dot")] NoSecondDot
}
