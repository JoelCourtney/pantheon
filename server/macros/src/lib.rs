//! This sub-crate contains macros that make content management easier.
//!
//! The intent is to make adding a new Modifier (see top-level crate) as easy as making a new struct
//! in a new file. Without macros or other means of generating source, that would require the
//! dev to also add module imports and map entries to register the new struct with the rest of dndcent,
//! and that sounds terrible and error-prone. These macros automate that process.

#[deny(missing_docs)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use quote::format_ident;
use syn;
use syn::Expr;

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
/// matches the name of the book with the scheme: all lower case, no punctuation, and spaces are replaced
/// with underscores.
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
            let size = t.elems.len();
            if size != 2 {
                (quote! {
                    compile_error!("expected str literal or tuple (str, str)");
                }).into()
            } else {
                let first = t.elems.first().unwrap();
                let last = t.elems.last().unwrap();
                let (dir_str, name) = match (first, last) {
                    (
                        Expr::Lit(syn::ExprLit{lit: syn::Lit::Str(lit1),..}),
                        Expr::Lit(syn::ExprLit{lit: syn::Lit::Str(lit2),..})
                    ) => (lit1.value(),lit2.value()),
                    _ => {
                        return quote! ({
                            compile_error!("both arguments must be str");
                        }).into()
                    }
                };
                let dir_str = format!("./src/content{}/{}", dir_str.as_str(), convert_to_fs(&*name));
                list_imports(dir_str)
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

fn list_imports(dir_str: String) -> TokenStream {
    let paths = match std::fs::read_dir(dir_str) {
        Ok(p) => p,
        Err(_e) => {
            return quote! {
                compile_error("args must be path to a directory");
                compile_error(#{e.to_string()})
            }.into()
        }
    };
    let mut acc = quote! {};
    for path in paths {
        let path = path.unwrap().path();
        if !path.ends_with("mod.rs") {
            let endpoint = path.file_name().unwrap().to_str().unwrap();
            if endpoint.ends_with(".rs") {
                let endpoint = format_ident!("{}", &endpoint[..endpoint.len()-3]);
                acc = quote! {
                    #acc
                    pub mod #endpoint;
                };
            } else {
                let endpoint = format_ident!("{}", endpoint);
                acc = quote! {
                    #acc
                    pub mod #endpoint;
                };
            }
        }
    }
    acc.into()
}

fn convert_to_fs(name: &str) -> String {
    strip_characters(&*name.to_owned().to_lowercase(), "'").replace(' ',"_")
}

fn strip_characters(original : &str, to_strip : &str) -> String {
    original.chars().filter(|&c| !to_strip.contains(c)).collect()
}