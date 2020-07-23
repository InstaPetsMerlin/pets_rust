use crate::datasource::db::Conn;
use crate::profile::domain::User;
use crate::profile::errors::ProfileError;


pub trait ProfileRepository {
    fn get_all_users(&self) -> Result<Vec<User>, ProfileError>;
    fn get_user_by_username(&self, username: String) -> Result<User, ProfileError>;
    fn insert_user(&self, user: User) -> Result<User, ProfileError>;
    fn update_user(&self, user: User) -> Result<User, ProfileError>;
    fn delete_user(&self, user_id: String) -> Result<User, ProfileError>;
}
