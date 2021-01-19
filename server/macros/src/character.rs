use proc_macro::TokenStream;
use quote::quote;

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
                match &field.ty {
                    syn::Type::Path(
                        syn::TypePath {
                            path: syn::Path {
                                segments: seg,
                                ..
                            },
                            ..
                        }
                    ) => {
                        let ty = &field.ty;
                        let first = &seg.first().unwrap();
                        if &first.ident.to_string() == "Staged" {
                            let target = match &first.arguments {
                                syn::PathArguments::AngleBracketed(
                                    syn::AngleBracketedGenericArguments {
                                        args,
                                        ..
                                    }
                                ) => {
                                    match args.first().unwrap() {
                                        syn::GenericArgument::Type(
                                            syn::Type::Path(
                                                arg_ty @ syn::TypePath { .. }
                                            )
                                        ) => {
                                            arg_ty
                                        }
                                        _ => unimplemented!()
                                    }
                                }
                                _ => {
                                    unimplemented!()
                                }
                            };
                            count_unresolved_acc = quote! {
                                #count_unresolved_acc
                                + self.#id.count_unresolved()
                            };
                            final_character_acc = quote! {
                                #final_character_acc
                                #id: #target,
                            };
                            finalize_acc = quote! {
                                #finalize_acc
                                #id: self.#id.unwrap(),
                            };
                        } else {
                            final_character_acc = quote! {
                                #final_character_acc
                                #id: #ty,
                            };
                            finalize_acc = quote! {
                                #finalize_acc
                                #id: self.#id,
                            };
                        }

                    }
                    _ => unimplemented!()
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