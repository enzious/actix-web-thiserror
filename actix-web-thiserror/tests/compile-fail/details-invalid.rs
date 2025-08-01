use actix_web_thiserror::ResponseError;
use thiserror::Error;

#[derive(Debug, Error, ResponseError)]
enum MacroErrors {
  #[response(details = )]
  #[error("invalid details")]
  InvalidDetails,
}

fn main() {}
