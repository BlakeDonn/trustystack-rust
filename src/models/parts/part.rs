use crate::diesel_schema::parts::parts;
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = parts)]
pub struct Part {
    pub id: i32,
    pub manufacturer_id: i32,
    pub category_id: i32,
    pub name: String,
    pub model: String,
    pub price: Option<BigDecimal>,
    #[serde(deserialize_with = "deserialize_optional_json")]
    pub common_specifications: Option<JsonValue>,
}

use serde::de::{self, Deserializer};

pub fn deserialize_optional_json<'de, D>(deserializer: D) -> Result<Option<JsonValue>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(ref s) if !s.is_empty() => {
            let json = serde_json::from_str(s).map_err(de::Error::custom)?;
            Ok(Some(json))
        }
        _ => Ok(None),
    }
}
