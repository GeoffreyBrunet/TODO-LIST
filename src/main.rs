#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate serde_derive;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket_contrib;

use diesel::pg::PgConnection;
use diesel::Connection;
use diesel::{Queryable, Insertable};
use dotenv::dotenv;
use rocket::response::content;
use rocket_contrib::json::Json;
use std::env;
use serde_derive::{Serialize, Deserialize};

mod connection;
mod schema;
use crate::schema::todolist;

#[derive(Queryable, Serialize)]
struct Todo {
    id: i32,
    title: String,
    description: String,
    deadline: String,
    done: bool
}

#[derive(Insertable, Deserialize)]
#[table_name = "todolist"]
struct NewTodo {
    title: String
}

#[get("/")]
fn index() -> content::Json<&'static str> {
    content::Json("{ 'hi': 'world' }")
}

#[get("/todo")]
fn get_todo() -> content::Json<&'static str> {
    content::Json("{ 'hi': 'todo' }")
}

#[get("/todo/<id>")]
fn get_todo_by_id(id: u32) -> content::Json<&'static str> {
    content::Json("{ 'hi': 'todo id' }")
}

// GET /todo?date=20210215

/*#[get("/todo?date=<get_date>")]
fn get_todo_by_date(get_date: u32) -> content::Json<&'static str> {
    content::Json("{ 'hi': 'todo get_date' }")
}*/

#[post("/todo", format = "application/json")]
fn post_todo() -> content::Json<&'static str> {
    content::Json("{ 'hi': 'todo' }")
}

#[put("/todo", data = "new_todo")]
fn put_todo(new_todo: Json<NewTodo>) -> Json<Todo> {
    content::Json("{ 'hi': 'todo' }")
}

#[delete("/todo/<id>")]
fn delete_todo(id: u32) -> content::Json<&'static str> {
    content::Json("{ 'hi': 'todo' }")
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/", routes![index,
        get_todo,
        get_todo_by_id,
        post_todo,
        put_todo,
        delete_todo])
        .launch();
}
