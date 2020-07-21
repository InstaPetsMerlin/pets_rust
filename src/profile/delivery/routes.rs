use rocket::http::RawStr;
use rocket::State;
use rocket_contrib::json::Json;
use serde_json::Value;

use crate::datasource::db::Conn as DbConn;
use crate::profile::delivery::api_key::ApiKey;
use crate::profile::delivery::rest_adapater::ProfileRestAdapter;
use crate::profile::repositories::implementations::profile::ProfileRepositoryImpl;
use crate::profile::repositories::models::{NewUser, User};
use crate::profile::repositories::models::LoginInfo;
use crate::profile::repositories::profile::ProfileRepository;
use crate::profile::use_case::authentication::generate_token;
use crate::profile::use_case::implementations::profile_manager_impl::ProfileManagerImpl;
use crate::profile::use_case::profile::ProfileManager;

#[get("/users", format = "application/json")]
pub fn get_all(
    conn: DbConn,
    key: ApiKey,
    adapter: State<ProfileRestAdapter<ProfileManagerImpl<ProfileRepositoryImpl>>>,
) -> Json<Value> {
    match adapter.get_all_user(key, conn) {
        Ok(users) => users,
        Err(_) => Json(json!({"error": ""})),
    }
}

#[post("/signup", format = "application/json", data = "<new_user>")]
pub fn new_user(
    conn: DbConn,
    new_user: Json<NewUser>,
    adapter: State<ProfileRestAdapter<ProfileManagerImpl<ProfileRepositoryImpl>>>,
) -> Json<Value> {
    match adapter.sing_up(new_user, conn) {
        Ok(response) => response,
        Err(_) => Json(json!({"error": "Could not create user"})),
    }
}

#[post("/login", format = "application/json", data = "<login_info>")]
pub fn login(
    conn: DbConn,
    login_info: Json<LoginInfo>,
    _adapter: State<ProfileRestAdapter<ProfileManagerImpl<ProfileRepositoryImpl>>>,
) -> Json<Value> {
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
pub fn get_user(
    conn: DbConn,
    username: &RawStr,
    key: ApiKey,
    adapter: State<ProfileRestAdapter<ProfileManagerImpl<ProfileRepositoryImpl>>>,
) -> Json<Value> {
    match adapter.get_user(username, key, conn) {
        Ok(result) => result,
        Err(_) => reject_credentials(),
    }
}
