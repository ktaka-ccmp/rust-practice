# Readme

## prep

```text
cd db/
sqlx database create --database-url "sqlite:./sqlite.db"
sqlx migrate add -r customer
vi migrations/20240701161707_customer.up.sql
vi migrations/20240701161707_customer.down.sql
sqlx migrate run --database-url sqlite:./sqlite.db
sqlx migrate revert --database-url sqlite:./sqlite.db
sqlx migrate run --database-url sqlite:./sqlite.db
```

```text
cargo add axum dotenv serde thiserror tracing tracing-subscriber utoipa
cargo add tokio --features=full
cargo add tower-http --features=trace
cargo add sqlx --features sqlite,runtime-tokio-rustls
cargo add utoipa-redoc --features=axum
cargo add utoipa-rapidoc --features=axum
cargo add utoipa-scalar --features=axum
cargo add utoipa-swagger-ui --features=axum
```

```text
cargo run
```
