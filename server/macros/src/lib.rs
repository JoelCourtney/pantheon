//! This sub-crate contains macros that make content management easier.
//!
//! The intent is to make adding a new Modifier (see top-level crate) as easy as making a new struct
//! in a new file. Without macros or other means of generating source, that would require the
//! dev to also add module imports and map entries to register the new struct with the rest of dndcent,
//! and that sounds terrible and error-prone. These macros automate that process.

#[deny(missing_docs)]

extern crate proc_macro;

mod helpers;

use proc_macro::TokenStream;
use quote::quote;
// use quote::format_ident;
use syn;
use syn::Expr;
use helpers::*;

/// Generates auto-imports and registry entries for the contents of a directory.
///
/// Works for both dirs that have directory children and dirs that have file children. It is assumed
/// that no directory has BOTH, other than the obligitory mod.rs, which is ignored. It should still
/// work if a dir does have both, but I'm not testing for it.
///
/// # Input
///
/// Input is either a 2-tuple of strings, containing a path and a name, or a single string path. Such as:
///
/// ```
/// macros::registry!("/official/players_handbook/races");
/// ```
///
/// Or:
///
/// ```
/// macros::registry!(("/official", "Player's Handbook"));
/// ```
///
/// Use the tuple version only for Book directories that require pretty printing. See
/// `/content/mod.rs` for file structure and naming convention. I'll repeat some here:
///
/// When using the tuple input, the name arg is the name of the dir *that contains the mod.rs file
/// you are registering*. Make this string exactly equal to the name of the book, including capitalization,
/// spaces, and punctuation. Don't include that directory in the path arg. Make sure the name of the dir
/// matches the name of the book, in lower snake case with no punctuation.
///
/// When using the single-string path input, make it the path of the current dir.
///
/// Always start one level down from `content`. I.E., the path will *always* start with one of:
///
/// - "/official"
/// - "/playtest"
/// - "/homebrew"
#[proc_macro]
pub fn registry(input: TokenStream) -> TokenStream {
    let ast: syn::Expr = syn::parse(input).unwrap();
    let gen = match ast {
        Expr::Tuple(t) => {
            match unwrap_string_tuple(t) {
                Ok((dir_str, name)) => {
                    let dir_str = format!("./src/content{}/{}", dir_str.as_str(), convert_to_fs(&*name));
                    list_imports(dir_str)
                }
                Err(e) => e
            }
        }
        Expr::Lit(syn::ExprLit{lit: syn::Lit::Str(lit),..}) => {
            let dir_str = format!("./src/content{}",lit.value());
            list_imports(dir_str)
        }
        _ => unimplemented!()
    };
    gen
}

/// Adds some boilerplate code for a Modify Implementor, and adds it to the registry.
///
/// Boilerplate includes Character and everything in modify.rs. It also automatically adds an `impl`
/// for the trait associated with the "kind" argument.
///
/// # Input
///
/// Input expects a 2-tuple of strings: ("Name", "Kind"), where Name is the name of the object as
/// displayed to the User and Kind is one of:
///
/// - Race
/// - Class
/// - Subclass
/// - Background
/// - Feat
/// - TODO("finish this")
///
/// Kind is case insensitive, but prefer the given capitalization anyway.
///
/// # Examples
///
/// ```
/// macros::register!(("Human", "Race"));
/// ```
#[proc_macro]
pub fn register(input: TokenStream) -> TokenStream {
    let ast: syn::Expr = syn::parse(input).unwrap();
    if let Expr::Tuple(t) = ast {
        match unwrap_string_tuple(t) {
            Ok((_name, _kind)) => {
                // let name_ident = format_ident!("{}", name);
                // let kind_ident = format_ident!("{}{}", &kind[0..1].to_uppercase(), &kind[1..].to_lowercase());
                (quote! {
                    // use crate::modify::*;
                    // use crate::character::*;

                    // impl #kind_ident for #name_ident {}
                }).into()
            }
            Err(e) => e
        }
    } else {
        (quote! {
            compile_error!("expected 2-tuple of strings");
        }).into()
    }
}
