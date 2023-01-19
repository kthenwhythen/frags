use actix_web::{
    get, guard, http::KeepAlive, post, web, App, HttpResponse, HttpServer, Responder, Result,
};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::Deserialize;
use std::{sync::Mutex, time::Duration};
use tokio;
use utoipa::{OpenApi, ToSchema};

// #[derive(OpenApi)]
// struct ApiDoc;

// this thing not respond on ""
#[get("/")]
async fn docs() -> impl Responder {
    HttpResponse::Ok().body("docs")
}

#[get("/redoc")]
async fn redoc() -> impl Responder {
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Redoc</title>
            <!-- needed for adaptive design -->
            <meta charset="utf-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1" />
            <link
            href="https://fonts.googleapis.com/css?family=Montserrat:300,400,700|Roboto:300,400,700"
            rel="stylesheet"
            />

            <!--
            Redoc doesn't change outer page styles
            -->
            <style>
            body {
                margin: 0;
                padding: 0;
            }
            </style>
        </head>
        <body>
            <!--
            Redoc element with link to your OpenAPI definition
            -->
            <redoc spec-url="http://localhost:8080/api"></redoc>
            <!--
            Link to Redoc JavaScript on CDN for rendering standalone element
            -->
            <script src="https://cdn.redoc.ly/redoc/latest/bundles/redoc.standalone.js"></script>
        </body>
        </html>
    "#;
    HttpResponse::Ok().content_type("text/html").body(html)
}

#[derive(ToSchema, Deserialize)]
struct OpenapiPath {
    openapi_path: String,
}

// yml also redirect to this
#[get("/{openapi}")]
async fn openapi_s(openapi: web::Path<OpenapiPath>) -> impl Responder {
    // let open_api = ApiDoc::openapi().to_yaml().unwrap();
    HttpResponse::Ok().body("open_api")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let PORT: &str = "9001";
    let args: Vec<String> = std::env::args().collect();
    let service_name: &str = &args[0].split('/').nth(2).unwrap_or("");

    println!("starting {} on port {}...", &service_name, PORT);

    HttpServer::new(move || {
        App::new().service(
            web::scope("/docs")
                .service(docs)
                .service(redoc)
                .service(openapi_s),
        )
    })
    .bind(format!("0.0.0.0:{}", PORT))?
    .run()
    .await
}
