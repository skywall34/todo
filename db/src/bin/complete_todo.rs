use self::models::Todo;
use diesel::prelude::*;
use db::*;
use std::env::args;

fn main() {
    use self::schema::todo::dsl::{todo, completed};

    let id = args()
        .nth(1)
        .expect("complete_todo requires a todo id")
        .parse::<i32>()
        .expect("Invalid ID");

    let connection = &mut establish_connection();

    let updated_todo = diesel::update(todo.find(id))
        .set(completed.eq(true))
        .get_result::<Todo>(connection)
        .unwrap();

    println!("Completed todo {}", updated_todo.title);
}