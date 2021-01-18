use crate::feature::FeatureSerial;
use crate::character::Character;

mod system;
mod official;
mod playtest;
mod homebrew;
mod custom_traits;

use custom_traits::*;

macros::registry!(9);

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
    fn write_features(&self) -> Vec<FeatureSerial> { vec! [] }

    fn receive_feat_choice(&mut self, _choice: &str, _feat_index: usize, _feature_index: usize, _choice_index: usize) -> Result<(),()> {
        Err(())
    }
    fn write_feats(&self) -> Vec<Vec<FeatureSerial>> { vec! [] }

    fn initialize(&self, _: &mut Character) {}
    fn modify(&self, _: &mut Character) {}
    fn finalize(&self, _: &mut Character) {}
}
