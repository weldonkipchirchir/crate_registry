use reqwest::StatusCode;
use rocket::serde::json::Value;
use serde_json::json;
mod common;

#[test]
fn test_get_crates() {
    let client = common::get_client_with_logged_in_user();
    let response = client
        .get(format!("{}/crates", common::APP_HOST))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[test]
fn test_get_a_crate() {
    let client = common::get_client_with_logged_in_editor();
    let rustacean_json: Value = common::create_test_rustacean(&client);

    let crate_json: Value = common::create_test_crate(&client, rustacean_json.clone());

    let response = client
        .get(format!("{}/crates/{}", common::APP_HOST, crate_json["id"]))
        .send()
        .unwrap();
    let status = response.status();

    let json: Value = response.json().unwrap();

    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        json,
        json!({
           "id": crate_json["id"],
           "rustacean_id":rustacean_json["id"],
              "code":"rust",
              "name":"foo",
              "version":"0.1",
            "description":"rust crate",
           "created_at":crate_json["created_at"]
        })
    );

    common::delete_test_crate(&client, crate_json);
    common::delete_test_rustacean(&client, rustacean_json);
}

#[test]
fn test_create_crates() {
    let client = common::get_client_with_logged_in_admin();

    let json1: Value = common::create_test_rustacean(&client);

    let json: Value = common::create_test_crate(&client, json1.clone());
    assert_eq!(
        json,
        json!({
           "id": json["id"],
           "rustacean_id":json1["id"],
              "code":"rust",
              "name":"foo",
              "version":"0.1",
            "description":"rust crate",
           "created_at":json["created_at"]
        })
    );
    common::delete_test_crate(&client, json);
    common::delete_test_rustacean(&client, json1);
}

#[test]
fn test_update_crate() {
    let client = common::get_client_with_logged_in_admin();

    // Create a new Rustacean
    let rustacean_json: Value = common::create_test_rustacean(&client);

    // Create a new Crate
    let crate_json: Value = common::create_test_crate(&client, rustacean_json.clone());

    // Update the created Crate
    let response = client
        .put(format!("{}/crates/{}", common::APP_HOST, crate_json["id"]))
        .json(&json!({
            "id": crate_json["id"],
            "rustacean_id": rustacean_json["id"],
            "code": "rust1.0",
            "name": "weldon",
            "version": "0.1",
            "description": "rust crate",
            "created_at": crate_json["created_at"]
        }))
        .send()
        .unwrap();

    let status = response.status();
    let response_text = response.text().unwrap();

    if status != StatusCode::OK {
        panic!(
            "Expected status 200 OK but got {}: {}",
            status, response_text
        );
    }

    let updated_crate_json: Value = serde_json::from_str(&response_text).unwrap();

    assert_eq!(
        updated_crate_json,
        json!({
            "id": crate_json["id"],
            "rustacean_id": rustacean_json["id"],
            "code": "rust1.0",
            "name": "weldon",
            "version": "0.1",
            "description": "rust crate",
            "created_at": crate_json["created_at"]
        })
    );
    common::delete_test_crate(&client, updated_crate_json);
    common::delete_test_rustacean(&client, rustacean_json);
}

#[test]
fn test_delete_crate() {
    let client = common::get_client_with_logged_in_admin();

    // Create a new Rustacean
    let rustacean_json: Value = common::create_test_rustacean(&client);

    // Create a new Crate
    let crate_json: Value = common::create_test_crate(&client, rustacean_json.clone());

    // Delete the created Crate
    let response = client
        .delete(format!("{}/crates/{}", common::APP_HOST, crate_json["id"]))
        .send()
        .unwrap();

    let status = response.status();

    assert_eq!(status, StatusCode::NO_CONTENT);
}
