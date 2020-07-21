use crate::datasource::db::Conn;
use crate::profile::domain::User;
use crate::profile::errors::ProfileError;


pub trait ProfileRepository {
    fn get_all_users(&self, conn: Conn) -> Result<Vec<User>, ProfileError>;
    fn get_user_by_username(&self, username: String, conn: Conn) -> Result<User, ProfileError>;
    fn insert_user(&self, user: User, conn: Conn) -> Result<User, ProfileError>;
}
