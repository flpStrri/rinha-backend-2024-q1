use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Action {
    #[serde(rename = "c")]
    Credit,
    #[serde(rename = "d")]
    Debt,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Transaction {
    #[serde(rename(serialize = "_id", deserialize = "_id"))]
    #[serde(with = "mongodb::bson::serde_helpers::uuid_1_as_binary")]
    pub id: Uuid,
    pub description: String,
    pub action: Action,
    pub value: i64,
    pub timestamp: OffsetDateTime,
}
