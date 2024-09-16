use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// User object
#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub(super) struct User {
    /// Table id for the User.
    #[schema(example = 1)]
    pub id: i64,
    /// Full name of the user.
    #[schema(example = "John Doe")]
    pub name: String,
    /// Email address of the user.
    #[schema(example = "john.doe@example.com")]
    pub email: String,
    /// Hashed password for the user.
    #[schema(example = "hashed_password_here")]
    pub password: String,
    /// Role of the user, either `job_seeker` or `employer`.
    #[schema(example = "job_seeker")]
    pub role: UserRole,
    /// Timestamp of when the user registered.
    #[serde(with = "chrono::serde::ts_seconds")]
    #[serde(rename = "created_at")]
    #[schema(example = "2024-09-16T15:30:00Z")]
    pub created_at: DateTime<Utc>,
    /// Timestamp of the last update to the user profile.
    #[serde(with = "chrono::serde::ts_seconds")]
    #[serde(rename = "updated_at")]
    #[schema(example = "2024-09-16T15:30:00Z")]
    pub updated_at: DateTime<Utc>,
}

/// Request to update existing `User` item.
#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub(super) struct UserUpdateRequest {
    /// Optional new value for the `User` name.
    #[schema(example = "Jane Doe")]
    pub name: Option<String>,
    /// Optional new value for the `User` email.
    #[schema(example = "jane.doe@example.com")]
    pub email: Option<String>,
    /// Optional new value for the `User` password.
    #[schema(example = "new_hashed_password_here")]
    pub password: Option<String>,
    /// Optional new value for the `User` role.
    #[schema(example = "employer")]
    pub role: Option<UserRole>,
}

/// Enum for user roles.
#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub enum UserRole {
    #[schema(rename = "job_seeker")]
    JobSeeker,
    #[schema(rename = "employer")]
    Employer,
}