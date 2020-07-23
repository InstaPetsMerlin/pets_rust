use rocket::http::{RawStr, Status};
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use serde_json::Value;

use crate::datasource::db::Conn as DbConn;
use crate::profile::delivery::api_key::ApiKey;
use crate::profile::delivery::rest_adapater::ProfileRestAdapter;
use crate::profile::delivery::user_request::UserRequest;
use crate::profile::repositories::implementations::models::{NewUser, User};
use crate::profile::repositories::implementations::models::LoginInfo;
use crate::profile::repositories::implementations::profile::ProfileRepositoryImpl;
use crate::profile::use_case::authentication::generate_token;
use crate::profile::use_case::implementations::profile_manager_impl::ProfileManagerImpl;
use crate::profile::delivery::response::HttpResponse;

#[get("/users", format = "application/json")]
pub fn get_all(
    key: ApiKey,
    adapter: State<ProfileRestAdapter<ProfileManagerImpl<ProfileRepositoryImpl>>>,
) -> Json<Value> {
    match adapter.get_all_user(key) {
        Ok(users) => users,
        Err(_) => Json(json!({"error": ""})),
    }
}

#[post("/signup", format = "application/json", data = "<new_user>")]
pub fn new_user(
    new_user: Json<NewUser>,
    adapter: State<ProfileRestAdapter<ProfileManagerImpl<ProfileRepositoryImpl>>>,
) -> Json<Value> {
    match adapter.sing_up(new_user) {
        Ok(response) => response,
        Err(_) => Json(json!({"error": "Could not create user"})),
    }
}

#[post("/login", format = "application/json", data = "<login_info>")]
pub fn login(
    login_info: Json<LoginInfo>,
    adapter: State<ProfileRestAdapter<ProfileManagerImpl<ProfileRepositoryImpl>>>,
) -> Json<Value> {
    match adapter.login(login_info) {
        Ok(response) => response,
        Err(_) => reject_credentials(),
    }
}

fn reject_credentials() -> Json<Value> {
    Json(json!({"result": "unauthorized"}))
}

fn authorize_credentials(user: &[User]) -> Json<Value> {
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

#[put("/users", format = "application/json", data = "<user_request>")]
pub fn update_user(
    key: ApiKey,
    user_request: Json<UserRequest>,
    adapter: State<ProfileRestAdapter<ProfileManagerImpl<ProfileRepositoryImpl>>>,
) -> Json<Value> {
    match adapter.update_user(user_request, key) {
        Ok(result) => result,
        Err(_) => reject_credentials(),
    }
}

#[delete("/users", format = "application/json", data = "<user_request>")]
pub fn delete_user(
    user_request: Json<UserRequest>,
    key: ApiKey,
    adapter: State<ProfileRestAdapter<ProfileManagerImpl<ProfileRepositoryImpl>>>,
) -> Json<Value> {
    match adapter.delete_user(user_request.id.to_string(), key) {
        Ok(result) => result,
        Err(_) => reject_credentials(),
    }
}

#[get("/users/<username>", format = "application/json")]
pub fn get_user(
    username: &RawStr,
    key: ApiKey,
    adapter: State<ProfileRestAdapter<ProfileManagerImpl<ProfileRepositoryImpl>>>,
) -> Json<Value> {
    match adapter.get_user(username, key) {
        Ok(result) => result,
        Err(_) => reject_credentials(),
    }
}