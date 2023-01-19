use db::*;
use std::io::{stdin, Read};


// Script to write a new todo
fn main() {
    let connection = &mut establish_connection();

    let mut title = String::new();
    let mut creator = String::new();
    let mut body = String::new();

    println!("What would you like your title to be?");
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end();

    println!("What would you like your creator name to be?");
    stdin().read_line(&mut creator).unwrap();
    let creator = creator.trim_end();

    println!(
        "\nOk! Let's write {} (Press {} when finished)\n",
        title, EOF
    );

    stdin().read_to_string(&mut body).unwrap();

    let todo = create_todo(connection, creator, title, &body);

    println!("\nSaved draft {} with id {}", title, todo.id);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOG: &str = "CTRL+Z";