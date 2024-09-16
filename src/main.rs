mod models;
mod db;
mod routes;
mod utils;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;
use crate::utils::init_db::initialize_database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    match initialize_database() {
        Ok(()) => println!("Database initialized successfully."),
        Err(err) => eprintln!("Failed to initialize the database: {}", err),
    }

    /*#[derive(OpenApi)]
    #[openapi(
        info(title = "Job Board API",
            description = "Job Board API. Please contact support directly if something is unclear or not working as intended.",
            contact(
                name = "Support",
                email = "mjovanc@icloud.com"
            )),
        paths(
            product::get_products,
            product::get_product_by_id,
        ),
        components(
            schemas(product::Product, crate::util::ErrorResponse, crate::util::Pagination<Product>)
        ),
        tags(
            (name = "product", description = "Product management endpoints.")
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
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("stenexpo"))),
            )
        }
    }

    let store = Data::new(ProductStore::default());
    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec!["Accept", "Content-Type", "stenexpo"])
            .supports_credentials()
            .max_age(3600);

        // Swagger will only work using the --release flag otherwise we will get 404
        let app = App::new()
            .wrap(Logger::default())
            .configure(product::configure(store.clone()))
            .wrap(cors)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            );

        app
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await*/

    Ok(())
}