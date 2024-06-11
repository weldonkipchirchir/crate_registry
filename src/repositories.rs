use crate::models::*;
use crate::schema::*;
use chrono::{Duration, Utc};
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

    pub fn find_since(c: &mut PgConnection, hours_since: i32) -> QueryResult<Vec<Crate>> {
        let since = Utc::now().naive_utc() - Duration::hours(hours_since.into());

        crates::table
            .filter(crates::created_at.ge(since))
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

pub struct UserRepository;

impl UserRepository {
    pub fn create_user(
        c: &mut PgConnection,
        record: NewUser,
        role_codes: Vec<RoleCode>,
    ) -> QueryResult<User> {
        let user: User = diesel::insert_into(users::table)
            .values(record)
            .get_result::<User>(c)?;

        for role_code in &role_codes {
            let new_user_role = {
                if let Ok(role) = RoleRepository::find_by_code(c, role_code) {
                    NewUserRole {
                        user_id: user.id,
                        role_id: role.id,
                    }
                } else {
                    let new_role = NewRole {
                        code: role_code.clone(),
                        name: role_code.as_str().to_owned(),
                    };
                    let role = RoleRepository::create_role(c, new_role)?;
                    NewUserRole {
                        user_id: user.id,
                        role_id: role.id,
                    }
                }
            };
            diesel::insert_into(users_roles::table)
                .values(new_user_role)
                .execute(c)?;
        }
        Ok(user)
    }

    // Delete a record
    pub fn delete_user(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
        // Delete the user roles next
        let user_roles_deletion = diesel::delete(diesel::QueryDsl::filter(
            users_roles::table,
            users_roles::user_id.eq(id),
        ))
        .execute(c)?;

        // Delete the user first
        let user_deletion = diesel::delete(users::table.find(id)).execute(c)?;

        // Return the total number of deletions
        Ok(user_deletion + user_roles_deletion)
    }

    //find with roles
    pub fn find_user_with_roles(
        c: &mut PgConnection,
    ) -> QueryResult<Vec<(User, Vec<(UserRole, Role)>)>> {
        let users = users::table.load::<User>(c)?;

        let user_roles = users_roles::table
            .inner_join(roles::table)
            .load::<(UserRole, Role)>(c)?
            .grouped_by(&users);

        Ok(users.into_iter().zip(user_roles).collect())
    }

    //find_by_usernamet
    pub fn find_by_username(c: &mut PgConnection, username: &String) -> QueryResult<User> {
        diesel::QueryDsl::filter(users::table, users::username.eq(username)).get_result::<User>(c)
    }

    // Retrieve single record
    pub fn find(c: &mut PgConnection, id: i32) -> QueryResult<User> {
        users::table.find(id).get_result::<User>(c)
    }
}

pub struct RoleRepository;

impl RoleRepository {
    pub fn find_by_code(c: &mut PgConnection, code: &RoleCode) -> QueryResult<Role> {
        diesel::QueryDsl::filter(roles::table, roles::code.eq(code)).get_result::<Role>(c)
    }

    pub fn find_by_ids(c: &mut PgConnection, ids: Vec<i32>) -> QueryResult<Vec<Role>> {
        diesel::QueryDsl::filter(roles::table, roles::id.eq_any(ids)).load::<Role>(c)
    }

    pub fn find_by_user(c: &mut PgConnection, user: &User) -> QueryResult<Vec<Role>> {
        let user_roles = UserRole::belonging_to(user).load::<UserRole>(c)?;
        let role_ids: Vec<i32> = user_roles.iter().map(|ur| ur.role_id).collect();
        Self::find_by_ids(c, role_ids)
    }

    pub fn create_role(c: &mut PgConnection, record: NewRole) -> QueryResult<Role> {
        diesel::insert_into(roles::table)
            .values(record)
            .get_result(c)
    }
}
