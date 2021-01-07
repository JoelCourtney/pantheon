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
use std::collections::{HashSet, HashMap};

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
/// macros::register!("/official/players_handbook/races");
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
#[proc_macro]
pub fn register(input: TokenStream) -> TokenStream {
    let ast: syn::Expr = syn::parse(input).unwrap();
    let mut collection_name: String = "".to_string();
    let gen = match ast {
        Expr::Tuple(t) => {
            match unwrap_string_tuple(t) {
                Ok((dir_str, name)) => {
                    let dir_str = format!("./src/content{}", dir_str.as_str());
                    collection_name = name.to_owned();
                    list_imports(dir_str)
                }
                Err(e) => e
            }
        }
        Expr::Lit(syn::ExprLit{lit: syn::Lit::Str(lit),..}) => {
            let dir_str = format!("./src/content{}",lit.value());
            collection_name = dir_str[dir_str.rfind('/').expect("dir must contain /")+1..].to_owned();
            list_imports(dir_str)
        }
        _ => unimplemented!()
    };
    (quote! {
        #gen
        pub const COLLECTION_NAME: &'static str = #collection_name;
    }).into()
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
pub fn race(_: TokenStream, input: TokenStream) -> TokenStream {
    content_prelude("Race", input)
}

#[proc_macro_attribute]
pub fn feat(_: TokenStream, input: TokenStream) -> TokenStream {
    content_prelude("Feat", input)
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
            let pascal_ident = format_ident!("{}", pascal_name);

            let string_part: TokenStream2 = format!(r##"(indoc! {{r#"{}"#}})"##,
                    text
            ).parse().unwrap();
            (quote! {
                impl Describe for #pascal_ident {
                    fn description_with_title() -> &'static str {
                        #string_part
                    }
                }
            }).into()
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
#[proc_macro_attribute]
pub fn registry(args: TokenStream, _: TokenStream) -> TokenStream {
    let declared_content_files: usize = syn::parse::<syn::LitInt>(args).unwrap().base10_parse::<usize>().unwrap();
    let mut counted_content_files: usize = 0;

    let registration = collect_registration();

    let mut registry_struct_fields = quote! {};
    let mut content_constructor_functions = quote! {};
    let mut registry_constructor_fields = quote! {};
    for (key, entries) in &registration {
        let type_ident_plural = format_ident!("{}", key);
        let type_string_upper = string_to_content_type(&key);
        let type_ident_upper = format_ident!("{}", type_string_upper);
        let type_string_lower = type_string_upper.to_lowercase();
        let type_ident_lower= format_ident!("{}", type_string_lower);

        registry_struct_fields = quote! {
            #registry_struct_fields
            #type_ident_plural: HashMap<&'static str, (Registration, fn() -> Box<dyn #type_ident_upper>)>,
        };

        content_constructor_functions = quote! {
            #content_constructor_functions
            pub fn #type_ident_lower(&self, search_name: &str) -> Option<Box<dyn #type_ident_upper>> {
                match self.#type_ident_plural.get(search_name) {
                    Some((_, construct)) => Some(construct()),
                    None => None
                }
            }
        };

        let mut registry_constructor_field_entries = quote! {};
        for (collection, source, content) in entries {
            counted_content_files += 1;

            let collection_ident = format_ident!("{}", collection);
            let source_ident = format_ident!("{}", source);
            let content_ident = format_ident!("{}", content);

            registry_constructor_field_entries = quote! {
                #registry_constructor_field_entries
                #collection_ident::#source_ident::#type_ident_plural::#content_ident::CONTENT_NAME => (
                    Registration {
                        collection: #collection_ident::COLLECTION_NAME,
                        source: #collection_ident::#source_ident::COLLECTION_NAME
                    },
                    #collection_ident::#source_ident::#type_ident_plural::#content_ident::new as fn() -> Box<dyn #type_ident_upper>
                ),
            }
        }
        registry_constructor_fields = quote! {
            #registry_constructor_fields
            #type_ident_plural: hashmap! {
                #registry_constructor_field_entries
            },
        };
    }
    if counted_content_files == declared_content_files {
        (quote! {
            #[derive(Debug)]
            pub struct Registry {
                #registry_struct_fields
            }

            use std::collections::HashMap;
            use crate::character::*;
            use maplit::hashmap;

            impl Registry {
                pub fn new() -> Self {
                    Registry {
                        #registry_constructor_fields
                    }
                }

                #content_constructor_functions
            }
        }).into()
    } else {
        format!(
            r#"compile_error!("content file count mismatch: expected {}, found {}");"#,
            declared_content_files,
            counted_content_files
        ).parse().unwrap()
    }
}

fn string_to_content_type(str: &str) -> String {
    (match str {
        "races" => "Race",
        "feats" => "Feat",
        _ => panic!("unknown content type")
    }).to_string()
}

fn collect_registration() -> HashMap<
    String /*type name*/,
    HashSet<(
        String /*collection dirname*/,
        String /*source dirname*/,
        String /*content filename*/
    )>
> {
    use std::path::Component::Normal;
    use walkdir::WalkDir;

    let mut result: HashMap<String, HashSet<(String, String, String)>> = HashMap::new();
    for entry in WalkDir::new("src/content") {
        let entry = entry.unwrap();
        let file_name = entry.file_name().to_str().unwrap();
        if file_name.rfind(".rs") != None && file_name != "mod.rs" {
            let comps: Vec<std::path::Component> = entry.path().components().collect();
            let collection = match comps.get(2).unwrap() {
                Normal(s) => s.to_str().unwrap().to_owned(),
                _ => panic!()
            };
            let source = match comps.get(3).unwrap() {
                Normal(s) => s.to_str().unwrap().to_owned(),
                _ => panic!()
            };
            let typ = match comps.get(4).unwrap() {
                Normal(s) => s.to_str().unwrap().to_owned(),
                _ => panic!()
            };
            let content = file_name[..file_name.len()-3].to_string();
            if !result.contains_key(&typ) {
                result.insert(typ.to_owned(), HashSet::new());
            }
            result.get_mut(&typ).unwrap().insert((collection.to_owned(), source.to_owned(), content.to_owned()));
        }
    }
    result
}