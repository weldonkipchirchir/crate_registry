//define endpoints for CRUD Operation;
use crate::{auth, repositories::UserRepository, CacheConn, DBConnection};
use rocket::{
    http::Status,
    response::status::Custom,
    serde::json::{serde_json::json, Json},
};
use rocket_db_pools::{deadpool_redis::redis::AsyncCommands, Connection};
use serde_json::Value;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[post("/login", format = "json", data = "<credentials>")]
pub async fn login(
    db: DBConnection,
    mut cache: Connection<CacheConn>,
    credentials: Json<Credentials>,
) -> Result<Value, Custom<Value>> {
    let username = credentials.username.clone();
    let user = db
        .run(move |c| UserRepository::find_by_username(c, &username))
        .await
        .map_err(|e| Custom(Status::InternalServerError, json!({"error": e.to_string()})))?;

    let session_id = auth::authorize_user(&user, &credentials).map_err(|_| {
        Custom(
            Status::Unauthorized,
            json!({"error": "Could not authorize user"}),
        )
    })?;

    cache
        .set_ex::<_, _, ()>(format!("sessions/{}", session_id), user.id, 3 * 60 * 60)
        .await
        .map(|_| json!({"token": session_id}))
        .map_err(|err| {
            Custom(
                Status::InternalServerError,
                json!({"error": err.to_string()}),
            )
        })
}
