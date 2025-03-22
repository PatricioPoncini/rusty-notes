# Rusty Notes 🦀
A simple note-taking backend built with Rust, using the Axum framework for the API and PostgreSQL as the primary database technology. This backend allows you to create, read, update, and delete notes, with a focus on performance, scalability, and ease of use.

# 🚀 Running the Server
## Start the Containers
Before running the server, ensure that the necessary containers are running. You can start the containers with:
```shell
docker compose up -d
```

## Start the server
Once the containers are up and running, start the server with:
```shell
cargo run
```

## 🧪 Execute test cases
Run the following command to initiate the test process. It will first start the PostgreSQL container required for testing the database connection, and then execute the tests to verify that everything is working as expected.
```shell
make test
```

## 🛠️ Building the Project
Compile the project using:
```shell
make build
```

## 🐳 Access the Container
Use this command to enter the container and connect directly to the database:
```shell
docker exec -it postgres_db psql -U admin -d axum_db
```

## ✨ Format code
Run the following command to automatically format the code:
```shell
make format
```

## 📝 Documentation
Once the server is running, the documentation will be available at `/doc`.

## ⚙ GitHub Actions
In this project, I have integrated GitHub Actions to automate tasks such as building, testing, and formatting the code. The workflow includes the following actions:

- **Build**: Compiles the project to ensure that there are no compilation issues.
- **Test**: Executes the test cases after spinning up the required PostgreSQL container for testing database interactions.
- **Format**: Automatically formats the code to ensure it adheres to the project's style guide.
- **Clippy**: Runs Rust's linter to detect any potential issues in the code.

This automated process helps ensure that every commit and pull request is validated, making the development process smoother and more reliable.