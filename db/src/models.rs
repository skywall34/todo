use diesel::prelude::*;
use crate::schema::todo;

#[derive(Queryable)]
pub struct Todo {
    pub id: i32,
    pub creator: String,
    pub title: String,
    pub body: String,
    pub completed: bool
}

#[derive(Insertable)]
#[diesel(table_name = todo)]
pub struct NewTodo<'a> {
    pub creator: &'a str,
    pub title: &'a str,
    pub body: &'a str
}