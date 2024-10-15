use crate::types::errors::DataImportError;
use csv::ReaderBuilder;
use diesel::insert_into;
use diesel::pg::PgConnection;
use diesel::prelude::*;

/// Imports data from CSV files into the database.
///
/// # Arguments
///
/// * `conn` - A mutable reference to the database connection.
///
/// # Returns
///
/// * `Ok(())` if data import is successful.
/// * `Err(DataImportError)` if an error occurs.
pub fn run_data_import(conn: &mut PgConnection) -> Result<(), DataImportError> {
    import_manufacturers(conn)?;
    import_categories(conn)?;
    import_parts(conn)?;
    import_cpu_specs(conn)?;
    import_gpu_specs(conn)?;
    import_memory_specs(conn)?;
    import_storage_specs(conn)?;
    Ok(())
}

fn import_manufacturers(conn: &mut PgConnection) -> Result<(), DataImportError> {
    import_manufacturers_with_path(conn, "./data/csv/manufacturers.csv")
}

pub fn import_manufacturers_with_path(
    conn: &mut PgConnection,
    path: &str,
) -> Result<(), DataImportError> {
    use crate::diesel_schema::parts::manufacturers::dsl::*;
    use crate::models::parts::manufacturer::Manufacturer;

    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(path)?;

    for result in rdr.deserialize() {
        let record: Manufacturer = result?;
        insert_into(manufacturers)
            .values(&record)
            .on_conflict(id)
            .do_nothing()
            .execute(conn)?;
    }

    Ok(())
}

fn import_categories(conn: &mut PgConnection) -> Result<(), DataImportError> {
    use crate::diesel_schema::parts::categories::dsl::*;
    use crate::models::parts::category::Category;

    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path("./data/csv/categories.csv")?;

    for result in rdr.deserialize() {
        let record: Category = result?;
        insert_into(categories)
            .values(&record)
            .on_conflict(id)
            .do_nothing()
            .execute(conn)?;
    }

    Ok(())
}

fn import_parts(conn: &mut PgConnection) -> Result<(), DataImportError> {
    use crate::diesel_schema::parts::parts::dsl::*;
    use crate::models::parts::part::Part;

    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .double_quote(true)
        .from_path("./data/csv/parts.csv")?;

    for result in rdr.deserialize() {
        let record: Part = result?;

        insert_into(parts)
            .values(&record)
            .on_conflict(id)
            .do_nothing()
            .execute(conn)?;
    }

    Ok(())
}

fn import_cpu_specs(conn: &mut PgConnection) -> Result<(), DataImportError> {
    use crate::diesel_schema::parts::cpu_specs::dsl::*;
    use crate::models::parts::cpu_spec::CpuSpec;

    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path("./data/csv/cpu_specs.csv")?;

    for result in rdr.deserialize() {
        let record: CpuSpec = result?;
        insert_into(cpu_specs)
            .values(&record)
            .on_conflict(part_id)
            .do_nothing()
            .execute(conn)?;
    }

    Ok(())
}

fn import_gpu_specs(conn: &mut PgConnection) -> Result<(), DataImportError> {
    use crate::diesel_schema::parts::gpu_specs::dsl::*;
    use crate::models::parts::gpu_spec::GpuSpec;

    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path("./data/csv/gpu_specs.csv")?;

    for result in rdr.deserialize() {
        let mut record: GpuSpec = result?;

        // Deserialize outputs from JSON array string
        if let Some(outputs_str) = record.outputs.as_ref() {
            let outputs_vec: Vec<String> = outputs_str.to_vec();
            record.outputs = Some(outputs_vec);
        }

        insert_into(gpu_specs)
            .values(&record)
            .on_conflict(part_id)
            .do_nothing()
            .execute(conn)?;
    }

    Ok(())
}

fn import_memory_specs(conn: &mut PgConnection) -> Result<(), DataImportError> {
    use crate::diesel_schema::parts::memory_specs::dsl::*;
    use crate::models::parts::memory_spec::MemorySpec;

    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path("./data/csv/memory_specs.csv")?;

    for result in rdr.deserialize() {
        let record: MemorySpec = result?;
        insert_into(memory_specs)
            .values(&record)
            .on_conflict(part_id)
            .do_nothing()
            .execute(conn)?;
    }

    Ok(())
}

fn import_storage_specs(conn: &mut PgConnection) -> Result<(), DataImportError> {
    use crate::diesel_schema::parts::storage_specs::dsl::*;
    use crate::models::parts::storage_spec::StorageSpec;

    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path("./data/csv/storage_specs.csv")?;

    for result in rdr.deserialize() {
        let record: StorageSpec = result?;
        insert_into(storage_specs)
            .values(&record)
            .on_conflict(part_id)
            .do_nothing()
            .execute(conn)?;
    }

    Ok(())
}
