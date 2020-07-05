use super::db::Conn as DbConn;
use rocket_contrib::json::Json;
use super::models::{User, NewUser};
use serde_json::Value;
use rocket::http::RawStr;

#[post("/users", format = "application/json")]
pub fn get_all(conn: DbConn) -> Json<Value> {
    let users = User::get_all_users(&conn);
    Json(json!({
        "status": 200,
        "result": users,
    }))
}

#[post("/user", format = "application/json", data = "<new_user>")]
pub fn new_user(conn: DbConn, new_user: Json<NewUser>) -> Json<Value> {
    Json(json!({
        "status": User::insert_user(new_user.into_inner(), &conn),
        "result": User::get_all_users(&conn).first(),
    }))
}

#[get("/users/<username>", format = "application/json")]
pub fn find_user(conn: DbConn, username: &RawStr) -> Json<Value> {
    Json(json!({
        "result": User::get_user_by_username(&String::from(username.as_str()), &conn),
    }))
}