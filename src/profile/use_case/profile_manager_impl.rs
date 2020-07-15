use crate::profile::domain::User;
use crate::profile::use_case::profile::ProfileManager;

pub struct ProfileManagerImpl {

}

impl ProfileManager for ProfileManagerImpl{
    fn create_user(&self, user: User) -> User {
        unimplemented!()
    }

    fn update_user(&self, user: User) -> User {
        unimplemented!()
    }

    fn delete_user(&self, user: User) -> User {
        unimplemented!()
    }

    fn get_user_by_username(&self, username: String) -> User {
        unimplemented!()
    }

    fn get_all_users(&self) -> Vec<User> {
        unimplemented!()
    }
}