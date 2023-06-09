use diesel::{Insertable, Queryable};
use diesel;
use serde::{Serialize, Deserialize};
use crate::schema::user;
pub mod put_user_input_dto;

#[derive(Deserialize, Serialize)]
pub struct UserInput {
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
}

#[derive(Deserialize, Serialize, Insertable, Queryable)]
#[diesel(table_name = user)]
pub struct UserInsert {
    pub email: String,
    pub password: String,
    pub roles: String,
    pub brand_id: i32,
    pub firstname: String,
    pub lastname: String,
}