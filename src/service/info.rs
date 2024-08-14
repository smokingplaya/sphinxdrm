use actix_web::{get, http::header::ContentType, HttpResponse};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
struct SphinxInfo {
  author: &'static str,
  description: &'static str,
  version: &'static str,
}

#[get("/sphinx")]
pub async fn info() -> HttpResponse {
  let response = json!(SphinxInfo {
    author: env!("CARGO_PKG_AUTHORS"),
    description: env!("CARGO_PKG_DESCRIPTION"),
    version: env!("CARGO_PKG_VERSION")
  }).to_string();

  HttpResponse::Ok().insert_header(ContentType::json()).body(response)
}