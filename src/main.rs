pub mod models;
pub mod schema;
pub mod db_connect;
pub mod repositories;
pub mod controllers;
pub mod managers;
pub mod dto;
pub mod utils;
pub mod guards;
pub mod errors;
pub mod services;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header};

use rocket::{routes, Request, Response};

use rocket_dyn_templates::Template;
extern crate rocket_cors;


#[macro_use] extern crate rocket;

pub struct CORS;


#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PUT, DELETE, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}


#[launch]
fn rocket() -> _ {

    rocket::build()
        .register("/", catchers![errors::unauthorized, errors::forbidden, errors::notfound])
        .mount("/", routes![all_options])
        .mount("/user", routes![
                controllers::user::post_user,
                controllers::user::put_user
            ])
        .mount("/auth", routes![
            controllers::auth::login
        ])
        .mount("/message", routes![
                controllers::message::post_message,
                controllers::message::get_message
            ])
        .attach(Template::fairing())
        .attach(CORS)

}

