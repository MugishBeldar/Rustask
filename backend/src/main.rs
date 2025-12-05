use actix_web::{App, HttpResponse, HttpServer, Responder, get, web::Data};
use rustask_api::{
    AppState,
    routes::configure_routes,
    config::{database::Database, logger},
    usecases::sample_usecase::SampleUseCase,
};
use tracing_actix_web::TracingLogger;

#[get("/")]
async fn hello() -> impl Responder {
    tracing::info!("Hello endpoint called");
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize logger
    logger::init_logger();
    tracing::info!("Starting Rustask API server...");

    // Initialize database connection
    let db = Database::init().await;
    tracing::info!("Database connection established");

    // Initialize use cases
    let sample_usecase = SampleUseCase::new(db.sample_repository());
    tracing::info!("Use cases initialized");

    // Create app state
    let app_data = Data::new(AppState { sample_usecase });

    let server_address = "localhost:5001";
    tracing::info!("Starting HTTP server on {}", server_address);

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())  // Add request logging middleware
            .app_data(app_data.clone())
            .service(hello)
            .configure(configure_routes)
    })
    .bind(server_address)?
    .run()
    .await
}
