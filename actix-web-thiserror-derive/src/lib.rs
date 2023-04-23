#![feature(proc_macro_quote)]

use proc_macro::TokenStream;

mod response_error;

#[proc_macro_derive(ResponseError, attributes(response))]
pub fn derive_response_error(input: TokenStream) -> TokenStream {
  response_error::derive_response_error(input)
}
