use std::error::Error;

use diesel::{ExpressionMethods, QueryDsl};

use crate::datasource::db::Conn;
use crate::datasource::schema::users;
use crate::datasource::schema::users::dsl::users as all_users;
use crate::profile::domain::User;
use crate::profile::errors::ProfileError;
use crate::profile::repositories::models::{User as UserModel, NewUser};
use crate::profile::repositories::profile::ProfileRepository;

pub struct ProfileRepositoryImpl {
    conn: Conn,
}

impl ProfileRepositoryImpl {
    pub fn new(conn: Conn) -> Self {
        Self { conn }
    }
}

impl ProfileRepository for ProfileRepositoryImpl {
    fn get_all_users(&self) -> Result<Vec<User>, ProfileError> {
       match UserModel::get_all_users(&self.conn) {
           Ok(users) => Ok(users.into_iter().map(|user|User{ username: user.username }).collect()),
           Err(_) => Err(ProfileError::ProfileDBError("Database problem".to_string())),
       }

    }

    fn get_user_by_username(&self, username: String) -> Result<User, ProfileError> {
        match UserModel::get_user_by_username(username, &self.conn) {
            Ok(users) => {
                let user_list: Vec<User> = users
                    .into_iter()
                    .map(|us| User {
                        username: us.username,
                    })
                    .collect();
                match user_list.into_iter().nth(0) {
                    None => Err(ProfileError::ProfileNotFoundError(
                        "Error not Found".to_string(),
                    )),
                    Some(u) => Ok(User {
                        username: u.username,
                    }),
                }
            }
            Err(e) => Err(ProfileError::ProfileDBError("Database problem".to_string())),
        }
    }

    fn insert_user(&self, user: User) -> Result<User, ProfileError> {
        let new_user = &NewUser {
            username: user.username,
            password: "123456".to_string(),
            first_name: "default".to_string()
        };
        match UserModel::insert_user(new_user, &self.conn){
            Ok(user) => Ok(User{ username: user.username }),
            Err(_) => Err(ProfileError::ProfileDBError("Database problem".to_string())),
        }
    }
}
