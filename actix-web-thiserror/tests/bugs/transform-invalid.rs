use actix_web_thiserror::ResponseError;
use thiserror::Error;

#[derive(Debug, Error, ResponseError)]
#[response(transform = invalid)]
enum MacroErrors {
  #[error("invalid transform")]
  InvalidTransform,
}

fn main() {}
