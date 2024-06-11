use crate::{models::User, rocket_routes::authorization::Credentials};
use argon2::PasswordHash;
use rand::{distributions::Alphanumeric, Rng};

pub fn authorize_user(
    user: &User,
    credentials: &Credentials,
) -> Result<String, argon2::password_hash::Error> {
    let db_password_hash = PasswordHash::new(&user.password)?;
    let argon = argon2::Argon2::default();
    use argon2::password_hash::PasswordVerifier;
    argon.verify_password(credentials.password.as_bytes(), &db_password_hash)?;

    Ok(rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(128)
        .map(char::from)
        .collect())
}
