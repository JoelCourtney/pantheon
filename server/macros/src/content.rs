use proc_macro::TokenStream;
use syn::export::TokenStream2;
use inflector::Inflector;
use quote::{quote, format_ident};

pub(crate) fn describe(text: String) -> TokenStream {
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
            let snake_name = to_fs_friendly(&pretty_name);
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

pub(crate) fn prelude(kind: &str, ast: syn::DeriveInput) -> TokenStream {
    // TODO("accept pretty name arg")
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

fn to_fs_friendly(name: &str) -> String {
    strip_characters(&*name.to_owned().to_lowercase(), "'").replace(' ',"_")
}

fn strip_characters(original : &str, to_strip : &str) -> String {
    original.chars().filter(|&c| !to_strip.contains(c)).collect()
}