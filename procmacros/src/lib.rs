// lets make a procmacro that enters a code from url and inserts it into the code

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};
use reqwest::blocking::get;

#[proc_macro]
pub fn fetch_code(input: TokenStream) -> TokenStream {
    let url = parse_macro_input!(input as LitStr).value();
    let response = get(&url).expect("Failed to fetch code from URL");
    let codestring: String = response.text().expect("Failed to read response text");
    let code: proc_macro2::TokenStream = codestring.parse().expect("Failed to parse code from response");


    let tokens = quote! {
        #code
    };

    tokens.into()
}
