use mongodb::{Client, Database as MongoDatabase, Collection};
use std::env;
use crate::{models::sample::Sample, repositories::sample_repository::SampleRepository};

/// Database connection manager
/// This handles MongoDB connection and provides access to collections and repositories
#[derive(Clone)]
pub struct Database {
    db: MongoDatabase,
}

impl Database {
    /// Initialize MongoDB connection
    pub async fn init() -> Self {
        // Getting mongo db uri from environment variable
        let uri = match env::var("MONGO_URI") {
            Ok(uri) => uri,
            Err(_) => panic!("environment variable MONGO_URI is not set"),
        };

        let database_name = match env::var("MONGO_DB_NAME") {
            Ok(name) => name,
            Err(_) => panic!("environment variable MONGO_DB_NAME is not set"),
        };

        // Creating mongo db client
        let client = match Client::with_uri_str(&uri).await {
            Ok(client) => client,
            Err(e) => panic!("Failed to initialize MongoDB client: {}", e),
        };

        // Selecting database name
        let db = client.database(database_name.as_str());

        Self { db }
    }

    /// Get reference to the MongoDB database
    pub fn get_database(&self) -> &MongoDatabase {
        &self.db
    }

    /// Get Sample collection
    pub fn get_sample_collection(&self) -> Collection<Sample> {
        self.db.collection("sample")
    }

    /// Get Sample repository
    pub fn sample_repository(&self) -> SampleRepository {
        SampleRepository::new(self.get_sample_collection())
    }
}

