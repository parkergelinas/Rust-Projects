# Rust Actix-Web API System

## Introduction

This project is a Rust backend system developed with Actix-Web. It demonstrates a simple user management system with basic CRUD (Create, Read, Update, Delete) functionality. The API allows for creating new users, retrieving user information, updating user details, and deleting users.

## Requirements

- Rust Programming Language
- Cargo (Rust's package manager)
- PostgreSQL Database

## Setup

### Environment Setup

1. Ensure Rust and Cargo are installed on your system. If not, follow the installation instructions on the [official Rust website](https://www.rust-lang.org/tools/install).
2. Install PostgreSQL and create a database for the project.

### Project Setup

1. Clone the repository to your local machine.

   ```bash
   git clone https://github.com/parkergelinas/rust_api.git
   ```

2. Navigate into the project directory.

   ```bash
   cd rust_api
   ```

3. Create a `.env` file in the root of your project directory and add your database connection string:

   ```dotenv
   DATABASE_URL=postgres://username:password@localhost/dbname
   ```

4. Run the project with Cargo:

   ```bash
   cargo run
   ```

## API Endpoints

### Create User

- **Endpoint:** `/users`
- **Method:** POST
- **Body:**

  ```json
  {
    "username": "john_doe",
    "email": "john_doe@example.com"
  }
  ```

- **Description:** Creates a new user with the provided username and email.

### Get User

- **Endpoint:** `/users/{id}`
- **Method:** GET
- **Description:** Retrieves a user's information by their ID.

### Update User

- **Endpoint:** `/users/{id}`
- **Method:** PUT
- **Body:**

  ```json
  {
    "username": "new_username",
    "email": "new_email@example.com"
  }
  ```

- **Description:** Updates the information for the user with the specified ID.

### Delete User

- **Endpoint:** `/users/{id}`
- **Method:** DELETE
- **Description:** Deletes the user with the specified ID.

## Running the Tests

Explain how to run the automated tests for this system, if applicable.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
