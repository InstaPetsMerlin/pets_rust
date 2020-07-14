use rocket::http::RawStr;

use rocket_contrib::json::Json;
use serde_json::Value;

use crate::datasource::db::Conn as DbConn;
use crate::profile::delivery::api_key::is_valid;
use crate::profile::delivery::api_key::ApiKey;
use crate::profile::repositories::models::LoginInfo;
use crate::profile::repositories::models::{NewUser, User};
use crate::profile::use_case::authentication::generate_token;

#[post("/users", format = "application/json")]
pub fn get_all(conn: DbConn) -> Json<Value> {
    let users = User::get_all_users(&conn);
    Json(json!({
        "status": 200,
        "result": users,
    }))
}

#[post("/signup", format = "application/json", data = "<new_user>")]
pub fn new_user(conn: DbConn, new_user: Json<NewUser>) -> Json<Value> {
    let user = new_user.into_inner();
    let status = User::insert_user(&user, &conn);
    let token = match status {
        true => generate_token(&user.username, &user.first_name),
        false => String::from(""),
    };

    Json(json!(
    {
        "status": status,
        "result": User::get_all_users(&conn).first(),
        "token": token,
    }))
}

#[post("/login", format = "application/json", data = "<login_info>")]
pub fn login(conn: DbConn, login_info: Json<LoginInfo>) -> Json<Value> {
    let user = User::get_user_by_username(&String::from(login_info.username.as_str()), &conn);

    let token = match user.len() {
        1 => generate_token(
            &user.first().unwrap().username,
            &user.first().unwrap().first_name,
        ),
        _ => String::from(""),
    };
    Json(json!(
    {
        "result": user.first(),
        "token": token,
    }))
}

#[get("/users/<username>", format = "application/json")]
pub fn find_user(conn: DbConn, username: &RawStr, key: ApiKey) -> Json<Value> {
    let _result = return match is_valid(&*key.0) {
        Ok(_) => Json(json!({
            "result": User::get_user_by_username(&String::from(username.as_str()), &conn),
        })),
        Err(_) => Json(json!({
            "result": User::get_user_by_username(&String::from(username.as_str()), &conn),
        })),
    };
}
