use assert_json_diff::assert_json_include;
use reqwest::StatusCode;

#[tokio::test]
async fn returns_dev_body_when_in_storage() {
    let test_app = crate::helpers::spawn_app().await;
    let response = reqwest::Client::new()
        .get(format!("{}/clientes/1/extrato", test_app.address))
        .send()
        .await
        .expect("failed request");

    assert_eq!(response.status(), StatusCode::OK);
    let response_body = response
        .json::<std::collections::HashMap<String, serde_json::Value>>()
        .await
        .unwrap();
    assert_eq!(response_body["saldo"]["total"], -9098);
    assert_eq!(response_body["saldo"]["limite"], 100000);
    assert_json_include!(
        actual: response_body["ultimas_transacoes"],
        expected: serde_json::json!([{
          "valor": 10,
          "tipo": "c",
          "descricao": "whatever",
        }])
    );
}

#[tokio::test]
async fn returns_404_not_found_when_customer_not_in_storage() {
    let test_app = crate::helpers::spawn_app().await;

    let response = reqwest::Client::new()
        .get(format!("{}/clientes/6/extrato", test_app.address))
        .send()
        .await
        .expect("failed request");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
