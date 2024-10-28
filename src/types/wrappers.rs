use bigdecimal::BigDecimal;
use juniper::{GraphQLScalarValue, InputValue, Value};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BigDecimalWrapper(pub BigDecimal);

#[juniper::graphql_scalar(description = "A custom scalar for BigDecimal, represented as a string")]
impl<S> GraphQLScalar for BigDecimalWrapper
where
    S: juniper::ScalarValue,
{
    fn resolve(&self) -> Value {
        Value::scalar(self.0.to_string())
    }

    fn from_input_value(value: &InputValue) -> Option<BigDecimalWrapper> {
        value
            .as_string_value()
            .and_then(|s| BigDecimal::from_str(s).ok())
            .map(BigDecimalWrapper)
    }

    fn from_str(value: &str) -> juniper::ParseScalarResult<S> {
        Self::from_str(value)
    }
}

