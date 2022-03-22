mod server;

#[proc_macro]
pub fn match_raw_files(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::ExprArray = syn::parse(input).expect("expected expr array");
    server::match_raw_files(ast)
}