use crate::dto::credentials::{Credentials};
use crate::dto::user::{UserInsert};
use crate::models::user::User;
use crate::repositories::{user, self};
use crate::utils::password::compare_hash;
use rocket::http::Status;


pub fn create(input_user: UserInsert) -> Result<User, Status> {
    let email = input_user.email.clone();
    
    match repositories::user::get_by_email(email.clone()) {
        Err(_) => {
            match user::create(input_user) {
                1 => {
                    match repositories::user::get_by_email(email.clone()) {
                        Ok(user) => Ok(user),
                        Err(status) => Err(status)
                    }
                },
                _ => Err(Status {code: 500})
            }
        },
        Ok(_) => Err(Status {code: 409})
    }
} 

pub fn check_credentials(credentials: Credentials) -> Result<User, Status>  {
    match repositories::user::get_by_email(credentials.email) {
        Ok(user) => {
            let verif =  compare_hash(credentials.password, user.password.clone());
            match verif {
                true => Ok(user),
                _ => Err(Status::Forbidden)
            }
        },
        Err(e) => Err(e)
    }
}

pub fn modify_user_password(user_id: i32, password: String, brand: i32) -> Result<User, Status> {

    let user = match repositories::user::get_by_id(user_id) {
        Ok(user) => user,
        Err(s) => return Err(s)
    };
    if user.brand_id != brand {
        return Err(Status::Forbidden);
    }
    match repositories::user::update_password(user_id, password) {
        Ok(_) => {
            let new_user = match repositories::user::get_by_id(user_id) {
                Ok(new_user) => new_user,
                Err(s) => return Err(s)
            };
            Ok(new_user)
        },
        Err(s) => Err(s)
    }

}