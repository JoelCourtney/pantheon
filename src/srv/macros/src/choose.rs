use proc_macro::TokenStream;
use syn::export::TokenStream2;
use quote::{quote, format_ident};
use inflector::Inflector;

pub(crate) fn choose(ast: syn::DeriveInput) -> TokenStream {
    let ident = ast.ident.clone();
    (match ast.data.clone() {
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            let vars = variants.iter().map( |var| {
                var.ident.to_string()
            }).collect();
            let process = process_choose_attribute(ident.to_string(), vars);
            quote! {
                #[derive(Debug, Serialize, Deserialize, Copy, Clone, Hash, Eq, PartialEq)]
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
    let max_length = 2;
    let enum_ident = format_ident!("{}", name);

    let mut choices = "".to_string();
    for var in &vars {
        if var != "Unknown" {
            choices.extend(format!(r#""{}","#, var).chars());
        }
    }
    let choices_tokens: TokenStream2 = choices.parse().unwrap();
    let mut match_rules = "".to_string();
    let mut reverse_match_rules = "".to_string();
    for var in &vars {
        if var != "Unknown" {
            match_rules.extend(format!(r#""{}" => {}::{},{}"#, var, name, var, "\n").chars());
        }
        reverse_match_rules.extend(format!(r#"{}::{} => "{}",{}"#, name, var, var, "\n").chars());
    }
    let match_rules_tokens: TokenStream2 = match_rules.parse().unwrap();
    let reverse_match_rules_tokens: TokenStream2 = reverse_match_rules.parse().unwrap();

    let mut acc = quote! {
        impl Default for #enum_ident {
            fn default() -> Self {
                #enum_ident::Unknown
            }
        }

        impl Choose for #enum_ident {
            fn choose(&mut self, choice: &str, index: usize) {
                if index == 0 {
                    *self = match choice {
                        #match_rules_tokens
                        _ => panic!(format!("choice not found: {}", choice))
                    }
                } else {
                    unimplemented!()
                }
            }
            fn to_choice(&self, _unique: bool) -> crate::feature::ChoiceSerial {
                crate::feature::ChoiceSerial {
                    current_choices: vec! [match &self {
                        #reverse_match_rules_tokens
                    }],
                    all_choices: vec![ vec![ #choices_tokens ] ]
                }
            }
        }
    };
    for i in 2..max_length+1 {
        let size: usize = i;
        acc = quote! {
            #acc
            impl Choose for [#enum_ident; #size] {
                fn choose(&mut self, choice: &str, index: usize) {
                    if index < self.len() {
                        self[index] = match choice {
                            #match_rules_tokens
                            _ => panic!(format!("choice not found: {}", choice))
                        }
                    }
                }
                fn to_choice(&self, unique: bool) -> crate::feature::ChoiceSerial {
                    crate::feature::ChoiceSerial {
                        current_choices: self.iter().map(|v| match v {
                            #reverse_match_rules_tokens
                            _ => unimplemented!()
                        }).collect(),
                        all_choices: {
                            (0..#size).map(
                                |index| {
                                    if !unique {
                                        vec! [ #choices_tokens ]
                                    } else {
                                        let current_choices: Vec<&'static str> = self.iter().filter_map(
                                            |v| {
                                                if *v != self[index] {
                                                    Some(match *v {
                                                        #reverse_match_rules_tokens
                                                        _ => unimplemented!()
                                                    })
                                                } else {
                                                    None
                                                }
                                            }
                                        ).collect();
                                        vec! [ #choices_tokens ].iter().filter_map(
                                            |v| if current_choices.iter().all(|w| v != w) {
                                                Some(*v)
                                            } else {
                                                None
                                            }
                                        ).collect()
                                    }
                                }
                            ).collect()
                        }
                    }
                }
            }
        };
    }
    acc
}

pub(crate) fn dynamic_choose(ast: syn::ItemTrait) -> TokenStream {
    let ident = ast.ident.clone();
    let lower_ident = format_ident!("{}", ident.to_string().to_snake_case());
    let get_all_ident = format_ident!("get_all_{}", lower_ident);
    let default_ident = format_ident!("default_{}", lower_ident);
    (quote! {
        #[typetag::serde]
        #ast
        impl Choose for Box<dyn #ident> {
            fn choose(&mut self, choice: &str, index: usize) {
                if index == 0 {
                    *self = crate::content::#lower_ident(choice).expect(&format!("choice not found: {}", choice));
                } else {
                    unimplemented!()
                }
            }
            fn to_choice(&self, unique: bool) -> crate::feature::ChoiceSerial {
                if !unique {
                    crate::feature::ChoiceSerial {
                        current_choices: vec! [ self.name() ],
                        all_choices: vec! [
                            crate::content::#get_all_ident()
                        ]
                    }
                } else {
                    unimplemented!()
                }
            }
        }

        impl Default for Box<dyn #ident> {
            fn default() -> Self {
                crate::content::#default_ident()
            }
        }
    }).into()
}
