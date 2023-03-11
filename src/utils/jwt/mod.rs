use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::Utc;
use serde::{Serialize, Deserialize};
use crate::models::user::User;
use dotenvy::dotenv;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct JWT {
    pub email: String,
    pub id: i32,
    pub iat: i64,
    pub exp: i64,
    pub roles: Vec<String>,
    pub firstname: String,
    pub lastname: String,
}


pub fn create_jwt(data: User) -> String {
    dotenv().ok(); 
    let now = Utc::now().timestamp();
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(env::var("JWT_TOKEN_DURATION").unwrap().parse::<i64>().unwrap()))
        .expect("valid timestamp")
        .timestamp();
        
    let data = JWT {
        email: data.email,
        id: data.id,
        iat: now,
        exp: expiration,
        roles: serde_json::from_str(&data.roles).unwrap(),
        firstname: data.firstname,
        lastname: data.lastname 
    };

    let jwt_secret = env::var("JWT_SECRET") //on tente de récuperer l'url de la BDD depuis l'environnement
            .expect("JWT_SECRET must be set");

    encode(&Header::default(), &data, &EncodingKey::from_secret(jwt_secret.as_ref())).unwrap()
}