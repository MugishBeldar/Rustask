use actix_web::{HttpResponse, post, web::{Data, Json, ServiceConfig}};

use crate::{models::sample::{Sample, SampleRequest}, AppState};

#[post("/sample")]
pub async fn create_sample(app_state: Data<AppState>, request: Json<SampleRequest>) -> HttpResponse {
    match Sample::try_from(request.0) {
        Ok(sample) => match app_state.sample_usecase.create_sample(sample).await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(e) => HttpResponse::InternalServerError().body(e),
        },
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}

/// Configure all sample-related routes
pub fn configure_sample_routes(cfg: &mut ServiceConfig) {
    cfg.service(create_sample);
    // Add more sample routes here as you create them:
    // cfg.service(get_sample);
    // cfg.service(get_all_samples);
    // cfg.service(update_sample);
    // cfg.service(delete_sample);
}
