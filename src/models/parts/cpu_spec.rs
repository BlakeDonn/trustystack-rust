// src/models/parts/cpu_spec.rs

use crate::diesel_schema::parts::cpu_specs;
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = cpu_specs)]
pub struct CpuSpec {
    pub part_id: i32,
    pub cores: Option<i32>,
    pub threads: Option<i32>,
    pub base_clock_speed: Option<BigDecimal>,
    pub max_boost_clock_speed: Option<BigDecimal>,
    pub tdp: Option<i32>,
    pub socket_type: Option<String>,
    pub cache_size: Option<BigDecimal>,
    pub integrated_graphics: Option<bool>,
    pub process_technology: Option<BigDecimal>,
}
