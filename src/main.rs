#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket_contrib;

use dotenv::dotenv;
use rocket::response::content;

//mod schema;
//mod connection;

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

#[put("/todo/<id>", format = "application/json")]
fn put_todo(id: u32) -> content::Json<&'static str> {
    content::Json("{ 'hi': 'todo' }")
}

#[delete("/todo/<id>")]
fn delete_todo(id: u32) -> content::Json<&'static str> {
    content::Json("{ 'hi': 'todo' }")
}

fn main() {
    dotenv().ok();
    rocket::ignite()
        .mount("/", routes![index,
        get_todo,
        get_todo_by_id,
        post_todo,
        put_todo,
        delete_todo])
        .launch();
}
