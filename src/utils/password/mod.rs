use argon2::{password_hash::SaltString, Argon2, PasswordHasher, PasswordHash, PasswordVerifier};
use rand::rngs::OsRng;


pub fn hash_password(new_password: String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let encoded = Argon2::default()
    .hash_password(new_password.as_bytes(), &salt)
    .unwrap()
    .to_string();
    encoded
}


pub fn compare_hash(password: String, hash: String) -> bool {
    let parsed_hash = PasswordHash::new(&hash).unwrap();

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}