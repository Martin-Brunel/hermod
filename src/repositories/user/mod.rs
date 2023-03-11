use rocket::http::Status;

use crate::db_connect::{establish_connection};
use crate::dto::user::{ UserInsert};
use crate::schema;
use crate::models::user::User;
use diesel::{RunQueryDsl, QueryDsl};
use diesel::ExpressionMethods;

pub fn create(new_user: UserInsert) -> usize {
    use self::schema::user::dsl::*; 
    let mut c = establish_connection();
    let inserted = diesel::insert_into(user)
    .values(&new_user)
    .execute(&mut c)
    .expect("Impossible de charger les users");
    inserted
}

pub fn get_by_email(email_value: String) -> Result<User, Status> {
    use self::schema::user::dsl::*; 
    let mut c = establish_connection();
    match user
        .filter(email.eq(email_value))
        .filter(is_deleted.eq(0))
        .first::<User>(&mut c) {
            Ok(results) => Ok(results),
            Err(_)=> Err(Status::NotFound)
        }
}