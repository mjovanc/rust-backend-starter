use std::env;
use actix_web::{delete, get, post, put, HttpResponse, Responder};
use actix_web::web::{Data, Json, Path, Query, ServiceConfig};
use chrono::Utc;
use rusqlite::Connection;
use serde::Deserialize;
use log::{error, info};
use crate::db::job;
use crate::models::job::{Job, JobUpdateRequest, EmploymentType};
use crate::models::JobStore;
use crate::utils::{ErrorResponse, PaginationJob};

#[derive(Deserialize)]
pub struct JobQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub(crate) fn configure(store: Data<JobStore>) -> impl FnOnce(&mut ServiceConfig) {
    move |config: &mut ServiceConfig| {
        config
            .app_data(store)
            .service(get_jobs)
            .service(get_job_by_id)
            .service(create_job)
            .service(update_job)
            .service(delete_job);
    }
}

/// Get list of jobs with pagination.
///
/// This endpoint needs `api_key` authentication in order to call.
///
/// List jobs from the database with pagination support.
#[utoipa::path(
    context_path = "/v1",
    tag = "jobs",
    params(
        ("limit" = Option<usize>, Query, description = "Maximum number of items to return", example = 10),
        ("offset" = Option<usize>, Query, description = "Offset for pagination", example = 0),
    ),
    responses(
        (status = 200, description = "List current job items with pagination metadata", body = PaginationJob<Vec<Job>>),
        (status = 401, description = "Unauthorized to get jobs", body = ErrorResponse, example = json!(ErrorResponse::Unauthorized(String::from("Missing API Key")))),
    ),
    security(
        (),
        ("api_key" = [])
    )
)]
#[get("/jobs")]
pub(super) async fn get_jobs(query: Query<JobQuery>) -> impl Responder {
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

    let total_count = job::get_total_count(&mut conn).unwrap_or_else(|e| {
        error!("Error getting total count from the database: {:?}", e);
        0
    });

    match job::get_all(&mut conn, limit, offset) {
        Ok(jobs) => {
            let page = (offset / limit) + 1;
            let pagination = PaginationJob {
                page,
                count: total_count,
                items: jobs,
            };
            HttpResponse::Ok().json(pagination)
        }
        Err(e) => {
            error!("Error getting jobs from the database: {:?}", e);
            HttpResponse::InternalServerError().json(ErrorResponse::InternalError(
                "Error getting jobs from the database".to_string(),
            ))
        }
    }
}

/// Get job by given job id.
///
/// This endpoint needs `api_key` authentication in order to call.
///
/// Return found `Job` with status 200 or 404 not found if `Job` is not found from the database.
#[utoipa::path(
    context_path = "/v1",
    tag = "jobs",
    params(
        ("id", description = "Unique ID of the job", example = 1)
    ),
    responses(
        (status = 200, description = "Job found", body = Job),
        (status = 401, description = "Unauthorized to get job", body = ErrorResponse, example = json!(ErrorResponse::Unauthorized(String::from("missing api key")))),
        (status = 404, description = "Job not found", body = ErrorResponse, example = json!(ErrorResponse::NotFound(String::from("id = 1"))))
    ),
    security(
        (),
        ("api_key" = [])
    )
)]
#[get("/jobs/{id}")]
pub(super) async fn get_job_by_id(id: Path<i64>) -> impl Responder {
    let id = id.into_inner();
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "not set".to_string());
    let mut conn = Connection::open(&db_url).unwrap();

    if let Ok(Some(job)) = job::get_by_id(&mut conn, id) {
        HttpResponse::Ok().json(job)
    } else {
        HttpResponse::NotFound().json(ErrorResponse::NotFound(format!("Job with ID {} not found", id)))
    }
}

