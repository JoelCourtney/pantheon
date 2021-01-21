use std::fmt::Debug;
use macros::dynamic_choose;
use crate::feature::Choose;
use crate::character::Character;

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
    fn hit_dice(&self) -> usize;
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
    fn declare(&self, _c: &mut Character) {}
    fn iterate(&self, _c: &mut Character) {}
    fn last(&mut self, _c: &mut Character) {}
}

#[macro_export]
macro_rules! properties {
    ($($what:ident : $t:ty = $val:literal),*) => {
        fn name(&self) -> &'static str { NAME }
        $(fn $what(&self) -> $t { $val })*
    }
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
    fn declare(&self, _c: &mut Character, _lvl: usize) {}
    fn iterate(&self, _c: &mut Character, _lvl: usize) {}
    fn last(&mut self, _c: &mut Character, _lvl: usize) {}
}

#[dynamic_choose]
pub trait EldritchInvocation: Debug {
    fn name(&self) -> &'static str;
    fn declare(&self, _c: &mut Character, _lvl: usize) {}
    fn iterate(&self, _c: &mut Character, _lvl: usize) {}
    fn last(&mut self, _c: &mut Character, _lvl: usize) {}
}