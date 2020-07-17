use std::error::Error;

use rocket::http::RawStr;
use rocket_contrib::json::Json;
use serde_json::Value;

use crate::datasource::db::Conn;
use crate::datasource::schema::users::columns::username;
use crate::profile::delivery::api_key::{is_valid, ApiKey};
use crate::profile::domain::User;
use crate::profile::use_case::profile::ProfileManager;

pub struct ProfileRestAdapter {
    profile_manager: Box<dyn ProfileManager>,
}

impl ProfileRestAdapter {
    pub fn new(profile_manager: Box<dyn ProfileManager>) -> Self {
        Self { profile_manager }
    }
    pub fn get_user(&self, user_name: &RawStr, key: ApiKey) -> Result<Json<Value>, Box<dyn Error>> {
        match is_valid(&*key.0) {
            Ok(_) => Ok(self.find_user(user_name.to_string())),
            Err(e) => Err(e.into()),
        }
    }

    pub fn get_all_user(&self, conn: Conn, key: ApiKey) -> Result<Json<Value>, Box<dyn Error>> {
        return match is_valid(&*key.0) {
            Ok(_) => Ok(self.find_users(conn)),
            Err(e) => Err(e.into()),
        };
    }

    fn find_users(&self, conn: Conn) -> Json<Value> {
        match self.profile_manager.get_all_users(conn) {
            Ok(users) => {
                let result: Vec<PresentationUser> = users
                    .into_iter()
                    .map(|u| PresentationUser {
                        username: u.username,
                    })
                    .collect();
                Json(json!({ "result": result }))
            }
            Err(e) => Json(json!({ "result": Vec::<String>::new() })),
        }
    }

    fn find_user(&self, user_name: String) -> Json<Value> {
        match self.profile_manager.get_user_by_username(user_name) {
            Ok(user) => {
                let result = PresentationUser {
                    username: user.username,
                };
                Json(json!({ "result": result }))
            }
            Err(e) => Json(json!({"error": String::from("NOT FOUND")})),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PresentationUser {
    pub username: String,
}
