use crate::graphql_schema::context::Context;
//use crate::graphql_schema::parts::cpu::query::{CpuGraphql, CpuQuery};
use crate::graphql_schema::prebuilt::query::{Prebuilt, PrebuiltQuery};
use crate::graphql_schema::service::query::{Service, ServiceQuery};
use crate::graphql_schema::software::query::{Software, SoftwareQuery};
use juniper::FieldResult;

/// RootQuery struct that defines the available GraphQL queries in the API.
pub struct RootQuery;

#[juniper::graphql_object(Context = Context)]
impl RootQuery {
    /// Returns the current API version.
    fn api_version(&self) -> &str {
        log::info!("api_version query resolved.");
        "1.0"
    }

    /// Fetches popular prebuilts from the database.
    fn popular_prebuilts(&self) -> FieldResult<Vec<Prebuilt>> {
        log::info!("popularPrebuilts query resolved.");
        Ok(PrebuiltQuery::get_prebuilts())
    }

    /// Fetches services from the database.
    fn services(&self) -> FieldResult<Vec<Service>> {
        log::info!("services query resolved.");
        Ok(ServiceQuery::get_services())
    }

    /// Fetches software solutions from the database.
    fn software_solutions(&self) -> FieldResult<Vec<Software>> {
        log::info!("softwareSolutions query resolved.");
        Ok(SoftwareQuery::get_softwares())
    }
}
