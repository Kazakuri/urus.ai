# urus.ai

A work in progress URL shortening service that will reside at https://urus.ai/.

# Development

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