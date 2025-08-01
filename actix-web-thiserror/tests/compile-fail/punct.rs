use actix_web_thiserror::ResponseError;
use thiserror::Error;

#[derive(Debug, Error, ResponseError)]
enum MacroErrors {
  #[response(status = 200; )]
  #[error("invalid punctuation")]
  InvalidPunct,
}

fn main() {}
