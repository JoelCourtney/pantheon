#[macro_export]
macro_rules! name {
    ($name:literal) => {
        #[allow(unused_imports)] use crate::character::*;
        #[allow(unused_imports)] use crate::feature::{Feature, Choice::*, Choose};
        #[allow(unused_imports)] use crate::misc::*;
        #[allow(unused_imports)] use crate::moves::*;
        #[allow(unused_imports)] use crate::{properties, description, choose};
        #[allow(unused_imports)] use crate::content::traits::*;
        #[allow(unused_imports)] use crate::content::common::*;
        #[allow(unused_imports)] use proc_macros::{def, dynamic_choose, content};
        #[allow(unused_imports)] use serde::{Serialize, Deserialize};
        #[allow(unused_imports)] use indoc::indoc;
        #[allow(unused_imports)] use std::fmt::Debug;
        #[allow(unused_imports)] use std::convert::From;
        #[allow(unused_imports)] use enum_iterator::IntoEnumIterator;

        pub const NAME: &'static str = $name;
    }
}

#[macro_export]
macro_rules! properties {
    ($($what:ident : $t:ty = $val:expr),*) => {
        fn name(&self) -> &'static str { NAME }

        $(fn $what(&self) -> $t { $val })*
    };
    ($($bool_bois:ident),+;$($what:ident : $t:ty = $val:expr),*) => {
        fn name(&self) -> &'static str { NAME }

        $(fn $bool_bois(&self) -> bool { true })*
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

#[macro_export]
macro_rules! description {
    ($text:tt) => {
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

#[macro_export]
macro_rules! choose {
    ($e:item) => {
        #[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, enum_iterator::IntoEnumIterator, proc_macros::StaticChoose)]
        $e
    }
}
