#[cfg(all(feature = "server", feature = "client"))]
compile_error!("feature \"foo\" and feature \"bar\" cannot be enabled at the same time");

#[cfg(feature = "server")]
mod server;

#[cfg(feature = "server")]
#[proc_macro]
pub fn match_raw_files(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::ExprArray = syn::parse(input).expect("expected expr array");
    server::match_raw_files(ast)
}

#[cfg(feature = "client")]
mod client;

#[cfg(feature = "client")]
#[proc_macro]
pub fn expand_carriers(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    client::expand_carriers(input.into()).into()
}