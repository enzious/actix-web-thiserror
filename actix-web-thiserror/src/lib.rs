//! A crate that extends the [thiserror] crate functionality to automatically
//! return a proper [actix-web] response.
//!
//! ## Error definition
//! ```rust
//! #[derive(Debug, Error, ResponseError)]
//! pub enum Base64ImageError {
//!   #[response(reason = "INVALID_BASE64")]
//!   #[error("base64 invalid")]
//!   Base64Invalid(#[from] base64::DecodeError),
//!   #[response(reason = "INVALID_MIME")]
//!   #[error("mime invalid")]
//!   MimeInvalid(#[from] FromStrError),
//!   #[error("invalid image")]
//!   #[response(reason = "INVALID_IMAGE")]
//!   InvalidImage(#[from] image::ImageError),
//!   #[response(reason = "INVALID_IMAGE_FORMAT")]
//!   #[error("invalid image format")]
//!   InvalidImageFormat,
//!   #[response(reason = "INVALID_STRING")]
//!   #[error("invalid string")]
//!   InvalidString,
//! }
//! ```
//!
//! ## Error implementation
//! ```rust
//! use actix_web::*;
//! use fuzion::utils::image::Base64ImageError;
//!
//! pub async fn error_test() -> Result<HttpResponse, Error> {
//!   Err(Base64ImageError::InvalidImageFormat)?
//! }
//! ```
//!
//! ## Error response
//!
//! The reason is a string that maybe be given to the client in some
//! form to explain the error if appropriate. Here it is an enum that can be
//! localized.
//!
//! **Note:** This response has been formatted by a [`ResponseTransform`][response_transform].
//!
//! ```
//! {
//!     "result": 0,
//!     "reason": "INVALID_IMAGE_FORMAT"
//! }
//! ```
//!
//! ## Error logging
//!
//! In the logs, you can show the error text automatically when the error is
//! returned out through a http response.
//!
//! ```
//! Apr 23 02:19:35.211 ERRO Response error: invalid image format
//!     Base64ImageError(InvalidImageFormat), place: example/src/handler.rs:5 example::handler
//! ```
//!
//! [thiserror]: https://docs.rs/thiserror
//! [actix-web]: https://docs.rs/actix-web
//! [response_transform]: crate::ResponseTransform

use std::sync::Arc;

use actix_web::HttpResponse;
use arc_swap::ArcSwap;
use lazy_static::lazy_static;

/// A trait that transforms information about an [thiserror] error into
/// a response as desired by the implementor.
///
/// [thiserror]: https://docs.rs/thiserror
#[allow(dead_code)]
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
