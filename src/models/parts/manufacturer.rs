// src/models/parts/manufacturer.rs

use crate::diesel_schema::parts::manufacturers;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = manufacturers)]
pub struct Manufacturer {
    pub id: i32,
    pub name: String,
    pub website: Option<String>,
}
