use diesel::PgConnection;
use r2d2::{Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;

use crate::datasource::db;
use crate::datasource::db::Conn;
use crate::post::domain::Post;
use crate::post::errors::PostError;
use crate::post::repositories::models::Post as PostModel;
use crate::post::repositories::post::PostRepository;

pub struct PostRepositoryImpl { conn: Pool<ConnectionManager<PgConnection>> }

impl PostRepositoryImpl {
    pub fn new() -> Self {
        Self {
            conn: db::init_pool()
        }
    }
}

impl PostRepository for PostRepositoryImpl {
    fn insert_post(&self, post: Post) -> Result<Post, PostError> {
        unimplemented!()
    }

    fn update_post(&self, post: Post) -> Result<Post, PostError> {
        unimplemented!()
    }

    fn delete_post(&self, post_id: String) -> Result<Post, PostError> {
        unimplemented!()
    }

    fn get_post_by_id(&self, id: i32) -> Result<Post, PostError> {
        unimplemented!()
    }

    fn get_all_posts(&self) -> Result<Vec<Post>, PostError> {
        let pool = self.conn.get().expect("could not get DB");
        // let pool = self.conn.get().unwrap_or_else(|e|{
        //     Err(PostError::Other("".to_string()))
        // });
        let connection = Conn(pool);
        match PostModel::get_all_posts(&connection) {
            Ok(posts) => Ok(posts.into_iter()
                .map(|p| Post {
                    id: p.id,
                    user_id: p.user_id,
                    text: p.text,
                    image: p.image,
                })
                .collect()),
            Err(e) => Err(e),
        }
    }
}