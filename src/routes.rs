use rocket::response::content;

#[get("/")]
pub fn index() -> content::Json<&'static str> {
    content::Json("{ 'hi': 'world' }")
}

#[get("/todo")]
pub fn get_todo() -> content::Json<&'static str> {
    content::Json("{ 'hi': 'todo' }")
}

#[get("/todo/<id>")]
pub fn get_todo_by_id(id: u32) -> content::Json<&'static str> {
    content::Json("{ 'hi': 'todo id' }")
}

// GET /todo?date=20210215

/*#[get("/todo?date=<get_date>")]
fn get_todo_by_date(get_date: u32) -> content::Json<&'static str> {
    content::Json("{ 'hi': 'todo get_date' }")
}*/

#[post("/todo", format = "application/json")]
pub fn post_todo() -> content::Json<&'static str> {
    content::Json("{ 'hi': 'todo' }")
}

#[put("/todo/<id>", format = "application/json")]
pub fn put_todo(id: u32) -> content::Json<&'static str> {
    content::Json("{ 'hi': 'todo' }")
}

#[delete("/todo/<id>")]
pub fn delete_todo(id: u32) -> content::Json<&'static str> {
    content::Json("{ 'hi': 'todo' }")
}