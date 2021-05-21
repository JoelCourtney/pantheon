//! This sub-crate contains macros that make content management easier.

extern crate proc_macro;

mod choose;
mod content;
mod registry;
mod character;
mod server;

use proc_macro::TokenStream;

/// Generates the Registry struct and some of its methods.
///
/// Crawls through the file tree of `src/content` to find all content structs, and hard-codes
/// a function that enumerates each of these structs and their auto-generated constructors in a giant
/// hashmap.
///
/// Note that the FS crawl happens at compile-time, not runtime, and that every value placed in the
/// hashmap is a compile-time constant.
///
/// # Example
///
/// Don't use this macro yourself. It should only be used in `content/mod.rs`.
#[proc_macro]
pub fn registry(_: TokenStream) -> TokenStream {
    registry::registry()
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
/// two skills or one skill and Thieves' Tools). This is a fake example, and not how it is actually
/// implemented in the Rogue class.
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
/// impl Race for VariantHuman {
///     fn last(&mut self, c: &mut Character) {
///         c.race_traits.extend(vec! [
///             Feature (
///                 "**Ability Score Increase:** Two different ability scores of your choice increase by 1.",
///                 Any(&mut self.abilities)
///             ),
///             // ... add more choice features depending on the result. See rogue.rs for full example.
///         ]);
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn choose(_args: TokenStream, input: TokenStream) -> TokenStream {
    let ast: syn::ItemEnum = syn::parse(input)
        .expect("parse choose arg failed");
    choose::choose(ast)
}

#[proc_macro_attribute]
pub fn dynamic_choose(_: TokenStream, input: TokenStream) -> TokenStream {
    let ast: syn::ItemTrait = syn::parse(input).expect("expected trait declaration");
    choose::dynamic_choose(ast)
}

/// Derives the count_unresolved and finalize methods for the Character struct. It also generates
/// the FinalCharacter struct, which is returned from finalize.
///
/// count_unresolved counts the number of initializers, modifiers, and finalizers that
/// have not yet run. This is used to look for deadlock. If the count is nonzero
/// and doesn't decrease after an iteration, then there is a deadlock.
///
/// finalize unwraps all the Staged-wrapped and Map<Staged>-wrapped values in Character, so
/// it can be serialized without the unnecessary Staged structs.
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

#[proc_macro]
pub fn i(input: TokenStream) -> TokenStream {
    content::stages(input, "initialize").into()
}

#[proc_macro]
pub fn m(input: TokenStream) -> TokenStream {
    content::stages(input, "modify").into()
}

#[proc_macro]
pub fn f(input: TokenStream) -> TokenStream {
    content::stages(input, "finalize").into()
}

#[proc_macro]
pub fn unique_id(_input: TokenStream) -> TokenStream {
    let id = content::next_id();
    (quote::quote! { #id }).into()
}