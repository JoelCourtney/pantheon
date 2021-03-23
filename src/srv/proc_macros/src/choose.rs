use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, format_ident};
use inflector::Inflector;

pub fn choose(ast: syn::ItemEnum) -> TokenStream {
    let ident = ast.ident.clone();
    let name = ident.to_string();

    let vars: Vec<(String, String)> = ast.clone().variants.iter().map(
        |var| {
            match &var.discriminant {
                Some(
                    (_,
                        syn::Expr::Lit(
                            syn::ExprLit {
                                lit: syn::Lit::Str(s), ..
                            }
                        )
                    )
                ) => (var.ident.to_string(), s.value()),
                _ => (var.ident.to_string(), var.ident.to_string())
            }
        }
    ).collect();

    let mut stripped_ast = ast.clone();
    for var in &mut stripped_ast.variants {
        var.discriminant = None;
    }

    let enum_ident = format_ident!("{}", name);

    let mut choices = "".to_string();
    let mut match_rules = "".to_string();
    let mut reverse_match_rules = "".to_string();
    let mut map_fields = "".to_string();
    let mut map_unwrap_fields = "".to_string();
    let mut map_get_rules = "".to_string();
    let mut map_get_mut_rules = "".to_string();
    let mut map_count = "".to_string();
    let mut serialize_rules = "".to_string();
    let mut deserialize_rules = "".to_string();
    for (i, (var, pretty)) in vars.iter().enumerate() {
        let snake_var = var.to_snake_case();
        if var != "Unknown" {
            choices.extend(format!(r#""{}","#, pretty).chars());
            match_rules.extend(format!(r#""{}" => {}::{},{}"#, pretty, name, var, "\n").chars());
            map_fields.extend(format!("pub {}: T,\n", snake_var).chars());
            map_unwrap_fields.extend(format!("{}: self.{}.unwrap(),\n", snake_var, snake_var).chars());
            map_get_rules.extend(format!("{}::{} => Some(&self.{}),\n", name, var, snake_var).chars());
            map_get_mut_rules.extend(format!("{}::{} => Some(&mut self.{}),\n", name, var, snake_var).chars());
            map_count.extend(format!("self.{}.count_unresolved() +\n", snake_var).chars());
        }
        reverse_match_rules.extend(format!(r#"{}::{} => "{}",{}"#, name, var, pretty, "\n").chars());
        serialize_rules.extend(format!("{}::{} => serializer.serialize_unit_variant(\"{}\", {}, \"{}\"),\n", name, var, name, i, pretty).chars());
        deserialize_rules.extend(format!("\"{}\" => Ok({}::{}),\n", pretty, name, var).chars());
    }
    let choices_tokens: TokenStream2 = choices.parse().expect("choices parse failed");
    let match_rules_tokens: TokenStream2 = match_rules.parse().expect("match rules parse failed");
    let reverse_match_rules_tokens: TokenStream2 = reverse_match_rules.parse().expect("reverse match rules failed");
    let map_fields_tokens: TokenStream2 = map_fields.parse().expect("map fields parse failed");
    let map_unwrap_fields_tokens: TokenStream2 = map_unwrap_fields.parse().expect("map unwrap fields parse failed");
    let map_get_rules_tokens: TokenStream2 = map_get_rules.parse().expect("map get rules parse failed");
    let map_get_mut_rules_tokens: TokenStream2 = map_get_mut_rules.parse().expect("map get mut rules parse failed");
    let map_count_tokens: TokenStream2 = map_count.parse().expect("map count parse failed");
    let serialize_rules_tokens: TokenStream2 = serialize_rules.parse().expect("serialize rules parse failed");
    let deserialize_rules_tokens: TokenStream2 = deserialize_rules.parse().expect("deserialize rules parse failed");

    let map_ident = format_ident!("{}Map", name);
    let visitor_ident = format_ident!("{}DeserializeVisitor", name);
    let error_string = format!("expected a pretty-print variant of enum {}", name);

    (quote! {
        #[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, enum_iterator::IntoEnumIterator)]
        #stripped_ast

        impl #enum_ident {
            pub fn name(&self) -> &'static str {
                match self {
                    #reverse_match_rules_tokens
                }
            }
            pub fn known(&self) -> bool {
                *self != #enum_ident::Unknown
            }
        }

        impl Choose for #enum_ident {
            fn choose(&mut self, choice: &str, index: usize) {
                if index == 0 {
                    *self = match choice {
                        #match_rules_tokens
                        _ => panic!("choice not found: {}", choice)
                    }
                } else {
                    panic!("choice index should be zero for single choice, was {}", index)
                }
            }
            fn to_choice(&self, _unique: bool) -> crate::feature::ChoiceSerial {
                crate::feature::ChoiceSerial {
                    current_choices: vec! [ self.name() ],
                    all_choices: vec![ vec![ #choices_tokens ] ]
                }
            }
        }

        impl<const N: usize> Choose for [#enum_ident; N] {
            fn choose(&mut self, choice: &str, index: usize) {
                if index < N {
                    self[index] = match choice {
                        #match_rules_tokens
                        _ => panic!("choice not found: {}", choice)
                    }
                }
            }
            fn to_choice(&self, unique: bool) -> crate::feature::ChoiceSerial {
                let all_choices: Vec<&str> = vec![ #choices_tokens ];
                let current_choices: Vec<&str> = self.iter().map(|v| v.name()).collect();
                crate::feature::ChoiceSerial::from_vecs(current_choices, all_choices, unique)
            }
        }

        impl Choose for Vec<#enum_ident> {
            fn choose(&mut self, choice: &str, index: usize) {
                if index < self.len() {
                    self[index] = match choice {
                        #match_rules_tokens
                        _ => panic!("choice not found: {}", choice)
                    }
                }
            }
            fn to_choice(&self, unique: bool) -> crate::feature::ChoiceSerial {
                let all_choices: Vec<&str> = vec![ #choices_tokens ];
                let current_choices: Vec<&str> = self.iter().map(|v| v.name()).collect();
                crate::feature::ChoiceSerial::from_vecs(current_choices, all_choices, unique)
            }
        }

        impl Default for #enum_ident {
            fn default() -> Self {
                #enum_ident::Unknown
            }
        }

        #[derive(Debug, Serialize, Deserialize, Default, Clone)]
        pub struct #map_ident<T>
            where T: std::fmt::Debug + serde::Serialize + Default {
            #map_fields_tokens
        }

        impl<T> #map_ident<T>
            where T: std::fmt::Debug + serde::Serialize + Default {
            pub fn get(&self, var: #enum_ident) -> Option<&T> {
                match var {
                    #map_get_rules_tokens
                    #enum_ident::Unknown => None
                }
            }

            pub fn get_known(&self, var: #enum_ident) -> &T {
                self.get(var).unwrap()
            }

            pub fn get_mut(&mut self, var: #enum_ident) -> Option<&mut T> {
                match var {
                    #map_get_mut_rules_tokens
                    #enum_ident::Unknown => None
                }
            }

            pub fn get_mut_known(&mut self, var: #enum_ident) -> &mut T {
                self.get_mut(var).unwrap()
            }
        }

        impl<T> #map_ident<crate::character::Staged<T>>
            where T: std::fmt::Debug + serde::Serialize + Default {
            pub fn unwrap(self) -> #map_ident<T> {
                #map_ident {
                    #map_unwrap_fields_tokens
                }
            }

            pub fn count_unresolved(&self) -> u32 {
                #map_count_tokens 0
            }
        }

        impl Serialize for #enum_ident {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where S: serde::Serializer {
                match self {
                    #serialize_rules_tokens
                }
            }
        }

        struct #visitor_ident;

        impl<'de> serde::de::Visitor<'de> for #visitor_ident {
            type Value = #enum_ident;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(#error_string)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where E: serde::de::Error {
                match value {
                    #deserialize_rules_tokens
                    s => panic!("cannot deserialize: {}", s)
                }
            }
        }

        impl<'de> Deserialize<'de> for #enum_ident {
            fn deserialize<D>(deserializer: D) -> Result<#enum_ident, D::Error>
            where D: serde::Deserializer<'de> {
                deserializer.deserialize_str(#visitor_ident)
            }
        }
    }).into()
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
                    panic!("index must be 0 for dynamic single choice, was {}", index)
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
                    panic!("index must be less than {}, was {}", N, index)
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
                    panic!("index must be less than {}, was {}", self.len(), index)
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
