use actix_web::{web, HttpResponse, Responder};

use crate::core::v1::models;

#[utoipa::path(
    get,
    context_path = "/user-service/v1",
    path = "/users",
    responses(
        (status = 200, description = "Docs found succesfully"),
        (status = 404, description = "Docs was not found")
    ),
)]
pub async fn get_users() -> impl Responder {
    // logic to retrieve all users from a database
    let users = vec![
        models::User {
            id: 1,
            name: "John Doe".to_owned(),
            email: "johndoe@example.com".to_owned(),
        },
        models::User {
            id: 2,
            name: "Jane Doe".to_owned(),
            email: "janedoe@example.com".to_owned(),
        },
    ];
    HttpResponse::Ok().json(users)
}

#[utoipa::path(
    get,
    context_path = "/user-service/v1",
    path = "/user/{id}",
    responses(
        (status = 200, description = "Docs found succesfully"),
        (status = 404, description = "Docs was not found")
    ),
)]
pub async fn get_user(info: web::Path<i32>) -> impl Responder {
    // logic to retrieve a user by id from a database
    let user = models::User {
        id: *info,
        name: "John Doe".to_owned(),
        email: "johndoe@example.com".to_owned(),
    };
    HttpResponse::Ok().json(user)
}

#[utoipa::path(
    post,
    context_path = "/user-service/v1",
    path = "/user",
    request_body = User,
    responses(
        (status = 200, description = "Docs found succesfully", body = [User]),
        (status = 404, description = "Docs was not found")
    ),
)]
pub async fn create_user(user: web::Json<models::User>) -> impl Responder {
    // logic to insert a new user into a database
    HttpResponse::Created().json(user.into_inner())
}

#[utoipa::path(
    put,
    context_path = "/user-service/v1",
    path = "/user/{id}",
    request_body = User,
    responses(
        (status = 200, description = "Docs found succesfully"),
        (status = 404, description = "Docs was not found")
    ),
)]
pub async fn update_user(info: web::Path<i32>, user: web::Json<models::User>) -> impl Responder {
    // logic to update a user by id in a database
    let updated_user = models::User {
        id: *info,
        name: user.name.clone(),
        email: user.email.clone(),
    };
    HttpResponse::Ok().json(updated_user)
}

#[utoipa::path(
    delete,
    context_path = "/user-service/v1",
    path = "/user/{id}",
    request_body = User,
    responses(
        (status = 200, description = "Docs found succesfully"),
        (status = 404, description = "Docs was not found")
    ),
)]
pub async fn delete_user(info: web::Path<i32>) -> impl Responder {
    // logic to delete a user by id from a database
    let user = models::User {
        id: *info,
        name: "John Doe".to_owned(),
        email: "johndoe@example.com".to_owned(),
    };
    HttpResponse::Ok().json(user)
}
