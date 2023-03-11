use chrono::NaiveDateTime;
use rocket::http::Status;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


use crate::db_connect::{establish_connection};
use crate::dto::message::insert_message_dto::InsertMessageDto;
use crate::schema::{self, message};

use diesel::{RunQueryDsl, QueryDsl, Queryable, Insertable};
use diesel::ExpressionMethods;

#[derive(Queryable, Deserialize, Serialize, Insertable, ToSchema, Debug)]
#[diesel(table_name = message)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Session))]
pub struct Message {
    pub id: i32,
    pub user_id: i32,
    pub data: String,
    pub is_read: i8,
    pub session_id: i32,
    pub created_at: NaiveDateTime,
    pub is_deleted: i8,
    pub deleted_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl Message {

    pub fn create(new_message: InsertMessageDto) -> usize {
        use self::schema::message::dsl::*; 
        let mut c = establish_connection();
        let inserted = diesel::insert_into(message)
        .values(&new_message)
        .execute(&mut c)
        .expect("Impossible de charger les messages");
        inserted
    }

    pub fn get_after_insert(user_id_input: i32) -> Result<Message, Status> {
        use self::schema::message::dsl::*; 
        let mut c = establish_connection();
        match message
            .filter(user_id.eq(user_id_input))
            .filter(is_deleted.eq(0))
            .order(created_at.desc())
            .first::<Message>(&mut c) {
                Ok(results) => Ok(results),
                Err(_)=> Err(Status::NotFound)
            }
    }

    pub fn get_messages_to_send(session_id_input: i32) -> Result<Vec<Message>, Status> {
        use self::schema::message::dsl::*; 
        let mut c = establish_connection();
        match message
            .filter(session_id.eq(session_id_input))
            .filter(is_deleted.eq(0))
            .filter(is_read.eq(0))
            .load(&mut c) {
                Ok(results) => Ok(results),
                Err(_)=> Err(Status::NotFound)
            }
    }

    pub fn mark_message_as_read(message_id: i32) -> Result<usize, Status> {
        use self::schema::message::dsl::*; 
        let mut c = establish_connection();
        match diesel::update(message)
            .filter(id.eq(message_id))
            .set(is_read.eq(1))
            .execute(&mut c) {
                Ok(results) => Ok(results),
                Err(_)=> Err(Status::NotFound)
            }
    }

}
