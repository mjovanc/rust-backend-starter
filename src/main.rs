mod models;
mod db;
mod routes;
mod utils;
mod auth;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;
use crate::models::{ApplicationStore, JobStore, UserStore};
use crate::utils::init_db::initialize_database;
use crate::utils::{PaginationUser, PaginationJob, PaginationApplication, ErrorResponse};
use crate::models::{User, Job, Application, UserRole, EmploymentType, ApplicationStatus};
use crate::routes::{user, job, application};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    match initialize_database() {
        Ok(()) => println!("Database initialized successfully."),
        Err(err) => eprintln!("Failed to initialize the database: {}", err),
    }

    #[derive(OpenApi)]
    #[openapi(
        info(title = "Job Board API",
            description = "The Job Board API provides endpoints for managing job postings, user applications, and user profiles. It allows employers to create and manage job listings, while job seekers can apply for jobs, view their applications, and update their profiles. The API includes functionalities for authentication and role management, ensuring secure and efficient interactions between job seekers and employers.",
            version = "1.0.0",
            contact(
                name = "Support",
                email = "info@example.com"
            )),
        paths(
            user::get_users,
            user::get_user_by_id,
            user::create_user,
            user::update_user,
            user::delete_user,
            job::get_jobs,
            job::get_job_by_id,
            job::create_job,
            job::update_job,
            job::delete_job,
            application::get_applications,
            application::get_application_by_id,
            application::create_application,
            application::update_application,
            application::delete_application,
        ),
        components(
            schemas(
                User,
                UserRole,
                Job,
                EmploymentType,
                Application,
                ApplicationStatus,
                PaginationUser,
                PaginationJob,
                PaginationApplication,
                ErrorResponse
            )
        ),
        tags(
            (name = "users", description = "User endpoints."),
            (name = "jobs", description = "Job endpoints."),
            (name = "applications", description = "Application endpoints.")
        ),
        modifiers(&SecurityAddon)
    )]
    struct ApiDoc;

    struct SecurityAddon;

    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            let components = openapi.components.as_mut().unwrap();
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
            )
        }
    }

    let user_store = Data::new(UserStore::default());
    let job_store = Data::new(JobStore::default());
    let application_store = Data::new(ApplicationStore::default());

    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin() // Change this if you don't want to allow any origin to access the API
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec!["Accept", "Content-Type", "Authorization"])
            .supports_credentials()
            .max_age(3600);

        let app = App::new()
            .wrap(Logger::default())
            .app_data(user_store.clone())
            .app_data(job_store.clone())
            .app_data(application_store.clone())
            .wrap(cors)
            .configure(|cfg| {
                cfg.service(web::scope("/v1")
                    .configure(|scope| {
                        user::configure(user_store.clone())(scope);
                    }));
            })
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            );

        app
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}