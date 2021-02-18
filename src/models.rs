use diesel::*;
use diesel::sql_types::*;

use super::schema::todolist;

#[derive(Queryable, QueryableByName)]
#[table_name = "todolist"]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub deadline: String,
    pub done: bool
}
