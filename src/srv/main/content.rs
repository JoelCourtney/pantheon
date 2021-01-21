mod system;
mod official;
mod playtest;
mod homebrew;
pub(crate) mod traits;
pub(crate) mod common;

macros::registry!(15);

/// Contains where to find a particular file.
///
/// collection = one of "official", "playtest", "homebrew"
/// source = name of book or homebrew creator
#[derive(Eq, PartialEq, Debug, Hash)]
struct Registration {
    collection: &'static str,
    source: &'static str,
}

