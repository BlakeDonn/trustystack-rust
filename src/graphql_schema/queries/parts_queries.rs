use crate::graphql_schema::context::Context;
use crate::graphql_schema::parts::part_graphql::PartGraphQL;
use crate::models::parts::part::Part;
use diesel::prelude::*;
use juniper::FieldResult;
use log::error;

/// Fetches all parts from the database with optional pagination.
pub fn get_all_parts(
    context: &Context,
    limit_val: Option<i32>,
    offset_val: Option<i32>,
) -> FieldResult<Vec<PartGraphQL>> {
    use crate::diesel_schema::parts::parts::dsl::*;

    let mut conn = context.get_connection().map_err(|e| {
        error!("Database connection error: {}", e);
        juniper::FieldError::new(
            "Database connection error",
            juniper::Value::scalar(e.to_string()),
        )
    })?;

    let mut query = parts.into_boxed();

    if let Some(l) = limit_val {
        query = query.limit(l as i64); // Diesel expects i64 for limit and offset
    }

    if let Some(o) = offset_val {
        query = query.offset(o as i64);
    }

    let part_list = query
        .select(Part::as_select())
        .load::<Part>(&mut conn)
        .map_err(|e| {
            error!("Error fetching parts: {}", e);
            juniper::FieldError::new(
                "Error fetching parts",
                juniper::Value::scalar(e.to_string()),
            )
        })?;

    let graphql_parts = part_list.into_iter().map(PartGraphQL::from_part).collect();

    Ok(graphql_parts)
}

/// Fetches a specific part by ID from the database.
pub fn get_part_by_id(context: &Context, part_id_val: i32) -> FieldResult<Option<PartGraphQL>> {
    use crate::diesel_schema::parts::parts::dsl::*;

    let mut conn = context.get_connection().map_err(|e| {
        error!("Database connection error: {}", e);
        juniper::FieldError::new(
            "Database connection error",
            juniper::Value::scalar(e.to_string()),
        )
    })?;

    let part_opt = parts
        .filter(id.eq(part_id_val))
        .select(Part::as_select())
        .first::<Part>(&mut conn)
        .optional()
        .map_err(|e| {
            error!("Error fetching part: {}", e);
            juniper::FieldError::new("Error fetching part", juniper::Value::scalar(e.to_string()))
        })?;

    match part_opt {
        Some(part) => Ok(Some(PartGraphQL::from_part(part))),
        None => Ok(None),
    }
}
