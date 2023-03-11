use rocket::http::Status;
use rocket::serde::{ json::Json};
use crate::dto::user::UserInsert;
use crate::dto::user::put_user_input_dto::PutUserInputDto;
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
            brand_id: user.brand_id.clone(),
            firstname: user_input.firstname.clone(),
            lastname: user_input.lastname.clone(),
        };
        match managers::user::create(struct_user) {
            Ok(new_user) => Ok(Json(new_user)),
            Err(_) => Err(Status { code: 400 })
        }
    }
}

#[put("/admin",  format = "json", data="<put_user_input_dto>")]
pub fn put_user(authorised: Security, put_user_input_dto: Json<PutUserInputDto>) -> Result<Json<User>, Status> {
    let user = authorised.user;
    let is_admin = user.roles.contains("ROLE_ADMIN");
    if !is_admin {
        return Err(Status::Forbidden);
    }
    let input_data = put_user_input_dto.into_inner();
    match managers::user::modify_user_password(input_data.user_id, hash_password(input_data.password.clone()), user.brand_id) {
        Ok(new_user) => Ok(Json(new_user)),
        Err(status) => Err(status)
    }
}