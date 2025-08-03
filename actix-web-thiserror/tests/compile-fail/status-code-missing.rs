use actix_web_thiserror::ResponseError;
use thiserror::Error;

#[derive(Debug, Error, ResponseError)]
enum MacroErrors {
  #[response(status)]
  #[error("invalid status code")]
  InvalidStatusCode,
}

fn main() {}
