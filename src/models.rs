use chrono::{NaiveDate, NaiveDateTime};
use crate::schema::*;

//only for existing record
#[derive(Queryable)]
struct Rustacean{
    id:i32,
    name: String,
    email:String,
    created_at:NaiveDateTime
}


//add a record
#[derive(Insertable)]
#[table_name="rustaceans"]
struct NewRustacean {
    name: String,
    email:String,
}

//only for existing record
#[derive(Queryable, Associations)]
struct Crate{
    id:i32,
    rustacean_id:i32,
    code: String,
    name:String,
    version:String,
    description: Option<String>,
    created_at:NaiveDateTime   
}

//add a record
#[derive(Insertable)]
#[table_name="crates"]
struct NewCrate{
    rustacean_id:i32,
    code: String,
    name:String,
    version:String,
    description: Option<String>,
}