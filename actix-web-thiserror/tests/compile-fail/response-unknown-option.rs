use actix_web_thiserror::ResponseError;
use thiserror::Error;

#[derive(Debug, Error, ResponseError)]
enum MacroErrors {
  #[response(unknown_param = "This should fail")]
  #[error("invalid image format")]
  UnknownResponseParameter,
}

fn main() {}
