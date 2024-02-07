use reqwest::StatusCode;

#[tokio::test]
async fn returns_200_with_balance_body_given_a_valid_request() {
    let test_app = crate::helpers::spawn_app().await;

    let response = reqwest::Client::new()
        .post(format!("{}/clientes/1/transacoes", test_app.address))
        .json(&serde_json::json!({
            "valor": 1000,
            "tipo" : "d",
            "descricao" : "descricao"
        }))
        .send()
        .await
        .expect("failed request");

    assert_eq!(response.status(), StatusCode::OK);
    let response_body = response
        .json::<std::collections::HashMap<String, serde_json::Value>>()
        .await
        .unwrap();
    assert_eq!(response_body["saldo"], 123);
    assert_eq!(response_body["limite"], 456);
}

#[tokio::test]
async fn returns_422_with_error_body_given_a_too_long_description() {
    let test_app = crate::helpers::spawn_app().await;

    let response = reqwest::Client::new()
        .post(format!("{}/clientes/1/transacoes", test_app.address))
        .json(&serde_json::json!({
            "valor": 1000,
            "tipo" : "d",
            "descricao" : "some important money that I must send you!"
        }))
        .send()
        .await
        .expect("failed request");

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn returns_422_with_error_body_given_a_too_short_description() {
    let test_app = crate::helpers::spawn_app().await;

    let response = reqwest::Client::new()
        .post(format!("{}/clientes/1/transacoes", test_app.address))
        .json(&serde_json::json!({
            "valor": 1000,
            "tipo" : "d",
            "descricao" : ""
        }))
        .send()
        .await
        .expect("failed request");

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}
