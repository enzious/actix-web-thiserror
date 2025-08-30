//! # actix-web-thiserror
//!
//! [![License](https://img.shields.io/github/license/enzious/actix-web-thiserror)](https://github.com/enzious/actix-web-thiserror/blob/master/LICENSE.md)
//! [![Contributors](https://img.shields.io/github/contributors/enzious/actix-web-thiserror)](https://github.com/enzious/actix-web-thiserror/graphs/contributors)
//! [![GitHub Repo stars](https://img.shields.io/github/stars/enzious/actix-web-thiserror?style=social)](https://github.com/enzious/actix-web-thiserror)
//! [![crates.io](https://img.shields.io/crates/v/actix-web-thiserror.svg)](https://crates.io/crates/actix-web-thiserror)
//!
//! A crate that extends the [thiserror] crate functionality to automatically
//! return a proper [actix-web] response.
//!
//! ## Documentation
//!
//! - [API Documentation](https://docs.rs/actix-web-thiserror)
//!
//! ## Error definition
//! ```rust
//! use actix_web_thiserror::ResponseError;
//! use thiserror::Error;
//!
//! #[derive(Debug, Error, ResponseError)]
//! pub enum Base64ImageError {
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
//! # use actix_web_thiserror::ResponseError;
//! # use actix_web::*;
//! # use thiserror::Error;
//! #
//! # #[derive(Debug, Error, ResponseError)]
//! # pub enum Base64ImageError {
//! #   #[response(reason = "INVALID_IMAGE_FORMAT")]
//! #   #[error("invalid image format")]
//! #   InvalidImageFormat,
//! # }
//! #
//! pub async fn error_test() -> Result<HttpResponse, Error> {
//!   Err(Base64ImageError::InvalidImageFormat)?
//! }
//! ```
//!
//! ## Error response
//!
//! The `reason` is a string that may be given to the client in some form to explain
//! the error, if appropriate. Here it is as an enum that can be localized.
//!
//! **Note:** This response has been formatted by a [`ResponseTransform`][response_transform].
//!
//! ```json
//! {
//!     "result": 0,
//!     "reason": "INVALID_IMAGE_FORMAT"
//! }
//! ```
//!
//! ## Error logging
//!
//! Error logging should be done in the [`ResponseTransform::transform`][ResponseTransform::transform] method.
//!
//! ```ignore
//! fn transform(
//!   &self,
//!   name: &str,
//!   err: &dyn std::error::Error,
//!   status_code: actix_web::http::StatusCode,
//!   reason: Option<serde_json::Value>,
//!   _type: Option<String>,
//!   details: Option<serde_json::Value>,
//! ) -> HttpResponse {
//!   if let Some(backtrace) = request_ref::<std::backtrace::Backtrace>(&err) {
//!     log::error!("Response error: {err}\n{name}\n{backtrace}");
//!   } else {
//!     log::error!("Response error: {err}\n{name}");
//!   }
//!
//!   HttpResponse::InternalServerError().finish()
//! }
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
#[allow(unused)]
pub trait ResponseTransform {
  fn transform(
    &self,
    name: &str,
    err: &dyn std::error::Error,
    status_code: actix_web::http::StatusCode,
    reason: Option<serde_json::Value>,
    _type: Option<String>,
    details: Option<serde_json::Value>,
  ) -> HttpResponse {
    actix_web::HttpResponse::build(status_code).finish()
  }

  fn default_error_status_code(&self) -> actix_web::http::StatusCode {
    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
  }
}

struct ReflexiveTransform;

impl ResponseTransform for ReflexiveTransform {}

lazy_static! {
  static ref RESPONSE_TRANSFORM: ArcSwap<Box<dyn ResponseTransform + Sync + Send>> =
    ArcSwap::from(Arc::new(Box::new(ReflexiveTransform {}) as _));
}

/// Sets the default global transform for errors into responses.
pub fn set_global_transform(transform: impl ResponseTransform + Sync + Send + 'static) {
  RESPONSE_TRANSFORM.swap(Arc::new(Box::new(transform)));
}

#[doc(hidden)]
pub fn apply_global_transform(
  name: &str,
  err: &dyn std::error::Error,
  status_code: actix_web::http::StatusCode,
  reason: Option<serde_json::Value>,
  _type: Option<String>,
  details: Option<serde_json::Value>,
) -> HttpResponse {
  ResponseTransform::transform(
    (**RESPONSE_TRANSFORM.load()).as_ref(),
    name,
    err,
    status_code,
    reason,
    _type,
    details,
  )
}

#[doc(hidden)]
pub fn default_global_error_status_code() -> actix_web::http::StatusCode {
  ResponseTransform::default_error_status_code((**RESPONSE_TRANSFORM.load()).as_ref())
}

#[doc(hidden)]
pub trait ThiserrorResponse {
  fn status_code(&self) -> Option<actix_web::http::StatusCode> {
    None
  }

  fn reason(&self) -> Option<Option<serde_json::Value>> {
    None
  }

  fn _type(&self) -> Option<Option<String>> {
    None
  }

  fn details(&self) -> Option<Option<serde_json::Value>> {
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
