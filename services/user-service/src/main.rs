use actix_web::{App, HttpServer};
use colored::*;

mod constants;
mod core;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!(
        "starting {} on {}...",
        constants::SERVICE_NAME.green(),
        format!("http://localhost:{}", constants::PORT).cyan()
    );

    HttpServer::new(|| App::new().configure(routes::routes_config))
        .bind(format!("{}:{}", constants::IP, constants::PORT))?
        .run()
        .await
}
