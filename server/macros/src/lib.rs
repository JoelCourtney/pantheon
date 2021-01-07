//! This sub-crate contains macros that make content management easier.
//!
//! The intent is to make adding a new Modifier (see top-level crate) as easy as making a new struct
//! in a new file. Without macros or other means of generating source, that would require the
//! dev to also add module imports and map entries to register the new struct with the rest of dndcent,
//! and that sounds terrible and error-prone. These macros automate that process.

#[deny(missing_docs)]

extern crate proc_macro;
extern crate inflector;

mod helpers;

use proc_macro::{TokenStream};
use quote::quote;
use quote::format_ident;
use syn;
use syn::{Expr, Data};
use helpers::*;
use inflector::Inflector;
use syn::export::TokenStream2;

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

/// Registers a race struct and pastes some boilerplate code.
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
/// Copied from players_handbook/races/human.rs:
///
/// ```
/// #[macros::race]
/// struct Human {
///     extra_language: Language
/// }
/// ```
///
/// This will require the dev to impl the Modify, Featured, and Describe traits. See their docs for deats.
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
pub fn race(_args: TokenStream, input: TokenStream) -> TokenStream {
    // TODO("accept pretty name arg")
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let pascal_name_ident = ast.ident.clone();
    (quote! {
        use crate::character::*;
        use crate::modify::*;
        use crate::feature::*;
        use crate::misc::*;
        use crate::describe::*;
        use macros::{def, describe};
        use serde::{Serialize, Deserialize};
        use indoc::indoc;

        #[typetag::serde]
        impl Race for #pascal_name_ident {}

        #[derive(Debug, Serialize, Deserialize, Default)]
        #ast
    }).into()
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
/// See `/official/players_handbook/races/human.rs` for a full example. Small example:
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
    let h1 = text.find('#');
    match h1 {
        Some(mut i) => {
            i += 1;
            while text.chars().nth(i) == Some(' ') {
                i += 1;
            }
            let newline = match &text[i..].find('\n') {
                Some(j) => j + i,
                None => text.len()
            };
            let pretty_name: String = text[i..newline].to_string();
            let snake_name = convert_to_fs(&pretty_name);
            let pascal_name = snake_name.to_pascal_case();

            // BOW BEFORE MY RAW STRINGS
            format!(r##"impl Describe for {} {{
                fn describe() -> String {{ (indoc! {{r#"{}"#}}).to_string()}} }}"##,
                    pascal_name, text
            ).parse().unwrap()
        }
        None => {
            (quote! {
                compile_error!("description must start with H1 with name, such as: # Example Title");
            }).into()
        }
    }
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
/// #[class("Example Rogue")]
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
    let ident = ast.ident.clone();
    (match ast.data.clone() {
        Data::Enum(syn::DataEnum { variants, .. }) => {
            let vars = variants.iter().map( |var| {
                var.ident.to_string()
            }).collect();
            let process = process_choose_attribute(ident.to_string(), vars);
            quote! {
                #[derive(Debug, Serialize, Deserialize, Copy, Clone)]
                #ast
                #process
            }
        }
        _ => quote! {
            compile_error("Choose derive only valid for enums");
        }
    }).into()
}

fn process_choose_attribute(name: String, vars: Vec<String>) -> TokenStream2 {
    let enum_ident = format_ident!("{}", name);
    let choice_ident = format_ident!("{}Choice", name);
    let mut acc = quote! {
        impl Default for #enum_ident {
            fn default() -> Self {
                #enum_ident::Unknown
            }
        }

        impl Choose for #enum_ident {
            fn choice<'a>(loc : &'a mut Self) -> Box<dyn Choice + 'a> {
                Box::new( #choice_ident { loc } )
            }
        }

        #[derive(Debug)]
        pub struct #choice_ident <'a> {
            loc: &'a mut #enum_ident
        }
    };
    let mut text = format!("impl Choice for {}Choice<'_> {{\n", name);
    text.extend("fn choices(&self) -> Vec<&str> {\nvec![".chars());
    for var in &vars {
        text.extend(format!(r#""{}","#, var).chars());
    }
    text.extend("\n]}\nfn choose(&mut self, choice: &str) { \n *self.loc = match choice {\n".chars());
    for var in vars {
        text.extend(format!(r#""{}" => {}::{},"#, var, name, var).chars());
    }
    text.extend("_ => unimplemented!() }}}".chars());
    let rest: TokenStream2 = text.parse().unwrap();
    acc = quote! {
        #acc
        #rest
    };
    acc
}
