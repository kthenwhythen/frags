use actix_web::{web, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::resource("/app")
      .route(web::get().to(|| async { HttpResponse::Ok().body("app") }))
      .route(web::head().to(HttpResponse::MethodNotAllowed))
  );
}