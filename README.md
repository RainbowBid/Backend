# Backend

Installation steps:

1. Clone the repository
2. Install [Docker](https://docs.docker.com/get-docker/)
3. Install [Rust](https://www.rust-lang.org/tools/install)
4. Create a `Secrets.dev.toml` file in the `Backend/api` directory with the following content:
```toml
ALLOWED_ORIGIN = "http://localhost:10000"
JWT_KEY = "secret"
JWT_DURATION_MINUTES = "60"
FINALIZE_AUCTIONS_CRON = "1/60 * * * * *"
```
5. Install [Shuttle CLI](https://docs.shuttle.rs/getting-started/installation)
6. Install Sqlx CLI: `cargo install sqlx-cli --no-default-features --features postgres`
7. Run `cargo shuttle run` in the `Backend` directory to allow Shuttle to set up the database container
8. Run `sqlx migrate run --database-url=<DSN from Shuttle stdout>` in the `Backend` directory to run the migrations
9. Rerun `cargo shuttle run` in the `Backend` directory to restart the server