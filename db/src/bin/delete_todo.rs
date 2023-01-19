use diesel::prelude::*;
use db::*;
use std::env::args;

fn main() {
    use self::schema::todo::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = &mut establish_connection();
    let num_deleted = diesel::delete(todo.filter(title.like(pattern)))
        .execute(connection)
        .expect("Error deleting todo");

    println!("Deleted {} todos", num_deleted);
}