use std::error::Error;

use diesel::{ExpressionMethods, QueryDsl};

use crate::datasource::db::Conn;
use crate::datasource::schema::users;
use crate::datasource::schema::users::dsl::users as all_users;
use crate::profile::domain::User;
use crate::profile::errors::ProfileError;
use crate::profile::repositories::models::User as UserModel;
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
    fn get_all_users(&self, conn: Conn) -> Result<Vec<User>, ProfileError> {
        let users = UserModel::get_all_users(&conn);
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
}
