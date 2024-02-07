use axum::extract::{Path, State};
use axum::{http::StatusCode, response::IntoResponse, Json};
use axum_valid::Valid;
use mongodb::Database;

use crate::structs::api;

#[tracing::instrument(name = "Creating a new transaction", skip(_client, _body))]
pub async fn create_transaction(
    State(_client): State<Database>,
    Path(id): Path<u64>,
    _body: Valid<Json<api::Transaction>>,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(api::Balance {
            balance: 123,
            limit: 456,
        }),
    )
}
