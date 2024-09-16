use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Job object
#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub(super) struct Job {
    /// Unique table id for the Job.
    #[schema(example = 1)]
    pub id: i64,
    /// Foreign key referencing the employer who posted the job.
    #[schema(example = 1)]
    pub employer_id: i64,
    /// Title of the job.
    #[schema(example = "Software Engineer")]
    pub title: String,
    /// Detailed job description.
    #[schema(example = "Responsible for developing and maintaining software applications.")]
    pub description: String,
    /// Location of the job.
    #[schema(example = "San Francisco, CA")]
    pub location: String,
    /// Salary or pay range for the job.
    #[schema(example = "$120,000 - $150,000")]
    pub salary: Option<String>,
    /// Type of employment.
    #[schema(example = "full_time")]
    pub employment_type: EmploymentType,
    /// Timestamp of when the job was posted.
    #[serde(with = "chrono::serde::ts_seconds")]
    #[serde(rename = "posted_at")]
    #[schema(example = "2024-09-16T15:30:00Z")]
    pub posted_at: DateTime<Utc>,
    /// Timestamp of the last update to the job posting.
    #[serde(with = "chrono::serde::ts_seconds")]
    #[serde(rename = "updated_at")]
    #[schema(example = "2024-09-16T15:30:00Z")]
    pub updated_at: DateTime<Utc>,
}

/// Request to update existing `Job` item.
#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub(super) struct JobUpdateRequest {
    /// Optional new value for the `Job` title.
    #[schema(example = "Senior Software Engineer")]
    pub title: Option<String>,
    /// Optional new value for the `Job` description.
    #[schema(example = "Responsible for leading software development projects.")]
    pub description: Option<String>,
    /// Optional new value for the `Job` location.
    #[schema(example = "New York, NY")]
    pub location: Option<String>,
    /// Optional new value for the `Job` salary.
    #[schema(example = "$130,000 - $160,000")]
    pub salary: Option<String>,
    /// Optional new value for the `Job` employment_type.
    #[schema(example = "contract")]
    pub employment_type: Option<EmploymentType>,
}

/// Enum for employment types.
#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub enum EmploymentType {
    #[schema(rename = "full_time")]
    FullTime,
    #[schema(rename = "part_time")]
    PartTime,
    #[schema(rename = "contract")]
    Contract,
}