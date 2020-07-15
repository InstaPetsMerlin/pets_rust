use crate::profile::domain::User;

pub trait ProfileManager {
    fn create_user(&self,user: User) -> User;
    fn update_user(&self,user: User) -> User;
    fn delete_user(&self,user: User) -> User;
    fn get_user_by_username(&self,username: String) -> User;
    fn get_all_users(&self) -> Vec<User>;
}