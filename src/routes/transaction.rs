use axum::extract::{Path, State};
use axum::http::header;
use axum::{http::StatusCode, response::IntoResponse, Json};
use axum_valid::Valid;
use mongodb::options::FindOneAndUpdateOptions;
use mongodb::{bson::doc, Collection, Database};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;
use tracing::error;

use crate::structs::transaction::Action;
use crate::structs::{api, customer};

#[tracing::instrument(name = "Creating a new transaction", skip(client, body))]
pub async fn create_transaction(
    State(client): State<Database>,
    Path(id): Path<u32>,
    body: Valid<Json<api::CreateTransactionBody>>,
) -> impl IntoResponse {
    let transaction_timestamp = OffsetDateTime::now_utc();
    let customer_store: Collection<customer::Customer> = client.collection("banking");
    let transaction_value = body.value * body.action.multiplier();
    let transaction_action = format!("{}", body.action);

    let update = doc! {
        "$inc": {
            "balance": transaction_value,
        },
        "$push": {
            "transactions": {
                "description": &body.description,
                "timestamp": transaction_timestamp.format(&Rfc3339).unwrap(),
                "value": body.value,
                "action": transaction_action,
            },
        }
    };
    let options = FindOneAndUpdateOptions::builder()
        .return_document(mongodb::options::ReturnDocument::After)
        .build();
    let filter = doc! { "_id": id};
    let updated_customer_account = customer_store
        .find_one_and_update(filter, update, options)
        .await;

    match updated_customer_account {
        Ok(Some(customer)) => Ok((
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json")],
            Json(api::BalanceBody {
                balance: customer.balance,
                limit: -customer.limit,
            }),
        )),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(error) => {
            dbg!(&error);
            error!("unexpected error on query: {}", error);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
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
