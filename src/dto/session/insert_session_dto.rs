use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::{ schema::session};

#[derive(Deserialize, Serialize, Insertable, Queryable, Clone, Debug)]
#[diesel(table_name = session)]
pub struct InsertSessionDto {
    pub user_id: i32
}