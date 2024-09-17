use std::env;
use actix_web::{delete, get, post, put, HttpResponse, Responder};
use actix_web::web::{Data, Json, Path, Query, ServiceConfig};
use rusqlite::Connection;
use serde::Deserialize;
use log::{error, info};
use crate::db::application::get_by_id;
use crate::db::user;
use crate::models::{User, UserStore};
use crate::models::user::UserUpdateRequest;
use crate::utils::{ErrorResponse, PaginationUser};

#[derive(Deserialize)]
pub struct UserQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub(crate) fn configure(store: Data<UserStore>) -> impl FnOnce(&mut ServiceConfig) {
    move |config: &mut ServiceConfig| {
        config
            .app_data(store)
            .service(get_users)
            .service(get_user_by_id)
            .service(create_user)
            .service(update_user)
            .service(delete_user);
    }
}

/// Get list of users with pagination.
///
/// This endpoint needs `api_key` authentication in order to call.
///
/// List users from the database with pagination support.
#[utoipa::path(
    context_path = "/v1",
    tag = "users",
    params(
        ("limit" = Option<usize>, Query, description = "Maximum number of items to return", example = 10),
        ("offset" = Option<usize>, Query, description = "Offset for pagination", example = 0),
    ),
    responses(
        (status = 200, description = "List current user items with pagination metadata", body = PaginationUser<Vec<User>>),
        (status = 401, description = "Unauthorized to get users", body = ErrorResponse, example = json!(ErrorResponse::Unauthorized(String::from("Missing API Key")))),
    ),
    security(
        (),
        ("api_key" = [])
    )
)]
#[get("/users")]
pub(super) async fn get_users(query: Query<UserQuery>) -> impl Responder {
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "not set".to_string());
    let mut conn = match Connection::open(&db_url) {
        Ok(conn) => conn,
        Err(e) => {
            error!("Error connecting to the database: {:?}", e);
            return HttpResponse::NotFound().json(ErrorResponse::NotFound(
                "Error connecting to the database".to_string(),
            ));
        }
    };

    let limit = query.limit.unwrap_or(10) as i64;
    let offset = query.offset.unwrap_or(0) as i64;

    let total_count = user::get_total_count(&mut conn).unwrap_or_else(|e| {
        error!("Error getting total count from the database: {:?}", e);
        0
    });

    match user::get_all(&mut conn, limit, offset) {
        Ok(users) => {
            let page = (offset / limit) + 1;
            let pagination = PaginationUser {
                page,
                count: total_count,
                items: users,
            };
            HttpResponse::Ok().json(pagination)
        }
        Err(e) => {
            error!("Error getting users from the database: {:?}", e);
            HttpResponse::NotFound().json(ErrorResponse::NotFound(
                "Error getting users from the database".to_string(),
            ))
        }
    }
}

/// Get user by given user id.
///
/// This endpoint needs `api_key` authentication in order to call.
///
/// Return found `User` with status 200 or 404 not found if `User` is not found from the database.
#[utoipa::path(
    context_path = "/v1",
    tag = "users",
    params(
        ("id", description = "Unique ID of the user", example = 1)
    ),
    responses(
        (status = 200, description = "User found", body = User),
        (status = 401, description = "Unauthorized to get user", body = ErrorResponse, example = json!(ErrorResponse::Unauthorized(String::from("missing api key")))),
        (status = 404, description = "User not found", body = ErrorResponse, example = json!(ErrorResponse::NotFound(String::from("id = 1"))))
    ),
    security(
        (),
        ("api_key" = [])
    )
)]
#[get("/users/{id}")]
pub(super) async fn get_user_by_id(id: Path<i64>) -> impl Responder {
    let id = id.into_inner();
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "not set".to_string());
    let mut conn = Connection::open(&db_url).unwrap();

    if let Ok(Some(user)) = user::get_by_id(&mut conn, id) {
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::NotFound().body(format!("User with ID {} not found", id))
    }
}

