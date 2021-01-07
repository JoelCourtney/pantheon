use proc_macro::TokenStream;
use quote::quote;
use quote::format_ident;
use syn;
use syn::Expr;

pub fn list_imports(dir_str: String) -> TokenStream {
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

pub fn convert_to_fs(name: &str) -> String {
    strip_characters(&*name.to_owned().to_lowercase(), "'").replace(' ',"_")
}

pub fn strip_characters(original : &str, to_strip : &str) -> String {
    original.chars().filter(|&c| !to_strip.contains(c)).collect()
}

pub fn unwrap_string_tuple(t: syn::ExprTuple) -> Result<(String, String), TokenStream> {
    let size = t.elems.len();
    if size != 2 {
        Err((quote! {
            compile_error!("expected str literal or tuple (str, str)");
        }).into())
    } else {
        let first = t.elems.first().unwrap();
        let last = t.elems.last().unwrap();
        match (first, last) {
            (
                Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit1), .. }),
                Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit2), .. })
            ) => Ok((lit1.value(), lit2.value())),
            _ => {
                Err(quote!({
                    compile_error!("both arguments must be str");
                }).into())
            }
        }
    }
}

pub fn content_prelude(kind: &str, input: TokenStream) -> TokenStream {
    // TODO("accept pretty name arg")
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let pascal_name_ident = ast.ident.clone();
    let kind_ident = format_ident!("{}", kind);
    (quote! {
        use crate::character::*;
        use crate::modify::*;
        use crate::feature::*;
        use crate::misc::*;
        use crate::describe::*;
        use macros::{def, describe, choose};
        use serde::{Serialize, Deserialize};
        use indoc::indoc;

        #[typetag::serde]
        impl #kind_ident for #pascal_name_ident {

        }

        pub fn new() -> Box<dyn #kind_ident> {
            Box::new( #pascal_name_ident {
                ..def!()
            } )
        }

        #[derive(Debug, Serialize, Deserialize, Default)]
        #ast
    }).into()
}