use std::collections::{HashMap, HashSet};
use std::iter::Peekable;

use proc_macro::TokenStream;
use proc_macro2::{token_stream::IntoIter, TokenTree};
use quote::quote;
use syn::DeriveInput;

pub fn derive_response_error(input: TokenStream) -> TokenStream {
  let ast = syn::parse_macro_input!(input as DeriveInput);

  let name = ast.ident;
  let name_str = name.to_string();

  let variants = if let syn::Data::Enum(syn::DataEnum { variants, .. }) = ast.data {
    variants
  } else {
    unimplemented!();
  };

  let (forwards, _internals, mut status_map, mut reason_map): (
    HashSet<proc_macro2::Ident>,
    HashSet<proc_macro2::Ident>,
    HashMap<proc_macro2::Ident, proc_macro2::TokenStream>,
    HashMap<proc_macro2::Ident, proc_macro2::TokenStream>,
  ) = variants.iter().fold(
    (
      HashSet::new(),
      HashSet::new(),
      HashMap::new(),
      HashMap::new(),
    ),
    |(mut forwards, mut internals, mut status_map, mut reason_map), variant| {
      let variant_ident = variant.ident.to_owned();

      for attr in &variant.attrs {
        if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "response" {
          let tokens = attr.tokens.to_owned();

          for token in tokens.into_iter() {
            if let TokenTree::Group(group) = token {
              let mut tokens = group.stream().into_iter().peekable();

              while tokens.peek().is_some() {
                if let Some(TokenTree::Ident(ident)) = tokens.next() {
                  let ident = ident.to_string();

                  match &ident as &str {
                    "internal" | "forward" => match &ident as &str {
                      "internal" => {
                        internals.insert(variant_ident.to_owned());
                      }

                      "forward" => {
                        forwards.insert(variant_ident.to_owned());
                      }

                      _ => {
                        panic!("Unknown #[response] option: {}", &ident);
                      }
                    },

                    _ => {
                      if tokens
                        .next()
                        .map(|punct| punct.to_string())
                        .filter(|punct| &punct as &str == "=")
                        .is_none()
                      {
                        panic!("Invalid #[response] options");
                      }

                      match &ident as &str {
                        "status" => {
                          let status_code = get_status_code(&mut tokens);

                          match status_code {
                            Some(status_code) => {
                              status_map.insert(variant_ident.to_owned(), status_code);
                            }
                            _ => panic!("Invalid `status` in #[response]"),
                          }
                        }

                        "reason" => {
                          let reason = get_reason(&mut tokens);

                          match reason {
                            Some(reason) => {
                              reason_map.insert(variant_ident.to_owned(), reason);
                            }
                            _ => panic!("Invalid `reason` in #[response]"),
                          }
                        }

                        _ => {
                          panic!("Unknown #[response] option: {}", &ident);
                        }
                      }
                    }
                  }
                }

                match tokens.next().and_then(|token| match token {
                  TokenTree::Punct(punct) => Some(punct.as_char()),
                  _ => None,
                }) {
                  Some(',') | None => {}
                  Some(token) => panic!("Invalid #[response] options: at token {:?}", &token),
                }
              }
            }
          }
        }
      }

      (forwards, internals, status_map, reason_map)
    },
  );

  let status_code_match = match status_map.len() {
    0 => None,
    _ => {
      let mut body: Vec<proc_macro2::TokenStream> = status_map
        .drain()
        .map(|(ident, status_code)| {
          quote! {
            #name::#ident { .. } => Some(#status_code),
          }
        })
        .collect();

      Some(proc_macro2::TokenStream::from_iter(body.drain(..)))
    }
  };

  let reason_match = match reason_map.len() {
    0 => None,
    _ => {
      let mut body: Vec<proc_macro2::TokenStream> = reason_map
        .drain()
        .map(|(ident, reason)| {
          quote! {
            #name::#ident { .. } => Some(serde_json::to_value(#reason).ok()),
          }
        })
        .collect();

      Some(proc_macro2::TokenStream::from_iter(body.drain(..)))
    }
  };

  let (status_code_forwards, reason_forwards) = match forwards.len() {
    0 => (None, None),
    _ => {
      let mut streams = vec![quote! { status_code }, quote! { reason }]
        .drain(..)
        .map(|func| {
          proc_macro2::TokenStream::from_iter(forwards.iter().map(|variant| {
            quote! {
              #name::#variant(inner) => ::actix_web_thiserror::ThiserrorResponse::#func(inner),
            }
          }))
        })
        .collect::<Vec<_>>();

      (Some(streams.remove(0)), Some(streams.remove(0)))
    }
  };

  let expanded = quote! {
    impl ::actix_web_thiserror::ThiserrorResponse for #name {
      fn status_code(&self) -> Option<http::StatusCode> {
        match self {
          #status_code_match
          _ => None,
        }
      }

      fn reason(&self) -> Option<Option<serde_json::Value>> {
        match self {
          #reason_match
          _ => None,
        }
      }
    }

    impl actix_web::error::ResponseError for #name {
      fn status_code(&self) -> http::StatusCode {
        match ::actix_web_thiserror::ThiserrorResponse::status_code(self) {
          Some(status_code) => status_code,
          _ => {
            match self {
              #status_code_forwards
              _ => None
            }
              .unwrap_or(http::StatusCode::OK)
          },
        }
      }

      fn error_response(&self) -> actix_web::HttpResponse {
        let reason: Option<serde_json::Value> = ::actix_web_thiserror::ThiserrorResponse::reason(self)
          .unwrap_or(match self {
            #reason_forwards
            _ => None,
          }
            .and_then(|value| value));

        error!("Response error: {err}\n\t{name}({err:?})", name = #name_str, err = &self);

        actix_web_thiserror::apply_global_transform(
          #name_str,
          &self,
          self.status_code(),
          reason,
        )
      }
    }
  };

  TokenStream::from(expanded)
}

