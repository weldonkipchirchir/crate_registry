use crate::models::*;
use crate::schema::*;
use diesel::prelude::*;
use diesel::PgConnection;

pub struct RustaceanRepository;

impl RustaceanRepository {
    // Retrieve multiple records
    pub fn find_multiple_records(c: &mut PgConnection, limit: i64) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table
            .limit(limit)
            .order(rustaceans::id.desc())
            .load::<Rustacean>(c)
    }

    // Retrieve single record
    pub fn find_record(c: &mut PgConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table.find(id).get_result::<Rustacean>(c)
    }

    // Delete a record
    pub fn delete_record(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(rustaceans::table.find(id)).execute(c)
    }

    // Create a record
    pub fn create_record(c: &mut PgConnection, record: NewRustacean) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans::table)
            .values(record)
            .get_result(c)
    }

    // Update a record
    pub fn update_record(
        c: &mut PgConnection,
        id: i32,
        update: Rustacean,
    ) -> QueryResult<Rustacean> {
        diesel::update(rustaceans::table.find(id))
            .set((
                rustaceans::email.eq(update.email.to_owned()),
                rustaceans::name.eq(update.name.to_owned()),
            ))
            .execute(c)?;

        Self::find_record(c, id)
    }
}

pub struct CrateRepository;

impl CrateRepository {
    // Retrieve multiple records
    pub fn find_multiple_records(c: &mut PgConnection, limit: i64) -> QueryResult<Vec<Crate>> {
        crates::table
            .limit(limit)
            .order(crates::id.desc())
            .load::<Crate>(c)
    }

    // Retrieve single record
    pub fn find_record(c: &mut PgConnection, id: i32) -> QueryResult<Crate> {
        crates::table.find(id).get_result::<Crate>(c)
    }

    // Delete a record
    pub fn delete_record(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(crates::table.find(id)).execute(c)
    }

    // Create a record
    pub fn create_record(c: &mut PgConnection, record: NewCrate) -> QueryResult<Crate> {
        diesel::insert_into(crates::table)
            .values(record)
            .get_result(c)
    }

    // Update a record
    pub fn update_record(c: &mut PgConnection, id: i32, update: Crate) -> QueryResult<Crate> {
        diesel::update(crates::table.find(id))
            .set((
                crates::code.eq(update.code.to_owned()),
                crates::name.eq(update.name.to_owned()),
                crates::version.eq(update.version.to_owned()),
                crates::description.eq(update.description.to_owned()),
                crates::created_at.eq(update.created_at),
            ))
            .execute(c)?;

        Self::find_record(c, id)
    }
}
