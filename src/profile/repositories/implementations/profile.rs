use diesel::PgConnection;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;

use crate::datasource::db;
use crate::datasource::db::Conn;
use crate::profile::domain::User;
use crate::profile::errors::ProfileError;
use crate::profile::repositories::implementations::models::{NewUser, User as UserModel};
use crate::profile::repositories::profile::ProfileRepository;

pub struct ProfileRepositoryImpl {
    conn: Pool<ConnectionManager<PgConnection>>
}

impl ProfileRepositoryImpl {
    pub fn new() -> Self {
        Self {
            conn: db::init_pool()
        }
    }
}

impl ProfileRepository for ProfileRepositoryImpl {
    fn get_all_users(&self) -> Result<Vec<User>, ProfileError> {
        let connection = Conn(self.conn.get().expect("could not get DB"));
        match UserModel::get_all_users(&connection) {
            Ok(users) => Ok(users
                .into_iter()
                .map(|user| User {
                    username: user.username,
                })
                .collect()),
            Err(_) => Err(ProfileError::ProfileDBError("Database problem".to_string())),
        }
    }

    fn get_user_by_username(&self, username: String) -> Result<User, ProfileError> {
        let connection = Conn(self.conn.get().expect("could not get DB"));
        match UserModel::get_user_by_username(username, &connection) {
            Ok(users) => {
                let user_list: Vec<User> = users
                    .into_iter()
                    .map(|us| User {
                        username: us.username,
                    })
                    .collect();
                match user_list.into_iter().next() {
                    None => Err(ProfileError::ProfileNotFoundError(
                        "Error not Found".to_string(),
                    )),
                    Some(u) => Ok(User {
                        username: u.username,
                    }),
                }
            }
            Err(_e) => Err(ProfileError::ProfileDBError("Database problem".to_string())),
        }
    }

    fn insert_user(&self, user: User) -> Result<User, ProfileError> {
        let connection = Conn(self.conn.get().expect("could not get DB"));
        let new_user = &NewUser {
            username: user.username,
            password: "123456".to_string(),
            first_name: "default".to_string(),
        };
        match UserModel::insert_user(new_user, &connection) {
            Ok(user) => Ok(User {
                username: user.username,
            }),
            Err(_) => Err(ProfileError::ProfileDBError("Database problem".to_string())),
        }
    }

    fn update_user(&self, user: User) -> Result<User, ProfileError> {
        let connection = Conn(self.conn.get().expect("could not get DB"));
        let user = &UserModel {
            id: 1,
            username: user.username,
            password: "123456".to_string(),
            first_name: "default".to_string(),
        };
        match UserModel::update_user(user, &connection) {
            Ok(user) => Ok(User {
                username: user.username,
            }),
            Err(_) => Err(ProfileError::ProfileDBError("Database problem".to_string())),
        }
    }

    fn delete_user(&self, user_id: String) -> Result<User, ProfileError> {
        let connection = Conn(self.conn.get().expect("could not get DB"));
        match UserModel::delete_user(user_id, &connection) {
            Ok(user) => Ok(User {
                username: user.username,
            }),
            Err(_) => Err(ProfileError::ProfileDBError("Database problem".to_string())),
        }
    }
}
