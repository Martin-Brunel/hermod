use rocket::http::Status;

use crate::db_connect::establish_connection;
use crate::dto::user::UserInsert;
use crate::models::user::User;
use crate::schema;
use diesel::ExpressionMethods;
use diesel::{QueryDsl, RunQueryDsl};


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
        .first::<User>(&mut c)
    {
        Ok(results) => Ok(results),
        Err(_) => Err(Status::NotFound),
    }
}

pub fn get_by_id(user_id_input: i32) -> Result<User, Status> {
    use self::schema::user::dsl::*;
    let mut c = establish_connection();
    match user
        .filter(id.eq(user_id_input))
        .filter(is_deleted.eq(0))
        .first::<User>(&mut c)
    {
        Ok(results) => Ok(results),
        Err(_) => Err(Status::NotFound),
    }
}

pub fn update_password(user_id: i32, new_password: String) -> Result<usize, Status> {
    use self::schema::user::dsl::*;
    let mut c = establish_connection();
    match diesel::update(user)
        .filter(id.eq(user_id))
        .set(password.eq(new_password))
        .execute(&mut c) {
            Ok(res) => Ok(res),
            Err(_) => Err(Status::BadRequest)
        }
}
