# bb8-todos

An axum web-service that manages simplistic todo lists.

The goal of this project is to benchmark [Axum](https://docs.rs/axum/latest/axum/)
using bb8 for pooling tokio-postgres connections.

## database

Install the `diesel` migration tool

```sh
cargo install diesel_cli --no-default-features --features postgres
```

Create database, and run migrations

```sh
diesel database setup
```
