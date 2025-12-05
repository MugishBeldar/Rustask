pub mod sample_routes;

use actix_web::web::ServiceConfig;

/// Configure all application routes
/// This is the central place to register all route modules
pub fn configure_routes(cfg: &mut ServiceConfig) {
    // API v1 routes
    cfg.service(
        actix_web::web::scope("/api/v1")
            .configure(sample_routes::configure_sample_routes)
            // Add more route modules here as you create them:
            // .configure(user_routes::configure_user_routes)
            // .configure(auth_routes::configure_auth_routes)
    );
}