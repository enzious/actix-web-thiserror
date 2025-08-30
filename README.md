# actix-web-thiserror

[![License](https://img.shields.io/github/license/enzious/actix-web-thiserror)](https://github.com/enzious/actix-web-thiserror/blob/master/LICENSE.md)
[![Contributors](https://img.shields.io/github/contributors/enzious/actix-web-thiserror)](https://github.com/enzious/actix-web-thiserror/graphs/contributors)
[![GitHub Repo stars](https://img.shields.io/github/stars/enzious/actix-web-thiserror?style=social)](https://github.com/enzious/actix-web-thiserror)
[![crates.io](https://img.shields.io/crates/v/actix-web-thiserror.svg)](https://crates.io/crates/actix-web-thiserror)

A crate that extends the [thiserror] crate functionality to automatically
return a proper [actix-web] response.

## Documentation

- [API Documentation](https://docs.rs/actix-web-thiserror)

## Error definition
```rust
use actix_web_thiserror::ResponseError;
use thiserror::Error;

#[derive(Debug, Error, ResponseError)]
pub enum Base64ImageError {
  #[response(reason = "INVALID_IMAGE_FORMAT")]
  #[error("invalid image format")]
  InvalidImageFormat,
  #[response(reason = "INVALID_STRING")]
  #[error("invalid string")]
  InvalidString,
}
```

## Error implementation
```rust
pub async fn error_test() -> Result<HttpResponse, Error> {
  Err(Base64ImageError::InvalidImageFormat)?
}
```

## Error response

The `reason` is a string that may be given to the client in some form to explain
the error, if appropriate. Here it is as an enum that can be localized.

**Note:** This response has been formatted by a [`ResponseTransform`][response_transform]. To specify a custom ResponseTransform, implement [`ResponseTransform`][response_transform] and add `#[response(transform = custom)]` under your derive.

```
{
    "result": 0,
    "reason": "INVALID_IMAGE_FORMAT"
}
```

## Error logging

Error logging should be done in the [`ResponseTransform::transform`][ResponseTransform::transform] method.

```rust
fn transform(
  &self,
  name: &str,
  err: &dyn std::error::Error,
  status_code: actix_web::http::StatusCode,
  reason: Option<serde_json::Value>,
  _type: Option<String>,
  details: Option<serde_json::Value>,
) -> HttpResponse {
  if let Some(backtrace) = request_ref::<std::backtrace::Backtrace>(&err) {
    log::error!("Response error: {err}\n{name}\n{backtrace}");
  } else {
    log::error!("Response error: {err}\n{name}");
  }

  HttpResponse::InternalServerError().finish()
}
```

[thiserror]: https://docs.rs/thiserror
[actix-web]: https://docs.rs/actix-web
[response_transform]: crate::ResponseTransform
[ResponseTransform::transform]: crate::ResponseTransform::transform
