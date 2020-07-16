use std::error::Error;

use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::datasource::schema::users;
use crate::datasource::schema::users::dsl::users as all_users;

#[derive(Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub first_name: String,
}

// decode request data
#[derive(Deserialize)]
pub struct UserData {
    pub username: String,
}

// this is to insert users to database
#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub first_name: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

impl User {
    pub fn get_all_users(conn: &PgConnection) -> Result<Vec<User>, Box<dyn Error>> {
        match all_users
            .order(users::id.desc())
            .load::<User>(conn) {
            Ok(users) => Ok(users),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn insert_user(user: &NewUser, conn: &PgConnection) -> bool {
        diesel::insert_into(users::table)
            .values(user)
            .execute(conn)
            .is_ok()
    }

    pub fn get_user_by_username(
        username: &String,
        conn: &PgConnection,
    ) -> Result<Vec<User>, Box<dyn Error>> {
        return match all_users
            .filter(users::username.eq(username))
            .load::<User>(conn)
        {
            Ok(users) => Ok(users),
            Err(e) => Err(Box::new(e)),
        };
    }
}
