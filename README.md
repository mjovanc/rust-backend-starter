# Rust Backend Starter

A Rust Backend Boilerplate using Actix, SQLite, OpenAPI/Swagger, and more.

This project serves as a starting point for building a robust backend in Rust, featuring key tools and libraries such as Actix for web server functionality, SQLite for database management, and OpenAPI for automatic API documentation.

---

## Features

- **Actix Web**: A powerful and fast web framework for Rust.
- **SQLite**: Lightweight, embedded SQL database for simple data persistence.
- **OpenAPI (Swagger)**: Auto-generate API documentation.
- **JWT Authentication**: Secure user authentication with JSON Web Tokens.
- **Environment Configuration**: Using `dotenvy` for managing environment variables.
- **Serde**: Easy serialization and deserialization of data structures.
- **Logging**: Integrated logging with `env_logger`.
- **CORS**: Cross-Origin Resource Sharing support for secure frontend-backend interactions.
- **DevOps**: Ready for containerization and deployment.

---

## Getting Started

Follow these instructions to set up and run the project on your local machine.

### Prerequisites

- **Rust**: Install [Rust](https://www.rust-lang.org/tools/install) 1.78+
- **SQLite**: Ensure SQLite is installed for local database use.

### Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/mjovanc/rust-backend-starter.git
    cd rust-backend-starter
    ```

2. Install dependencies:

    ```bash
    cargo build
    ```

3. Set up environment variables:

   Create a `.env` file in the root directory and configure your environment variables:

    ```env
    DATABASE_URL=sqlite://data.db
    SECRET_KEY=your_jwt_secret_key
    ```

5. Start the development server:

    ```bash
    cargo run
    ```

The server should now be running at `http://localhost:8080`.