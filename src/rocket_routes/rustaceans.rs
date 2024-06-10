//define endpoints for CRUD Operation;
use crate::{
    models::{NewRustacean, Rustacean, User},
    repositories::RustaceanRepository,
    DBConnection, EditorUser,
};
use rocket::{
    http::Status,
    response::status::{Custom, NoContent},
    serde::json::{serde_json::json, Json},
};
use serde_json::Value;

#[get("/rustaceans")]
pub async fn get_rustaceans(db: DBConnection, _user: User) -> Result<Value, Custom<Value>> {
    db.run(
        |c| match RustaceanRepository::find_multiple_records(c, 100) {
            Ok(rustaceans) => Ok(json!(rustaceans)),
            Err(err) => {
                eprintln!("Error fetching rustaceans: {:?}", err);
                Err(Custom(
                    Status::InternalServerError,
                    json!("Something went wrong"),
                ))
            }
        },
    )
    .await
}

#[get("/rustaceans/<id>")]
pub async fn view_rustacean(
    db: DBConnection,
    _user: User,
    id: i32,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| match RustaceanRepository::find_record(c, id) {
        Ok(rustacean) => Ok(json!(rustacean)),
        Err(err) => {
            eprintln!("Error fetching rustaceans: {:?}", err);
            Err(Custom(
                Status::InternalServerError,
                json!("Something went wrong"),
            ))
        }
    })
    .await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
pub async fn create_rustacean(
    db: DBConnection,
    _user: EditorUser,
    new_rustacean: Json<NewRustacean>,
) -> Result<Custom<Value>, Custom<Value>> {
    db.run(
        |c| match RustaceanRepository::create_record(c, new_rustacean.into_inner()) {
            Ok(rustacean) => Ok(Custom(Status::Created, json!(rustacean))),
            Err(err) => {
                eprintln!("Error fetching rustaceans: {:?}", err);
                Err(Custom(
                    Status::InternalServerError,
                    json!("Something went wrong"),
                ))
            }
        },
    )
    .await
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
pub async fn update_rustacean(
    db: DBConnection,
    _user: EditorUser,
    id: i32,
    rustacean: Json<Rustacean>,
) -> Result<Value, Custom<Value>> {
    db.run(
        move |c| match RustaceanRepository::update_record(c, id, rustacean.into_inner()) {
            Ok(rustacean) => Ok(json!(rustacean)),
            Err(err) => {
                eprintln!("Error updating rustacean: {:?}", err);
                Err(Custom(
                    Status::InternalServerError,
                    json!("Something went wrong"),
                ))
            }
        },
    )
    .await
}

#[delete["/rustaceans/<id>"]]
pub async fn delete_rustacean(
    db: DBConnection,
    _user: EditorUser,
    id: i32,
) -> Result<NoContent, Custom<Value>> {
    db.run(move |c| match RustaceanRepository::delete_record(c, id) {
        Ok(_) => Ok(NoContent),
        Err(err) => {
            eprintln!("Error fetching rustaceans: {:?}", err);
            Err(Custom(
                Status::InternalServerError,
                json!("Something went wrong"),
            ))
        }
    })
    .await
}
