use std::error::Error;

use rocket::http::RawStr;
use rocket_contrib::json::Json;
use serde_json::Value;

use crate::datasource::db::Conn as DbConn;
use crate::profile::delivery::api_key::is_valid;
use crate::profile::delivery::api_key::ApiKey;
use crate::profile::delivery::profile::{PresentationUser, ProfileRestAdapter};
use crate::profile::repositories::implementations::profile::ProfileRepositoryImpl;
use crate::profile::repositories::models::LoginInfo;
use crate::profile::repositories::models::{NewUser, User};
use crate::profile::use_case::authentication::generate_token;
use crate::profile::use_case::profile_manager_impl::ProfileManagerImpl;

#[get("/users", format = "application/json")]
pub fn get_all(conn: DbConn) -> Json<Value> {
    match User::get_all_users(&conn) {
        Ok(users) => {
            let result: Vec<PresentationUser> = users
                .into_iter()
                .map(|u| PresentationUser {
                    username: u.username,
                })
                .collect();
            Json(json!({ "result": result }))
        }
        Err(_) => Json(json!({"result": ""})),
    }
}

#[post("/signup", format = "application/json", data = "<new_user>")]
pub fn new_user(conn: DbConn, new_user: Json<NewUser>) -> Json<Value> {
    let user = new_user.into_inner();
    match User::insert_user(&user, &conn){
        Ok(User) => Json(json!({"result": "unauthorized"})),
        Err(_) => Json(json!({"Error": "unauthorized"})),
    }
}

#[post("/login", format = "application/json", data = "<login_info>")]
pub fn login(conn: DbConn, login_info: Json<LoginInfo>) -> Json<Value> {
    return match User::get_user_by_username(String::from(login_info.username.as_str()), &conn) {
        Ok(u) => authorize_credentials(&u),
        Err(_) => reject_credentials(),
    };
}

fn reject_credentials() -> Json<Value> {
    Json(json!({"result": "unauthorized"}))
}

fn authorize_credentials(user: &Vec<User>) -> Json<Value> {
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
pub fn get_user(conn: DbConn, username: &RawStr, key: ApiKey) -> Json<Value> {
    let box_profile_repo = Box::new(ProfileRepositoryImpl::new(conn));
    let box_profile_manager = Box::new(ProfileManagerImpl::new(box_profile_repo));
    let profile_rest_adapter = ProfileRestAdapter::new(box_profile_manager);
    match profile_rest_adapter.get_user(username, key) {
        Ok(result) => result,
        Err(_) => reject_credentials(),
    }
}
