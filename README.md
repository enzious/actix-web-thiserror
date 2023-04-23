# actix-web-thiserror

[![License](https://img.shields.io/github/license/enzious/actix-web-thiserror)](https://github.com/enzious/actix-web-thiserror/blob/master/LICENSE.md)
[![Contributors](https://img.shields.io/github/contributors/enzious/actix-web-thiserror)](https://github.com/enzious/actix-web-thiserror/graphs/contributors)
[![GitHub Repo stars](https://img.shields.io/github/stars/enzious/actix-web-thiserror?style=social)](https://github.com/enzious/actix-web-thiserror)
[![crates.io](https://img.shields.io/crates/v/actix-web-thiserror.svg)](https://crates.io/crates/actix-web-thiserror)

A crate that extends the [thiserror] crate functionality to automatically
return a proper [actix-web] response.

## Error definition
```rust
use actix_web_thiserror::ResponseError;
# use log::error;
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
```rust,ignore
use actix_web::*;
use fuzion::utils::image::Base64ImageError;

pub async fn error_test() -> Result<HttpResponse, Error> {
  Err(Base64ImageError::InvalidImageFormat)?
}
```

## Error response

The reason is a string that may be given to the client in some form to explain
the error, if appropriate. Here it is an enum that can be localized.

**Note:** This response has been formatted by a [`ResponseTransform`][response_transform].

```ignore
{
    "result": 0,
    "reason": "INVALID_IMAGE_FORMAT"
}
```

## Error logging

The error text automatically prints to the log when the error is returned out
through a http response.

```ignore
Apr 23 02:19:35.211 ERRO Response error: invalid image format
    Base64ImageError(InvalidImageFormat), place: example/src/handler.rs:5 example::handler
```

[thiserror]: https://docs.rs/thiserror
[actix-web]: https://docs.rs/actix-web
[response_transform]: crate::ResponseTransform
