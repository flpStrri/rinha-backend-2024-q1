use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Person {
    #[serde(rename(serialize = "_id", deserialize = "_id"))]
    #[serde(with = "mongodb::bson::serde_helpers::uuid_1_as_binary")]
    pub id: Uuid,
    pub nickname: String,
    pub name: String,
    pub birth_date: NaiveDate,
    pub stacks: Option<Vec<String>>,
}
