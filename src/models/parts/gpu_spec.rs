// src/models/parts/gpu_spec.rs

use crate::diesel_schema::parts::gpu_specs;
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = gpu_specs)]
pub struct GpuSpec {
    pub part_id: i32,
    pub cuda_cores: Option<i32>,
    pub vram_size: Option<BigDecimal>,
    pub vram_type: Option<String>,
    pub tdp: Option<i32>,
    pub memory_bandwidth: Option<BigDecimal>,
    pub interface: Option<String>,
    pub form_factor: Option<String>,
    pub outputs: Option<Vec<String>>,
    pub length: Option<i32>,
}
