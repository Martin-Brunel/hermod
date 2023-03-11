use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}
