use crate::diesel_schema::parts::cpu::cpu as DieselCpu;
use crate::graphql_schema::context::Context;
use crate::models::parts::cpu::CPU;
use diesel::prelude::*;
use juniper::{FieldResult, GraphQLObject};

/// Represents the CPU structure used in GraphQL queries.
#[derive(GraphQLObject)]
pub struct CpuGraphql {
    pub id: i32,
    pub name: String,
    pub price: String,
    pub core_count: i32,
    pub core_clock: String,
    pub boost_clock: String,
    pub tdp: i32,
    pub integrated_graphics: Option<String>,
    pub smt: bool,
}

/// Converts the Diesel CPU model to the GraphQL CPU model.
impl From<CPU> for CpuGraphql {
    fn from(cpu: CPU) -> Self {
        CpuGraphql {
            id: cpu.id,
            name: cpu.name,
            price: cpu.price.to_string(),
            core_count: cpu.core_count,
            core_clock: cpu.core_clock,
            boost_clock: cpu.boost_clock,
            tdp: cpu.tdp,
            integrated_graphics: cpu.integrated_graphics,
            smt: cpu.smt,
        }
    }
}

/// Provides the CPU-related queries for GraphQL.
pub struct CpuQuery;

impl CpuQuery {
    /// Fetches all CPUs from the database and returns them as a vector of `CpuGraphql` objects.
    pub fn get_cpus(context: &Context) -> FieldResult<Vec<CpuGraphql>> {
        let mut connection = context.get_connection()?;
        let items = DieselCpu::table.load::<CPU>(&mut connection)?;
        Ok(items.into_iter().map(CpuGraphql::from).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::parts::cpu::CPU;
    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    #[test]
    fn test_cpu_graphql_conversion() {
        let cpu = CPU {
            id: 1,
            name: String::from("Test CPU"),
            price: BigDecimal::from_str("299.99").unwrap(),
            core_count: 8,
            core_clock: String::from("3.5GHz"),
            boost_clock: String::from("4.2GHz"),
            tdp: 65,
            integrated_graphics: Some(String::from("Intel UHD")),
            smt: true,
        };

        let cpu_graphql = CpuGraphql::from(cpu);

        assert_eq!(cpu_graphql.id, 1);
        assert_eq!(cpu_graphql.name, "Test CPU");
        assert_eq!(cpu_graphql.core_count, 8);
        assert_eq!(cpu_graphql.integrated_graphics.unwrap(), "Intel UHD");
    }
}
