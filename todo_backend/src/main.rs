
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;


use diesel::{prelude::*, table, Insertable, Queryable};
use rocket::{serde::json::Json};
use rocket::response::{Debug, status::Created};
use rocket_sync_db_pools::database;
use serde::{Deserialize, Serialize};


table! {
    todo (id) {
        id -> Int4,
        creator -> Varchar,
        title -> Varchar,
        body -> Text,
        completed -> Bool,
    } 
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable)]
#[table_name = "todo"]
struct Todo {
    id: i32,
    creator: String,
    title: String,
    body: String,
    completed: bool
}

#[derive(Clone, Serialize, Deserialize, Insertable)]
#[table_name = "todo"]
struct NewTodo{
    creator:  String,
    title: String,
    body: String
}

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;


// fn establish_connection() -> PgConnection {
//     dotenv().ok();

//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     PgConnection::establish(&database_url)
//         .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
// }

#[database("my_db")]
pub struct Db(diesel::PgConnection);


#[post("/", data = "<todo>")]
async fn create_todo(connection: Db, todo: Json<NewTodo>) -> Result<Created<Json<NewTodo>>> {
    let todo_value = todo.clone();

    connection.run(move |conn| {
        diesel::insert_into(todo::table)
            .values(&*todo_value)
            .execute(conn)
    }).await?;

    Ok(Created::new("/").body(todo))

}


#[get("/random")]
fn get_random_todo() -> Json<Todo> {
    Json(
        Todo {
            id: 1,
            creator: "Mike Shin".to_string(),
            title: "My first Todo".to_string(),
            body: "This is my first todo list".to_string(),
            completed: false,
        }
    )
}

#[get("/<id>")]
fn get_todo(id: i32) -> Json<Todo> {
    Json(
        Todo {
            id,
            creator: "Mike Shin".to_string(),
            title: "Some title".to_string(),
            body: "Some body".to_string(),
            completed: false,
        }
    )
}

#[get("/")]
async fn get_all_todos(connection: Db) -> Json<Vec<Todo>> {
    connection
        .run(|c| todo::table.load(c))
        .await
        .map(Json)
        .expect("Failed to fetch blog posts")
}


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {

    let rocket = rocket::build();
    rocket
        .attach(Db::fairing())
        .mount("/", routes![index])
        .mount("/todos", routes![
            get_random_todo, 
            get_todo, 
            get_all_todos, 
            create_todo
        ]
    )
}