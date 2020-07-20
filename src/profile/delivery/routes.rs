use std::error::Error;

use rocket::http::RawStr;
use rocket_contrib::json::Json;
use serde_json::Value;

use crate::datasource::db::Conn as DbConn;
use crate::profile::delivery::api_key::is_valid;
use crate::profile::delivery::api_key::ApiKey;
use crate::profile::delivery::rest_adapater::{PresentationUser, ProfileRestAdapter};
use crate::profile::repositories::implementations::profile::ProfileRepositoryImpl;
use crate::profile::repositories::models::LoginInfo;
use crate::profile::repositories::models::{NewUser, User};
use crate::profile::use_case::authentication::generate_token;
use crate::profile::use_case::implementations::profile_manager_impl::ProfileManagerImpl;
use rocket::State;
use crate::profile::use_case::profile::ProfileManager;
use crate::profile::repositories::profile::ProfileRepository;

#[get("/users", format = "application/json")]
pub fn get_all(
    conn: DbConn,
    key: ApiKey,
) -> Json<Value> {
    let profile_repo = ProfileRepositoryImpl::new(conn);
    let manager = ProfileManagerImpl::new(profile_repo);
    let adapter = ProfileRestAdapter::new(manager);
    match adapter.get_all_user(key){
        Ok(users) => users,
        Err(_) => Json(json!({"error": ""})),
    }
}

#[post("/signup", format = "application/json", data = "<new_user>")]
pub fn new_user(conn: DbConn, new_user: Json<NewUser>) -> Json<Value> {
    let profile_repo = ProfileRepositoryImpl::new(conn);
    let manager = ProfileManagerImpl::new(profile_repo);
    let profile_rest_adapter = ProfileRestAdapter::new(manager);
    match profile_rest_adapter.sing_up(new_user){
        Ok(response) => response,
        Err(_) => Json(json!({"error": "Could not create user"})),
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
    let profile_repo = ProfileRepositoryImpl::new(conn);
    let manager = ProfileManagerImpl::new(profile_repo);
    let profile_rest_adapter = ProfileRestAdapter::new(manager);
    match profile_rest_adapter.get_user(username, key) {
        Ok(result) => result,
        Err(_) => reject_credentials(),
    }
}
