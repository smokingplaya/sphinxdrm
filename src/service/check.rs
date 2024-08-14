use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::{redis::{self, ResultError}, service::{badrequest, response}};

#[allow(unused)]
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
struct DRMData {
  macAddress: String,
  productName: String
}

/*
  redis:
    mac-address = String[] (JSON Array)

    example:
      5C-E9-31-F9-94-4E: ["", "", ""]
      2A-59-31-A9-92-4E: [""]
      ... etc
*/
#[allow(unused)]
#[post("/sphinx/check")]
async fn check(body: web::Json<DRMData>) -> HttpResponse {
  let connection = redis::create_connection();

  if connection.check_err() {
    return badrequest();
  }

  // Здесь мы можем безопасно анврапать, потому-что выше проверили на ошибку
  let mut con = connection.unwrap();

  response(con.has(&body.macAddress, &body.productName).unwrap_or(false))
}