use diesel::{Queryable};
use serde::{Serialize, Deserialize};



#[derive(Deserialize, Serialize, Queryable, Clone, Debug)]
#[diesel(table_name = message)]
pub struct InputMessageDto {
    pub user_id: i32,
    pub data: String,
}