use crate::models::parts::cpu_spec::CpuSpec;
use juniper::GraphQLObject;

/// `CpuSpecGraphQL` struct representing CPU Specifications in the GraphQL schema.
#[derive(GraphQLObject)]
#[graphql(description = "CPU Specifications")]
pub struct CpuSpecGraphQL {
    pub part_id: i32,
    pub cores: Option<i32>,
    pub threads: Option<i32>,
    pub base_clock_speed: Option<String>,
    pub max_boost_clock_speed: Option<String>,
    pub tdp: Option<i32>,
    pub socket_type: Option<String>,
    pub cache_size: Option<String>,
    pub integrated_graphics: Option<bool>,
    pub process_technology: Option<String>,
}

impl CpuSpecGraphQL {
    /// Converts a `CpuSpec` model into a `CpuSpecGraphQL`.
    pub fn from_cpu_spec(cpu_spec: CpuSpec) -> Self {
        CpuSpecGraphQL {
            part_id: cpu_spec.part_id,
            cores: cpu_spec.cores,
            threads: cpu_spec.threads,
            base_clock_speed: cpu_spec.base_clock_speed.map(|b| b.to_string()),
            max_boost_clock_speed: cpu_spec.max_boost_clock_speed.map(|m| m.to_string()),
            tdp: cpu_spec.tdp,
            socket_type: cpu_spec.socket_type,
            cache_size: cpu_spec.cache_size.map(|c| c.to_string()),
            integrated_graphics: cpu_spec.integrated_graphics,
            process_technology: cpu_spec.process_technology.map(|p| p.to_string()),
        }
    }
}
