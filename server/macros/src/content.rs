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

pub(crate) fn prelude(kind: &str, ast: syn::DeriveInput, pretty_name: String) -> TokenStream {
    // TODO("accept pretty name arg")
    let pascal_name_ident = ast.ident.clone();
    let kind_ident = format_ident!("{}", kind);
    let pretty_name_string = match pretty_name.as_str() {
        "" => pascal_name_ident.to_string(),
        _ => pretty_name
    };
    let feat_gen = match kind {
        "Race" | "Class" => quote! {
            pub const FEAT_GENERATOR: FeatGenerator = FeatGenerator::#kind_ident;
        },
        _ => quote !{}
    };
    (quote! {
        use crate::character::*;
        use crate::modify::*;
        use crate::feature::*;
        use crate::misc::*;
        use crate::describe::*;
        use macros::{def, describe, choose, traits, features, feats};
        use serde::{Serialize, Deserialize};
        use indoc::indoc;

        #[typetag::serde]
        impl #kind_ident for #pascal_name_ident {
            fn content_name(&self) -> &'static str {
                CONTENT_NAME
            }
        }

        pub fn new() -> Box<dyn #kind_ident> {
            Box::new( #pascal_name_ident {
                ..def!()
            } )
        }

        pub const CONTENT_NAME: &'static str = #pretty_name_string;

        #feat_gen

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

pub(crate) fn pretty_name(args: TokenStream) -> String {
    match syn::parse::<syn::LitStr>(args) {
        Ok(s) => s.value(),
        Err(_) => "".to_string()
    }
}

pub(crate) fn features(ast: syn::ExprArray) -> TokenStream {
    /* text, choice, unique */
    let mut entries: Vec<(syn::FieldValue, Option<syn::Expr>, bool)> = vec![];
    for elem in &ast.elems {
        match elem {
            syn::Expr::Struct(s) => {
                let mut text: Option<syn::FieldValue> = None;
                let mut choice: Option<syn::Expr> = None;
                let mut unique = false;
                for field in &s.fields {
                    match field.member {
                        syn::Member::Named(ref name) => {
                            match name.to_string().as_str() {
                                "text" => {
                                    text = Some(field.clone());
                                }
                                "choice" => {
                                    match choice {
                                        None => choice = Some(field.expr.clone()),
                                        Some(_) => unimplemented!()
                                    }
                                }
                                "unique_choice" => {
                                    match choice {
                                        None => choice = Some(field.expr.clone()),
                                        Some(_) => unimplemented!()
                                    }
                                    unique = true;
                                }
                                _ => unimplemented!()
                            }
                        }
                        _ => unimplemented!()
                    }
                }
                match text {
                    None => unimplemented!(),
                    Some(f) => entries.push((f, choice, unique))
                }
            }
            _ => unimplemented!()
        }
    }
    let mut serials_acc = quote! {};
    let mut choices_acc = quote! {};
    for (i,(text, choice, unique)) in entries.iter().enumerate() {
        match choice {
            None => {
                serials_acc = quote! {
                    #serials_acc
                    FeatureSerial {
                        #text,
                        choose: ChooseSerial {
                            current_choices: vec![],
                            all_choices: vec![ vec![] ]
                        }
                    },
                };
            },
            Some(c) => {
                serials_acc = quote! {
                    #serials_acc
                    FeatureSerial {
                        #text,
                        choose: #c.to_choose_serial(#unique)
                    },
                };
                choices_acc = quote! {
                    #choices_acc
                    #i => #c.choose(choice, choice_index),
                }
            }
        }
    }
    (quote! {
        fn receive_choice(&mut self, choice: &str, feature_index: usize, choice_index: usize) {
            match feature_index {
                #choices_acc
                _ => panic!(format!("feature at index {} does not have a choice associated with it", feature_index))
            }
        }

        fn write_features(&self) -> Vec<FeatureSerial> {
            vec! [ #serials_acc ]
        }
    }).into()
}

pub(crate) fn feats(ast: syn::ExprArray) -> TokenStream {
    let mut list_acc = quote! {};
    let mut receive_acc = quote! {};
    for (i,elem) in ast.elems.iter().enumerate() {
        list_acc = quote! {
            #list_acc
            #elem.write_features()
        };
        receive_acc = quote! {
            #receive_acc
            #i => #elem.receive_choice(choice, feature_index, choice_index),
        }
    }
    (quote! {
        fn receive_feat_choice(&mut self, choice: &str, feat_index: usize, feature_index: usize, choice_index: usize) {
            match feat_index {
                #receive_acc
                _ => panic!(format!("feat index out of bounds: {}", feat_index))
            }
        }

        fn write_feats(&self) -> Vec<Vec<FeatureSerial>> {
            vec![ #list_acc ]
        }
    }).into()
}