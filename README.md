Welcome to the TODO git repository! This repository contains the source code for the TODO web application, which is written in Rust and uses the Rocket web framework and SEA ORM. 

It closely follows the rust-web-app by garrettudstrand here: https://github.com/garrettudstrand/rust-web-app and is mainly used to personally understand Rust with web applications. I would follow this guide [here](https://medium.com/better-programming/how-to-write-a-web-app-in-rust-part-1-3047156660a7) to really understand how to start writing web applications in Rust. 

## Prerequisites
Before you begin, make sure that you have the following software installed on your system:

- Git
- Rust (version 1.31 or higher)
- A web browser
- A text editor

You will also need to install the following Rust dependencies:

- Rocket
- Sea-ORM
- Serde

Be sure to check out the Cargo.toml file for the necessary dependencies.

## Getting Started

To get started with the TODO web application, follow these steps:

1. Clone the repository:

```bash
$ git clone https://github.com/skywall34/todo.git
```

2. Change into the repository directory:

```bash
$ cd todo_backend
```

3. Create a new database file using Postgres. I just created one on my local machine but feel free to use the docker-compose file:

```bash
$ docker-compose up
```

4. Start the Rocket server. This should also run the migrations to create the table assuming everything is setup on the database side:

```bash
$ cargo run
```

This will start the Rocket server and serve the TODO web application on http://localhost:3000 for debug mode. You can now open the web application in your web browser and start using it.

## Testing

TODO