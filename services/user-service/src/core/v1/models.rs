use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

use crate::core::v1::controllers;
use diesel::prelude::*;

#[derive(OpenApi)]
#[openapi(
    paths(
        controllers::get_users,
        controllers::get_user,
        controllers::create_user,
        controllers::update_user,
        controllers::delete_user
    ),
    components(schemas(User))
)]
pub struct ApiDoc;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Queryable, ToSchema)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
