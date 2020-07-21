

use crate::datasource::db::Conn;
use crate::profile::domain::User;
use crate::profile::errors::ProfileError;

pub trait ProfileManager {
    fn create_user(&self, user: User, conn: Conn) -> Result<User, ProfileError>;
    // fn update_user(&self,user: User) -> User;
    // fn delete_user(&self,user: User) -> User;
    fn get_user_by_username(&self, username: String, conn: Conn) -> Result<User, ProfileError>;
    fn get_all_users(&self, conn: Conn) -> Result<Vec<User>, ProfileError>;
}
