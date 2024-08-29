# Project Overview

This project is organized into several key modules that manage backend logic, database interactions, and the GraphQL API. Below is a detailed overview of the most critical components and their functions.

## Project Structure
```plaintext
.
├── Cargo.toml
├── README.md
├── diesel.toml
├── migrations
│   ├── 00000000000000_diesel_initial_setup
│   │   ├── down.sql
│   │   └── up.sql
│   └── 2024-08-28-014654_create_cpu_table
│       ├── down.sql
│       └── up.sql
├── src
│   ├── diesel_schema
│   │   └── parts
│   ├── graphql_schema
│   │   ├── context.rs
│   │   ├── parts
│   │   ├── prebuilt
│   │   ├── root_query.rs
│   │   ├── service
│   │   └── software
│   ├── lib.rs
│   ├── main.rs
│   └── models
│       └── parts
└── tests
    └── integration_test.rs

15 directories, 12 files

```

## Environment Variables

- **`DATABASE_URL`**: Must be defined in a `.env` file, specifying the connection string for the PostgreSQL database. This URL is used to configure the connection pool that the application will use to interact with the database.

## Running the Application

1. Ensure your `.env` file is properly configured with `DATABASE_URL`.
2. Start the server using `cargo run`. The server will be accessible at `http://127.0.0.1:8080/graphql` for handling GraphQL queries.

## Key Files

### `main.rs`

- **Purpose**: Acts as the entry point for the application. Initializes and configures the Actix-Web server, manages environment variables, sets up a connection pool to the PostgreSQL database, and initializes the GraphQL schema.
- **Details**:
  - **Logger Initialization**: Uses `env_logger` to set up logging based on environment settings.
  - **Environment Management**: Loads environment variables from a `.env` file via `dotenv`.
  - **Database Connection Pool**: Sets up a connection pool using Diesel's `ConnectionManager` for PostgreSQL.
  - **GraphQL Schema**: The GraphQL schema is initialized and shared across requests using an `Arc`.
  - **HTTP Server Setup**: Configures and runs an Actix-Web server, handling requests to the `/graphql` endpoint.

### `lib.rs`

- **Purpose**: Serves as a central module that ties together the various components of the application, such as the GraphQL schema, context, and models.
- **Details**:
  - **Modules**: Organizes code into modules, including `diesel_schema` for ORM, `graphql_schema` for GraphQL API, and `models` for data models.
  - **GraphQL Request Handling**: Provides the `graphql_handler` function, which executes GraphQL requests and returns results as JSON.
  - **Logging**: Logs key actions such as receiving and executing GraphQL requests.



## GraphQL Schema Modules

### `cpu/query.rs`

This module handles the GraphQL queries related to CPUs, including fetching CPU data from the database and transforming it into the GraphQL structure.

- **CpuGraphql struct**: Represents the CPU structure used in GraphQL queries.
- **From<CPU> for CpuGraphql**: Converts the Diesel CPU model to the GraphQL CPU model.
- **CpuQuery struct**: Provides the CPU-related queries for GraphQL.
  - **get_cpus(context: &Context) -> FieldResult<Vec<CpuGraphql>>**: Fetches all CPUs from the database and returns them as a vector of `CpuGraphql` objects.

### `prebuilt.rs`

This module handles GraphQL queries related to prebuilts, providing details on various prebuilt computer systems.

- **Spec struct**: Represents the specification of a prebuilt computer.
- **Prebuilt struct**: Represents a prebuilt computer used in GraphQL queries.
- **PrebuiltQuery struct**: Provides the queries related to prebuilt computers for GraphQL.
  - **get_prebuilts() -> Vec<Prebuilt>**: Returns a list of prebuilt computers as a vector of `Prebuilt`.

### `service.rs`

This module manages the GraphQL queries related to services offered.

- **Service struct**: Represents a service used in GraphQL queries.
  - **name() -> &str**: Returns the name of the service.
  - **description() -> &str**: Returns the description of the service.
  - **image_url() -> &str**: Returns the image URL of the service.
- **ServiceQuery struct**: Provides the service-related queries for GraphQL.
  - **get_services() -> Vec<Service>**: Returns a list of services as a vector of `Service`.

### `software.rs`

This module handles GraphQL queries related to software solutions.

