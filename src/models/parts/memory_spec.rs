// src/models/parts/memory_spec.rs

use crate::diesel_schema::parts::memory_specs;
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Selectable, Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = memory_specs)]
pub struct MemorySpec {
    pub part_id: i32,
    pub capacity: Option<i32>,
    pub speed: Option<i32>,
    pub memory_type: Option<String>,
    pub ecc: Option<bool>,
    pub buffered: Option<bool>,
    pub cas_latency: Option<BigDecimal>,
    pub form_factor: Option<String>,
    pub rgb_lighting: Option<bool>,
    pub kit_configuration: Option<String>,
    pub voltage: Option<BigDecimal>,
    pub heat_spreader: Option<bool>,
}
