# Microblog

A simple microblogging REST API implemented in Rust using the **Axum** server framework and **Hyper**. The project supports the creation and retrieval of blog posts stored in an SQLite database powered by `sqlx`.

## Features

- Create posts with a `title` and `content`.
- Retrieve all posts as a JSON array.
- In-memory testing using `tokio` and `reqwest`.
- SQLite as the backend database with support for migrations.

---

## Project Structure

```plaintext
.
├── migrations/          # Database schema and migration scripts
├── Cargo.toml           # Dependency configurations
├── src/
│   ├── main.rs          # Main entry point for the application
├── tests/
│   ├── e2e_tests.rs     # End-to-end tests
```

### Highlights

- Serialization and deserialization using `serde`.
- Routes handled by `hyper` directly with a JSON-based interface.
- SQLite database schema defined under `migrations/`.

---

## Installation and Setup

Follow these steps to set up the project and run the REST API:

### Prerequisites

- Rust (`>=1.70` recommended)
- SQLite (`>=3.x`)
- Install the migration tool for `sqlx` if you want to manage migrations.

### Clone the Repository

```bash
git clone https://github.com/mateusz-gorny/rust-poc.git
cd microblog
```

### Configure SQLite

The database expects an SQLite file. The default SQLite database is `microblog.db`.

#### Run Migrations

You can create and migrate the database schema using the script located under the `migrations/` directory:

```sql
-- Create the `posts` table
CREATE TABLE posts (
   id TEXT PRIMARY KEY,
   title TEXT NOT NULL,
   content TEXT NOT NULL
);
```

Alternatively, use the `sqlx` CLI (you must have `sqlx-cli` installed):

```bash
cargo install sqlx-cli --no-default-features --features "sqlite"
sqlx migrate run
```

---

## Usage

### Run the Server

To run the API server locally:

```bash
cargo run
```

The server will be available at `http://127.0.0.1:3000`.

---

### REST API Endpoints

#### 1. Fetch all posts

**GET** `/posts`

- **Description**: Retrieve all posts stored in the database as a JSON array.
- **Response Example**:

```json
[
    {
        "id": "6f7b88a4-5bb8-498e-be69-806e65865e11",
        "title": "My First Post",
        "content": "This is the content of the post."
    }
]
```

#### 2. Create a post

**POST** `/posts`

- **Body**:

```json
{
    "title": "Test Post",
    "content": "This is a test post."
}
```

- **Response Example**:

```json
{
    "id": "uuid-generated-here",
    "title": "Test Post",
    "content": "This is a test post."
}
```

---

## Testing

### Run Unit and End-to-End Tests

The application includes end-to-end tests located in `src/e2e_tests.rs`. These can be run using:

```bash
cargo test
```

### Test Frameworks Used:

- **Reqwest**: For sending HTTP requests during tests.
- **Serde JSON**: For parsing JSON responses.

### Testing Coverage:

- Creating a post (`POST /posts`)
- Fetching posts (`GET /posts`)

---

## Dependencies

- [Hyper](https://github.com/hyperium/hyper): High-performance HTTP library.
- [Tokio](https://tokio.rs/): Asynchronous runtime for Rust.
- [Serde](https://github.com/serde-rs/serde): Framework for serializing and deserializing Rust data.
- [Sqlx](https://github.com/launchbadge/sqlx): Async SQL toolkit with compile-time checked queries.
- [Reqwest](https://github.com/seanmonstar/reqwest): HTTP client for testing.

View all dependencies in the `Cargo.toml` file.

---

## Future Improvements

- Add support for authentication and user management.
- Implement pagination for the `/posts` route.
- Add middleware for request logging and rate-limiting.
- Dockerize the application for easier deployment.

---

## License

This project is licensed under the [MIT License](./LICENSE). Feel free to use, modify, and distribute this project.

---
