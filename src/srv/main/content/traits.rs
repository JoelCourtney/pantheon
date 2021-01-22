use std::fmt::Debug;
use macros::dynamic_choose;
use crate::feature::Choose;
use crate::character::Character;
use crate::moves::CastingTime;

#[macro_export]
macro_rules! name {
    ($name:literal) => {
        #[allow(unused_imports)] use crate::character::*;
        #[allow(unused_imports)] use crate::feature::*;
        #[allow(unused_imports)] use crate::misc::*;
        #[allow(unused_imports)] use crate::describe::*;
        #[allow(unused_imports)] use crate::moves::*;
        #[allow(unused_imports)] use crate::properties;
        #[allow(unused_imports)] use crate::content::traits::*;
        #[allow(unused_imports)] use crate::content::common::*;
        #[allow(unused_imports)] use macros::{def, describe, choose, dynamic_choose, content};
        #[allow(unused_imports)] use serde::{Serialize, Deserialize};
        #[allow(unused_imports)] use indoc::indoc;
        #[allow(unused_imports)] use std::fmt::Debug;

        pub const NAME: &'static str = $name;
    }
}

#[macro_export]
macro_rules! properties {
    ($($what:ident : $t:ty = $val:expr),*) => {
        fn name(&self) -> &'static str { NAME }
        $(fn $what(&self) -> $t { $val })*
    }
}

#[dynamic_choose]
pub trait Race: Debug {
    fn name(&self) -> &'static str;
    fn declare(&self, _c: &mut Character) {}
    fn iterate(&self, _c: &mut Character) {}
    fn last(&mut self, _c: &mut Character) {}
}

#[dynamic_choose]
pub trait Class: Debug {
    fn name(&self) -> &'static str;
    fn hit_dice(&self) -> u32;
    fn declare(&self, _c: &mut Character) {}
    fn iterate(&self, _c: &mut Character) {}
    fn last(&mut self, _c: &mut Character) {}
}

#[dynamic_choose]
pub trait Feat: Debug {
    fn name(&self) -> &'static str;
    fn declare(&self, _c: &mut Character) {}
    fn iterate(&self, _c: &mut Character) {}
    fn last(&mut self, _c: &mut Character) {}
}

#[dynamic_choose]
pub trait Spell: Debug {
    fn name(&self) -> &'static str;
    fn level(&self) -> usize;
    fn casting_time(&self) -> CastingTime;
    fn optional_ritual(&self) -> bool;
    // TODO
}

#[dynamic_choose]
pub trait HalflingSubrace: Debug {
    fn name(&self) -> &'static str;
    fn declare(&self, _c: &mut Character) {}
    fn iterate(&self, _c: &mut Character) {}
    fn last(&mut self, _c: &mut Character) {}
}

#[dynamic_choose]
pub trait RoguishArchetype: Debug {
    fn name(&self) -> &'static str;
    fn declare(&self, _c: &mut Character, _lvl: u32) {}
    fn iterate(&self, _c: &mut Character, _lvl: u32) {}
    fn last(&mut self, _c: &mut Character, _lvl: u32) {}
}

#[dynamic_choose]
pub trait EldritchInvocation: Debug {
    fn name(&self) -> &'static str;
    fn declare(&self, _c: &mut Character, _lvl: u32) {}
    fn iterate(&self, _c: &mut Character, _lvl: u32) {}
    fn last(&self, _c: &mut Character, _lvl: u32) {}
}