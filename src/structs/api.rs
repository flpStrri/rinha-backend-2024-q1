use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use validator::Validate;

use crate::structs::transaction;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CreatePersonBody {
    #[serde(rename(deserialize = "apelido", serialize = "apelido"))]
    pub nickname: String,
    #[serde(rename(deserialize = "nome", serialize = "nome"))]
    pub name: String,
    #[serde(rename(deserialize = "nascimento", serialize = "nascimento"))]
    pub birth_date: NaiveDate,
    #[serde(rename(deserialize = "stack", serialize = "stack"))]
    pub stacks: Option<Vec<String>>,
}

#[derive(Debug, Default, Deserialize)]
pub struct SearchPersonQuery {
    #[serde(rename(deserialize = "t"))]
    pub search_term: String,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct PersonBody {
    #[serde(with = "mongodb::bson::serde_helpers::uuid_1_as_binary")]
    pub id: Uuid,
    #[serde(rename(serialize = "apelido", deserialize = "apelido"))]
    pub nickname: String,
    #[serde(rename(serialize = "nome", deserialize = "nome"))]
    pub name: String,
    #[serde(rename(serialize = "nascimento", deserialize = "nascimento"))]
    pub birth_date: NaiveDate,
    #[serde(rename(serialize = "stack", deserialize = "stack"))]
    pub stacks: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Validate, PartialEq)]
pub struct Transaction {
    #[serde(rename(serialize = "valor", deserialize = "valor"))]
    pub value: u64,
    #[serde(rename(serialize = "descricao", deserialize = "descricao"))]
    #[validate(length(min = 1, max = 10))]
    pub description: String,
    #[serde(rename(serialize = "tipo", deserialize = "tipo"))]
    pub action: transaction::Action,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Balance {
    #[serde(rename(serialize = "limite", deserialize = "limite"))]
    pub limit: u64,
    #[serde(rename(serialize = "saldo", deserialize = "saldo"))]
    pub balance: i64,
}
