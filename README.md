# Axum Backend 🦀
A backend built with Rust and Axum.

## 🚀 Running the Server
Start the server with:
```bash
cargo run
```
By default, it will run on port `:8080`

## 🛠️ Building the Project
Compile the project using:
```bash
cargo build
```

## 🐳 Access the Container
Use this command to enter the container and connect directly to the database:
```bash
docker exec -it postgres_db psql -U admin -d axum_db
```