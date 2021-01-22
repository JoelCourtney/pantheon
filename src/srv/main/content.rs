mod system;
mod official;
mod playtest;
mod homebrew;
pub(crate) mod traits;
pub(crate) mod common;

macros::registry!(17);

/// Contains where to find a particular file.
///
/// collection = one of "official", "playtest", "homebrew"
/// source = name of book or homebrew creator
#[derive(Eq, PartialEq, Debug, Hash)]
struct Registration {
    collection: &'static str,
    source: &'static str,
}

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
        fn description(&self) -> &'static str { Self::description_with_title() }
        fn description_no_title(&self) -> &'static str { Self::description_without_title() }
        $(fn $what(&self) -> $t { $val })*
    }
}

#[macro_export]
macro_rules! register {
    ($($mods:ident),*) => {
        $(pub mod $mods;)*
    };
    ($name:literal, $($mods:ident),*) => {
        pub const NAME: &'static str = $name;
        $(pub mod $mods;)*
    }
}