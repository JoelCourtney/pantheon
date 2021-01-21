//! This sub-crate contains macros that make content management easier.
//!
//! The intent is to make adding a new Modifier (see top-level crate) as easy as making a new struct
//! in a new file. Without macros or other means of generating source, that would require the
//! dev to also add module imports and map entries to register the new struct with the rest of dndcent,
//! and that sounds terrible and error-prone. These macros automate that process.

extern crate proc_macro;

mod choose;
mod content;
mod register;
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

/// Generates auto-imports for the contents of a directory, and registers the directory under
/// a pretty-print name.
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
/// macros::register!("/official/players_handbook/race");
/// ```
///
/// Or:
///
/// ```
/// macros::register!(("/official", "Player's Handbook"));
/// ```
///
/// Use the tuple version only for Book directories that require pretty printing. See
/// `/content/mod.rs` for file structure and naming convention. I'll repeat some here:
///
/// When using the tuple input, the second arg is the name of the dir *that contains the mod.rs file
/// you are registering*. Make this string exactly equal to the name of the book, including capitalization,
/// spaces, and punctuation. Do include that directory in the path arg.
///
/// When using the single-string path input, make it the path of the current dir. This will autogenerate
/// a name to be registered, based on the name of the directory.
///
/// Always start one level down from `content`. I.E., the path will *always* start with one of:
///
/// - "/official"
/// - "/playtest"
/// - "/homebrew"
/// - "/system"
#[proc_macro]
pub fn register(input: TokenStream) -> TokenStream {
    let ast: syn::Expr = syn::parse(input).unwrap();
    register::register(ast)
}

/// Registers a race struct and pastes some boilerplate code. Must be the first line of the file.
///
/// If no name argument is given, the name will be registered as the name of the struct. Otherwise
/// it will be registered under the name given. This is needed when the name contains spaces, dashes,
/// or other fancy characters.
///
/// Calling this macro is required for all race structs. Your race will not be useable by the user
/// otherwise. It applies the derive macro for Debug, Serialize, Deserialize, and Default for you.
/// If you want to provide your own impls for those four, you can't. That feature will be implemented
/// later if needed.
///
/// This also pastes in common and required use declarations.
///
/// # Example
///
/// Copied from players_handbook/race/human.rs:
///
/// ```
/// #[macros::race]
/// struct Human {
///     extra_language: Language
/// }
/// ```
///
/// This will require the dev to impl the Modify, Featured, and Describe traits. See their docs for deats.
/// While the `macros::` prefix is a little ugly, this macro includes a `use` for all other relevant
/// macros, so you don't have to do that for future macros used in this file.
///
/// ```
/// impl Modify for Human {
///     // ...
/// }
/// impl Featured for Human {
///     // ...
/// }
/// describe! { r"#
///     # Human
///
///     yes hello i am human
/// "# }
/// ```
#[proc_macro_attribute]
pub fn race(args: TokenStream, input: TokenStream) -> TokenStream {
    let pretty = content::pretty_name(args);
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    content::prelude("Race", ast, pretty)
}

#[proc_macro_attribute]
pub fn class(args: TokenStream, input: TokenStream) -> TokenStream {
    let pretty = content::pretty_name(args);
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    content::prelude("Class", ast, pretty)
}

#[proc_macro_attribute]
pub fn feat(args: TokenStream, input: TokenStream) -> TokenStream {
    let pretty = content::pretty_name(args);
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    content::prelude("Feat", ast, pretty)
}

#[proc_macro_attribute]
pub fn spell(args: TokenStream, input: TokenStream) -> TokenStream {
    let pretty = content::pretty_name(args);
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    content::prelude("Spell", ast, pretty)
}

#[proc_macro_attribute]
pub fn custom_content(args: TokenStream, input: TokenStream) -> TokenStream {
    let (subtype, pretty) = match content::subtype_and_pretty_name(args) {
        Ok(t) => t,
        Err(s) => return s.into()
    };
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    content::prelude(&subtype, ast, pretty)
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
