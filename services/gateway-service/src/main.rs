use actix_web::{
    get, guard, http::KeepAlive, post, web, App, HttpResponse, HttpServer, Responder, Result,
};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::Deserialize;
use std::{sync::Mutex, time::Duration};
use tokio;
use utoipa::{OpenApi, ToSchema};

mod resources;

#[derive(OpenApi)]
#[openapi(paths(user), components(schemas(Info)))]
struct ApiDoc;

#[derive(ToSchema)]
struct AppState {
    app_name: String,
    counter: Mutex<i32>,
}

#[derive(ToSchema, Deserialize)]
struct Info {
    user_id: String,
    friend: String,
}

#[derive(ToSchema, Deserialize)]
struct QueryInfo {
    nickname: Option<String>,
}

#[get("/")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    HttpResponse::Ok().body(format!(
        "hello world from {app_name}. Request number: {counter}"
    ))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(format!("lol you or {}", req_body))
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("hey there!")
}

#[get("/all")]
async fn show_users() -> impl Responder {
    HttpResponse::Ok().body("users: you and someone")
}

#[get("/sleep")]
async fn sleep() -> impl Responder {
    println!("sleep start");
    tokio::time::sleep(Duration::from_secs(5)).await;
    println!("sleep end");
    HttpResponse::Ok().body("wake")
}

#[utoipa::path(
    get,
    path = "users/{user_id}/{friend}",
    responses(
        (status = 200, description = "user found succesfully", body = Info),
        (status = 404, description = "user was not found")
    ),
    params(
        ("id" = u64, Path, description = "user database id to get user for"),
    )
)]
#[get("/{user_id}/{friend}")]
async fn user(info: web::Path<Info>, query_info: web::Query<QueryInfo>) -> Result<String> {
    Ok(format!(
        "Welcome {}, user_id {}! Query nickname {:?}",
        info.friend, info.user_id, query_info.nickname
    ))
}

#[get("/api")]
async fn api() -> impl Responder {
    let open_api = ApiDoc::openapi().to_yaml().unwrap();
    HttpResponse::Ok().body(open_api)
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("starting frags server...");

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    let app_state = web::Data::new(AppState {
        app_name: String::from("actix web"),
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .configure(resources::app::config)
            .app_data(app_state.clone())
            .service(web::scope("/users").service(show_users).service(user))
            .service(hello)
            .service(echo)
            .service(api)
            .service(redoc)
            .service(
                web::scope("/hey")
                    // guard that filter requests on 'Host' header field
                    .guard(guard::Header("Host", "localhost:8080"))
                    .route("", web::get().to(manual_hello)),
            )
            .service(sleep)
    })
    .keep_alive(KeepAlive::Os)
    .shutdown_timeout(10)
    // .bind_openssl("0.0.0.0:8080", builder)?
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
