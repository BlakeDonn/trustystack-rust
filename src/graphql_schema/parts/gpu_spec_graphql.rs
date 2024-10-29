// src/graphql_schema/parts/gpu_spec_graphql.rs

use crate::graphql_schema::context::Context;
use crate::models::parts::gpu_spec::GpuSpec;
use juniper::graphql_object;

/// `GpuSpecGraphQL` struct representing GPU specifications in the GraphQL schema.
pub struct GpuSpecGraphQL {
    pub part_id: i32,
    pub cuda_cores: Option<i32>,
    pub vram_size: Option<String>,
    pub vram_type: Option<String>,
    pub tdp: Option<i32>,
    pub memory_bandwidth: Option<String>,
    pub interface: Option<String>,
    pub form_factor: Option<String>,
    pub outputs: Option<Vec<Option<String>>>,
    pub length: Option<i32>,
}

impl GpuSpecGraphQL {
    /// Converts a `GpuSpec` model into a `GpuSpecGraphQL`.
    pub fn from_gpu_spec(gpu_spec: GpuSpec) -> Self {
        GpuSpecGraphQL {
            part_id: gpu_spec.part_id,
            cuda_cores: gpu_spec.cuda_cores,
            vram_size: gpu_spec.vram_size.map(|v| v.to_string()),
            vram_type: gpu_spec.vram_type,
            tdp: gpu_spec.tdp,
            memory_bandwidth: gpu_spec.memory_bandwidth.map(|mb| mb.to_string()),
            interface: gpu_spec.interface,
            form_factor: gpu_spec.form_factor,
            outputs: gpu_spec.outputs,
            length: gpu_spec.length,
        }
    }
}

#[graphql_object(context = Context)]
impl GpuSpecGraphQL {
    fn part_id(&self) -> i32 {
        self.part_id
    }

    fn cuda_cores(&self) -> Option<i32> {
        self.cuda_cores
    }

    fn vram_size(&self) -> Option<&str> {
        self.vram_size.as_deref()
    }

    fn vram_type(&self) -> Option<&str> {
        self.vram_type.as_deref()
    }

    fn tdp(&self) -> Option<i32> {
        self.tdp
    }

    fn memory_bandwidth(&self) -> Option<&str> {
        self.memory_bandwidth.as_deref()
    }

    fn interface(&self) -> Option<&str> {
        self.interface.as_deref()
    }

    fn form_factor(&self) -> Option<&str> {
        self.form_factor.as_deref()
    }

    fn outputs(&self) -> Option<&[Option<String>]> {
        self.outputs.as_deref()
    }

    fn length(&self) -> Option<i32> {
        self.length
    }
}
