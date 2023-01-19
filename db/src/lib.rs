pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use self::models::{NewTodo, Todo};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_todo(conn: &mut PgConnection, creator: &str, title: &str, body: &str) -> Todo {
     use crate::schema::todo;

     let new_todo = NewTodo { creator, title, body};

     diesel::insert_into(todo::table)
        .values(&new_todo)
        .get_result(conn)
        .expect("Error saving new todo")
}