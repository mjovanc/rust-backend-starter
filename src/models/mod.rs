use std::sync::Mutex;

pub mod user;
pub mod job;
pub mod application;

pub use user::User;
pub use user::UserRole;
pub use job::Job;
pub use job::EmploymentType;
pub use application::Application;
pub use application::ApplicationStatus;

/// Store for user-related data
#[derive(Default)]
pub struct UserStore {
    users: Mutex<Vec<User>>,
}

/// Store for job-related data
#[derive(Default)]
pub struct JobStore {
    jobs: Mutex<Vec<Job>>,
}

/// Store for application-related data
#[derive(Default)]
pub struct ApplicationStore {
    applications: Mutex<Vec<Application>>,
}