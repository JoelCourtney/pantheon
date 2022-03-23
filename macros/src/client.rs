use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::{Group, TokenTree};

pub(crate) fn expand_carriers(stream: TokenStream2) -> TokenStream2 {
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

    let stream: Vec<TokenTree> = stream.into_iter().collect();
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
                        expand_carriers(group.stream()),
                    )),
                );
            }
            other => res.insert(0, other.clone()),
        }
    }
    res.into_iter().collect()
}
