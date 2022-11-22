/// Imports a common prelude for all content, and
/// sets the DNDCENT_NAME const for the module.
#[macro_export]
macro_rules! name {
    () => {
        DNDCENT_NAME
    };
    ($name:literal) => {
        #[allow(unused_imports)] use crate::character::*;
        #[allow(unused_imports)] use crate::ui::{Element, Chooseable, Event};
        #[allow(unused_imports)] use crate::misc::*;
        #[allow(unused_imports)] use crate::moves::*;
        #[allow(unused_imports)] use crate::{properties, description, name};
        #[allow(unused_imports)] use crate::content::traits::*;
        #[allow(unused_imports)] use crate::content::common::*;
        #[allow(unused_imports)] use proc_macros::{choose, dynamic_choose, content, i, m, f, asi_or_feat, asi_or_feat_fields};
        #[allow(unused_imports)] use serde::{Serialize, Deserialize};
        #[allow(unused_imports)] use std::fmt::Debug;
        #[allow(unused_imports)] use enum_iterator::IntoEnumIterator;
        #[allow(unused_imports)] use indoc::indoc;

        pub const DNDCENT_NAME: &'static str = $name;
    }
}

/// Shorthand for getter functions in content. Place directly inside the impl block.
/// Doesn't need to be included if empty.
///
/// Boolean properties are listed first, then other type properties.
/// Boolean property getters always have a default implementation in
/// the trait that returns `false`. Listing a boolean property overrides it
/// and returns `true`.
/// Then the other properties are a list in the form:
/// ```
/// ident: type = value,
/// ident2: type2 = value2
/// ```
///
/// ## Example
///
/// Copied from "Cloak of Elvenkind":
///
/// ```
/// properties! {
///     magical, attunable;
///
///     equipable: Equipable = Equipable::Yes,
///     rarity: Rarity = Rarity::Uncommon,
///     weight: Option<u32> = None,
///     cost: Option<u32> = None
/// }
/// ```
#[macro_export]
macro_rules! properties {
    ($($what:ident : $t:ty = $val:expr),*) => {
        $(fn $what(&self) -> $t { $val })*
    };
    ($($bool_bois:ident),+;$($what:ident : $t:ty = $val:expr),*) => {
        $(fn $bool_bois(&self) -> bool { true })*
        $(fn $what(&self) -> $t { $val })*
    }
}

/// Shorthand for importing a named content collection.
///
/// First accepts an optional string name of the collection, then
/// a list of module ident names to be declared.
///
/// ## Example
///
/// ```
/// crate::register! {
///     "Player's Handbook"
///     background
///     class
///     feat
///     halfling_subrace
///     race
///     roguish_archetype
/// }
/// ```
#[macro_export]
macro_rules! register {
    ($name:literal $($mods:ident)*) => {
        #[allow(dead_code)]
        pub const DNDCENT_NAME: &'static str = $name;

        $(pub mod $mods;)*
    }
}

/// Formats and processes the text description of the content.
///
/// The text argument is passed through the `indoc::indoc!` macro for you.
#[macro_export]
macro_rules! description {
    ($text:tt) => {
        fn name(&self) -> &'static str { DNDCENT_NAME }
        fn description(&self) -> &'static str {
            indoc! { $text }
        }
        fn description_no_title(&self) -> &'static str {
            let text: &'static str = self.description();
            let newline = text.find('\n')
                .expect("first first newline failed");
            let newline = newline + 1 + &text[newline+1..].find('\n')
                .expect("find second newline failed");
            &text[newline+1..]
        }
    }
}