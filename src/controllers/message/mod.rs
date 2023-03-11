use chrono::{NaiveDate, DateTime, Local};
use rocket::http::{Status};
use rocket::serde::{ json::Json};
use serde::{Serialize, Deserialize};
use crate::dto::message::input_message_dto::InputMessageDto;
use crate::dto::session::insert_session_dto::InsertSessionDto;
use crate::guards::get_security::GetSecurity;
use crate::guards::security::Security;
use crate::repositories::session_repository;
use crate::{managers, repositories};
use crate::repositories::message_repository::Message;
use rocket::response::stream::{Event, EventStream};
use rocket::tokio::time::{self, Duration};


#[derive(Debug, Serialize, Deserialize)]
pub struct Test {
    message: String
}


#[post("/",  format = "json", data="<message_input>")]
pub fn post_message(authorised: Security, message_input: Json<InputMessageDto>) -> Result<Json<Message>, Status> {
    let user = authorised.user;
    let is_admin = user.roles.contains("ROLE_ADMIN");
    match is_admin {
        true => {
            match managers::message::create(message_input.into_inner(), user) {
                Ok(message) => Ok(Json(message)),
                Err(_) => Err(Status::BadRequest)
            }  
        },
        _ => Err(Status::Forbidden)
    }
}

#[get("/listen")]
pub fn get_message(authorised:GetSecurity) -> Result<EventStream![], Status> {
    let user = authorised.user;
    let is_user = user.roles.contains("ROLE_USER");
    let brand = match repositories::brand::Brand::get_by_id(user.brand_id) {
        Ok(brand) => brand,
        Err(_) => return Err(Status::BadRequest)
    };
    let interval = match brand.interval {
        Some(interval) => interval,
        _ => 1
    };
    let insert_session = match session_repository::Session::create(InsertSessionDto {
        user_id: user.id
    }) {
        Ok(session) => session,
        Err(s) => return Err(s)
    };
    
    match is_user {
        true => Ok(EventStream! {
            let mut interval = time::interval(Duration::from_secs(interval.try_into().unwrap()));
            loop {
                let messages: Vec<Message> = Message::get_messages_to_send(insert_session.id).unwrap();
                for message in &messages {
                    println!("{:?}", message);
                    Message::mark_message_as_read(message.id.clone()).expect("");
                    yield Event::data(message.data.clone());
                }
                interval.tick().await;
            }
        }),
        _ => Err(Status::Forbidden)
    }
}