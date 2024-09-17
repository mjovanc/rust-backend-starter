use std::env;
use actix_web::{delete, get, post, put, HttpResponse, Responder};
use actix_web::web::{Data, Json, Path, Query, ServiceConfig};
use chrono::Utc;
use rusqlite::Connection;
use serde::Deserialize;
use log::{error, info};
use crate::db::application_db;
use crate::models::application::{Application, ApplicationUpdateRequest};
use crate::models::ApplicationStore;
use crate::utils::{ErrorResponse, Pagination};
use utoipa::ToSchema;

/// Query parameters for pagination
#[derive(Deserialize, ToSchema)]
pub struct ApplicationQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub(crate) fn configure(store: Data<ApplicationStore>) -> impl FnOnce(&mut ServiceConfig) {
    move |config: &mut ServiceConfig| {
        config
            .app_data(store)
            .service(get_applications)
            .service(get_application_by_id)
            .service(create_application)
            .service(update_application)
            .service(delete_application);
    }
}

/// Get a list of applications with pagination.
///
/// This endpoint requires `api_key` authentication.
///
/// List applications from the database with pagination support.
#[utoipa::path(
    context_path = "/v1",
    tag = "applications",
    params(
        ("limit" = Option<usize>, Query, description = "Maximum number of items to return", example = 10),
        ("offset" = Option<usize>, Query, description = "Offset for pagination", example = 0),
    ),
    responses(
        (status = 200, description = "List of applications with pagination metadata", body = Pagination<Application>),
        (status = 401, description = "Unauthorized to get applications", body = ErrorResponse, example = json!(ErrorResponse::Unauthorized(String::from("Missing API Key")))),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("api_key" = [])
    )
)]
#[get("/applications")]
pub async fn get_applications(query: Query<ApplicationQuery>) -> impl Responder {
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

    let limit = query.limit.unwrap_or(10) as i64;
    let offset = query.offset.unwrap_or(0) as i64;

    let total_count = application_db::get_total_count(&mut conn).unwrap_or_else(|e| {
        error!("Error getting total count from the database: {:?}", e);
        0
    });

    match application_db::get_all(&mut conn, limit, offset) {
        Ok(applications) => {
            let page = (offset / limit) + 1;
            let pagination = Pagination {
                page,
                count: total_count,
                items: applications,
            };
            HttpResponse::Ok().json(pagination)
        }
        Err(e) => {
            error!("Error getting applications from the database: {:?}", e);
            HttpResponse::InternalServerError().json(ErrorResponse::InternalError(
                "Error getting applications from the database".to_string(),
            ))
        }
    }
}

/// Get an application by its ID.
///
/// This endpoint requires `api_key` authentication.
///
/// Return found `Application` or a 404 if the `Application` is not found.
#[utoipa::path(
    context_path = "/v1",
    tag = "applications",
    params(
        ("id" = i64, Path, description = "Unique ID of the application", example = 1)
    ),
    responses(
        (status = 200, description = "Application found", body = Application),
        (status = 401, description = "Unauthorized to get application", body = ErrorResponse, example = json!(ErrorResponse::Unauthorized(String::from("Missing API Key")))),
        (status = 404, description = "Application not found", body = ErrorResponse, example = json!(ErrorResponse::NotFound(String::from("Application ID not found")))),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("api_key" = [])
    )
)]
#[get("/applications/{id}")]
pub async fn get_application_by_id(id: Path<i64>) -> impl Responder {
    let id = id.into_inner();
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "not set".to_string());
    let mut conn = Connection::open(&db_url).unwrap();

    match application_db::get_by_id(&mut conn, id) {
        Ok(Some(application)) => HttpResponse::Ok().json(application),
        Ok(None) => HttpResponse::NotFound().json(ErrorResponse::NotFound(format!("Application with ID {} not found", id))),
        Err(e) => {
            error!("Error retrieving application with ID {}: {:?}", id, e);
            HttpResponse::InternalServerError().json(ErrorResponse::InternalError(
                "Error retrieving application".to_string(),
            ))
        }
    }
}

