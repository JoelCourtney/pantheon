use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, format_ident};
use inflector::Inflector;

pub(crate) fn choose(ast: syn::DeriveInput) -> TokenStream {
    let ident = ast.ident.clone();
    (match ast.data.clone() {
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            let vars = variants.iter().map( |var| {
                var.ident.to_string()
            }).collect();
            let process = process_enum_choose(ident.to_string(), vars);
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

fn process_enum_choose(name: String, vars: Vec<String>) -> TokenStream2 {
    let enum_ident = format_ident!("{}", name);

    let mut choices = "".to_string();
    for var in &vars {
        if var != "Unknown" {
            choices.extend(format!(r#""{}","#, var).chars());
        }
    }
    let choices_tokens: TokenStream2 = choices.parse().expect("choices parse failed");
    let mut match_rules = "".to_string();
    let mut reverse_match_rules = "".to_string();
    for var in &vars {
        if var != "Unknown" {
            match_rules.extend(format!(r#""{}" => {}::{},{}"#, var, name, var, "\n").chars());
        }
        reverse_match_rules.extend(format!(r#"{}::{} => "{}",{}"#, name, var, var, "\n").chars());
    }
    let match_rules_tokens: TokenStream2 = match_rules.parse().expect("match rules parse failed");
    let reverse_match_rules_tokens: TokenStream2 = reverse_match_rules.parse().expect("reverse match rules failed");

    quote! {
        impl Choose for #enum_ident {
            fn choose(&mut self, choice: &str, index: usize) {
                if index == 0 {
                    *self = match choice {
                        #match_rules_tokens
                        _ => panic!(format!("choice not found: {}", choice))
                    }
                } else {
                    panic!(format!("choice index should be zero for single choice, was {}", index))
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

        impl<const N: usize> Choose for [#enum_ident; N] {
            fn choose(&mut self, choice: &str, index: usize) {
                if index < N {
                    self[index] = match choice {
                        #match_rules_tokens
                        _ => panic!(format!("choice not found: {}", choice))
                    }
                }
            }
            fn to_choice(&self, unique: bool) -> crate::feature::ChoiceSerial {
                let all_choices: Vec<&str> = vec![ #choices_tokens ];
                let current_choices: Vec<&str> = self.iter().map(|v| match v {
                    #reverse_match_rules_tokens
                    _ => panic!(format!("reverse match token not found: {:?}", v))
                }).collect();
                crate::feature::ChoiceSerial::from_vecs(current_choices, all_choices, unique)
            }
        }

        impl Choose for Vec<#enum_ident> {
            fn choose(&mut self, choice: &str, index: usize) {
                if index < self.len() {
                    self[index] = match choice {
                        #match_rules_tokens
                        _ => panic!(format!("choice not found: {}", choice))
                    }
                }
            }
            fn to_choice(&self, unique: bool) -> crate::feature::ChoiceSerial {
                let all_choices: Vec<&str> = vec![ #choices_tokens ];
                let current_choices: Vec<&str> = self.iter().map(|v| match v {
                    #reverse_match_rules_tokens
                    _ => panic!(format!("reverse match token not found: {:?}", v))
                }).collect();
                crate::feature::ChoiceSerial::from_vecs(current_choices, all_choices, unique)
            }
        }

        impl Default for #enum_ident {
            fn default() -> Self {
                #enum_ident::Unknown
            }
        }
    }
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
                    *self = crate::content::#lower_ident(choice).expect(&format!("choice not found: {:?}", choice));
                } else {
                    panic!(format!("index must be 0 for dynamic single choice, was {}", index))
                }
            }
            fn to_choice(&self, _unique: bool) -> crate::feature::ChoiceSerial {
                crate::feature::ChoiceSerial {
                    current_choices: vec! [ self.name() ],
                    all_choices: vec! [
                        crate::content::#get_all_ident()
                    ]
                }
            }
        }

        impl<const N: usize> Choose for [Box<dyn #ident>; N] {
            fn choose(&mut self, choice: &str, index: usize) {
                if index < N {
                    self[index] = crate::content::#lower_ident(choice).expect(&format!("choice not found: {}", choice));
                } else {
                    panic!(format!("index must be less than {}, was {}", N, index))
                }
            }
            fn to_choice(&self, unique: bool) -> crate::feature::ChoiceSerial {
                let current_choices: Vec<&str> = self.iter().map(|v| v.name()).collect();
                let all_choices: Vec<&str> = crate::content::#get_all_ident();
                crate::feature::ChoiceSerial::from_vecs(current_choices, all_choices, unique)
            }
        }

        impl Choose for Vec<Box<dyn #ident>> {
            fn choose(&mut self, choice: &str, index: usize) {
                if index < self.len() {
                    self[index] = crate::content::#lower_ident(choice).expect(&format!("choice not found: {}", choice));
                } else {
                    panic!(format!("index must be less than {}, was {}", self.len(), index))
                }
            }
            fn to_choice(&self, unique: bool) -> crate::feature::ChoiceSerial {
                let current_choices: Vec<&str> = self.iter().map(|v| v.name()).collect();
                let all_choices: Vec<&str> = crate::content::#get_all_ident();
                crate::feature::ChoiceSerial::from_vecs(current_choices, all_choices, unique)
            }
        }

        impl Default for Box<dyn #ident> {
            fn default() -> Self {
                crate::content::#default_ident()
            }
        }
    }).into()
}
