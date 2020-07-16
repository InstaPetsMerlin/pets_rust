use std::error::Error;

use crate::datasource::db::Conn;
use crate::profile::domain::{User, ProfileError};
use crate::profile::repositories::profile::ProfileRepository;
use crate::profile::use_case::profile::ProfileManager;

pub struct ProfileManagerImpl {
    box_profile_repo: Box<dyn ProfileRepository>
}

impl ProfileManagerImpl {
    pub(crate) fn new(box_profile_repo: Box<dyn ProfileRepository>) -> Self {
        Self { box_profile_repo }
    }
}

impl ProfileManager for ProfileManagerImpl {
    // fn create_user(&self, user: User) -> User {
    //     unimplemented!()
    // }
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

    fn get_all_users(&self, conn: Conn) -> Result<Vec<User>, ProfileError> {
        let user = User {
            username: "all users".to_string()
        };
        Ok(vec![user])
    }
}