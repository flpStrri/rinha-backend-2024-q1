use serde::{Deserialize, Serialize};

use crate::structs::transaction::Transaction;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Customer {
    #[serde(rename(serialize = "_id", deserialize = "_id"))]
    pub id: u32,
    pub limit: i64,
    pub balance: i64,
    pub transactions: Vec<Transaction>,
}
