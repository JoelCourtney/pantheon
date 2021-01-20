mod system;
mod official;
mod playtest;
mod homebrew;
mod custom_traits;
pub(crate) mod common;

use custom_traits::*;
use crate::character::Character;

macros::registry!(14);

/// Contains where to find a particular file.
///
/// collection = one of "official", "playtest", "homebrew"
/// source = name of book or homebrew creator
#[derive(Eq, PartialEq, Debug, Hash)]
struct Registration {
    collection: &'static str,
    source: &'static str,
}

pub trait Content: Sync + Send {
    fn declare(&self, _c: &mut Character) {}
    fn iterate(&self, _c: &mut Character) {}
    fn last(&mut self, _c: &mut Character) {}
}

pub trait LeveledContent: Sync + Send {
    fn declare(&self, _c: &mut Character, _lvl: usize) {}
    fn iterate(&self, _c: &mut Character, _lvl: usize) {}
    fn last(&mut self, _c: &mut Character, _lvl: usize) {}
}