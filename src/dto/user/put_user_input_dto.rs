use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]

pub struct PutUserInputDto {
    pub password: String,
    pub user_id: i32,
}