use std::fmt::Debug;
use proc_macros::dynamic_choose;
use crate::ui::Chooseable;
use crate::character::Character;
use crate::misc::{CastingTime, Equipable, Rarity, Equipped};

#[dynamic_choose]
pub trait Race: Debug {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn description_no_title(&self) -> &'static str;

    fn resolve(&mut self, _c: &mut Character) {}
}

#[dynamic_choose]
pub trait Class: Debug {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn description_no_title(&self) -> &'static str;

    fn hit_dice(&self) -> u32;

    fn resolve(&mut self, _c: &mut Character, _level: u32, _index: usize) {}
}

#[dynamic_choose]
pub trait Feat: Debug {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn description_no_title(&self) -> &'static str;

    fn resolve(&mut self, _c: &mut Character) {}
}

#[dynamic_choose]
pub trait Background: Debug {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn description_no_title(&self) -> &'static str;

    fn resolve(&mut self, _c: &mut Character) {}
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

/// NOTE if implementing a Weapon item, do:
/// - initialize c.attack_moves
/// - add the attack associated with this item IN INITIALIZATION
/// - mark the ability score associated with the +hit modifier.
/// - set the name of the attack identically to the name of the weapon.
///
/// Do not:
/// - automatically add the ability score (other content might change which score is used)
/// - check for proficiency and add the proficiency bonus (the common rules will do that).
#[dynamic_choose]
pub trait Item: Debug {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn description_no_title(&self) -> &'static str;

    fn equipable(&self) -> Equipable;
    fn rarity(&self) -> Rarity { Rarity::Common }
    fn weight(&self) -> Option<u32> { None }
    fn cost(&self) -> Option<u32> { None }

    fn magical(&self) -> bool { false }
    fn attunable(&self) -> bool { false }
    fn ammunition(&self) -> bool { false }

    fn resolve(&mut self, _c: &mut Character, _equipped: Equipped, _attuned: bool) {}
}

#[dynamic_choose]
pub trait HalflingSubrace: Debug {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn description_no_title(&self) -> &'static str;

    fn resolve(&mut self, _c: &mut Character) {}
}

#[dynamic_choose]
pub trait RoguishArchetype: Debug {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn description_no_title(&self) -> &'static str;

    fn resolve(&mut self, _c: &mut Character, _level: u32, _index: usize) {}
}

#[dynamic_choose]
pub trait EldritchInvocation: Debug {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn description_no_title(&self) -> &'static str;

    fn resolve(&mut self, _c: &mut Character, _level: u32, _index: usize) {}
}