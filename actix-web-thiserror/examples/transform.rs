use actix_web_thiserror::ResponseError;
use actix_web_thiserror::ResponseTransform;
use thiserror::Error;

#[expect(unused)]
#[derive(Debug, Error, ResponseError)]
#[response(transform = custom)]
enum MyErrors {
  #[error("to be transformed")]
  ToBeTransform,
}

/// This could be simply `impl MyErrors`, however by
/// implementing `ResponseTransform` the compile error
/// is more informative.
impl ResponseTransform for MyErrors {
  fn transform(
    &self,
    _name: &str,
    _err: &dyn std::error::Error,
    _status_code: actix_web::http::StatusCode,
    _reason: Option<serde_json::Value>,
    _type: Option<String>,
    _details: Option<serde_json::Value>,
  ) -> actix_web::HttpResponse {
    actix_web::HttpResponse::InternalServerError().into()
  }
}

fn main() {}
