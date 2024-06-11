#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
extern crate diesel_migrations;
pub mod auth;
pub mod command;
pub mod mail;
pub mod models;
pub mod repositories;
pub mod rocket_routes;
pub mod schema;

use diesel::PgConnection;
use models::{RoleCode, User};
use repositories::{RoleRepository, UserRepository};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use rocket_db_pools::{deadpool_redis, Connection, Database};
use rocket_sync_db_pools::database;

#[database ["postgres"]]
pub struct DBConnection(PgConnection);

#[derive(Database)]
#[database("redis")]
pub struct CacheConn(deadpool_redis::Pool);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Authorization: Bearer SESSION_ID_128_CHARS_LONG
        let session_header = request
            .headers()
            .get_one("Authorization")
            .map(|v| v.split_whitespace().collect::<Vec<_>>())
            .filter(|v| v.len() == 2 && v[0] == "Bearer");

        if let Some(header_value) = session_header {
            let mut cache = request.guard::<Connection<CacheConn>>().await.unwrap();
            let db = request.guard::<DBConnection>().await.unwrap();
            let result = cache.get(format!("sessions/{}", header_value[1])).await;
            // Handle result here
            if let Ok(user_id) = result {
                match db.run(move |c| UserRepository::find(c, user_id)).await {
                    Ok(user) => Outcome::Success(user),
                    Err(_) => Outcome::Error((Status::InternalServerError, ())),
                }
            } else {
                Outcome::Error((Status::Unauthorized, ()))
            }
        } else {
            Outcome::Error((Status::Unauthorized, ()))
        }
    }
}

//request guard for editor user

pub struct EditorUser(User);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for EditorUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user = request.guard::<User>().await.unwrap();
        let db = request.guard::<DBConnection>().await.unwrap();
        let has_access = db
            .run(|c| {
                let roles_result = RoleRepository::find_by_user(c, &user);
                match roles_result {
                    Ok(roles) => {
                        let accessible = roles.into_iter().any(|r| match r.code {
                            RoleCode::Admin => true,
                            RoleCode::Editor => true,
                            _ => false,
                        });
                        accessible.then_some(EditorUser(user))
                    }
                    Err(_) => None,
                }
            })
            .await;

        match has_access {
            Some(editor_user) => Outcome::Success(editor_user),
            _ => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}
