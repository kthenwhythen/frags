use actix_web::{http, web, HttpRequest, HttpResponse, Responder};
use utoipa::OpenApi;

use crate::core::v2::models;

#[utoipa::path(
    get,
    context_path = "/docs/v2",
    path = "/",
    responses(
        (status = 200, description = "found succesfully"),
        (status = 404, description = "was not found")
    ),
)]
pub async fn docs() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
        <html>
            <body>
                <h1>Documentation</h1>
                <a href="/swagger-ui">swagger-ui</a>
            </body>
        </html>
        "#,
    )
}

// hack for enabling support for scopes in routes.rs
#[utoipa::path(
    get,
    context_path = "/docs/v2",
    path = "/openapi.json",
    responses(
        (status = 200, description = "found succesfully"),
        (status = 404, description = "was not found")
    ),
)]
pub async fn openapi() -> impl Responder {
    HttpResponse::Ok().json(models::ApiDoc::openapi().clone())
}

pub async fn handle_empty_path(req: HttpRequest) -> impl Responder {
    HttpResponse::Found()
        .append_header((http::header::LOCATION, format!("{}/", req.path())))
        .finish()
}
