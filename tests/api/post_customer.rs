use mongodb::bson::doc;
use mongodb::Collection;
use reqwest::header::LOCATION;
use reqwest::StatusCode;
use rinha_backend_2023_q3::structs::customer;

#[tokio::test]
async fn returns_200_with_customer_body_given_a_valid_request() {
    let test_app = crate::helpers::spawn_app().await;
    let customer_store: Collection<customer::Customer> =
        test_app.mongodb_pool.collection("banking");

    let response = reqwest::Client::new()
        .post(format!("{}/clientes", test_app.address))
        .json(&serde_json::json!({
            "id": 1,
            "limite" : 2000,
        }))
        .send()
        .await
        .expect("failed request");

    assert_eq!(response.status(), StatusCode::CREATED);
    assert!(response
        .headers()
        .get(LOCATION)
        .expect("header not found")
        .to_str()
        .expect("not ASCII value")
        .eq("/clientes/1"));

    let response_body = response
        .json::<std::collections::HashMap<String, serde_json::Value>>()
        .await
        .unwrap();
    assert_eq!(response_body["id"], 1);
    assert_eq!(response_body["limite"], 2000);

    let inserted_result = customer_store
        .find_one(doc! {"_id": 1}, None)
        .await
        .expect("successfull query")
        .expect("found customer");
    assert_eq!(inserted_result.id, 1)
}
