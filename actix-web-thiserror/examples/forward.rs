use actix_web_thiserror::ResponseError;
use thiserror::Error;

#[derive(Debug, Error, ResponseError)]
enum Inner {
  #[expect(dead_code)]
  #[response(status = 404)]
  #[error("inner error")]
  NotFound,
}

#[allow(unused)]
#[derive(Debug, Error, ResponseError)]
enum Outer {
  #[response(forward)]
  #[error(transparent)]
  Inner(Inner),
}

fn main() {}
