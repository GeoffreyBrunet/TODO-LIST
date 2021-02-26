#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate rocket_contrib;

mod schema;

use std::ops::{Deref, DerefMut};

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
pub struct Todo {
    id: i32,
    title: String,
    description: String,
    deadline: i32,
    done: bool
}

#[derive(Insertable, Serialize, Deserialize, AsChangeset, Clone)]
#[table_name = "todo"]
pub struct NewTodo {
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

#[get("/todo/<get_date>")]
fn get_todo_by_date(conn: DbConn, get_date: i32) -> Json<Vec<Todo>> {
    let todos = todo::table
        .filter(todo::deadline.eq(get_date))
        .load::<Todo>(&*conn)
        .unwrap();
    Json(todos)
}

#[post("/todo", format = "application/json", data = "<new_todo>")]
fn post_todo(conn: DbConn, new_todo: Json<NewTodo>) -> Json<Todo> {
    let result = diesel::insert_into(todo::table)
        .values(&new_todo.0)
        .get_result(&*conn)
        .unwrap();
    Json(result)
}

#[put("/todo/<id>", format = "application/json", data = "<mod_todo>")]
fn put_todo(conn: DbConn, id: i32, mod_todo: Json<NewTodo>) -> Json<bool> {
    //let new_mod_todo =  mod_todo.deref()
    let result = diesel::update(todo::table.find(id))
        .set(&mod_todo.0)
        .get_result::<Todo>(&*conn)
        .is_ok();
    Json(result)
}

#[delete("/todo/<id>")]
fn delete_todo(conn: DbConn, id: i32) -> Json<bool> {
    let result = diesel::delete(todo::table.find(id))
        .execute(&*conn)
        .is_ok();
    Json(result)
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
