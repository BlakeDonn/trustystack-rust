# Rust / GQL api

[![Deployment](https://github.com/BlakeDonn/trustystack-rust/actions/workflows/trigger-infra.yml/badge.svg)](https://github.com/blakedonn/trustystack-rust/actions)

[![Doc Generation](https://github.com/BlakeDonn/trustystack-rust/actions/workflows/doc-gen.yml/badge.svg)](https://github.com/blakedonn/trustystack-rust/actions)

[![Tests](https://github.com/BlakeDonn/trustystack-rust/actions/workflows/rust-tests.yml/badge.svg)](https://github.com/blakedonn/trustystack-rust/actions)

## Table of Contents

- [Overview](#overview)
- [Documentation](#documentation)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Configuration](#configuration)
- [Database Setup](#database-setup)
- [Running the Application](#running-the-application)
- [Running Tests](#running-tests)
- [Workflow Instructions](#workflow-instructions)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

## Overview

The TrustyStack Rust Backend is a GraphQL API built with Rust, Actix-Web, Diesel, and Juniper. It provides endpoints for managing computer parts information, integrating with a PostgreSQL database.

## Documentation

Comprehensive documentation is available at:

🔗 **[TrustyStack Rust Backend Documentation](https://blakedonn.github.io/trustystack-rust/rust_backend/index.html)**

## Prerequisites

- **Rust** (latest stable version)
- **Cargo** (comes with Rust)
- **PostgreSQL** (version 12 or higher)
- **Diesel CLI** (for database migrations)
- **Docker** (optional, for containerized deployment)

## Installation

### Clone the Repository

```plaintext
git clone https://github.com/yourusername/trustystack-rust.git
cd trustystack-rust
```

### Install Dependencies

```plaintext
cargo build
```

## Configuration

### Environment Variables

Create a `.env` file in the root directory with the following content:

```plaintext
DATABASE_URL=postgres://username:password@localhost/database_name
TEST_DATABASE_URL=postgres://username:password@localhost/test_database_name
RUST_LOG=info
```

> **Note:** Replace `username`, `password`, `database_name`, and `test_database_name` with your actual PostgreSQL credentials and database names.

## Database Setup

### Install Diesel CLI

```plaintext
cargo install diesel_cli --no-default-features --features postgres
```

### Run Migrations

```plaintext
diesel setup
diesel migration run
```

## Running the Application

### Running with Cargo

```plaintext
cargo run
```

The server will start at [http://0.0.0.0:8080](http://0.0.0.0:8080).

### Running with Docker

#### Build the Docker Image

```plaintext
docker build -t trustystack-rust-backend .
```

#### Run the Docker Container
:wa

```plaintext
docker run -p 8080:8080 --env-file .env trustystack-rust-backend
```

## Running Tests

```plaintext
cargo test
```
## Layout
```plaintext
.
├── .env
├── .github
│   └── workflows
│       ├── doc-gen.yml
│       ├── rust-tests.yml
│       └── trigger-infra.yml
├── .gitignore
├── Cargo.toml
├── Dockerfile
├── README.md
├── data
│   └── csv
│       ├── categories.csv
│       ├── cpu_specs.csv
│       ├── gpu_specs.csv
│       ├── manufacturers.csv
│       ├── memory_specs.csv
│       ├── parts.csv
│       └── storage_specs.csv
├── diesel.toml
├── entrypoint.sh
├── example.env
├── migrations
│   ├── .keep
│   ├── 00000000000000_diesel_initial_setup
│   │   ├── down.sql
│   │   └── up.sql
│   ├── 001_2024-10-01-003932_enable_uuid_extension
│   │   ├── down.sql
│   │   └── up.sql
│   ├── 2024-10-02-182125_create_manufacturers_and_categories
│   │   ├── down.sql
│   │   └── up.sql
│   ├── 2024-10-02-182135_create_parts
│   │   ├── down.sql
│   │   └── up.sql
│   ├── 2024-10-02-182142_create_part_specifications
│   │   ├── down.sql
│   │   └── up.sql
│   └── 2024-10-02-182155_create_configurations
│       ├── down.sql
│       └── up.sql
├── src
│   ├── bin
│   │   └── migrate.rs
│   ├── data_import.rs
│   ├── diesel_schema
│   │   ├── configurations.rs
│   │   ├── mod.rs
│   │   ├── parts.rs
│   ├── graphql_handler.rs
│   ├── graphql_schema
│   │   ├── context.rs
│   │   ├── custom_scalars
│   │   │   └── big_decimal_scalar.rs
│   │   ├── mod.rs
│   │   ├── parts
│   │   │   ├── category_graphql.rs
│   │   │   ├── cpu_spec_graphql.rs
│   │   │   ├── gpu_spec_graphql.rs
│   │   │   ├── manufacturer_graphql.rs
│   │   │   ├── memory_spec_graphql.rs
│   │   │   ├── mod.rs
│   │   │   ├── part_graphql.rs
│   │   │   └── storage_spec_graphql.rs
│   │   ├── prebuilt
│   │   │   ├── mod.rs
│   │   │   └── query.rs
│   │   ├── queries
│   │   │   ├── categories_queries.rs
│   │   │   ├── manufacturers_queries.rs
│   │   │   ├── mod.rs
│   │   │   └── parts_queries.rs
│   │   ├── root_query.rs
│   │   ├── schema.rs
│   │   ├── service
│   │   │   ├── mod.rs
│   │   │   └── query.rs
│   │   ├── software
│   │   │   ├── mod.rs
│   │   │   └── query.rs
│   │   └── users
│   ├── lib.rs
│   ├── main.rs
│   ├── middleware
│   │   ├── auth.rs
│   │   ├── logging.rs
│   │   ├── mod.rs
│   │   └── timing.rs
│   ├── migration.rs
│   ├── models
│   │   ├── auth
│   │   │   ├── mod.rs
│   │   ├── mod.rs
│   │   └── parts
│   │       ├── category.rs
│   │       ├── cpu_spec.rs
│   │       ├── gpu_spec.rs
│   │       ├── manufacturer.rs
│   │       ├── memory_spec.rs
│   │       ├── mod.rs
│   │       ├── part.rs
│   │       └── storage_spec.rs
│   └── types
│       ├── errors.rs
│       ├── mod.rs
│       └── wrappers.rs
└── tests
    ├── data_import_tests.rs
    ├── integration_test.rs
    ├── migration_tests.rs
    └── utils.rs

30 directories, 88 files
.github/workflows
├── doc-gen.yml
├── rust-tests.yml
└── trigger-infra.yml
```
## Workflow Instructions

### Building the Application

#### Development Build

```plaintext
cargo build
```

#### Release Build

```plaintext
cargo build --release
```

### Running Migrations Programmatically

You can run migrations using the embedded migration scripts:

```plaintext
cargo run --bin migrate
```

### Linting and Formatting

Ensure your code adheres to Rust standards:

```plaintext
cargo fmt
cargo clippy
```

### Generating Documentation

```plaintext
cargo doc --no-deps --document-private-items
```

Generated documentation can be found in the `target/doc` directory.

### Test Coverage

To generate a test coverage report using `cargo tarpaulin`:

```plaintext
cargo tarpaulin --out Html
```

The coverage report will be located at `tarpaulin-report.html`.

## Continuous Integration and Deployment

The project uses GitHub Actions for CI/CD. Workflows are defined in the `.github/workflows` directory.

### Workflows

- **Build and Test:** Automatically builds the project and runs tests on each push.
- **Deploy Documentation:** Deploys generated documentation to GitHub Pages when changes are pushed to the `deploy-pages` branch.
- **Code Coverage:** Generates test coverage reports using `cargo tarpaulin`.

### GitHub Pages Deployment

Documentation is automatically deployed to GitHub Pages on push to deploy-pages.

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository.
2. Create a new feature branch (`git checkout -b feature/my-feature`).
3. Commit your changes (`git commit -am 'Add new feature'`).
4. Push to the branch (`git push origin feature/my-feature`).
5. Open a Pull Request.

### Guidelines

- Write clear, concise commit messages.
- Ensure all tests pass before submitting a PR.
- Update documentation and add tests for new features.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact
