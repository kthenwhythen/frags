use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use utoipa_swagger_ui::Url;

use crate::core::v1;

pub fn routes_config(_routes_config: &mut web::ServiceConfig) {
    _routes_config
        .service(
            SwaggerUi::new("/user-service/swagger-ui/{_:.*}").urls(vec![(
                Url::new("user-service-v1", "/user-service/v1/openapi.json"),
                v1::models::ApiDoc::openapi().clone(),
            )]),
        )
        .service(
            web::scope("/user-service").service(
                web::scope("/v1")
                    .route("/users", web::get().to(v1::controllers::get_users))
                    .route("/user/{id}", web::get().to(v1::controllers::get_user))
                    .route("/user", web::post().to(v1::controllers::create_user))
                    .route("/user/{id}", web::put().to(v1::controllers::update_user))
                    .route("/user/{id}", web::delete().to(v1::controllers::delete_user)),
            ),
        );
}
