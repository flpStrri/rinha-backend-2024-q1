use reqwest::header::LOCATION;
use reqwest::StatusCode;

#[tokio::test]
async fn returns_200_ok_when_in_storage() {
    let test_app = crate::helpers::spawn_app().await;
    let post_response = reqwest::Client::new()
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
    let location_header = &post_response
        .headers()
        .get(LOCATION)
        .expect("header not found")
        .to_str()
        .expect("not ASCII value");

    let response = reqwest::Client::new()
        .get(format!("{}{}", &test_app.address, &location_header))
        .send()
        .await
        .expect("failed request");

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn returns_dev_body_when_in_storage() {
    let test_app = crate::helpers::spawn_app().await;
    let post_response = reqwest::Client::new()
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
    let location_header = &post_response
        .headers()
        .get(LOCATION)
        .expect("header not found")
        .to_str()
        .expect("not ASCII value");

    let response = reqwest::Client::new()
        .get(format!("{}{}", &test_app.address, &location_header))
        .send()
        .await
        .expect("failed request");

    let response_body = response
        .json::<std::collections::HashMap<String, serde_json::Value>>()
        .await
        .unwrap();
    assert_eq!(response_body["apelido"], String::from("foo"));
    assert_eq!(response_body["nome"], String::from("bar"));
    assert_eq!(response_body["nascimento"], String::from("2020-12-03"));
    assert_eq!(
        response_body["stack"],
        serde_json::json!([String::from("Rust"), String::from("Python")])
    );
}

#[tokio::test]
async fn returns_404_not_found_when_not_in_storage() {
    let test_app = crate::helpers::spawn_app().await;

    let response = reqwest::Client::new()
        .get(format!(
            "{}/pessoas/e50408fa-e368-4ccd-9ade-851fdb553e0f",
            test_app.address
        ))
        .send()
        .await
        .expect("failed request");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
