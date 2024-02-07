use reqwest::header::LOCATION;
use reqwest::StatusCode;

#[tokio::test]
async fn returns_200_with_dev_body_given_a_valid_body() {
    let test_app = crate::helpers::spawn_app().await;

    let response = reqwest::Client::new()
        .post(format!("{}/pessoas", test_app.address))
        .json(&serde_json::json!({
            "apelido": "foo",
            "nome": "bye",
            "nascimento": "1992-11-23",
            "stack": ["Rust", "Ruby"]
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
        .starts_with("/pessoas/"));
    let response_body = response
        .json::<std::collections::HashMap<String, serde_json::Value>>()
        .await
        .unwrap();
    assert_eq!(response_body["apelido"], String::from("foo"));
    assert_eq!(response_body["nome"], String::from("bye"));
    assert_eq!(response_body["nascimento"], String::from("1992-11-23"));
    assert_eq!(
        response_body["stack"],
        serde_json::json!([String::from("Rust"), String::from("Ruby")])
    );
}

#[tokio::test]
async fn returns_200_with_dev_body_given_a_valid_stackless_body() {
    let test_app = crate::helpers::spawn_app().await;

    let response = reqwest::Client::new()
        .post(format!("{}/pessoas", test_app.address))
        .json(&serde_json::json!({
            "apelido": "foo",
            "nome": "bye",
            "nascimento": "1992-11-23",
            // "stack": ["Rust", "Ruby"]
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
        .starts_with("/pessoas/"));
    let response_body = response
        .json::<std::collections::HashMap<String, serde_json::Value>>()
        .await
        .unwrap();
    assert_eq!(response_body["apelido"], String::from("foo"));
    assert_eq!(response_body["nome"], String::from("bye"));
    assert_eq!(response_body["nascimento"], String::from("1992-11-23"));
    assert_eq!(response_body["stack"], serde_json::Value::Null);
}

#[tokio::test]
async fn returns_422_unprocessable_entity_when_missing_name() {
    let test_app = crate::helpers::spawn_app().await;

    let response = reqwest::Client::new()
        .post(format!("{}/pessoas", test_app.address))
        .json(&serde_json::json!({
           "apelido": "foo",
            "nascimento": "1992-11-23",
            "stack": ["Rust"]
        }))
        .send()
        .await
        .expect("failed request");

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn returns_422_unprocessable_entity_when_missing_nickname() {
    let test_app = crate::helpers::spawn_app().await;

    let response = reqwest::Client::new()
        .post(format!("{}/pessoas", test_app.address))
        .json(&serde_json::json!({
            "nome": "foo",
            "nascimento": "1992-11-23",
            "stack": ["Rust"]
        }))
        .send()
        .await
        .expect("failed request");

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn returns_422_unprocessable_entity_given_invalid_stack_content() {
    let test_app = crate::helpers::spawn_app().await;

    let response = reqwest::Client::new()
        .post(format!("{}/pessoas", test_app.address))
        .json(&serde_json::json!({
            "nome": "foo",
            "apelido": "bar",
            "nascimento": "1992-11-23",
            "stack": [1, "Rust"]
        }))
        .send()
        .await
        .expect("failed request");

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn returns_422_unprocessable_entity_given_invalid_name() {
    let test_app = crate::helpers::spawn_app().await;

    let response = reqwest::Client::new()
        .post(format!("{}/pessoas", test_app.address))
        .json(&serde_json::json!({
           "nome": 1,
            "apelido": "bar",
            "nascimento": "1992-11-23",
            "stack": ["Rust"]
        }))
        .send()
        .await
        .expect("failed request");

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}
