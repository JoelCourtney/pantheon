mod client;

#[proc_macro]
pub fn expand_carriers(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    client::expand_carriers(input.into()).into()
}