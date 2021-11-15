mod system;
mod official;
mod playtest;
mod homebrew;
pub(crate) mod traits;
pub(crate) mod common;

use serde::{Deserialize, Serialize};

proc_macros::registry!();

/// Contains where to find a particular file.
///
/// collection = one of "official", "playtest", "homebrew"
/// source = name of book or homebrew creator
#[derive(Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
pub struct Registration<'a> {
    collection: &'a str,
    source: &'a str,
    kind: &'a str,
    name: &'a str
}
