pub(crate) mod add;
pub(crate) mod info;
pub(crate) mod check;

pub fn badrequest() -> actix_web::HttpResponse {
  actix_web::HttpResponse::BadRequest().finish()
}

pub fn response(condition: bool) -> actix_web::HttpResponse {
  if condition {
    actix_web::HttpResponse::Ok()
  } else {
    actix_web::HttpResponse::BadRequest()
  }.finish()
}