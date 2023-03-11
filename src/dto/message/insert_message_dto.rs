use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::{repositories::session_repository::Session, schema::message};

use super::input_message_dto::InputMessageDto;

#[derive(Deserialize, Serialize, Insertable, Queryable, Clone, Debug)]
#[diesel(table_name = message)]
pub struct InsertMessageDto {
    pub user_id: i32,
    pub data: String,
    pub session_id: i32,
}

impl InsertMessageDto {
    pub fn from_input_message(input_message: InputMessageDto, session: Session) -> Self {
        InsertMessageDto {
            user_id: input_message.user_id,
            data: input_message.data,
            session_id: session.id,
        }
    }
}