/// Create a new job.
///
/// This endpoint needs `api_key` authentication in order to call.
///
/// Create a new `Job` in the database.
#[utoipa::path(
    request_body = Job,
    context_path = "/v1",
    tag = "jobs",
    responses(
        (status = 201, description = "Job created successfully", body = Job),
        (status = 401, description = "Unauthorized to create job", body = ErrorResponse, example = json!(ErrorResponse::Unauthorized(String::from("missing api key")))),
        (status = 400, description = "Invalid job data", body = ErrorResponse, example = json!(ErrorResponse::BadRequest(String::from("Invalid job data"))))
    ),
    security(
        (),
        ("api_key" = [])
    )
)]
#[post("/jobs")]
pub(super) async fn create_job(job: Json<Job>) -> impl Responder {
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

    let job = job.into_inner();

    match job::create(&mut conn, job.clone()) {
        Ok(_) => {
            info!("Job created successfully: {:?}", job);
            HttpResponse::Created().json(job)
        }
        Err(e) => {
            error!("Error creating job: {:?}", e);
            HttpResponse::InternalServerError().json(ErrorResponse::InternalError(
                "Error creating job".to_string(),
            ))
        }
    }
}

/// Update an existing job.
///
/// This endpoint needs `api_key` authentication in order to call.
///
/// Update an existing `Job` in the database.
#[utoipa::path(
    context_path = "/v1",
    tag = "jobs",
    params(
        ("id", description = "Unique ID of the job", example = 1)
    ),
    request_body = JobUpdateRequest,
    responses(
        (status = 200, description = "Job updated successfully", body = Job),
        (status = 401, description = "Unauthorized to update job", body = ErrorResponse, example = json!(ErrorResponse::Unauthorized(String::from("missing api key")))),
        (status = 404, description = "Job not found", body = ErrorResponse, example = json!(ErrorResponse::NotFound(String::from("id = 1")))),
        (status = 400, description = "Invalid job update data", body = ErrorResponse, example = json!(ErrorResponse::BadRequest(String::from("Invalid job update data"))))
    ),
    security(
        (),
        ("api_key" = [])
    )
)]
#[put("/jobs/{id}")]
pub(super) async fn update_job(
    id: Path<i64>,
    job_update_request: Json<JobUpdateRequest>,
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

    // Retrieve the existing job to update
    let existing_job = match job::get_by_id(&mut conn, id) {
        Ok(Some(job)) => job,
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("Error retrieving job with ID {}: {:?}", id, e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let updated_job = Job {
        id: existing_job.id,
        employer_id: existing_job.employer_id,
        title: job_update_request.title.clone().unwrap_or(existing_job.title),
        description: job_update_request.description.clone().unwrap_or(existing_job.description),
        location: job_update_request.location.clone().unwrap_or(existing_job.location),
        salary: Some(job_update_request.salary.clone().unwrap_or(existing_job.salary.unwrap_or_default())),
        employment_type: job_update_request.employment_type.clone().unwrap_or(existing_job.employment_type),
        posted_at: existing_job.posted_at,
        updated_at: Utc::now(),
    };

    match job::update(&mut conn, id, updated_job.clone()) {
        Ok(_) => HttpResponse::Ok().json(updated_job),
        Err(e) => {
            error!("Error updating job with ID {}: {:?}", id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Delete an existing job.
///
/// This endpoint needs `api_key` authentication in order to call.
///
/// Delete an existing `Job` from the database.
#[utoipa::path(
    context_path = "/v1",
    tag = "jobs",
    params(
        ("id", description = "Unique ID of the job", example = 1)
    ),
    responses(
        (status = 204, description = "Job deleted successfully"),
        (status = 401, description = "Unauthorized to delete job", body = ErrorResponse, example = json!(ErrorResponse::Unauthorized(String::from("missing api key")))),
        (status = 404, description = "Job not found", body = ErrorResponse, example = json!(ErrorResponse::NotFound(String::from("id = 1")))),
    ),
    security(
        (),
        ("api_key" = [])
    )
)]
#[delete("/jobs/{id}")]
pub(super) async fn delete_job(id: Path<i64>) -> impl Responder {
    let id = id.into_inner();
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "not set".to_string());
    let mut conn = Connection::open(&db_url).unwrap();

    match job::delete(&mut conn, id) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            error!("Error deleting job with ID {}: {:?}", id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}