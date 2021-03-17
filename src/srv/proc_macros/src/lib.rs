//! This sub-crate contains macros that make content management easier.
//!
//! The intent is to make adding a new Modifier (see top-level crate) as easy as making a new struct
//! in a new file. Without macros or other means of generating source, that would require the
//! dev to also add module imports and map entries to register the new struct with the rest of dndcent,
//! and that sounds terrible and error-prone. These macros automate that process.

extern crate proc_macro;
extern crate darling;

mod choose;
mod content;
mod registry;
mod character;
mod server;

use proc_macro::TokenStream;
use quote::quote;

/// Generates the Registry struct and some of its methods.
///
/// Crawls through the file tree of `src/content` to find all content structs, and hard-codes
/// a function that enumerates each of these structs and their auto-generated constructors in a giant
/// hashmap.
///
/// Note that the FS crawl happens at compile-time, not runtime, and that every value placed in the
/// hashmap is a compile-time constant.
///
/// # Input
///
/// The input arg is a usize, the number of content files it its expected to find. If it does not find exactly
/// that number of files, it throws a compile error. This count does not include mod.rs's, only
/// content files.
///
/// # Example
///
/// Don't use this macro yourself. It should only be used in `content/mod.rs`.
#[proc_macro]
pub fn registry(_: TokenStream) -> TokenStream {
    registry::registry()
}

/// I'm lazy. Seriously, I'm this lazy.
///
/// It expands into `Default::default()`. Because I'm lazy.
///
/// ## Example
///
/// ```
/// Trait {
///     name: "Example Trait",
///     description: "It doesn't do anything other than be a described trait.",
///     ..def!()
/// }
/// ```
#[proc_macro]
pub fn def(_: TokenStream) -> TokenStream {
    (quote! {
        Default::default()
    }).into()
}

/// Derives a bunch of boilerplate to make the enum chooseable through the Choose trait.
///
/// This allows an enum to be displayed as a dropdown for the user to pick something from. It would
/// be a derive macro, but I also wanted to apply a derive macro to the input, so its an attribute.
///
///
/// # Example
///
/// Suppose the dev wants to give the user a choice (for example Rogue expertise: choice between
/// two skills or one skill and Thieves' Tools).
///
/// ```
/// #[macros::class("Example Rogue")]
/// struct ExampleRogue {
///     choice: ExpertiseChoice
/// }
/// ```
///
/// Below is the definition of ExpertiseChoice. Note the `Unknown` variant, which is **required**.
///
/// ```
/// #[choose]
/// enum ExpertiseChoice {
///     TwoSkills,
///     OnePlusThievesTools,
///     Unknown
/// }
/// ```
///
/// The enum can now be used to give the user a choice in the Featured trait.
///
/// ```
/// impl Featured for ExampleRogue {
///     fn features(&self) -> Vec<Feature> {
///         let mut features = vec![
///             Feature {
///                 name: "Expertise",
///                 description: "do the expertise thing",
///                 choice: ExpertiseChoice::choose(&mut self.choice)
///             },
///         ];
///         // ... add more choice features depending on the result. See rogue.rs for full example.
///     }
/// }
/// ```
#[proc_macro_derive(StaticChoose, attributes(choose))]
pub fn choose(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input)
        .expect("parse choose arg failed");
    choose::choose(ast)
}

#[proc_macro_attribute]
pub fn dynamic_choose(_: TokenStream, input: TokenStream) -> TokenStream {
    let ast: syn::ItemTrait = syn::parse(input).expect("expected trait declaration");
    choose::dynamic_choose(ast)
}

#[proc_macro_derive(FinalizeCharacter)]
pub fn derive_finalize(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).expect("expected derive input");
    character::finalize(ast)
}

#[proc_macro_attribute]
pub fn content(_args: TokenStream, input: TokenStream) -> TokenStream {
    let ast: syn::ItemImpl = syn::parse(input)
        .expect("parse content arg failed");
    content::content(ast)
}

#[proc_macro]
pub fn match_raw_files(input: TokenStream) -> TokenStream {
    let ast: syn::ExprArray = syn::parse(input).expect("expected expr array");
    server::match_raw_files(ast)
}
