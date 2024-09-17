use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub mod init_db;

/// Pagination metadata
#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct Pagination<T> {
    pub page: i64,
    pub count: i64,
    pub items: Vec<T>,
}

/// API endpoint error responses
#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub enum ErrorResponse {
    /// When the resource is not found (e.g., user, job, application).
    NotFound(String),
    /// When there is a conflict in the request (e.g., conflicting data).
    Conflict(String),
    /// When the request is unauthorized due to missing or invalid credentials.
    Unauthorized(String),
    /// When there is an internal server error or an unexpected condition.
    InternalError(String),
    /// When the request is bad due to incorrect or missing parameters.
    BadRequest(String),
    /// When an operation is not allowed or is forbidden.
    Forbidden(String),
    /// When a requested resource already exists.
    AlreadyExists(String),
}
