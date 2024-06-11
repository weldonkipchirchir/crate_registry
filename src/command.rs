use argon2::password_hash::{rand_core::OsRng, PasswordHasher, SaltString};
use chrono::{Datelike, Utc};
use diesel::{Connection, PgConnection};
use lettre::transport::smtp::authentication::Credentials;
use tera::{Context, Tera};

use crate::{
    mail::HtmlMailer,
    models::{NewUser, RoleCode},
    repositories::{CrateRepository, RoleRepository, UserRepository},
};

fn load_db_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("Cannot load DB url from env");
    PgConnection::establish(&database_url).expect("Cannot connect to postgres")
}

fn load_template_engine() -> Tera {
    Tera::new("templates/**/*.html").unwrap_or_else(|e| panic!("Parsing error(s): {}", e))
}

pub fn create_user(username: String, password: String, role_codes: Vec<String>) {
    let mut connection = load_db_connection();
    //generate salt
    let salt = SaltString::generate(OsRng);
    let argon = argon2::Argon2::default();
    //hash password
    let hash_password = argon.hash_password(password.as_bytes(), &salt).unwrap();
    let new_user = NewUser {
        username,
        password: hash_password.to_string(),
    };

    let role_codes = role_codes
        .iter()
        .map(|v| RoleCode::from_string(v.to_owned()).unwrap())
        .collect();
    let user = UserRepository::create_user(&mut connection, new_user, role_codes)
        .expect("user creation failed");
    println!("Created user with id {:#?}", user);
    let roles = RoleRepository::find_by_user(&mut connection, &user).unwrap();
    println!("Roles assigned {:#?}", roles);
}

pub fn list_users() {
    let mut c = load_db_connection();

    let users = UserRepository::find_user_with_roles(&mut c).unwrap();

    for user in users {
        println!("{:#?}", user);
    }
}

pub fn delete_user(id: i32) {
    let mut c = load_db_connection();

    let delete_user = UserRepository::delete_user(&mut c, id).expect("user deletion failed");

    println!("Deleted user with id {:#?}", delete_user);
}

pub fn send_digest(to: String, hours_since: i32) {
    let mut c = load_db_connection();
    let tera = load_template_engine();

    let crates = CrateRepository::find_since(&mut c, hours_since).unwrap();

    if crates.len() > 0 {
        let mut context = Context::new();
        let year = Utc::now().year();
        context.insert("crates", &crates);
        context.insert("year", &year);

        let smtp_host = std::env::var("SMTP_HOST").expect("Cannot load smtp host from env");
        let smtp_username =
            std::env::var("SMTP_USERNAME").expect("Cannot load smtp username from env");
        let smtp_password =
            std::env::var("SMTP_PASSWORD").expect("Cannot load smtp password from env");

        let credentials = Credentials::new(smtp_username, smtp_password);
        let mailer = HtmlMailer {
            smtp_host,
            credentials,
            template_engine: tera,
        };
        mailer
            .send_email(&to, "email/digest.html", &context)
            .unwrap();
    }
}
