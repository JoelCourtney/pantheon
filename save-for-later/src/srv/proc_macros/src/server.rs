use proc_macro::TokenStream;
use quote::quote;
use walkdir::WalkDir;

pub(crate) fn match_raw_files(ast: syn::ExprArray) -> TokenStream {
    let strings: Vec<String> = ast.elems.iter()
        .map(|e| {
            match e {
                syn::Expr::Lit(
                    syn::ExprLit {
                        lit: syn::Lit::Str(
                            lit_str
                        ),
                        ..
                    }
                ) => lit_str.value(),
                _ => panic!("all args must be lit str")
            }
        })
        .collect();
    let mut match_rules_acc = quote! {};
    for string in strings {
        for entry in WalkDir::new(format!("src/www/build/{}",string)) {
            let entry = entry.expect("expected dir entry");
            let path = entry.path();
            if path.is_file() {
                let match_rule = path.strip_prefix("src/www/build/").unwrap().to_str().unwrap();
                let match_product = format!("../../www/build/{}", match_rule);
                match_rules_acc = quote! {
                    #match_rules_acc
                    #match_rule => &include_bytes!(#match_product)[..],
                }
            }
        }
    }
    (quote! {
        match path.to_str().unwrap() {
            #match_rules_acc
            _ => return None
        }
    }).into()
}