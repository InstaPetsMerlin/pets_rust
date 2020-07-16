use std::error::Error;

use diesel::{ExpressionMethods, QueryDsl};

use crate::datasource::db::Conn;
use crate::datasource::schema::users;
use crate::datasource::schema::users::dsl::users as all_users;
use crate::profile::domain::User;
use crate::profile::repositories::profile::ProfileRepository;
use crate::profile::repositories::models::User as UserModel;


struct ProfileRepositoryImpl {
    conn: Conn
}

impl ProfileRepository for ProfileRepositoryImpl {
    // fn get_all_users(&self, conn: Conn) -> Result<Vec<User>, Box<dyn Error>> {
    //     let users = UserModel::get_all_users(&conn);
    // }
}