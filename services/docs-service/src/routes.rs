use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use utoipa_swagger_ui::Url;

use crate::core::v1;
use crate::core::v2;

pub fn routes_config(_routes_config: &mut web::ServiceConfig) {
    _routes_config
        .service(SwaggerUi::new("/docs/swagger-ui/{_:.*}").urls(vec![
            (
                Url::new("docs-v1", "/docs/v1/openapi.json"),
                v1::models::ApiDoc::openapi().clone(),
            ),
            (
                Url::new("docs-v2", "/docs/v2/openapi.json"),
                v2::models::ApiDoc::openapi().clone(),
            ),
        ]))
        .service(
            web::scope("/docs")
                .service(
                    web::scope("/v1")
                        .route("", web::get().to(v1::controllers::get_docs))
                        .route("/", web::get().to(v1::controllers::get_docs)),
                )
                .service(
                    web::scope("/v2")
                        .route("", web::get().to(v2::controllers::docs))
                        .route("/", web::get().to(v2::controllers::docs)),
                ),
        );
}
