use std::fmt::Debug;
use crate::content::{Content, LeveledContent};
use macros::dynamic_choose;
use crate::feature::Choose;

#[dynamic_choose]
pub trait HalflingSubrace: Content + Debug {
    fn content_name(&self) -> &'static str;
}

#[dynamic_choose]
pub trait RoguishArchetype: LeveledContent + Debug {
    fn content_name(&self) -> &'static str;
}

#[dynamic_choose]
pub trait EldritchInvocation: Content + Debug {
    fn content_name(&self) -> &'static str;
}