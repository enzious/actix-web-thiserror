use actix_web_thiserror::ResponseError;
use thiserror::Error;

#[derive(Debug, Error, ResponseError)]
enum MacroErrors {
  #[response(type = 10)]
  #[error("invalid type")]
  InvalidType,
}

fn main() {}
