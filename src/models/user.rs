use std::fmt;
use chrono::{DateTime, Utc};
use rusqlite::{Error, ToSql};
use rusqlite::types::{FromSql, FromSqlResult, ToSqlOutput, ValueRef};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// User object
#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct User {
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
pub struct UserUpdateRequest {
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

impl ToSql for UserRole {
    fn to_sql(&self) -> Result<ToSqlOutput, Error> {
        Ok(ToSqlOutput::from(self.to_string()))
    }
}

impl FromSql for UserRole {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let s: String = value.as_str()?.to_string();
        match s.as_str() {
            "job_seeker" => Ok(UserRole::JobSeeker),
            "employer" => Ok(UserRole::Employer),
            _ => Err(rusqlite::types::FromSqlError::InvalidType),
        }
    }
}

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let role_str = match self {
            UserRole::JobSeeker => "job_seeker",
            UserRole::Employer => "employer",
        };
        write!(f, "{}", role_str)
    }
}