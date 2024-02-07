use crate::structs::person;
use axum::extract::State;
use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
};
use mongodb::{Collection, Database};

pub async fn count_persons(State(client): State<Database>) -> impl IntoResponse {
    let devs_store: Collection<person::Person> = client.collection("devs");
    let found_dev = devs_store.count_documents(None, None).await;

    match found_dev {
        Ok(count) => Ok((
            StatusCode::OK,
            [(header::CONTENT_TYPE, String::from("text/plain"))],
            format!("{}", count),
        )),
        Err(error) => {
            println!("count: {}", error);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
