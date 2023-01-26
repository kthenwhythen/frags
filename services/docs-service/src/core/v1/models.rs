use serde::Deserialize;
use utoipa::{OpenApi, ToSchema};

use crate::core::v1::controllers;

#[derive(OpenApi)]
#[openapi(paths(controllers::get_docs), components(schemas(OpenapiPath)))]
pub struct ApiDoc;

#[derive(ToSchema, Deserialize)]
pub struct OpenapiPath {
    openapi_path: String,
}
