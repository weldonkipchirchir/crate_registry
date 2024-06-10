use reqwest::StatusCode;
use serde_json::{json, Value};
pub mod common;

#[test]
fn test_get_rustaceans() {
    let client = common::get_client_with_logged_in_admin();
    let response = client
        .get(format!("{}/rustaceans", common::APP_HOST))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let rustacean1 = common::create_test_rustacean(&client);
    let rustacean2 = common::create_test_rustacean(&client);

    let response = client
        .get(format!("{}/rustaceans", common::APP_HOST))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&rustacean1));
    assert!(json.as_array().unwrap().contains(&rustacean2));
    common::delete_test_rustacean(&client, rustacean1);
    common::delete_test_rustacean(&client, rustacean2);
}

#[test]
fn test_get_a_rustacean() {
    let client = common::get_client_with_logged_in_admin();
    let json = common::create_test_rustacean(&client);
    assert_eq!(
        json,
        json!({
           "id": json["id"],
           "email":"weldon23@gmail.com",
           "name":"foo",
           "created_at":json["created_at"]
        })
    );

    let response = client
        .get(format!("{}/rustaceans/{}", common::APP_HOST, json["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert_eq!(
        json,
        json!({
           "id": json["id"],
           "email":"weldon23@gmail.com",
           "name":"foo",
           "created_at":json["created_at"]
        })
    );
    common::delete_test_rustacean(&client, json)
}

#[test]
fn test_create_rustacean() {
    let client = common::get_client_with_logged_in_admin();
    let json = common::create_test_rustacean(&client);
    assert_eq!(
        json,
        json!({
           "id": json["id"],
           "email":"weldon23@gmail.com",
           "name":"foo",
           "created_at":json["created_at"]
        })
    );
    common::delete_test_rustacean(&client, json)
}

#[test]
fn testt_delete_rustacean() {
    let client = common::get_client_with_logged_in_admin();
    let json = common::create_test_rustacean(&client);
    common::delete_test_rustacean(&client, json)
}

#[test]
fn test_update_rustacean() {
    let client = common::get_client_with_logged_in_admin();

    // Create a new Rustacean
    let json = common::create_test_rustacean(&client);

    // Update the created Rustacean
    let response = client
        .put(format!("{}/rustaceans/{}", common::APP_HOST, json["id"]))
        .json(&json!(
           {
              "id": json["id"],
              "email":"weldon23@gmail.com",
              "name":"weldon",
              "created_at": json["created_at"]
           }
        ))
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

    let json: Value = serde_json::from_str(&response_text).unwrap();

    assert_eq!(
        json,
        json!({
           "id": json["id"],
           "email":"weldon23@gmail.com",
           "name":"weldon",
           "created_at":json["created_at"]
        })
    );
    common::delete_test_rustacean(&client, json)
}
