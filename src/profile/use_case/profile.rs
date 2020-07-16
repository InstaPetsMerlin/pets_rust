use std::error::Error;

use crate::profile::domain::{User, ProfileError};
use crate::datasource::db::Conn;

pub trait ProfileManager {
    // fn create_user(&self,user: User) -> User;
    // fn update_user(&self,user: User) -> User;
    // fn delete_user(&self,user: User) -> User;
    fn get_user_by_username(&self,username: String) -> Result<User, ProfileError>;
    fn get_all_users(&self, conn: Conn) -> Result<Vec<User>, ProfileError>;
}