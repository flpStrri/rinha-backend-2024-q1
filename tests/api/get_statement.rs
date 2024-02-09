use assert_json_diff::assert_json_include;
use reqwest::StatusCode;

#[tokio::test]
async fn returns_200_ok_with_statement_body_when_customer_in_storage() {
    let test_app = crate::helpers::spawn_app().await;
    reqwest::Client::new()
        .post(format!("{}/clientes", test_app.address))
        .json(&serde_json::json!({
            "id": 3,
            "limite" : 30000
        }))
        .send()
        .await
        .expect("failed request");
    reqwest::Client::new()
        .post(format!("{}/clientes/3/transacoes", test_app.address))
        .json(&serde_json::json!({
            "valor": 10000,
            "tipo" : "c",
            "descricao" : "inicial"
        }))
        .send()
        .await
        .expect("failed request");
    reqwest::Client::new()
        .post(format!("{}/clientes/3/transacoes", test_app.address))
        .json(&serde_json::json!({
            "valor": 20000,
            "tipo" : "d",
            "descricao" : "descricao"
        }))
        .send()
        .await
        .expect("failed request");

    let response = reqwest::Client::new()
        .get(format!("{}/clientes/3/extrato", test_app.address))
        .send()
        .await
        .expect("failed request");

    assert_eq!(response.status(), StatusCode::OK);
    let response_body = response
        .json::<std::collections::HashMap<String, serde_json::Value>>()
        .await
        .unwrap();
    assert_eq!(response_body["saldo"]["total"], -10000);
    assert_eq!(response_body["saldo"]["limite"], 30000);
    assert_json_include!(
        actual: response_body["ultimas_transacoes"],
        expected: serde_json::json!([
            {
                "valor": 10000,
                "tipo": "c",
                "descricao": "inicial",
            },
            {
                "valor": 20000,
                "tipo": "d",
                "descricao": "descricao",
            },
        ])
    );
}

#[tokio::test]
async fn returns_200_ok_with_statement_body_with_only_the_last_10_transactions() {
    let test_app = crate::helpers::spawn_app().await;
    reqwest::Client::new()
        .post(format!("{}/clientes", test_app.address))
        .json(&serde_json::json!({
            "id": 3,
            "limite" : 30000
        }))
        .send()
        .await
        .expect("failed request");
    for i in 0..15 {
        reqwest::Client::new()
            .post(format!("{}/clientes/3/transacoes", test_app.address))
            .json(&serde_json::json!({
                "valor": 10000,
                "tipo" : "c",
                "descricao" : format!("inicial {}", i)
            }))
            .send()
            .await
            .expect("failed request");
    }

    let response = reqwest::Client::new()
        .get(format!("{}/clientes/3/extrato", test_app.address))
        .send()
        .await
        .expect("failed request");

    assert_eq!(response.status(), StatusCode::OK);
    let response_body = response
        .json::<std::collections::HashMap<String, serde_json::Value>>()
        .await
        .unwrap();
    assert_eq!(response_body["saldo"]["total"], 150000);
    assert_eq!(response_body["saldo"]["limite"], 30000);
    assert_json_include!(
        actual: response_body["ultimas_transacoes"],
        expected: serde_json::json!([
            {
                "valor": 10000,
                "tipo": "c",
                "descricao": "inicial 5",
            },
            {
                "valor": 10000,
                "tipo": "c",
                "descricao": "inicial 6",
            },
            {
                "valor": 10000,
                "tipo": "c",
                "descricao": "inicial 7",
            },
            {
                "valor": 10000,
                "tipo": "c",
                "descricao": "inicial 8",
            },
            {
                "valor": 10000,
                "tipo": "c",
                "descricao": "inicial 9",
            },
            {
                "valor": 10000,
                "tipo": "c",
                "descricao": "inicial 10",
            },
            {
                "valor": 10000,
                "tipo": "c",
                "descricao": "inicial 11",
            },
            {
                "valor": 10000,
                "tipo": "c",
                "descricao": "inicial 12",
            },
            {
                "valor": 10000,
                "tipo": "c",
                "descricao": "inicial 13",
            },
            {
                "valor": 10000,
                "tipo": "c",
                "descricao": "inicial 14",
            },
        ])
    )
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
