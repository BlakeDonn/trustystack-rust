// src/models/parts/storage_spec.rs

use crate::diesel_schema::parts::storage_specs;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = storage_specs)]
pub struct StorageSpec {
    pub part_id: i32,
    pub capacity: Option<i32>,
    pub interface: Option<String>,
    pub form_factor: Option<String>,
    pub sequential_read_speed: Option<i32>,
    pub sequential_write_speed: Option<i32>,
    pub nand_type: Option<String>,
    pub controller: Option<String>,
    pub endurance: Option<i32>,
    pub encryption_support: Option<bool>,
}
