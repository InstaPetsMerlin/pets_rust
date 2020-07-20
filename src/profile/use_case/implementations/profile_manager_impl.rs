use std::error::Error;

use crate::datasource::db::Conn;
use crate::profile::domain::User;
use crate::profile::errors::ProfileError;
use crate::profile::repositories::profile::ProfileRepository;
use crate::profile::use_case::profile::ProfileManager;

pub struct ProfileManagerImpl<T:ProfileRepository> {
    box_profile_repo: T,
}

impl<T:ProfileRepository> ProfileManagerImpl<T> {
    pub(crate) fn new(profile_repo: T) -> Self {
        Self { box_profile_repo: profile_repo }
    }
}

impl<T:ProfileRepository> ProfileManager for ProfileManagerImpl<T> {
    fn create_user(&self, user: User) -> Result<User, ProfileError> {
        self.box_profile_repo.insert_user(user)
    }
    //
    // fn update_user(&self, user: User) -> User {
    //     unimplemented!()
    // }
    //
    // fn delete_user(&self, user: User) -> User {
    //     unimplemented!()
    // }
    //
    fn get_user_by_username(&self, username: String) -> Result<User, ProfileError> {
        self.box_profile_repo.get_user_by_username(username)
    }

    fn get_all_users(&self) -> Result<Vec<User>, ProfileError> {
        self.box_profile_repo.get_all_users()
    }
}
