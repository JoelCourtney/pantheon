use proc_macro::TokenStream;
use syn::export::TokenStream2;
use quote::{quote, format_ident};

pub(crate) fn choose(ast: syn::DeriveInput) -> TokenStream {
    let ident = ast.ident.clone();
    (match ast.data.clone() {
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            let vars = variants.iter().map( |var| {
                var.ident.to_string()
            }).collect();
            let process = process_choose_attribute(ident.to_string(), vars);
            quote! {
                #[derive(Debug, Serialize, Deserialize, Copy, Clone, Hash)]
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
    let choice_ident = format_ident!("{}Choice", name);
    let choice_array_ident = format_ident!("{}ArrayChoice", name);
    let mut acc = quote! {
        impl Default for #enum_ident {
            fn default() -> Self {
                #enum_ident::Unknown
            }
        }

        impl Choose for #enum_ident {
            fn choose<'a>(&'a mut self) -> Box<dyn Choice + 'a> {
                Box::new( #choice_ident { loc: self } )
            }
        }

        #[derive(Debug)]
        pub struct #choice_ident<'a> {
            loc: &'a mut #enum_ident
        }

        #[derive(Debug)]
        pub struct #choice_array_ident<'a> {
            locs: Vec<&'a mut #enum_ident>
        }
    };
    for i in 2..max_length+1 {
        let size: usize = i;
        acc = quote! {
            #acc
            impl Choose for [#enum_ident; #size] {
                fn choose<'a>(&'a mut self) -> Box<dyn Choice + 'a> {
                    Box::new( #choice_array_ident { locs: self.iter_mut().collect() } )
                }
            }
        };
    }
    let mut choices = "".to_string();
    for var in &vars {
        if var != "Unknown" {
            choices.extend(format!(r#""{}","#, var).chars());
        }
    }
    let choices_tokens: TokenStream2 = choices.parse().unwrap();
    let mut match_rules = "".to_string();
    for var in &vars {
        if var != "Unknown" {
            match_rules.extend(format!(r#""{}" => {}::{},"#, var, name, var).chars());
        }
    }
    let match_rules_tokens: TokenStream2 = match_rules.parse().unwrap();
    acc = quote! {
        #acc

        impl Choice for #choice_ident<'_> {
            fn choices(&self, index: usize) -> Vec<&'static str> {
                match index {
                    0 => vec![ #choices_tokens ],
                    _ => unimplemented!()
                }
            }
            fn choose(&mut self, choice: &str, index: usize) {
                match index {
                    0 => {
                        *self.loc = match choice {
                            #match_rules_tokens
                            _ => unimplemented!()
                        }
                    }
                    _ => unimplemented!()
                }
            }
        }
        impl Choice for #choice_array_ident<'_> {
            fn choices(&self, index: usize) -> Vec<&'static str> {
                if index < self.locs.len() {
                    vec! [ #choices_tokens ]
                } else {
                    unimplemented!()
                }
            }
            fn choose(&mut self, choice: &str, index: usize) {
                if index < self.locs.len() {
                    *self.locs[index] = match choice {
                        #match_rules_tokens
                        _ => unimplemented!()
                    }
                } else {
                    unimplemented!()
                }
            }
        }
    };
    acc
}

pub(crate) fn dynamic_choose(ast: syn::ItemTrait) -> TokenStream {
    let ident = ast.ident.clone();
    let lower_ident = format_ident!("{}", ident.to_string().to_lowercase());
    let get_all_ident = format_ident!("get_all_{}", lower_ident);
    let choice_ident = format_ident!("{}Choice", ident);
    let default_ident = format_ident!("default_{}", lower_ident);
    (quote! {
        #[typetag::serde]
        #ast
        impl Choose for Box<dyn #ident> {
            fn choose<'a>(&'a mut self) -> Box<dyn Choice + 'a> {
                Box::new( #choice_ident { locs: vec![ self ] } )
            }
        }

        #[derive(Debug)]
        struct #choice_ident<'a> {
            pub locs: Vec<&'a mut Box<dyn #ident>>
        }

        impl Choice for #choice_ident<'_> {
            fn choices(&self, index: usize) -> Vec<&'static str> {
                match index {
                    0 => content::#get_all_ident(),
                    _ => unimplemented!()
                }
            }
            fn choose(&mut self, choice: &str, index: usize) {
                **self.locs.get_mut(index).unwrap() = content::#lower_ident(choice).expect(
                    &format!("content not found: {}", choice)
                )
            }
        }

        impl Default for Box<dyn #ident> {
            fn default() -> Self {
                content::#default_ident()
            }
        }
    }).into()
}
