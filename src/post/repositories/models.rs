use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

use crate::datasource::schema::posts;
use crate::datasource::schema::posts::dsl::posts as all_posts;

#[derive(Serialize, Queryable)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub text: String,
    pub image: String,
    // pub date: NaiveDateTime,
}

// decode request data
#[derive(Deserialize)]
pub struct PostSearchParam {
    pub user_id: i32,
}

// this is to insert posts to database
#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "posts"]
pub struct NewPost {
    pub user_id: i32,
    pub text: String,
    pub image: String,
}

impl Post {
    pub fn get_all_posts(conn: &PgConnection) -> Vec<Post> {
        all_posts
            .order(posts::id.desc())
            .load::<Post>(conn)
            .expect("error!")
    }

    pub fn insert_post(post: &NewPost, conn: &PgConnection) -> bool {
        diesel::insert_into(posts::table)
            .values(post)
            .execute(conn)
            .is_ok()
    }

    pub fn get_posts_by_user_id(user_id: i32, conn: &PgConnection) -> Result<Vec<Post>, Error> {
        return match all_posts
            .filter(posts::user_id.eq(user_id))
            .load::<Post>(conn)
        {
            Ok(posts) => Ok(posts),
            Err(e) => Err(e),
        };
    }
}
