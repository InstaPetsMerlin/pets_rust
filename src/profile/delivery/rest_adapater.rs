use std::error::Error;

use rocket::http::RawStr;
use rocket_contrib::json::Json;
use serde_json::Value;

use crate::datasource::db::Conn;
use crate::datasource::schema::users::columns::username;
use crate::profile::delivery::api_key::{is_valid, ApiKey};
use crate::profile::domain::User;
use crate::profile::use_case::profile::ProfileManager;
use crate::profile::repositories::models::NewUser;
use crate::profile::errors::ProfileError;
use crate::profile::delivery::sign_up_response::SignUpResponse;
use crate::profile::use_case::authentication::generate_token;

pub struct ProfileRestAdapter<T:ProfileManager> {
    profile_manager: T,
}

impl<T:ProfileManager> ProfileRestAdapter<T> {
    pub fn new(profile_manager: T) -> Self {
        Self { profile_manager }
    }

    pub fn sing_up(&self, new_user: Json<NewUser>) -> Result<Json<Value>, ProfileError>{
        match self.profile_manager.create_user(User{ username: new_user.into_inner().username }) {
            Ok(user) => {
                let token = generate_token(&user.username, &user.username);
                let response = SignUpResponse { token, user: PresentationUser{ username: user.username } };
                Ok(Json(json!(response)))
            },
            Err(_) => Err(ProfileError::Other("Could not create user".to_string())),
        }
    }
    pub fn get_user(&self, user_name: &RawStr, key: ApiKey) -> Result<Json<Value>, Box<dyn Error>> {
        match is_valid(&*key.0) {
            Ok(_) => Ok(self.find_user(user_name.to_string())),
            Err(e) => Err(e.into()),
        }
    }

    pub fn get_all_user(&self,key: ApiKey) -> Result<Json<Value>, ProfileError> {
        return match is_valid(&*key.0) {
            Ok(_) => {self.find_users()},
            Err(e) => Err(ProfileError::Other("Not Authorized".to_string())),
        };
    }

    fn find_users(&self) -> Result<Json<Value>,ProfileError >{
        match self.profile_manager.get_all_users() {
            Ok(users) => {
                let result: Vec<PresentationUser> = users
                    .into_iter()
                    .map(|u| PresentationUser {
                        username: u.username,
                    })
                    .collect();
                Ok(Json(json!({ "users": result , "total" : result.len()})))
            }
            Err(e) => Err(ProfileError::ProfileNotFoundError("User not found".to_string())),
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
