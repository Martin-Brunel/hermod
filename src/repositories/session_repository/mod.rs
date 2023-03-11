use chrono::NaiveDateTime;
use rocket::http::Status;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


use crate::db_connect::{establish_connection};
use crate::dto::session::insert_session_dto::InsertSessionDto;
use crate::schema::{self, session};

use diesel::{RunQueryDsl, QueryDsl, Queryable, Insertable};
use diesel::ExpressionMethods;

#[derive(Queryable, Deserialize, Serialize, Insertable, ToSchema, Debug)]
#[diesel(table_name = session)]
#[diesel(belongs_to(User))]
pub struct Session {
    pub id: i32,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub is_deleted: i8,
    pub deleted_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl Session {
    pub fn create(new_session: InsertSessionDto) -> Result<Session, Status> {
        use self::schema::session::dsl::*; 
        let mut c = establish_connection();
        let inserted = diesel::insert_into(session)
        .values(&new_session)
        .execute(&mut c)
        .expect("Impossible de charger les messages");
        Self::get_last_user_session(new_session.user_id)
    }

    pub fn get_last_user_session(user_id_input: i32) -> Result<Session, Status> {
        use self::schema::session::dsl::*; 
        let mut c = establish_connection();
        match session
            .filter(user_id.eq(user_id_input))
            .filter(is_deleted.eq(0))
            .order(created_at.desc())
            .first::<Session>(&mut c) {
                Ok(results) => Ok(results),
                Err(_)=> Err(Status::NotFound)
            }
    }
}