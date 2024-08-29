use crate::graphql_schema::context::Context;
use crate::graphql_schema::parts::cpu::query::CpuQuery;
use crate::graphql_schema::prebuilt::query::PrebuiltQuery;
use crate::graphql_schema::service::query::ServiceQuery;
use crate::graphql_schema::software::query::SoftwareQuery;

pub struct RootQuery;

#[juniper::graphql_object(Context = Context)]
impl RootQuery {
    fn api_version(&self) -> &str {
        "1.0"
    }

    fn service_queries(&self) -> ServiceQuery {
        ServiceQuery
    }

    fn software_queries(&self) -> SoftwareQuery {
        SoftwareQuery
    }

    fn prebuilt_queries(&self) -> PrebuiltQuery {
        PrebuiltQuery
    }

    fn cpu_query(&self) -> CpuQuery {
        CpuQuery
    }
}
