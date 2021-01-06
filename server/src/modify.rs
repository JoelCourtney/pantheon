use crate::character::Character;
use crate::feature::*;

/// Any object that alters the character at some point during resolution.
///
/// The Character's Race, Classes, Subclasses, etc., all impact the character sheet. This trait is
/// shared by all objects the act during the resolution process. The process occurs sequentially in three phases:
///
/// - initialize
/// - modify
/// - finalize
///
/// When writing a Modify implementor, keep in mind that every other Modify attached to the character
/// is also running, and in an unknown order. All you know is that all `initialize`s run, then all
/// `modify`s, and then all `finalize`s. Use implementations of these methods to get precedence.
///
/// # Examples
///
/// Lets say we want to alter the characters STR stat. There are three common operations:
///
/// - setting base STR
/// - incrementing or decrementing STR
/// - overriding final STR
///
/// These three fit nicely with `initialize`, `modify`, and `finalize` respectively. Putting that
/// logic in a different phase would likely break in certain combinations with other Modifys.
///
/// # Rules phases
///
/// There are `initialize_rules` and `finalize_rules` that are not alterable. These are common effects
/// that apply to all characters regardless of build, and are run before and after the three Modify
/// phases respectively.
pub trait Modify {
    fn initialize(&self, _: &mut Character) {}
    fn modify(&self, _: &mut Character) {}
    fn finalize(&self, _: &mut Character) {}
}

pub trait Race: Modify {
    fn traits(&mut self) -> Vec<Feature>;
}
pub trait Class: Modify {}
pub trait Subclass: Modify {}
pub trait Background: Modify {}
pub trait Feat: Modify {}
pub trait Item: Modify {}