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

fn to_fs_friendly(name: &str) -> String {
    strip_characters(&*name.to_owned().to_lowercase(), "'").replace(' ',"_")
}

fn strip_characters(original : &str, to_strip : &str) -> String {
    original.chars().filter(|&c| !to_strip.contains(c)).collect()
}

pub(crate) fn content(ast: syn::ItemImpl) -> TokenStream {
    let clone = ast.clone();
    let ty = match clone.trait_ {
        Some((_, p, _)) => p,
        None => panic!("must be impl for something")
    };
    let name = clone.self_ty;
    (quote! {
        pub fn new() -> Box<dyn #ty> {
            Box::new( #name {
                ..Default::default()
            } )
        }
        #[typetag::serde]
        #ast
    }).into()
}