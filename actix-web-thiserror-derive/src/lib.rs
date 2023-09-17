use proc_macro::TokenStream;

mod response_error;

#[proc_macro_derive(ResponseError, attributes(response))]
pub fn derive_response_error(input: TokenStream) -> TokenStream {
  response_error::derive_response_error(input)
}

#[proc_macro_derive(ResponseErrorTransform, attributes(response))]
pub fn derive_response_transform(input: TokenStream) -> TokenStream {
  response_error::derive_response_error_transform(input)
}