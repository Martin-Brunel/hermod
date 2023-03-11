use rocket::http::Status;

use crate::dto::message::input_message_dto::InputMessageDto;
use crate::dto::message::insert_message_dto::InsertMessageDto;
use crate::models::user::User;
use crate::repositories::{self, session_repository};
use crate::repositories::message_repository::Message;

pub fn create(input_message: InputMessageDto, user: User) -> Result<Message, Status> {
    let email = user.email.clone();
    let user_to = match repositories::user::get_by_email(email.clone()) {
        Ok(user_to) => user_to,
        Err(_) => return Err(Status { code: 409 }),
    };
    if user.brand_id != user_to.brand_id {
        return Err(Status::Forbidden);
    }
    let session = match session_repository::Session::get_last_user_session(user_to.id) {
        Ok(session) => session,
        Err(_) => return Err(Status::NotFound),
    };
    let insertable_messge = InsertMessageDto::from_input_message(input_message, session);
    match repositories::message_repository::Message::create(insertable_messge.clone()) {
        1 => match repositories::message_repository::Message::get_after_insert(insertable_messge.user_id) {
            Ok(message) => Ok(message),
            Err(status) => Err(status),
        },
        _ => Err(Status { code: 500 }),
    }
}
