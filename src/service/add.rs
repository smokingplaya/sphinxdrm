use std::env;
use actix_files::NamedFile;
use actix_web::{get, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::{redis::{self, ResultError}, service::{badrequest, response}};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
struct AddData {
  token: String,
  macAddress: String,
  productName: String
}

// // Я вообще хотел put, но HTML формы поддерживают только get/post, так что...
#[put("/sphinx/add")]
pub async fn add(body: web::Json<AddData>) -> HttpResponse {
  // make sure that token is allowed
  let allowed_tokens = env::var("ALLOWEDTOKENS")
    .unwrap_or("[]".to_string());

  let tokens: Vec<String> = serde_json::from_str(allowed_tokens.as_str()).unwrap();

  if !tokens.contains(&body.token) {
    return badrequest();
  }

  // redis

  let connection = redis::create_connection();

  if connection.check_err() {
    return badrequest();
  }

  let mut con = connection.unwrap();

  response(con.add(&body.macAddress, &body.productName).is_ok())
}

#[get("/sphinx/add")]
pub async fn page_add() -> impl Responder {
  NamedFile::open_async("./static/add.html").await
}