use jsonwebtoken::{Algorithm, decode, Validation, DecodingKey};
use rocket::{Request, request::{ Outcome}, http::Status};
use dotenvy::dotenv;
use serde::Serialize;
use std::env;

use crate::{utils::jwt::JWT, repositories, models::user::User, errors::ErrorObject};
pub struct GetSecurity {
   pub user: User,
}

#[derive(Debug, Serialize)]
pub enum ApiKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for GetSecurity {
    type Error = ErrorObject;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        dotenv().ok(); 
        let key = match req.query_value::<String>("token") {
            Some(token) => token,
            _ => return  Outcome::Failure((Status::Forbidden, ErrorObject {
                message: String::from("No token found")
            })) 
        };

        let token = match key {
            Ok(token) => token,
            Err(_) => return Outcome::Failure((Status::Forbidden, ErrorObject {
                message: String::from("No token found")
            })) 
        };
        
        let jwt_secret = env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set");

        let jwt = token;
        match decode::<JWT>(jwt.as_ref(), &DecodingKey::from_secret(jwt_secret.as_ref()), &Validation::new(Algorithm::HS256)) {
            Ok(token) => {
                match repositories::user::get_by_email(token.claims.email) {
                    Ok(user) => {
                        let security = GetSecurity {
                            user: user
                        };
                        Outcome::Success(security)
                    },
                    Err(_e) => Outcome::Failure((Status::Forbidden, ErrorObject {
                        message: String::from("No user found")
                    })) 
                }
            },
            Err(_e) => {
                Outcome::Failure((Status::Forbidden, ErrorObject {
                    message: String::from("No valid token")
                }))
            }
        }

    }
}