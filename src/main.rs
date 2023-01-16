use actix_web::{
    get, guard, http::KeepAlive, post, web, App, HttpResponse, HttpServer, Responder, Result,
};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::Deserialize;
use std::{sync::Mutex, time::Duration};
use tokio;

mod resources;

struct AppState {
    app_name: String,
    counter: Mutex<i32>,
}

#[derive(Deserialize)]
struct Info {
    user_id: u32,
    friend: String,
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

#[get("/{user_id}/{friend}")]
async fn user(info: web::Path<Info>) -> Result<String> {
    Ok(format!(
        "Welcome {}, user_id {}!",
        info.friend, info.user_id
    ))
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
