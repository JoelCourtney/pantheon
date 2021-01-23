use std::fmt::Debug;
use macros::dynamic_choose;
use crate::feature::Choose;
use crate::character::Character;
use crate::misc::{CastingTime, Equipable, Rarity};

#[dynamic_choose]
pub trait Race: Debug {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn description_no_title(&self) -> &'static str;

    fn declare(&self, _c: &mut Character) {}
    fn iterate(&self, _c: &mut Character) {}
    fn last(&mut self, _c: &mut Character) {}
}

#[dynamic_choose]
pub trait Class: Debug {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn description_no_title(&self) -> &'static str;

    fn hit_dice(&self) -> u32;
    fn declare(&self, _c: &mut Character, _level: u32, _first: bool) {}
    fn iterate(&self, _c: &mut Character, _level: u32, _first: bool) {}
    fn last(&mut self, _c: &mut Character, _level: u32, _first: bool) {}
}

#[dynamic_choose]
pub trait Feat: Debug {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn description_no_title(&self) -> &'static str;

    fn declare(&self, _c: &mut Character) {}
    fn iterate(&self, _c: &mut Character) {}
    fn last(&mut self, _c: &mut Character) {}
}

#[dynamic_choose]
pub trait Spell: Debug {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn description_no_title(&self) -> &'static str;

    fn level(&self) -> usize;
    fn casting_time(&self) -> CastingTime;
    fn optional_ritual(&self) -> bool { false }
    // TODO
}

#[dynamic_choose]
pub trait Item: Debug {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn description_no_title(&self) -> &'static str;

    fn equipable(&self) -> Equipable;
    fn rarity(&self) -> Rarity;
    fn weight(&self) -> Option<u32>;
    fn cost(&self) -> Option<u32>;
    fn magical(&self) -> bool { false }
    fn attunement(&self) -> bool { false }

    fn declare(&self, _c: &mut Character) {}
    fn iterate(&self, _c: &mut Character) {}
}

#[dynamic_choose]
pub trait HalflingSubrace: Debug {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn description_no_title(&self) -> &'static str;

    fn declare(&self, _c: &mut Character) {}
    fn iterate(&self, _c: &mut Character) {}
    fn last(&mut self, _c: &mut Character) {}
}

#[dynamic_choose]
pub trait RoguishArchetype: Debug {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn description_no_title(&self) -> &'static str;

    fn declare(&self, _c: &mut Character, _lvl: u32) {}
    fn iterate(&self, _c: &mut Character, _lvl: u32) {}
    fn last(&mut self, _c: &mut Character, _lvl: u32) {}
}

#[dynamic_choose]
pub trait EldritchInvocation: Debug {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn description_no_title(&self) -> &'static str;

    fn declare(&self, _c: &mut Character, _lvl: u32) {}
    fn iterate(&self, _c: &mut Character, _lvl: u32) {}
    fn last(&self, _c: &mut Character, _lvl: u32) {}
}