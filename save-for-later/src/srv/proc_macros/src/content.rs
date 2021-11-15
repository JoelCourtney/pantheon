use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, format_ident, ToTokens};
use syn::{Token, Fields, Field};
use syn::parse::Parser;
use std::sync::atomic::{AtomicU64, Ordering};
use syn::Type::Verbatim;
use std::iter::FromIterator;

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

pub(crate) fn stages(input: TokenStream, stage: &'static str) -> TokenStream2 {
    let ast = match syn::punctuated::Punctuated::<syn::Expr, Token![;]>::parse_terminated
        .parse(input.clone()) {
        Ok(ast) => ast.into_iter(),
        Err(..) => panic!("input to resolution macros must be expressions separated by semicolons.")
    };

    let mut tag: Option<u64> = None;

    let request_stage = format_ident!("request_{}", stage);
    let confirm_stage = format_ident!("confirm_{}", stage);
    let mut acc: TokenStream2 = quote! {};
    for seg in ast {
        let id = next_id();
        let (id_expr, make_hash) = match tag {
            Some(t) => {
                let hash_ident = format_ident!("dndcent_stage_hash_{}", id);
                let hasher_ident = format_ident!("dndcent_stage_hasher_{}", t);
                (
                    quote! {#hash_ident},
                    quote! {
                        #id.hash(&mut #hasher_ident);
                        let #hash_ident = #hasher_ident.finish();
                    }
                )
            }
            None => (quote! { #id }, quote! {})
        };
        match seg {
            syn::Expr::Assign(
                syn::ExprAssign {
                    left,
                    right,
                    ..
                }
            ) => {
                let expanded_right = expand_carriers(right.to_token_stream());
                acc = quote! {
                    #acc
                    #make_hash
                    if (#left).#request_stage(#id_expr) {
                        match (|| -> Result<_, ()> {Ok(#expanded_right)})() {
                            Ok(v) => {
                                *#left = v;
                                (#left).#confirm_stage(#id_expr);
                            }
                            _ => {}
                        }
                    }
                }
            },
            syn::Expr::AssignOp(
                syn::ExprAssignOp {
                    left,
                    right,
                    op,
                    ..
                }
            ) => {
                let func = format_ident!("{}", match op {
                    syn::BinOp::AddEq(..) => "add_assign",
                    syn::BinOp::SubEq(..) => "sub_assign",
                    syn::BinOp::ShlEq(..) => "push",
                    syn::BinOp::ShrEq(..) => "extend",
                    _ => ""
                });
                let expanded_right = expand_carriers(right.to_token_stream());
                acc = quote! {
                    #acc
                    #make_hash
                    if (#left).#request_stage(#id_expr) {
                        match (|| -> Result<_, ()> {Ok(#expanded_right)})() {
                            Ok(v) => {
                                use std::ops::{AddAssign, SubAssign};
                                (*#left).#func(v);
                                (#left).#confirm_stage(#id_expr);
                            }
                            _ => {}
                        }
                    }
                };
            },
            expr => {
                tag = Some(id);
                let hasher_ident = format_ident!("dndcent_stage_hasher_{}", id);
                acc = quote! {
                    #acc
                    let mut #hasher_ident = std::collections::hash_map::DefaultHasher::new();
                    (#expr).hash(&mut #hasher_ident);
                }
            }
        }
    }
    quote! {
        {
            use std::hash::{Hash, Hasher};
            #acc
        }
    }
}

static ID: AtomicU64 = AtomicU64::new(0);
pub fn next_id() -> u64 {
    ID.fetch_add(1, Ordering::SeqCst)
}

fn expand_carriers(stream: TokenStream2) -> TokenStream2 {
    use proc_macro2::TokenTree;

    let done: TokenStream2 = ". r#final ( ) ?".parse().unwrap();

    let res: TokenStream2 = stream.into_iter().flat_map(
        |token| {
            match token.clone() {
                TokenTree::Punct(p) => match p.as_char() {
                    '?' => done.clone(),
                    _ => p.to_token_stream()
                }
                TokenTree::Group(g) => {
                    let (open, close) = match g.delimiter() {
                        proc_macro2::Delimiter::Parenthesis => ('(', ')'),
                        proc_macro2::Delimiter::Brace => ('{', '}'),
                        proc_macro2::Delimiter::Bracket => ('[', ']'),
                        proc_macro2::Delimiter::None => panic!("how")
                    };
                    format!("{}{}{}",
                            open,
                            expand_carriers(g.stream()),
                            close
                    ).parse().expect("expand parse failed")
                },
                t => t.to_token_stream()
            }
        }
    ).collect();
    return res
}

pub fn asi_or_feat(level: u32) -> TokenStream {
    let asi_or_feat_ident = format_ident!("asi_or_feat_{}", level);
    let asi_choices_ident = format_ident!("asi_choices_{}", level);
    let feat_choice_ident = format_ident!("feat_choice_{}", level);
    (quote! {
        if level >= #level {
            i! {
                c.class_features[index] <<= Element::Choice {
                    text: "**ASI or Feat:** You can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1, or you can take a Feat. As normal, you can't increase an ability score above 20 using this feature.",
                    data: &mut self.#asi_or_feat_ident,
                    unique: false
                };
            }
            match self.#asi_or_feat_ident {
                ASIOrFeat::ASI => {
                    let mut i = 0;
                    for ability in &self.#asi_choices_ident {
                        match c.abilities.get_mut(*ability) {
                            Some(a) => m!{ i; *a += 1 },
                            None => {}
                        }
                        i += 1;
                    }
                    i! {
                        c.class_features[index] <<= Element::Choice {
                            text: "**Ability Score Increase:** Choose two ability scores to increase.",
                            data: &mut self.#asi_choices_ident,
                            unique: false
                        }
                    }
                }
                ASIOrFeat::Feat => {
                    self.#feat_choice_ident.resolve(c);
                    i! {
                        c.class_features[index] <<= Element::Choice {
                            text: "**Feat:** Choose a feat.",
                            data: &mut self.#feat_choice_ident,
                            unique: false
                        }
                    }
                }
                ASIOrFeat::Unknown => {}
            }
        }
    }).into()
}

pub fn asi_or_feat_fields(levels: Vec<u32>, mut ast: syn::ItemStruct) -> TokenStream {
    let new_fields: Vec<Field> = levels.into_iter().flat_map(
        |level| vec! [
            Field {
                attrs: vec![],
                vis: syn::Visibility::Inherited,
                ident: Some(format_ident!("asi_or_feat_{}", level)),
                // colon_token: Some(Token!(:)),
                colon_token: None,
                ty: Verbatim(quote! {ASIOrFeat})
            },
            Field {
                attrs: vec![],
                vis: syn::Visibility::Inherited,
                ident: Some(format_ident!("asi_choices_{}", level)),
                // colon_token: Some(Token!(:)),
                colon_token: None,
                ty: Verbatim(quote! {[Ability; 2]})
            },
            Field {
                attrs: vec![],
                vis: syn::Visibility::Inherited,
                ident: Some(format_ident!("feat_choice_{}", level)),
                // colon_token: Some(Token!(:)),
                colon_token: None,
                ty: Verbatim(quote! {Box<dyn Feat>})
            }
        ]
    ).collect();
    match &mut ast.fields {
        Fields::Named(fields) => {
            fields.named.extend(new_fields);
        }
        Fields::Unnamed(_) => panic!("asi_or_feats only supports named fields"),
        u@Fields::Unit => {
            *u = Fields::Named(syn::FieldsNamed {
                brace_token: Default::default(),
                named: syn::punctuated::Punctuated::<Field, syn::Token![,]>::from_iter(new_fields)
            })
        }
    }
    (quote! {
        #ast
    }).into()
}