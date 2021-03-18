use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, format_ident};

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

pub(crate) fn stages(ast: syn::Expr, stage: &'static str) -> TokenStream2 {
    let stage_ident = format_ident!("{}", stage);
    let declare_ident = format_ident!("declare_{}", stage);
    match ast {
        syn::Expr::Assign(
            syn::ExprAssign {
                left,
                right,
                ..
            }
        ) => quote! {
            match (|| -> Result<_, ()> {Ok(#right)})() {
                Ok(v) if (#left).#stage_ident(NAME) => {
                    *#left = v;
                }
                _ => {}
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
            quote! {
                match (|| -> Result<_, ()> {Ok(#right)})() {
                    Ok(v) if (#left).#stage_ident(NAME) => {
                        (*#left).#func(v);
                    }
                    _ => {}
                }
            }
        },
        _ => quote! {
            #ast.#declare_ident(NAME)
        }
    }
}