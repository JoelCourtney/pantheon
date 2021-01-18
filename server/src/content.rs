mod system;
mod official;
mod playtest;
mod homebrew;
mod custom_traits;

use custom_traits::*;
use crate::feature::Feature;
use crate::character::Character;

macros::registry!(10);

/// Contains where to find a particular file.
///
/// collection = one of "official", "playtest", "homebrew"
/// source = name of book or homebrew creator
#[derive(Eq, PartialEq, Debug, Hash)]
struct Registration {
    collection: &'static str,
    source: &'static str,
}

pub trait Content {
    fn receive_choice(&mut self, _choice: &str, _feature_index: usize, _choice_index: usize) {
        unimplemented!()
    }
    fn features(&self) -> Vec<Feature> { vec! [] }

    fn receive_feat_choice(&mut self, _choice: &str, _feat_index: usize, _feature_index: usize, _choice_index: usize) -> Result<(),()> {
        Err(())
    }
    fn feats(&self) -> Vec<Vec<Feature>> { vec! [] }

    fn initialize(&self, _: &mut Character) {}
    fn modify(&self, _: &mut Character) {}
    fn finalize(&self, _: &mut Character) {}
}
