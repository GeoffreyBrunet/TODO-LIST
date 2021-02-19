#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate rocket_contrib;

mod schema;

use crate::schema::todo;
use rocket::{get, post, put, routes};
use rocket_contrib::json::Json;
use rocket_contrib::databases::{database, diesel::PgConnection};
use diesel::{Queryable, Insertable};
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[database("postgres")]
struct DbConn(PgConnection);

#[derive(Queryable, Serialize)]
struct Todo {
    id: i32,
    title: String,
    description: String,
    deadline: i32,
    done: bool
}

#[derive(Insertable, Deserialize)]
#[table_name = "todo"]
struct NewTodo {
    title: String,
    description: Option<String>,
    deadline: i32,
    done: Option<bool>
}

#[get("/")]
fn index() -> Json<&'static str> {
    Json("{ 'hi': 'world' }")
}

#[get("/todo")]
fn get_todo(conn: DbConn) -> Json<Vec<Todo>> {
    let todos = todo::table
        .order(todo::columns::id.desc())
        .load::<Todo>(&*conn)
        .unwrap();
    Json(todos)
}

#[get("/todo/<id>")]
fn get_todo_by_id(conn: DbConn, id: i32) -> Json<Vec<Todo>> {
    let todos = todo::table
        .order(todo::columns::id.desc())
        .filter(todo::id.eq(id))
        .load::<Todo>(&*conn)
        .unwrap();
    Json(todos)
}

// GET /todo?date=20210215

/*#[get("/todo?date=<get_date>")]
fn get_todo_by_date(get_date: i32) -> content::Json<&'static str> {
    content::Json("{ 'hi': 'todo get_date' }")
}*/

#[post("/todo", data = "<new_todo>")]
fn put_todo(conn: DbConn, new_todo: Json<NewTodo>) -> Json<Todo> {
    let result = diesel::insert_into(todo::table)
        .values(&new_todo.0)
        .get_result(&*conn)
        .unwrap();
    Json(result)
}

#[put("/todo", format = "application/json")]
fn post_todo() -> Json<&'static str> {
    Json("{ 'hi': 'todo' }")
}


#[delete("/todo/<id>")]
fn delete_todo(id: i32) -> Json<&'static str> {
        Json("{ 'hi': 'todo' }")
}

fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/", routes![index,
        get_todo,
        get_todo_by_id,
        post_todo,
        put_todo,
        delete_todo])
        .launch();
}
