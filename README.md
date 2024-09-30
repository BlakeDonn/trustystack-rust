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

```plaintext
docker run -p 8080:8080 --env-file .env trustystack-rust-backend
```

## Running Tests

```plaintext
cargo test
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
