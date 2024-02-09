use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
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

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateTransactionBody {
    #[serde(rename(serialize = "descricao", deserialize = "descricao"))]
    #[validate(length(min = 1, max = 10))]
    pub description: String,
    #[serde(rename(serialize = "tipo", deserialize = "tipo"))]
    pub action: transaction::Action,
    #[serde(rename(serialize = "valor", deserialize = "valor"))]
    #[validate(range(min = 1))]
    pub value: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BalanceBody {
    #[serde(rename(serialize = "limite", deserialize = "limite"))]
    pub limit: i64,
    #[serde(rename(serialize = "saldo", deserialize = "saldo"))]
    pub balance: i64,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct TransactionBody {
    #[serde(rename(serialize = "descricao", deserialize = "descricao"))]
    #[validate(length(min = 1, max = 10))]
    pub description: String,
    #[serde(rename(serialize = "tipo", deserialize = "tipo"))]
    pub action: transaction::Action,
    #[serde(rename(serialize = "valor", deserialize = "valor"))]
    #[validate(range(min = 1))]
    pub value: i64,
    #[serde(rename(serialize = "realizada_em", deserialize = "realizada_em"))]
    #[serde(with = "time::serde::iso8601")]
    pub timestamp: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FullBalanceBody {
    #[serde(rename(serialize = "limite", deserialize = "limite"))]
    pub limit: i64,
    #[serde(rename(serialize = "total", deserialize = "total"))]
    pub balance: i64,
    #[serde(rename(serialize = "data_extrato", deserialize = "data_extrato"))]
    #[serde(with = "time::serde::iso8601")]
    pub timestamp: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatementBody {
    #[serde(rename(serialize = "saldo", deserialize = "saldo"))]
    pub balance: FullBalanceBody,
    #[serde(rename(serialize = "ultimas_transacoes", deserialize = "ultimas_transacoes"))]
    pub transactions: Vec<TransactionBody>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateCustomerBody {
    pub id: u32,
    #[serde(rename(serialize = "limite", deserialize = "limite"))]
    pub limit: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerBody {
    pub id: u32,
    #[serde(rename(serialize = "limite", deserialize = "limite"))]
    pub limit: i64,
}
