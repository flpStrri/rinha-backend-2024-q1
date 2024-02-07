use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Action {
    #[serde(rename = "c")]
    Credit,
    #[serde(rename = "d")]
    Debt,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Transaction {
    pub action: Action,
    pub description: String,
    pub value: i64,
}
