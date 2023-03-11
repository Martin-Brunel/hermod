use jsonwebtoken::{Algorithm, decode, Validation, DecodingKey};
use rocket::{Request, request::{ Outcome}, http::Status};
use dotenvy::dotenv;
use serde::Serialize;
use std::env;

use crate::{utils::jwt::JWT, repositories, models::user::User, errors::ErrorObject};
pub struct UserAdmin {
   pub user: User,
}

#[derive(Debug, Serialize)]
pub enum ApiKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for UserAdmin {
    type Error = ErrorObject;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        dotenv().ok(); 
        let keys: Vec<_> = req.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Failure((rocket::http::Status::Forbidden, ErrorObject {
                message: String::from("No token found")
            }));
        }
        let jwt_secret = env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set");

        let jwt = keys[0];
        let jwt = jwt.replace("Bearer ", "");
        
        match decode::<JWT>(jwt.as_ref(), &DecodingKey::from_secret(jwt_secret.as_ref()), &Validation::new(Algorithm::HS256)) {
            Ok(token) => {
                match repositories::user::get_by_email(token.claims.email) {
                    Ok(user) => {
                        let roles: Vec<String> = serde_json::from_str(&user.roles[..]).unwrap();
                        let role = String::from("ROLE_ADMIN");
                        match roles.contains(&role) {
                            true => {
                                let security = UserAdmin {
                                    user: user
                                };
                                Outcome::Success(security)
                            },
                            _ => {
                                return Outcome::Failure((rocket::http::Status::Forbidden, ErrorObject {
                                    message: String::from("not admin user")
                                }))
                            }
                        }  
                    },
                    Err(_e) => Outcome::Failure((rocket::http::Status::Forbidden, ErrorObject {
                        message: String::from("No user found")
                    })) 
                }
            },
            Err(_e) => {
                Outcome::Failure((Status::Unauthorized, ErrorObject {
                    message: String::from("No valid token")
                }))
            }
        }

    }
}