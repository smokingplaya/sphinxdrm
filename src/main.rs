use actix_cors::Cors;
use actix_web::{http::header, App, HttpServer};
use std::env;

mod service;
mod redis;
mod dotenv;

extern crate log;
extern crate env_logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv::init();
  env_logger::init();

  let port = env::var("WEBSERVER_PORT")
    .unwrap_or("8080".to_string())
    .parse::<u16>().unwrap_or(8080);

  HttpServer::new(||
      App::new()
        .service(service::info::info)
        .service(service::check::check)
        .service(service::add::add)
        .service(service::add::page_add)
        .wrap(
          Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600)
      )
    )
    .bind(("0.0.0.0", port))?
    .run()
    .await
}