//! This sub-crate contains macros that make content management easier.
//!
//! The intent is to make adding a new Modifier (see top-level crate) as easy as making a new struct
//! in a new file. Without macros or other means of generating source, that would require the
//! dev to also add module imports and map entries to register the new struct with the rest of dndcent,
//! and that sounds terrible and error-prone. These macros automate that process.

extern crate proc_macro;

mod choose;
mod content;
mod registry;
mod character;

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
pub fn registry(args: TokenStream) -> TokenStream {
    let declared_content_files: usize = syn::parse::<syn::LitInt>(args).unwrap().base10_parse::<usize>().unwrap();
    registry::registry(declared_content_files)
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

/// Wraps a description string in a trait impl, because I'm lazy.
///
/// In doing so, it forces all descriptions to begin with "# NAME", where name is the pretty-print
/// version of the name; so there's some consistency at least.
///
/// The input string is wrapped in an `indoc! {}` call, so you can rely on that logic for formatting.
///
/// ## Examples
///
/// See `/official/players_handbook/race/human.rs` for a full example. Small example:
///
/// ```
/// describe! { r#"
///     # Example Description
/// "# }
/// ```
///
/// Would turn into:
///
/// ```
/// impl Describe for ExampleDescription {
///     fn describe() -> String {
///         (indoc! { r#"
///             # Example Description
///         "# }).to_string()
///     }
/// }
/// ```
#[proc_macro]
pub fn describe(input: TokenStream) -> TokenStream {
    let text: String = syn::parse::<syn::LitStr>(input).unwrap().value();
    content::describe(text)
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
#[proc_macro_attribute]
pub fn choose(_: TokenStream, input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
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
    let ast: syn::ItemImpl = syn::parse(input).unwrap();
    content::content(ast)
}