fn get_ident_stream(tokens: &mut Peekable<IntoIter>) -> Option<proc_macro2::TokenStream> {
  match tokens.next() {
    Some(TokenTree::Ident(value)) => {
      let mut ident_fragments = vec![TokenTree::Ident(value)];

      while tokens.peek().and_then(|token| match token {
        TokenTree::Punct(punct) => Some(punct.as_char()),
        _ => None,
      }) == Some(':')
      {
        ident_fragments.push(tokens.next().expect("should exist"));

        if !match tokens.next() {
          Some(TokenTree::Punct(punct)) => {
            if punct.as_char() == ':' {
              ident_fragments.push(TokenTree::Punct(punct));

              true
            } else {
              false
            }
          }
          _ => false,
        } {
          panic!("ident invalid")
        }

        if !match tokens.next() {
          Some(TokenTree::Ident(ident)) => {
            ident_fragments.push(TokenTree::Ident(ident));

            true
          }
          _ => false,
        } {
          panic!("ident invalid")
        }
      }

      Some(proc_macro2::TokenStream::from_iter(
        ident_fragments.drain(..),
      ))
    }

    _ => None,
  }
}

fn get_status_code_literal(tokens: &mut Peekable<IntoIter>) -> Option<proc_macro2::TokenTree> {
  match tokens.next() {
    Some(TokenTree::Literal(value)) => {
      if value
        .to_string()
        .parse::<u16>()
        .ok()
        .and_then(|status| http::StatusCode::from_u16(status).ok())
        .is_none()
      {
        panic!("invalid status code");
      }

      Some(TokenTree::Literal(value))
    }

    _ => None,
  }
}

fn get_string(tokens: &mut Peekable<IntoIter>) -> Option<proc_macro2::TokenTree> {
  match tokens.next() {
    Some(TokenTree::Literal(value)) => {
      let litty = syn::Lit::new(value.to_owned());

      match litty {
        syn::Lit::Str(_) => Some(TokenTree::Literal(value)),
        _ => None,
      }
    }

    _ => None,
  }
}

fn get_status_code(tokens: &mut Peekable<IntoIter>) -> Option<proc_macro2::TokenStream> {
  match tokens.peek() {
    Some(TokenTree::Ident(_)) => get_ident_stream(tokens).map(|tokens| tokens),

    Some(TokenTree::Literal(_)) => get_status_code_literal(tokens).map(|tokens| {
      quote! {
        http::StatusCode::from_u16(#tokens as u16)
          .unwrap_or_else(|_| unreachable!())
      }
    }),

    _ => None,
  }
}

fn get_reason(tokens: &mut Peekable<IntoIter>) -> Option<proc_macro2::TokenStream> {
  match tokens.peek() {
    Some(TokenTree::Ident(_)) => get_ident_stream(tokens).map(|tokens| tokens),

    Some(TokenTree::Literal(_)) => get_string(tokens).map(|tokens| tokens.into()),

    _ => None,
  }
}
