use mongodb::results::InsertOneResult;
use crate::{models::sample::Sample, repositories::sample_repository::SampleRepository};

/// Use case layer for Sample operations
/// This layer contains the application's business logic and use cases
#[derive(Clone)]
pub struct SampleUseCase {
    repository: SampleRepository,
}

impl SampleUseCase {
    /// Create a new SampleUseCase with the given repository
    pub fn new(repository: SampleRepository) -> Self {
        Self { repository }
    }

    /// Use case: Create a new sample
    pub async fn create_sample(&self, sample: Sample) -> Result<InsertOneResult, String> {
        // Add any business logic here (validation, transformation, etc.)
        // Example: Validate sample name is not empty
        // Example: Check user permissions
        // Example: Send notifications
        self.repository.create(sample).await
    }

    /// Use case: Get a sample by ID
    pub async fn get_sample(&self, id: String) -> Result<Sample, String> {
        match self.repository.get_by_id(id).await? {
            Some(sample) => Ok(sample),
            None => Err("Sample not found".to_string()),
        }
    }

    /// Use case: Get all samples
    pub async fn get_all_samples(&self) -> Result<Vec<Sample>, String> {
        self.repository.get_all().await
    }

    /// Use case: Update a sample
    pub async fn update_sample(&self, id: String, sample: Sample) -> Result<Sample, String> {
        match self.repository.update(id, sample).await? {
            Some(updated) => Ok(updated),
            None => Err("Sample not found".to_string()),
        }
    }

    /// Use case: Delete a sample
    pub async fn delete_sample(&self, id: String) -> Result<(), String> {
        let deleted = self.repository.delete(id).await?;
        if deleted {
            Ok(())
        } else {
            Err("Sample not found".to_string())
        }
    }
}

