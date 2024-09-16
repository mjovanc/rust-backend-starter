use std::fmt;
use chrono::{DateTime, Utc};
use rusqlite::ToSql;
use rusqlite::types::{FromSql, FromSqlResult, ToSqlOutput, ValueRef};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Application object
#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct Application {
    /// Unique table id for the Application.
    #[schema(example = 1)]
    pub id: i64,
    /// Foreign key referencing the job seeker who applied.
    #[schema(example = 1)]
    pub job_seeker_id: i64,
    /// Foreign key referencing the job that was applied for.
    #[schema(example = 1)]
    pub job_id: i64,
    /// Optional cover letter provided by the job seeker.
    #[schema(example = "I am very excited about this opportunity.")]
    pub cover_letter: Option<String>,
    /// Link to the resume or file.
    #[schema(example = "https://example.com/resume.pdf")]
    pub resume: Option<String>,
    /// Status of the application.
    #[schema(example = "pending")]
    pub status: ApplicationStatus,
    /// Timestamp of when the application was submitted.
    #[serde(with = "chrono::serde::ts_seconds")]
    #[serde(rename = "applied_at")]
    #[schema(example = "2024-09-16T15:30:00Z")]
    pub applied_at: DateTime<Utc>,
}

/// Request to update existing `Application` item.
#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct ApplicationUpdateRequest {
    /// Optional new value for the `Application` cover_letter.
    #[schema(example = "Updated cover letter here.")]
    pub cover_letter: Option<String>,
    /// Optional new value for the `Application` resume.
    #[schema(example = "https://example.com/updated_resume.pdf")]
    pub resume: Option<String>,
    /// Optional new value for the `Application` status.
    #[schema(example = "reviewed")]
    pub status: Option<ApplicationStatus>,
}

/// Enum for application statuses.
#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub enum ApplicationStatus {
    #[schema(rename = "pending")]
    Pending,
    #[schema(rename = "reviewed")]
    Reviewed,
    #[schema(rename = "accepted")]
    Accepted,
    #[schema(rename = "rejected")]
    Rejected,
}

impl ToSql for ApplicationStatus {
    fn to_sql(&self) -> Result<ToSqlOutput, rusqlite::Error> {
        Ok(ToSqlOutput::from(self.to_string()))
    }
}

impl FromSql for ApplicationStatus {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let s: String = value.as_str()?.to_string();
        match s.as_str() {
            "pending" => Ok(ApplicationStatus::Pending),
            "reviewed" => Ok(ApplicationStatus::Reviewed),
            "accepted" => Ok(ApplicationStatus::Accepted),
            "rejected" => Ok(ApplicationStatus::Rejected),
            _ => Err(rusqlite::types::FromSqlError::InvalidType),
        }
    }
}
impl fmt::Display for ApplicationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_str = match self {
            ApplicationStatus::Pending => "pending",
            ApplicationStatus::Reviewed => "reviewed",
            ApplicationStatus::Accepted => "accepted",
            ApplicationStatus::Rejected => "rejected",
        };
        write!(f, "{}", status_str)
    }
}