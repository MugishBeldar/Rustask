use mongodb::{Collection, results::InsertOneResult};
use futures::stream::TryStreamExt;
use crate::models::sample::Sample;

/// Repository for Sample collection operations
/// This handles all database operations for the Sample model
#[derive(Clone)]
pub struct SampleRepository {
    collection: Collection<Sample>,
}

impl SampleRepository {
    /// Create a new SampleRepository with the given collection
    pub fn new(collection: Collection<Sample>) -> Self {
        Self { collection }
    }

    /// Create a new sample in the database
    pub async fn create(&self, sample: Sample) -> Result<InsertOneResult, String> {
        self.collection
            .insert_one(sample)
            .await
            .map_err(|e| format!("Failed to create sample: {}", e))
    }

    /// Get a sample by ID
    pub async fn get_by_id(&self, id: String) -> Result<Option<Sample>, String> {
        self.collection
            .find_one(mongodb::bson::doc! {"_id": id})
            .await
            .map_err(|e| format!("Failed to get sample: {}", e))
    }

    /// Get all samples
    pub async fn get_all(&self) -> Result<Vec<Sample>, String> {
        let cursor = self.collection
            .find(mongodb::bson::doc! {})
            .await
            .map_err(|e| format!("Failed to get all samples: {}", e))?;

        cursor
            .try_collect()
            .await
            .map_err(|e| format!("Failed to collect samples: {}", e))
    }

    /// Update a sample by ID
    pub async fn update(&self, id: String, sample: Sample) -> Result<Option<Sample>, String> {
        self.collection
            .find_one_and_replace(mongodb::bson::doc! {"_id": id}, sample)
            .await
            .map_err(|e| format!("Failed to update sample: {}", e))
    }

    /// Delete a sample by ID
    pub async fn delete(&self, id: String) -> Result<bool, String> {
        let result = self.collection
            .delete_one(mongodb::bson::doc! {"_id": id})
            .await
            .map_err(|e| format!("Failed to delete sample: {}", e))?;

        Ok(result.deleted_count > 0)
    }
}

