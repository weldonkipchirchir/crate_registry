use std::process::Command;

use reqwest::blocking::ClientBuilder;
use reqwest::header::HeaderValue;
use reqwest::{blocking::Client, header::HeaderMap};
use reqwest::{header, StatusCode};
use serde_json::{json, Value};

pub static APP_HOST: &'static str = "http://127.0.0.1:8000";
pub fn create_test_rustacean(client: &Client) -> Value {
    let response = client
        .post(format!("{}/rustaceans", APP_HOST))
        .json(&json!(
           {
              "email":"weldon23@gmail.com",
              "name":"foo"
           }
        ))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    return response.json().unwrap();
}

pub fn delete_test_rustacean(client: &Client, rustacean: Value) {
    let response = client
        .delete(format!("{}/rustaceans/{}", APP_HOST, rustacean["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

pub fn create_test_crate(client: &Client, rustacean: Value) -> Value {
    let response = client
        .post(format!("{}/crates", APP_HOST))
        .json(&json!(
           {
              "rustacean_id":rustacean["id"],
              "code":"rust",
              "name":"foo",
              "version":"0.1",
            "description":"rust crate",
           }
        ))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    return response.json().unwrap();
}

pub fn delete_test_crate(client: &Client, a_crate: Value) {
    let response = client
        .delete(format!("{}/crates/{}", APP_HOST, a_crate["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

pub fn get_client_with_logged_in_admin() -> Client {
    let header_value = get_login_for_user("test_admin", "admin");
    let mut headers = HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_str(&header_value.as_str()).unwrap(),
    );

    ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap()
}
pub fn get_client_with_logged_in_user() -> Client {
    let header_value = get_login_for_user("test_viewer", "viewer");
    let mut headers = HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_str(&header_value.as_str()).unwrap(),
    );

    ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap()
}
pub fn get_client_with_logged_in_editor() -> Client {
    let header_value = get_login_for_user("test_editor", "editor");
    let mut headers = HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_str(&header_value.as_str()).unwrap(),
    );

    ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap()
}

fn get_login_for_user(username: &str, role: &str) -> String {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("cli")
        .arg("users")
        .arg("create")
        .arg(username)
        .arg("1234")
        .arg(role)
        .output();
    println!("{:?}", output);

    let client = Client::new();
    let response = client
        .post(format!("{}/login", APP_HOST))
        .json(&json!(
           {
              "username":username,
              "password":"1234"
           }
        ))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.get("token").is_some());
    assert_eq!(json["token"].as_str().unwrap().len(), 128);
    format!("Bearer {}", json["token"].as_str().unwrap())
}
