use std::error::Error;

use crate::datasource::db::Conn;
use crate::profile::domain::User;
use crate::profile::use_case::profile::ProfileManager;

pub struct ProfileManagerImpl {}

impl ProfileManagerImpl {
    pub(crate) fn new() -> Self {
        Self {}
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
    fn get_user_by_username(&self, username: String, conn: Conn) -> Result<User, Box<dyn Error>> {
        let user = User {
            username: "DAFER".to_string()
        };
        Ok(user)
    }

    fn get_all_users(&self, conn: Conn) -> Result<Vec<User>, Box<dyn Error>> {
        let user = User {
            username: "all users".to_string()
        };
        Ok(vec![user])
    }
}