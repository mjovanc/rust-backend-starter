/*use crate::util::ErrorResponse;
use crate::{API_KEY, API_KEY_NAME};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::HttpResponse;
use futures::future::LocalBoxFuture;
use log::info;
use std::future;
use std::future::Ready;
use crate::utils::ErrorResponse;

/// Require api key middleware will actually require valid api key
pub struct RequireApiKey;

impl<S> Transform<S, ServiceRequest> for RequireApiKey
where
    S: Service<
        ServiceRequest,
        Response = ServiceResponse<actix_web::body::BoxBody>,
        Error = actix_web::Error,
    >,
    S::Future: 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = actix_web::Error;
    type Transform = ApiKeyMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ready(Ok(ApiKeyMiddleware {
            service,
            log_only: false,
        }))
    }
}

/// Log api key middleware only logs about missing or invalid api keys
pub struct LogApiKey;

impl<S> Transform<S, ServiceRequest> for LogApiKey
where
    S: Service<
        ServiceRequest,
        Response = ServiceResponse<actix_web::body::BoxBody>,
        Error = actix_web::Error,
    >,
    S::Future: 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = actix_web::Error;
    type Transform = ApiKeyMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ready(Ok(ApiKeyMiddleware {
            service,
            log_only: true,
        }))
    }
}

pub struct ApiKeyMiddleware<S> {
    pub(crate) service: S,
    pub(crate) log_only: bool,
}

impl<S> Service<ServiceRequest> for ApiKeyMiddleware<S>
where
    S: Service<
        ServiceRequest,
        Response = ServiceResponse<actix_web::body::BoxBody>,
        Error = actix_web::Error,
    >,
    S::Future: 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, actix_web::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut core::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let response = |req: ServiceRequest, response: HttpResponse| -> Self::Future {
            Box::pin(async { Ok(req.into_response(response)) })
        };

        // Log the API key provided
        if let Some(key) = req.headers().get(API_KEY_NAME) {
            log::debug!("Received API key: {:?}", key.to_str());
        } else {
            log::info!("API key missing in request");
        }

        // MATCH HERE AGAINST DIFFERENT API KEYS
        match req.headers().get(API_KEY_NAME) {
            Some(key) if key.to_str().unwrap_or("") != API_KEY => {
                if self.log_only {
                    log::debug!("Incorrect API Key Provided!")
                } else {
                    return response(
                        req,
                        HttpResponse::Unauthorized().json(ErrorResponse::Unauthorized(
                            String::from("Incorrect API Key!"),
                        )),
                    );
                }
            }
            None => {
                if self.log_only {
                    log::debug!("Missing api key!!!")
                } else {
                    return response(
                        req,
                        HttpResponse::Unauthorized().json(ErrorResponse::Unauthorized(
                            String::from("Missing API Key!"),
                        )),
                    );
                }
            }
            _ => (), // just passthrough
        }

        if self.log_only {
            log::debug!("Performing operation")
        }

        let future = self.service.call(req);

        Box::pin(async move {
            let response = future.await?;

            Ok(response)
        })
    }
}*/