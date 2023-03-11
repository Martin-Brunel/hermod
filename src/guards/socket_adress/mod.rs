use rocket::{
    request::{Outcome},
    Request,
};
use serde::Serialize;


#[derive(Debug)]
pub struct RequestSocketAddr {
    pub socket_addr: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorObject {
    message: String,
}

#[rocket::async_trait]
impl<'r>  rocket::request::FromRequest<'r> for RequestSocketAddr {
    type Error = ErrorObject;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let socket_addr: String = req.remote().unwrap().to_string();

        Outcome::Success(Self {
            socket_addr: socket_addr,
        })
    }
}