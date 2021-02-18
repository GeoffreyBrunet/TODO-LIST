#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

pub mod cors;
//pub mod models;
pub mod routes;
pub mod schema;

fn main() {
    rocket::ignite()
        .mount("/", routes![routes::index,
        routes::get_todo,
        routes::get_todo_by_id,
        routes::post_todo,
        routes::put_todo,
        routes::delete_todo])
        .launch();
}
