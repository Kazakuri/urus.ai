# urus.ai

A work in progress URL shortening service that will reside at https://urus.ai/.

# Development

## Migration from polr

```
users -> users
  _          -> id
  username   -> display_name
  email      -> email
  _          -> email_verified
  password   -> password_hash
  created_at -> created_at
  updated_at -> updated_at

links -> urls
  _          -> id
  creator    -> user_id
  short_url  -> slug
  long_url   -> url
  clicks     -> visits
  created_at -> created_at
  updated_at -> updated_at
```

## Prerequisites
- Rust
- npm
- PostgresQL
- Faktory

## Getting Started

```bash
# Install diesel_cli to run database migrations
cargo install diesel_cli --no-default-features --features postgres

# Create the database and run the migrations
diesel database setup

# Install dependencies for our application CSS
npm install

# Run any new migrations after pulling
diesel migration run

# Run the main urus.ai process
cargo run --bin urusai

# Run the faktory worker process
cargo run --bin urusai_worker
```
## Dependencies

\# | Crate Name        | Description                                  |
-- | ----------------- | -------------------------------------------- |
1  | actix-files       | Static Files in Debug Mode                   |
2  | actix-identity    | User Login (cookies)                         |
3  | actix-rt          | Async Runtime                                |
4  | actix-web         | Web Framework                                |
5  | askama            | HTML Templating                              |
6  | bcrypt            | Password Compatibility (Polr)                |
8  | chrono            | PostgresQL `DATETIME`                        |
9  | deadpool          | DB Connection Pool                           |
9  | deadpool-postgres | PostgresQL integration for `deadpool`        |
10 | dotenv            | `.env` Environment Variables                 |
11 | env_logger        | `RUST_LOG`-based logging                     |
12 | faktory           | Job Queue                                    |
13 | futures           | Joining `async` Functions                    |
14 | lazy_static       | `static` Vectors and Regexes                 |
15 | lettre            | Email Transport                              |
16 | lettre_email      | Email Composer                               |
17 | log               | Logging Macros                               |
18 | nanoid            | Small, URL-friendly UUID                     |
19 | postgres-types    | Postgres Enum to Rust Enum                   |
20 | regex             | URL and Password Complexity Validation       |
21 | serde             | Struct Serialization                         |
22 | serde_derive      | Derive Macro for Serde                       |
23 | serde_json        | JSON Input and Output for Serde              |
24 | serde_urlencoded  | HTTP Form Input and Output for Serde         |
25 | sodiumoxide       | Password Hashing                             |
26 | thiserror         | Easy Error Messages                          |
27 | tokio-postgres    | Async PostgresQL Connection                  |
28 | uuid              | PostgresQL `UUID`                            |
