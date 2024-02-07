use reqwest::StatusCode;

#[tokio::test]
async fn returns_200_ok_with_dev_when_searching_by_nickname_with_exact_match() {
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
    let post_response_body = post_response
        .json::<std::collections::HashMap<String, serde_json::Value>>()
        .await
        .unwrap();

    let response = reqwest::Client::new()
        .get(format!("{}/pessoas?t=foo", &test_app.address))
        .send()
        .await
        .expect("failed request");

    assert_eq!(response.status(), StatusCode::OK);
    let mut response_body = response
        .json::<Vec<std::collections::HashMap<String, serde_json::Value>>>()
        .await
        .unwrap();
    let only_response: std::collections::HashMap<String, serde_json::Value> =
        response_body.pop().expect("a person in response");
    assert_eq!(only_response["id"], post_response_body["id"])
}

#[tokio::test]
async fn returns_200_ok_with_dev_when_searching_by_nickname_without_exact_match() {
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
    let post_response_body = post_response
        .json::<std::collections::HashMap<String, serde_json::Value>>()
        .await
        .unwrap();

    let response = reqwest::Client::new()
        .get(format!("{}/pessoas?t=fo", &test_app.address))
        .send()
        .await
        .expect("failed request");

    assert_eq!(response.status(), StatusCode::OK);
    let mut response_body = response
        .json::<Vec<std::collections::HashMap<String, serde_json::Value>>>()
        .await
        .unwrap();
    let only_response: std::collections::HashMap<String, serde_json::Value> =
        response_body.pop().expect("a person in response");
    assert_eq!(only_response["id"], post_response_body["id"])
}

#[tokio::test]
async fn returns_200_ok_with_dev_when_searching_by_name_without_exact_match() {
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
    let post_response_body = post_response
        .json::<std::collections::HashMap<String, serde_json::Value>>()
        .await
        .unwrap();

    let response = reqwest::Client::new()
        .get(format!("{}/pessoas?t=ba", &test_app.address))
        .send()
        .await
        .expect("failed request");

    assert_eq!(response.status(), StatusCode::OK);
    let mut response_body = response
        .json::<Vec<std::collections::HashMap<String, serde_json::Value>>>()
        .await
        .unwrap();
    let only_response: std::collections::HashMap<String, serde_json::Value> =
        response_body.pop().expect("a person in response");
    assert_eq!(only_response["id"], post_response_body["id"])
}

#[tokio::test]
async fn returns_200_ok_with_dev_when_searching_by_stack_without_exact_match() {
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
    let post_response_body = post_response
        .json::<std::collections::HashMap<String, serde_json::Value>>()
        .await
        .unwrap();

    let response = reqwest::Client::new()
        .get(format!("{}/pessoas?t=rus", &test_app.address))
        .send()
        .await
        .expect("failed request");

    assert_eq!(response.status(), StatusCode::OK);
    let mut response_body = response
        .json::<Vec<std::collections::HashMap<String, serde_json::Value>>>()
        .await
        .unwrap();
    let only_response: std::collections::HashMap<String, serde_json::Value> =
        response_body.pop().expect("a person in response");
    assert_eq!(only_response["id"], post_response_body["id"])
}
