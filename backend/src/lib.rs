pub mod config;
pub mod models;
pub mod usecases;
pub mod routes;
pub mod repositories;

use usecases::sample_usecase::SampleUseCase;

pub struct AppState {
    pub sample_usecase: SampleUseCase,
}