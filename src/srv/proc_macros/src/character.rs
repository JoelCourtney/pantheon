use proc_macro::TokenStream;
use quote::quote;
use syn::{PathSegment, TypePath};

pub(crate) fn finalize(ast: syn::DeriveInput) -> TokenStream {
    if &ast.ident.to_string() != "Character" {
        return quote! {
            compile_error!("must be applied to the Character struct only");
        }.into()
    }

    match ast.data {
        syn::Data::Struct(
            syn::DataStruct{
                fields: syn::Fields::Named(syn::FieldsNamed {
                    named: fields,
                    ..
                }),
                ..
            }
        ) => {
            let mut count_unresolved_acc = quote! {0};
            let mut final_character_acc = quote! {};
            let mut finalize_acc = quote! {};

            for field in &fields {
                let id = field.ident.clone().expect("expected named field");
                let ty = &field.ty;
                match ty {
                    syn::Type::Path(
                        syn::TypePath {
                            path: syn::Path {
                                segments: seg,
                                ..
                            },
                            ..
                        }
                    ) => {
                        let first = &seg.first()
                            .expect("path has no segments?");
                        if &first.ident.to_string() == "Staged" {
                            let target = extract_type_argument(first);
                            count_unresolved_acc = quote! {
                                #count_unresolved_acc
                                + self.#id.count_unresolved()
                            };
                            final_character_acc = quote! {
                                #final_character_acc
                                pub #id: #target,
                            };
                            finalize_acc = quote! {
                                #finalize_acc
                                #id: self.#id.unwrap(),
                            };
                        } else if extract_type_argument(first).path.segments.first().unwrap().ident.to_string() == "Staged" {
                            let target = extract_type_argument(extract_type_argument(first).path.segments.first().unwrap());
                            let map_ident = &first.ident;
                            count_unresolved_acc = quote! {
                                #count_unresolved_acc
                                + self.#id.count_unresolved()
                            };
                            final_character_acc = quote! {
                                #final_character_acc
                                pub #id: #map_ident<#target>,
                            };
                            finalize_acc = quote! {
                                #finalize_acc
                                #id: self.#id.unwrap(),
                            };
                        } else {
                            final_character_acc = quote! {
                                #final_character_acc
                                pub #id: #ty,
                            };
                            finalize_acc = quote! {
                                #finalize_acc
                                #id: self.#id,
                            };
                        }
                    }
                    _ => {
                        final_character_acc = quote! {
                            #final_character_acc
                            pub #id: #ty,
                        };
                        finalize_acc = quote! {
                            #finalize_acc
                            #id: self.#id,
                        };
                    }
                }
            }
            (quote! {
                impl Character {
                    fn count_unresolved(&self) -> u32 {
                        #count_unresolved_acc
                    }
                    pub fn finalize(self) -> FinalCharacter {
                        FinalCharacter {
                            #finalize_acc
                        }
                    }
                }

                #[derive(Serialize, Debug, Default)]
                pub struct FinalCharacter {
                    #final_character_acc
                }
            }).into()
        }
        _ => (quote! {
            compile_error!("must be applied to the Character struct only");
        }).into()
    }
}

fn extract_type_argument(first: &PathSegment) -> &TypePath {
    match &first.arguments {
        syn::PathArguments::AngleBracketed(
            syn::AngleBracketedGenericArguments {
                args,
                ..
            }
        ) => {
            match args.first().expect("path arg has no segments?") {
                syn::GenericArgument::Type(
                    syn::Type::Path(
                        arg_ty
                    )
                ) => arg_ty,
                _ => panic!("expected generic argument")
            }
        }
        _ => {
            panic!("expected angle bracketed type args")
        }
    }
}