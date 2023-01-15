use std::sync::Mutex;
mod resources;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, guard};

struct AppState {
    app_name: String,
    counter: Mutex<i32>,
}

#[get("/")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    HttpResponse::Ok().body(format!("hello world from {app_name}. Request number: {counter}"))
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state= web::Data::new(AppState {
        app_name: String::from("actix web"),
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .configure(resources::app::config)
            .app_data(app_state.clone())
            .service(web::scope("/users").service(show_users))
            .service(hello)
            .service(echo)
            .service(
                web::scope("/hey")
                    // guard that filter requests on 'Host' header field
                    .guard(guard::Header("Host", "localhost:8080"))
                    .route("", web::get().to(manual_hello))
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}