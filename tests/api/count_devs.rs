use reqwest::StatusCode;

#[tokio::test]
async fn returns_200_ok_with_0_when_storage_is_empty() {
    let test_app = crate::helpers::spawn_app().await;

    let response = reqwest::Client::new()
        .get(format!("{}/contagem-pessoas", &test_app.address))
        .send()
        .await
        .expect("failed request");

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.text().await.unwrap(), "0");
}

#[tokio::test]
async fn returns_200_ok_with_1_when_storage_got_one_dev() {
    let test_app = crate::helpers::spawn_app().await;
    reqwest::Client::new()
        .post(format!("{}/pessoas", test_app.address))
        .json(&serde_json::json!({
            "apelido": "foo",
            "nome": "bar",
            "nascimento": "2020-12-03",
            "stack": ["Rust", "Python"]
        }))
        .send()
        .await
        .expect("failed request");

    let response = reqwest::Client::new()
        .get(format!("{}/contagem-pessoas", &test_app.address))
        .send()
        .await
        .expect("failed request");

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.text().await.unwrap(), "1");
}
