# Axum Backend 🦀
A simple note-taking backend built with Rust, using the Axum framework for the API and PostgreSQL as the primary database technology. This backend allows you to create, read, update, and delete notes, with a focus on performance, scalability, and ease of use.

## 🚀 Running the Server
### Start the Containers
Before running the server, ensure that the necessary containers are running. You can start the containers with:
```bash
docker compose up -d
```

### Start the server
Once the containers are up and running, start the server with:
```bash
cargo run
```

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

## 📝 Documentation
Once the server is running, the documentation will be available at `/doc`.