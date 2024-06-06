use crate::schema::*;
use chrono::NaiveDateTime;
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
#[table_name = "rustaceans"]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}

//only for existing record
#[derive(Queryable, Associations, Serialize, Deserialize)]
#[belongs_to(Rustacean)]
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
#[table_name = "crates"]
pub struct NewCrate {
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}
