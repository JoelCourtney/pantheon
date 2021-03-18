use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, format_ident, ToTokens};
use syn::Token;
use syn::parse::Parser;

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
        Err(..) => match syn::punctuated::Punctuated::<syn::Expr, Token![,]>::parse_terminated
            .parse(input) {
            Ok(ast) => ast.into_iter(),
            Err(..) => panic!("must be separated with semicolons or commas")
        }
    };
    let stage_ident = format_ident!("{}", stage);
    let declare_ident = format_ident!("declare_{}", stage);
    let mut acc: TokenStream2 = quote! {};
    for seg in ast {
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
                    match (|| -> Result<_, ()> {Ok(#expanded_right)})() {
                        Ok(v) if (#left).#stage_ident(NAME) => {
                            *#left = v;
                        }
                        _ => {}
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
                    match (|| -> Result<_, ()> {Ok(#expanded_right)})() {
                        Ok(v) if (#left).#stage_ident(NAME) => {
                            use std::ops::{AddAssign, SubAssign};
                            (*#left).#func(v);
                        }
                        _ => {}
                    }
                };
            },
            _ => {
                acc = if acc.is_empty() {
                    quote! {
                        #seg.#declare_ident(NAME)
                    }
                } else {
                    quote! {
                        #acc; #seg.#declare_ident(NAME)
                    }
                }
            }
        }
    }
    acc
}

fn expand_carriers(stream: TokenStream2) -> TokenStream2 {
    use proc_macro2::TokenTree;

    let done: TokenStream2 = ". done ( ) ?".parse().unwrap();

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
                        _ => panic!("how")
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