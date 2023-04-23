#![doc = include_str!("../../README.md")]

use std::sync::Arc;

use actix_web::HttpResponse;
use arc_swap::ArcSwap;
use lazy_static::lazy_static;

/// A trait that transforms information about an [thiserror] error into
/// a response as desired by the implementor.
///
/// [thiserror]: https://docs.rs/thiserror
#[allow(unused)]
pub trait ResponseTransform {
  fn transform(
    &self,
    name: &str,
    err: &dyn std::error::Error,
    status_code: http::StatusCode,
    reason: Option<serde_json::Value>,
  ) -> HttpResponse {
    actix_web::HttpResponse::build(status_code).finish()
  }
}

struct ReflexiveTransform;

impl ResponseTransform for ReflexiveTransform {}

lazy_static! {
  pub(crate) static ref RESPONSE_TRANSFORM: ArcSwap<Box<dyn ResponseTransform + Sync + Send>> =
    ArcSwap::from(Arc::new(Box::new(ReflexiveTransform {}) as _));
}

/// Sets the default global transform for errors into responses.
pub fn set_global_transform(transform: impl ResponseTransform + Sync + Send + 'static) {
  RESPONSE_TRANSFORM.swap(Arc::new(Box::new(transform)));
}

#[doc(hidden)]
pub trait ThiserrorResponse {
  fn status_code(&self) -> Option<http::StatusCode> {
    None
  }

  fn reason(&self) -> Option<Option<serde_json::Value>> {
    None
  }
}

#[allow(unused_imports)]
#[macro_use]
extern crate actix_web_thiserror_derive;

/// The derive implementation for extending [thiserror]
///
/// [thiserror]: https://docs.rs/thiserror
pub use actix_web_thiserror_derive::ResponseError;
