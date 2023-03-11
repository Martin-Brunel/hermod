use chrono::NaiveDateTime;
use diesel::{prelude::*};
use diesel;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use crate::schema::user;


#[derive(Queryable, Deserialize, Serialize, Insertable, ToSchema, Debug)]
#[diesel(table_name = user)]
#[diesel(belongs_to(Brand))]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub roles: String,
    pub brand_id: i32,
    pub firstname: String,
    pub lastname: String,
    pub created_at: NaiveDateTime,
    pub is_deleted: i8,
    pub deleted_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,

}