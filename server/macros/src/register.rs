use proc_macro::TokenStream;
use syn::export::TokenStream2;
use quote::{quote, format_ident};

pub(crate) fn register(ast: syn::Expr) -> TokenStream {
    use syn::Expr;

    let mut collection_name: String = "".to_string();
    let gen = match ast {
        Expr::Tuple(t) => {
            match unwrap_string_tuple(t) {
                Ok((dir_str, name)) => {
                    let dir_str = format!("./src/content{}", dir_str.as_str());
                    collection_name = name.to_owned();
                    list_imports(dir_str)
                }
                Err(e) => e
            }
        }
        Expr::Lit(syn::ExprLit{lit: syn::Lit::Str(lit),..}) => {
            let dir_str = format!("./src/content{}",lit.value());
            collection_name = dir_str[dir_str.rfind('/').expect("dir must contain /")+1..].to_owned();
            list_imports(dir_str)
        }
        _ => unimplemented!()
    };
    (quote! {
        #gen
        pub const COLLECTION_NAME: &'static str = #collection_name;
    }).into()
}

fn list_imports(dir_str: String) -> TokenStream2 {
    let paths = match std::fs::read_dir(dir_str) {
        Ok(p) => p,
        Err(_e) => {
            return quote! {
                compile_error!("args must be path to a directory");
                compile_error!(#{e.to_string()});
            }
        }
    };
    let mut acc = quote! {};
    for path in paths {
        let path = path.unwrap().path();
        if !path.ends_with("mod.rs") {
            let endpoint = path.file_name().unwrap().to_str().unwrap();
            if endpoint.ends_with(".rs") {
                let endpoint = format_ident!("{}", &endpoint[..endpoint.len()-3]);
                acc = quote! {
                    #acc
                    pub mod #endpoint;
                };
            } else {
                let endpoint = format_ident!("{}", endpoint);
                acc = quote! {
                    #acc
                    pub mod #endpoint;
                };
            }
        }
    }
    acc
}

pub(crate) fn unwrap_string_tuple(t: syn::ExprTuple) -> Result<(String, String), TokenStream2> {
    use syn::Expr;

    let size = t.elems.len();
    if size != 2 {
        Err(quote! {
            compile_error!("expected str literal or tuple (str, str)");
        })
    } else {
        let first = t.elems.first().unwrap();
        let last = t.elems.last().unwrap();
        match (first, last) {
            (
                Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit1), .. }),
                Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit2), .. })
            ) => Ok((lit1.value(), lit2.value())),
            _ => {
                Err(quote!({
                    compile_error!("both arguments must be str");
                }))
            }
        }
    }
}