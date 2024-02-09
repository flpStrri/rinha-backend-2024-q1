use axum::extract::State;
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use mongodb::{Collection, Database};

use crate::structs::{api, customer};

#[tracing::instrument(name = "Creating a new customer", skip(client, body))]
pub async fn create_customer(
    State(client): State<Database>,
    Json(body): Json<api::CreateCustomerBody>,
) -> impl IntoResponse {
    let customer = customer::Customer {
        id: body.id,
        limit: body.limit,
        balance: 0,
        transactions: vec![],
    };
    let customer_store: Collection<customer::Customer> = client.collection("banking");
    let inserted_result = customer_store.insert_one(&customer, None).await;
    match inserted_result {
        Ok(_) => Ok((
            StatusCode::CREATED,
            [
                (header::LOCATION, format!("/clientes/{}", &body.id)),
                (header::CONTENT_TYPE, String::from("application/json")),
            ],
            Json(api::CustomerBody {
                id: customer.id,
                limit: customer.limit,
            }),
        )),
        Err(error) => {
            println!("post: {}", error);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
