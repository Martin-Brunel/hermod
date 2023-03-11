use rocket::http::Status;
use rocket::serde::{ json::Json};
use crate::dto::user::UserInsert;
use crate::guards::security::Security;
use crate::managers;
use crate::utils::password::hash_password;
use crate::{ models::user::User, dto::user::UserInput };


#[post("/",  format = "json", data="<user_input>")]
pub fn post_user(authorised: Security, user_input: Json<UserInput>) -> Result<Json<User>, Status> {
    let user = authorised.user;
    let is_admin = user.roles.contains("ROLE_ADMIN");
    if !is_admin {
        Err(Status::Forbidden)
    } else {
        let struct_user = UserInsert {
            email: user_input.email.clone(),
            password: hash_password(user_input.password.clone()),
            roles: String::from("[\"ROLE_USER\"]"),
            brand_id: user_input.brand_id.clone(),
            firstname: user_input.firstname.clone(),
            lastname: user_input.lastname.clone(),
        };
        match managers::user::create(struct_user) {
            Ok(new_user) => Ok(Json(new_user)),
            Err(_) => Err(Status { code: 400 })
        }
    }
}