- **Software struct**: Represents a software used in GraphQL queries.
  - **name() -> &str**: Returns the name of the software.
  - **description() -> &str**: Returns the description of the software.
  - **image_url() -> &str**: Returns the image URL of the software.
- **SoftwareQuery struct**: Provides the software-related queries for GraphQL.
  - **get_softwares() -> Vec<Software>**: Returns a list of software solutions as a vector of `Software`.

### `context.rs`

This module is responsible for managing the database connection context used in GraphQL queries.

- **Context struct**: Represents the database connection context.
  - **new(db: Pool<ConnectionManager<PgConnection>>) -> Self**: Creates a new `Context` with the provided database pool.
  - **get_connection(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>, diesel::r2d2::PoolError>**: Retrieves a database connection from the pool.

### `graphql_schema/mod.rs`

This module serves as the entry point for all GraphQL schema-related modules.

- **Context module**: Provides the database connection context for GraphQL queries.
- **RootNode type**: Represents the schema’s root node, which combines the `RootQuery`, `EmptyMutation`, and `EmptySubscription` types.
- **create_schema() -> Schema**: Creates and returns the GraphQL schema with the root query, mutation, and subscription.

### `root_query.rs`

This module combines all the individual query modules into a single root query for the GraphQL schema.

- **RootQuery struct**: Combines queries from various domains like CPUs, prebuilts, services, and software.
  - **api_version(&self) -> &str**: Returns the API version.
  - **popular_prebuilts(&self, context: &Context) -> Vec<Prebuilt>**: Returns a list of popular prebuilts.
  - **services(&self, context: &Context) -> Vec<Service>**: Returns a list of services.
  - **software_solutions(&self, context: &Context) -> Vec<Software>**: Returns a list of software solutions.
  - **cpus(&self, context: &Context) -> Vec<CpuGraphql>**: Returns a list of CPUs.

## Diesel Schemas and Models

### `diesel_schema/parts/cpu.rs`

This file defines the schema for the `cpu` table using Diesel's `table!` macro. It maps the columns in the `cpu` table to their corresponding data types.

- **Schema Definition**:
  - `id -> Int4`: The primary key of the `cpu` table, represented as a 32-bit integer.
  - `name -> Varchar`: The name of the CPU, stored as a variable-length character string with a maximum length of 255 characters.
  - `price -> Numeric`: The price of the CPU, stored as a numeric value.
  - `core_count -> Int4`: The number of cores in the CPU, represented as a 32-bit integer.
  - `core_clock -> Varchar`: The base clock speed of the CPU, stored as a string with a maximum length of 50 characters.
  - `boost_clock -> Varchar`: The boost clock speed of the CPU, stored as a string with a maximum length of 50 characters.
  - `tdp -> Int4`: The thermal design power (TDP) of the CPU, represented as a 32-bit integer.
  - `integrated_graphics -> Nullable<Varchar>`: The integrated graphics of the CPU, stored as an optional string with a maximum length of 100 characters.
  - `smt -> Bool`: A boolean indicating whether simultaneous multithreading (SMT) is supported.

### `models/parts/cpu.rs`

This file defines the Rust struct that corresponds to the `cpu` table in the database. The struct is used by Diesel to map database records to Rust objects and vice versa.

- **CPU Struct**:
  - **Fields**:
    - `id: i32`: The primary key of the CPU record.
    - `name: String`: The name of the CPU.
    - `price: BigDecimal`: The price of the CPU, represented as a `BigDecimal` for precise numeric operations.
    - `core_count: i32`: The number of cores in the CPU.
    - `core_clock: String`: The base clock speed of the CPU.
    - `boost_clock: String`: The boost clock speed of the CPU.
    - `tdp: i32`: The thermal design power (TDP) of the CPU.
    - `integrated_graphics: Option<String>`: The integrated graphics of the CPU, represented as an optional string.
    - `smt: bool`: A boolean indicating whether simultaneous multithreading (SMT) is supported.

- **Attributes**:
  - `#[derive(Queryable, Insertable)]`: Indicates that the `CPU` struct can be used for querying and inserting data in the `cpu` table.
  - `#[diesel(table_name = cpu)]`: Associates the `CPU` struct with the `cpu` table in the database.

