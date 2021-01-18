use std::fmt::Debug;
use crate::content::Content;
use macros::dynamic_choose;
use crate::feature::Choose;

#[dynamic_choose]
pub trait HalflingSubrace: Content + Debug {
    fn content_name(&self) -> &'static str;
}