/// Create a new application.
///
/// This endpoint requires `api_key` authentication.
///
/// Create a new `Application` in the database.
#[utoipa::path(
    context_path = "/v1",
    tag = "applications",
    request_body = Application,
    responses(
        (status = 201, description = "Application created successfully", body = Application),
        (status = 401, description = "Unauthorized to create application", body = ErrorResponse, example = json!(ErrorResponse::Unauthorized(String::from("Missing API Key")))),
        (status = 400, description = "Invalid application data", body = ErrorResponse, example = json!(ErrorResponse::BadRequest(String::from("Invalid application data")))),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("api_key" = [])
    )
)]
#[post("/applications")]
pub async fn create_application(application: Json<Application>) -> impl Responder {
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

    let application = application.into_inner();

    match application_db::create(&mut conn, application.clone()) {
        Ok(_) => {
            info!("Application created successfully: {:?}", application);
            HttpResponse::Created().json(application)
        }
        Err(e) => {
            error!("Error creating application: {:?}", e);
            HttpResponse::InternalServerError().json(ErrorResponse::InternalError(
                "Error creating application".to_string(),
            ))
        }
    }
}

/// Update an existing application.
///
/// This endpoint requires `api_key` authentication.
///
/// Update an existing `Application` in the database.
#[utoipa::path(
    context_path = "/v1",
    tag = "applications",
    params(
        ("id" = i64, Path, description = "Unique ID of the application", example = 1)
    ),
    request_body = ApplicationUpdateRequest,
    responses(
        (status = 200, description = "Application updated successfully", body = Application),
        (status = 401, description = "Unauthorized to update application", body = ErrorResponse, example = json!(ErrorResponse::Unauthorized(String::from("Missing API Key")))),
        (status = 404, description = "Application not found", body = ErrorResponse, example = json!(ErrorResponse::NotFound(String::from("Application ID not found")))),
        (status = 400, description = "Invalid application update data", body = ErrorResponse, example = json!(ErrorResponse::BadRequest(String::from("Invalid application update data")))),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("api_key" = [])
    )
)]
#[put("/applications/{id}")]
pub async fn update_application(
    id: Path<i64>,
    application_update_request: Json<ApplicationUpdateRequest>,
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

    // Retrieve the existing application to update
    let existing_application = match application_db::get_by_id(&mut conn, id) {
        Ok(Some(application)) => application,
        Ok(None) => return HttpResponse::NotFound().json(ErrorResponse::NotFound(format!("Application with ID {} not found", id))),
        Err(e) => {
            error!("Error retrieving application with ID {}: {:?}", id, e);
            return HttpResponse::InternalServerError().json(ErrorResponse::InternalError(
                "Error retrieving application".to_string(),
            ));
        }
    };

    /// Create updated_application based on ApplicationUpdateRequest
    let updated_application = Application {
        id: existing_application.id,
        job_seeker_id: existing_application.job_seeker_id,
        job_id: existing_application.job_id,
        cover_letter: application_update_request.cover_letter.clone(),
        resume: application_update_request.resume.clone(),
        status: application_update_request.status.clone().unwrap_or_else(|| existing_application.status),
        applied_at: existing_application.applied_at,
    };

    match application_db::update(&mut conn, id, updated_application.clone()) {
        Ok(_) => HttpResponse::Ok().json(updated_application),
        Err(e) => {
            error!("Error updating application with ID {}: {:?}", id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Delete an existing application.
///
/// This endpoint requires `api_key` authentication.
///
/// Delete an existing `Application` from the database.
#[utoipa::path(
    context_path = "/v1",
    tag = "applications",
    params(
        ("id" = i64, Path, description = "Unique ID of the application", example = 1)
    ),
    responses(
        (status = 204, description = "Application deleted successfully"),
        (status = 401, description = "Unauthorized to delete application", body = ErrorResponse, example = json!(ErrorResponse::Unauthorized(String::from("Missing API Key")))),
        (status = 404, description = "Application not found", body = ErrorResponse, example = json!(ErrorResponse::NotFound(String::from("Application ID not found")))),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("api_key" = [])
    )
)]
#[delete("/applications/{id}")]
pub async fn delete_application(id: Path<i64>) -> impl Responder {
    let id = id.into_inner();
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "not set".to_string());
    let mut conn = Connection::open(&db_url).unwrap();

    match application_db::delete(&mut conn, id) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            error!("Error deleting application with ID {}: {:?}", id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}