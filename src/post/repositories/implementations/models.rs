
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::datasource::schema::posts;
use crate::datasource::schema::posts::dsl::posts as all_posts;
use crate::post::errors::PostError;

#[derive(Serialize, Queryable, AsChangeset)]
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
    pub fn get_all_posts(conn: &PgConnection) -> Result<Vec<Post>,PostError> {
        match all_posts
            .order(posts::id.desc())
            .load::<Post>(conn){
            Ok(posts) => Ok(posts),
            Err(_) => Err(PostError::PostDBError("could not get posts".to_string())),
        }
    }

    pub fn insert_post(post: &NewPost, conn: &PgConnection) -> Result<Post, PostError> {
        let inserted_post = diesel::insert_into(posts::table)
            .values(post)
            .get_result(conn);
        match inserted_post{
            Ok(post) => Ok(post),
            Err(_) => Err(PostError::PostDBError("could not insert posts for user id".to_string())),
        }
    }

    pub fn update_post(post: &Post, conn: &PgConnection) -> Result<Post, PostError> {
        let update_post = diesel::update(all_posts.find(post.id))
            .set(post)
            .get_result(conn);
        match update_post{
            Ok(post) => Ok(post),
            Err(_) => Err(PostError::PostDBError("could not update posts for user id".to_string())),
        }
    }

    pub fn delete_post(post_id: i32, conn: &PgConnection) -> Result<Post, PostError>{
        let id = post_id;
        let post = &Post{
            id,
            user_id: 0,
            text: "".to_string(),
            image: "".to_string()
        };
        let deleted_post = diesel::delete(all_posts.find(post.id))
            .get_result(conn);
        match deleted_post{
            Ok(post) => Ok(post),
            Err(_) => Err(PostError::PostDBError("could not delete posts for user id".to_string())),
        }
    }

    pub fn get_posts_by_user_id(user_id: i32, conn: &PgConnection) -> Result<Vec<Post>, PostError> {
        match all_posts
            .filter(posts::user_id.eq(user_id))
            .load::<Post>(conn)
        {
            Ok(posts) => Ok(posts),
            Err(_) => Err(PostError::PostDBError("could not get posts for user id".to_string())),
        }
    }
}
