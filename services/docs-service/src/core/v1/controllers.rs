use actix_web::{HttpResponse, Responder};

#[utoipa::path(
    get,
    context_path = "/docs/v1",
    path = "/",
    responses(
        (status = 200, description = "Docs found succesfully"),
        (status = 404, description = "Docs was not found")
    ),
)]
pub async fn get_docs() -> impl Responder {
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
