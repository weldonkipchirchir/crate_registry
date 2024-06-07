//define endpoints for CRUD Operation;
use crate::{
    models::{Crate, NewCrate},
    repositories::CrateRepository,
    DBConnection,
};
use rocket::{
    http::Status,
    response::status::{Custom, NoContent},
    serde::json::{serde_json::json, Json},
};
use serde_json::Value;

#[get("/crates")]
pub async fn get_crates(db: DBConnection) -> Result<Value, Custom<Value>> {
    db.run(|c| match CrateRepository::find_multiple_records(c, 100) {
        Ok(crates) => Ok(json!(crates)),
        Err(err) => {
            eprintln!("Error fetching crates: {:?}", err);
            Err(Custom(
                Status::InternalServerError,
                json!("Something went wrong"),
            ))
        }
    })
    .await
}

#[get("/crates/<id>")]
pub async fn view_crate(db: DBConnection, id: i32) -> Result<Value, Custom<Value>> {
    db.run(move |c| match CrateRepository::find_record(c, id) {
        Ok(a_crate) => Ok(json!(a_crate)),
        Err(err) => {
            eprintln!("Error fetching crates: {:?}", err);
            Err(Custom(
                Status::InternalServerError,
                json!("Something went wrong"),
            ))
        }
    })
    .await
}

#[post("/crates", format = "json", data = "<new_crates>")]
pub async fn create_crate(
    db: DBConnection,
    new_crates: Json<NewCrate>,
) -> Result<Custom<Value>, Custom<Value>> {
    db.run(
        |c| match CrateRepository::create_record(c, new_crates.into_inner()) {
            Ok(a_crate) => Ok(Custom(Status::Created, json!(a_crate))),
            Err(err) => {
                eprintln!("Error fetching crates: {:?}", err);
                Err(Custom(
                    Status::InternalServerError,
                    json!("Something went wrong"),
                ))
            }
        },
    )
    .await
}

#[put("/crates/<id>", format = "json", data = "<crates>")]
pub async fn update_crate(
    db: DBConnection,
    id: i32,
    crates: Json<Crate>,
) -> Result<Value, Custom<Value>> {
    db.run(
        move |c| match CrateRepository::update_record(c, id, crates.into_inner()) {
            Ok(a_crate) => Ok(json!(a_crate)),
            Err(err) => {
                eprintln!("Error fetching crates: {:?}", err);
                Err(Custom(
                    Status::InternalServerError,
                    json!("Something went wrong"),
                ))
            }
        },
    )
    .await
}

#[delete["/crates/<id>"]]
pub async fn delete_crate(db: DBConnection, id: i32) -> Result<NoContent, Custom<Value>> {
    db.run(move |c| match CrateRepository::delete_record(c, id) {
        Ok(_) => Ok(NoContent),
        Err(err) => {
            eprintln!("Error fetching crates: {:?}", err);
            Err(Custom(
                Status::InternalServerError,
                json!("Something went wrong"),
            ))
        }
    })
    .await
}
