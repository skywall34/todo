use self::models::*;
use diesel::prelude::*;
use db::*;

fn main() {
    use self::schema::todo::dsl::*;

    let connection = &mut establish_connection();
    let results = todo
        .filter(completed.eq(true))
        .limit(5)
        .load::<Todo>(connection)
        .expect("Error loading todo");

    println!("Displaying {} todos", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}