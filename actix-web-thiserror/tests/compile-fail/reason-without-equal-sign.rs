use actix_web_thiserror::ResponseError;
use thiserror::Error;

#[derive(Debug, Error, ResponseError)]
enum MacroErrors {
  #[response(reason)]
  #[error("reason without equal sign")]
  ReasonWithoutEqualSign,
}

fn main() {}
