use serde::{Deserialize, Serialize};
use std::fmt;
use time::OffsetDateTime;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Action {
    #[serde(rename = "c")]
    Credit,
    #[serde(rename = "d")]
    Debt,
}

impl Action {
    pub fn multiplier(&self) -> i64 {
        match self {
            Action::Credit => 1,
            Action::Debt => -1,
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::Credit => write!(f, "c"),
            Action::Debt => write!(f, "d"),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transaction {
    pub description: String,
    pub action: Action,
    pub value: i64,
    #[serde(with = "time::serde::rfc3339")]
    pub timestamp: OffsetDateTime,
}
