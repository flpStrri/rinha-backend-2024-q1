use axum::extract::{Path, State};
use axum::http::header;
use axum::{http::StatusCode, response::IntoResponse, Json};
use axum_valid::Valid;
use mongodb;
use mongodb::options::{FindOneAndUpdateOptions, FindOneOptions};
use mongodb::{bson::doc, Collection, Database};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;
use tracing::{error, info};

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
            match error.kind.as_ref() {
                mongodb::error::ErrorKind::Command(mongodb::error::CommandError {
                    code: 121,
                    ..
                }) => {
                    info!("transaction failed: customer {} has insufficient funds", id);
                    return Err(StatusCode::UNPROCESSABLE_ENTITY);
                }
                _ => {
                    error!("unexpected error on query: {}", error);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
    }
}

#[tracing::instrument(name = "Looking for a customer statement", skip(client))]
pub async fn get_statement(
    State(client): State<Database>,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    let customer_store: Collection<customer::Customer> = client.collection("banking");
    let projection = doc! {
        "transactions": { "$slice":  -10 }
    };
    let options = FindOneOptions::builder()
        .projection(Some(projection))
        .build();
    let queried_customer = customer_store.find_one(doc! {"_id": id}, options).await;

    match queried_customer {
        Ok(Some(customer)) => Ok((
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json")],
            Json(api::StatementBody {
                balance: api::FullBalanceBody {
                    balance: customer.balance,
                    limit: -customer.limit,
                    timestamp: OffsetDateTime::now_utc(),
                },
                transactions: customer
                    .transactions
                    .iter()
                    .map(|transaction| api::TransactionBody {
                        value: transaction.value,
                        action: transaction.action.clone(),
                        description: transaction.description.clone(),
                        timestamp: transaction.timestamp,
                    })
                    .collect(),
            }),
        )),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(error) => {
            println!("get_by_id: {}", error);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
