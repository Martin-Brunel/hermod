use chrono::NaiveDateTime;
use rocket::http::Status;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


use crate::db_connect::{establish_connection};
use crate::schema::{self, brand};

use diesel::{RunQueryDsl, QueryDsl, Queryable, Insertable};
use diesel::ExpressionMethods;

#[derive(Queryable, Deserialize, Serialize, Insertable, ToSchema, Debug)]
#[diesel(table_name = brand)]
pub struct Brand {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub is_deleted: i8,
    pub deleted_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub interval: Option<i32>
}

impl Brand {
    pub fn get_by_id(brand_id: i32) -> Result<Brand, Status> {
        use self::schema::brand::dsl::*; 
        let mut c = establish_connection();
        match brand
            .filter(id.eq(brand_id))
            .filter(is_deleted.eq(0))
            .first::<Brand>(&mut c) {
                Ok(results) => Ok(results),
                Err(_)=> Err(Status::NotFound)
            }
    }
}