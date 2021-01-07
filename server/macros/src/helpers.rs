use proc_macro::TokenStream;
use syn::export::TokenStream2;
use quote::quote;
use quote::format_ident;
use syn;
use syn::Expr;

pub fn list_imports(dir_str: String) -> TokenStream2 {
    let paths = match std::fs::read_dir(dir_str) {
        Ok(p) => p,
        Err(_e) => {
            return quote! {
                compile_error("args must be path to a directory");
                compile_error(#{e.to_string()})
            }
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
    acc
}

pub fn convert_to_fs(name: &str) -> String {
    strip_characters(&*name.to_owned().to_lowercase(), "'").replace(' ',"_")
}

pub fn strip_characters(original : &str, to_strip : &str) -> String {
    original.chars().filter(|&c| !to_strip.contains(c)).collect()
}

pub fn unwrap_string_tuple(t: syn::ExprTuple) -> Result<(String, String), TokenStream2> {
    let size = t.elems.len();
    if size != 2 {
        Err(quote! {
            compile_error!("expected str literal or tuple (str, str)");
        })
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
                }))
            }
        }
    }
}

pub fn content_prelude(kind: &str, input: TokenStream) -> TokenStream {
    // TODO("accept pretty name arg")
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let pascal_name_ident = ast.ident.clone();
    let kind_ident = format_ident!("{}", kind);
    let pretty_name_string = pascal_name_ident.to_string(); // TODO("allow customization")
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

        pub const CONTENT_NAME: &'static str = #pretty_name_string;

        #[derive(Debug, Serialize, Deserialize, Default)]
        #ast
    }).into()
}

pub fn process_choose_attribute(name: String, vars: Vec<String>) -> TokenStream2 {
    let enum_ident = format_ident!("{}", name);
    let choice_ident = format_ident!("{}Choice", name);
    let mut acc = quote! {
        impl Default for #enum_ident {
            fn default() -> Self {
                #enum_ident::Unknown
            }
        }

        impl Choose for #enum_ident {
            fn choose<'a>(loc: &'a mut Self) -> Box<dyn Choice + 'a> {
                Box::new( #choice_ident { locs: vec![ loc ] } )
            }
            fn choose_multiple<'a>(locs: Vec<&'a mut Self>) -> Box<dyn Choice + 'a> {
                Box::new( #choice_ident { locs } )
            }
        }

        #[derive(Debug)]
        pub struct #choice_ident<'a> {
            locs: Vec<&'a mut #enum_ident>
        }
    };
    let mut choices = "".to_string();
    for var in &vars {
        choices.extend(format!(r#""{}","#, var).chars());
    }
    let choices_tokens: TokenStream2 = choices.parse().unwrap();
    let mut match_rules = "".to_string();
    for var in &vars {
        match_rules.extend(format!(r#""{}" => {}::{},"#, var, name, var).chars());
    }
    let match_rules_tokens: TokenStream2 = match_rules.parse().unwrap();
    acc = quote! {
        #acc

        impl Choice for #choice_ident<'_> {
            fn choices(&self) -> Vec<&'static str> {
                vec![ #choices_tokens ]
            }
            fn choose(&mut self, choice: &str, index: usize) {
                **self.locs.get_mut(index).unwrap() = match choice {
                    #match_rules_tokens
                    _ => unimplemented!()
                }
            }
        }
    };
    acc
}
