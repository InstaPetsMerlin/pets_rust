use std::error::Error;

`use rocket::http::{RawStr, Status};
use rocket_contrib::json::Json;
use serde_json::Value;

use crate::profile::delivery::api_key::{ApiKey, is_valid};
use crate::profile::delivery::sign_up_response::SignUpResponse;
use crate::profile::delivery::user_request::UserRequest;
use crate::profile::domain::User;
use crate::profile::errors::ProfileError;
use crate::profile::repositories::implementations::models::{LoginInfo, NewUser};
use crate::profile::use_case::authentication::generate_token;
use crate::profile::use_case::profile::ProfileManager;

pub struct ProfileRestAdapter<T: ProfileManager> {
    profile_manager: T,
}

impl<T: ProfileManager> ProfileRestAdapter<T> {
    pub fn new(profile_manager: T) -> Self {
        Self { profile_manager }
    }

    pub fn sing_up(
        &self,
        new_user: Json<NewUser>,
    ) -> Result<Json<Value>, ProfileError> {
        match self.profile_manager.create_user(
            User {
                username: new_user.into_inner().username,
            }
        ) {
            Ok(user) => {
                let token = generate_token(&user.username, &user.username);
                let response = SignUpResponse {
                    token,
                    user: PresentationUser {
                        username: user.username,
                    },
                };
                Ok(Json(json!(response)))
            }
            Err(_) => Err(ProfileError::Other("Could not create user".to_string())),
        }
    }

    pub fn login(
        &self,
        login_info: Json<LoginInfo>,
    ) -> Result<Json<Value>, ProfileError> {
        match self.profile_manager.get_user_by_username(login_info.into_inner().username)
        {
            Ok(user) => {
                let token = generate_token(&user.username, &user.username);
                let response = SignUpResponse {
                    token,
                    user: PresentationUser {
                        username: user.username,
                    },
                };
                Ok(Json(json!(response)))
            }
            Err(_) => Err(ProfileError::Other("Could not create user".to_string())),
        }
    }

    pub fn get_user(
        &self,
        user_name: &RawStr,
        key: ApiKey,
    ) -> Result<Json<Value>, Box<dyn Error>> {
        match is_valid(&*key.0) {
            Ok(_) => Ok(self.find_user(user_name.to_string())),
            Err(e) => Err(e.into()),
        }
    }

    pub fn get_all_user(&self, key: ApiKey) -> Result<Json<Value>, ProfileError> {
        match is_valid(&*key.0) {
            Ok(_) => self.find_users(),
            Err(_e) => Err(ProfileError::Other("Not Authorized".to_string())),
        }
    }

    fn find_users(&self) -> Result<Json<Value>, ProfileError> {
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
            Err(_e) => Err(ProfileError::ProfileNotFoundError(
                "User not found".to_string(),
            )),
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
            Err(_e) => Json(json!({"error": String::from("NOT FOUND")})),
        }
    }

    pub fn update_user(&self, new_user: Json<UserRequest>, key: ApiKey) -> Result<Json<Value>, ProfileError> {
        match is_valid(&*key.0) {
            Ok(_) => match self.profile_manager.update_user(User { username: new_user.into_inner().username }) {
                Ok(user) => {
                    let result = PresentationUser { username: user.username };
                    Ok(Json(json!(result)))
                }
                Err(_) => Err(ProfileError::ProfileNotFoundError("Invalid credentials".to_string())),
            }
            Err(_) => Err(ProfileError::Other("Invalid credentials".to_string())),
        }
    }

    pub fn delete_user(&self, user_id: String, key: ApiKey) -> Result<Json<Value>, ProfileError> {
        match is_valid(&*key.0) {
            Ok(_) => match self.profile_manager.delete_user(user_id.to_string()) {
                Ok(user) => {
                    let result = PresentationUser { username: user.username };
                    Ok(Json(json!(result)))
                }
                Err(_) => Err(ProfileError::ProfileNotFoundError("Invalid credentials".to_string())),
            }
            Err(_) => Err(ProfileError::Other("Invalid credentials".to_string())),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PresentationUser {
    pub username: String,
}
