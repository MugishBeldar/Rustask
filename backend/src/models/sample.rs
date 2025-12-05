use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Sample {
    pub _id: ObjectId,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SampleRequest {
    pub name: String,
}

impl TryFrom<SampleRequest> for Sample {
    type Error = String;

    fn try_from(value: SampleRequest) -> Result<Self, Self::Error> {
        Ok(Sample {
            _id: ObjectId::new(),
            name: value.name,
        })
    }
}
 