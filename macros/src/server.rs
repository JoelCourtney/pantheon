use proc_macro::TokenStream;
use quote::{format_ident, quote};
use walkdir::WalkDir;

pub(crate) fn match_raw_files(ast: syn::ExprArray) -> TokenStream {
    let strings: Vec<String> = ast
        .elems
        .iter()
        .map(|e| match e {
            syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(lit_str),
                ..
            }) => lit_str.value(),
            _ => panic!("all args must be lit str"),
        })
        .collect();
    let prefix = &strings[0];
    let match_var = format_ident!("{}", &strings[1]);
    let dirs = &strings[2..];
    let mut match_rules_acc = quote! {};
    for dir in dirs {
        for entry in WalkDir::new(format!("{}", dir)) {
            let entry = entry.expect("expected dir entry");
            let path = entry.path();
            if path.is_file() {
                let match_rule = path.strip_prefix(dir).unwrap().to_str().unwrap();
                let match_product = format!("{}/{}/{}", prefix, dir, match_rule);
                match_rules_acc = quote! {
                    #match_rules_acc
                    #match_rule => &include_bytes!(#match_product)[..],
                }
            }
        }
    }
    (quote! {
        match #match_var.as_str() {
            #match_rules_acc
            s => panic!("file not found: {}", s)
        }
    })
    .into()
}
