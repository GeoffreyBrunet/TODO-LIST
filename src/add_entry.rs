use super::models::*;
use diesel::pg::PgConnection;
use crate::schema::Todo;

pub fn create_score(conn: PgConnection, title: String, description: String, deadline: String, done: i32) -> () {
    let new_score = Score {
        title: Some(title),
        description: Some(description),
        deadline: Some(deadline),
        done: Some(done)
    };   

    let ins = diesel::insert_into(scores::table)
        .values(new_score)
        .execute(&conn);
}