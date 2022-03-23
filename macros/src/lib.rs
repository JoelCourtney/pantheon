//! Procedural macros used by RPG system implementations.
//! 
//! This crate is re-exported through `pantheon::prelude::*`,
//! don't import it directly.

use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::{Group, TokenTree};

/// Expands instances of `x?` into `&*ops_macro_character.x.evaluate(ops_macro_character)?`.
/// 
/// *DO NOT USE DIRECTLY.* This is used automatically by the `ops` macro.
#[proc_macro]
pub fn expand_carriers(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let prefix: Vec<TokenTree> = "&*ops_macro_character."
        .parse::<TokenStream2>()
        .unwrap()
        .into_iter()
        .collect();
    let suffix: Vec<TokenTree> = ".evaluate(ops_macro_character)?"
        .parse::<TokenStream2>()
        .unwrap()
        .into_iter()
        .collect();

    let input: TokenStream2 = input.into();
    let stream: Vec<TokenTree> = input.into_iter().collect();
    let mut res: Vec<TokenTree> = Vec::new();

    let mut i = stream.len();
    while i > 0 {
        i -= 1;
        match &stream[i] {
            TokenTree::Punct(punct) => {
                if punct.to_string() == "?" && i != 0 {
                    match &stream[i - 1] {
                        id @ TokenTree::Ident(_) => {
                            let hold = res;
                            res = Vec::new();
                            res.extend_from_slice(&prefix);
                            res.push(id.clone());
                            res.extend_from_slice(&suffix);
                            res.extend_from_slice(&hold[..]);
                            i -= 1;
                        }
                        _ => res.insert(0, TokenTree::Punct(punct.clone())),
                    }
                } else {
                    res.insert(0, TokenTree::Punct(punct.clone()))
                }
            }
            TokenTree::Group(group) => {
                res.insert(
                    0,
                    TokenTree::Group(Group::new(
                        group.delimiter(),
                        expand_carriers(group.stream().into()).into(),
                    )),
                );
            }
            other => res.insert(0, other.clone()),
        }
    }
    res.into_iter().collect().into()
}
