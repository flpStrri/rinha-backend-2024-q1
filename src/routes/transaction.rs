use axum::extract::{Path, State};
use axum::http::header;
use axum::{http::StatusCode, response::IntoResponse, Json};
use axum_valid::Valid;
use mongodb::Database;
use time::OffsetDateTime;

use crate::structs::api;
use crate::structs::transaction::Action;

#[tracing::instrument(name = "Creating a new transaction", skip(_client, _body))]
pub async fn create_transaction(
    State(_client): State<Database>,
    Path(id): Path<u64>,
    _body: Valid<Json<api::CreateTransactionBody>>,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json")],
        Json(api::BalanceBody {
            balance: 123,
            limit: 456,
        }),
    )
}

#[tracing::instrument(name = "Looking for a customer statement", skip(_client))]
pub async fn get_statement(
    State(_client): State<Database>,
    Path(id): Path<u64>,
) -> impl IntoResponse {
    match id {
        6 => Err(StatusCode::NOT_FOUND),
        _ => Ok((
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json")],
            Json(api::StatementBody {
                balance: api::FullBalanceBody {
                    balance: -9098,
                    limit: 100000,
                    timestamp: OffsetDateTime::now_utc(),
                },
                transactions: Vec::from([api::TransactionBody {
                    description: String::from("whatever"),
                    value: 10,
                    action: Action::Credit,
                    timestamp: OffsetDateTime::now_utc(),
                }]),
            }),
        )),
    }
}
