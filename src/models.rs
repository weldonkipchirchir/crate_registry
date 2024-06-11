use std::io::Write;

use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::serialize::{Output, ToSql};
use diesel::{pg::Pg, sql_types::Text};
use serde::{Deserialize, Serialize};

//only for existing record
#[derive(Queryable, Serialize, Deserialize)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

//add a record
#[derive(Insertable, Deserialize)]
#[diesel(table_name = rustaceans)]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}

//only for existing record
#[derive(Queryable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(Rustacean))]
pub struct Crate {
    pub id: i32,
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}

//add a record
#[derive(Insertable, Deserialize)]
#[diesel(table_name = crates)]
pub struct NewCrate {
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Identifiable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Role {
    pub id: i32,
    pub code: RoleCode,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = roles)]
pub struct NewRole {
    pub code: RoleCode,
    pub name: String,
}

#[derive(Queryable, Associations, Serialize, Deserialize, Identifiable, Debug)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Role))]
#[diesel(table_name = users_roles)]
pub struct UserRole {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = users_roles)]
pub struct NewUserRole {
    pub user_id: i32,
    pub role_id: i32,
}

#[derive(AsExpression, FromSqlRow, Debug, Serialize, Deserialize)]
#[diesel(sql_type = diesel::sql_types::Text)]
pub enum RoleCode {
    Admin,
    Editor,
    Viewer,
}

impl diesel::deserialize::FromSql<diesel::sql_types::Text, Pg> for RoleCode {
    fn from_sql(bytes: diesel::pg::PgValue<'_>) -> diesel::deserialize::Result<Self> {
        match std::str::from_utf8(bytes.as_bytes()) {
            Ok("admin") => Ok(RoleCode::Admin),
            Ok("editor") => Ok(RoleCode::Editor),
            Ok("viewer") => Ok(RoleCode::Viewer),
            _ => Err("Unrecognized role code".into()),
        }
    }
}

impl ToSql<Text, Pg> for RoleCode {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match self {
            RoleCode::Admin => out.write_all(b"admin")?,
            RoleCode::Editor => out.write_all(b"editor")?,
            RoleCode::Viewer => out.write_all(b"viewer")?,
        }
        Ok(diesel::serialize::IsNull::No)
    }
}

impl RoleCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            RoleCode::Admin => "admin",
            RoleCode::Editor => "editor",
            RoleCode::Viewer => "viewer",
        }
    }

    pub fn from_string(s: String) -> Result<Self, Box<dyn std::error::Error>> {
        match s.as_str() {
            "admin" => Ok(RoleCode::Admin),
            "editor" => Ok(RoleCode::Editor),
            "viewer" => Ok(RoleCode::Viewer),
            _ => Err("Invalid  value to transform to ROle code".into()),
        }
    }
}

impl Clone for RoleCode {
    fn clone(&self) -> Self {
        match self {
            RoleCode::Admin => RoleCode::Admin,
            RoleCode::Editor => RoleCode::Editor,
            RoleCode::Viewer => RoleCode::Viewer,
        }
    }
}