/// Create a new user.
///
/// This endpoint needs `api_key` authentication in order to call.
///
/// Create a new `User` in the database.
#[utoipa::path(
    request_body = User,
    context_path = "/v1",
    tag = "users",
    responses(
        (status = 201, description = "User created successfully", body = User),
        (status = 401, description = "Unauthorized to create user", body = ErrorResponse, example = json!(ErrorResponse::Unauthorized(String::from("missing api key")))),
        (status = 400, description = "Invalid user data", body = ErrorResponse, example = json!(ErrorResponse::BadRequest(String::from("Invalid user data"))))
    ),
    security(
        (),
        ("api_key" = [])
    )
)]
#[post("/users")]
pub(super) async fn create_user(user: Json<UserUpdateRequest>) -> impl Responder {
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "not set".to_string());
    info!("DATABASE_URL = {:?}", db_url);
    let mut conn = match Connection::open(&db_url) {
        Ok(conn) => conn,
        Err(e) => {
            error!("Error connecting to the database: {:?}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse::InternalError(
                "Error connecting to the database".to_string(),
            ));
        }
    };

    let user = user.into_inner();

    match user::create(&mut conn, user.clone()) {
        Ok(_) => {
            info!("User created successfully: {:?}", user);
            HttpResponse::Created().json(user)
        }
        Err(e) => {
            error!("Error creating user: {:?}", e);
            HttpResponse::InternalServerError().json(ErrorResponse::InternalError(
                "Error creating user".to_string(),
            ))
        }
    }
}

/// Update an existing user.
///
/// This endpoint needs `api_key` authentication in order to call.
///
/// Update an existing `User` in the database.
#[utoipa::path(
context_path = "/v1",
    tag = "users",
    params(
        ("id", description = "Unique ID of the user", example = 1)
    ),
    request_body = UserUpdateRequest,
    responses(
        (status = 200, description = "User updated successfully", body = User),
        (status = 401, description = "Unauthorized to update user", body = ErrorResponse, example = json!(ErrorResponse::Unauthorized(String::from("missing api key")))),
        (status = 404, description = "User not found", body = ErrorResponse, example = json!(ErrorResponse::NotFound(String::from("id = 1"))))
    ),
    security(
        (),
        ("api_key" = [])
    )
)]
#[put("/users/{id}")]
pub(super) async fn update_user(
    id: Path<i64>,
    user_update_request: Json<UserUpdateRequest>,
) -> impl Responder {
    let id = id.into_inner();
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "not set".to_string());
    let mut conn = match Connection::open(&db_url) {
        Ok(conn) => conn,
        Err(e) => {
            error!("Error connecting to the database: {:?}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse::InternalError(
                "Error connecting to the database".to_string(),
            ));
        }
    };

    // Retrieve the existing user to update
    let existing_user = match user::get_by_id(&mut conn, id) {
        Ok(Some(user)) => user,
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("Error retrieving user with ID {}: {:?}", id, e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    // Create a new user with updated fields
    let updated_user = User {
        id: existing_user.id,
        name: user_update_request.name.clone().unwrap_or(existing_user.name),
        email: user_update_request.email.clone().unwrap_or(existing_user.email),
        password: user_update_request.password.clone().unwrap_or(existing_user.password),
        role: user_update_request.role.clone().unwrap_or(existing_user.role),
        created_at: Default::default(),
        updated_at: Default::default(),
    };

    // Call the update function
    match user::update(&mut conn, id, updated_user) {
        Ok(_) => {
            info!("Updated user...");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            eprintln!("Error updating user: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Delete a user by id.
///
/// This endpoint needs `api_key` authentication in order to call.
///
/// Delete the `User` from the database.
#[utoipa::path(
    context_path = "/v1",
    tag = "users",
    params(
        ("id", description = "Unique ID of the user", example = 1)
    ),
    responses(
        (status = 200, description = "User deleted successfully"),
        (status = 401, description = "Unauthorized to delete user", body = ErrorResponse, example = json!(ErrorResponse::Unauthorized(String::from("missing api key")))),
        (status = 404, description = "User not found", body = ErrorResponse, example = json!(ErrorResponse::NotFound(String::from("id = 1"))))
    ),
    security(
        (),
        ("api_key" = [])
    )
)]
#[delete("/users/{id}")]
pub(super) async fn delete_user(id: Path<i32>) -> impl Responder {
    let id = id.into_inner() as i64;
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "not set".to_string());
    let mut conn = Connection::open(&db_url).unwrap();

    match user::delete(&mut conn, id) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            error!("Error deleting user with ID {}: {:?}", id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}