use crate::datasource::db::Conn;
use crate::profile::domain::User;
use std::error::Error;

pub trait ProfileRepository {
    // fn get_all_users(&self, conn: Conn) -> Result<Vec<User>, Box<dyn Error>>;
}