use diesel::PgConnection;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;

use crate::datasource::db;
use crate::datasource::db::Conn;
use crate::post::domain::Post;
use crate::post::errors::PostError;
use crate::post::repositories::implementations::models::{NewPost, Post as PostModel};
use crate::post::repositories::post::PostRepository;

pub struct PostRepositoryImpl { conn: Pool<ConnectionManager<PgConnection>> }

impl PostRepositoryImpl {
    pub fn new() -> Self {
        Self { conn: db::init_pool() }
    }
}

impl PostRepository for PostRepositoryImpl {
    fn insert_post(&self, post: Post) -> Result<Post, PostError> {
        let connection = self.get_conn();
        let new_post = &NewPost {
            user_id: post.user_id,
            text: post.text,
            image: post.image,
        };
        match PostModel::insert_post(new_post, &connection) {
            Ok(result) => {
                let post_created = Post {
                    id: result.id,
                    user_id: result.user_id,
                    text: result.text,
                    image: result.image,
                };
                Ok(post_created)
            }
            Err(e) => Err(e),
        }
    }

    fn update_post(&self, post: Post) -> Result<Post, PostError> {
        let connection = self.get_conn();
        let post_to_update = &PostModel {
            id: post.id,
            user_id: post.user_id,
            text: post.text,
            image: post.image,
        };
        match PostModel::update_post(post_to_update, &connection) {
            Ok(result) => {
                let post_updated = Post {
                    id: result.id,
                    user_id: result.user_id,
                    text: result.text,
                    image: result.image,
                };
                Ok(post_updated)
            }
            Err(e) => Err(e),
        }
    }

    fn delete_post(&self, post_id: i32) -> Result<Post, PostError> {
        let connection = self.get_conn();
        match PostModel::delete_post(post_id, &connection) {
            Ok(result) => {
                let post_deleted = Post {
                    id: result.id,
                    user_id: result.user_id,
                    text: result.text,
                    image: result.image,
                };
                Ok(post_deleted)
            }
            Err(e) => Err(e),
        }
    }

    fn get_post_by_id(&self, id: i32) -> Result<Vec<Post>, PostError> {
        let connection = self.get_conn();
        match PostModel::get_posts_by_user_id(id, &connection) {
            Ok(posts) => {
                let result = posts.into_iter()
                    .map(|p| Post {
                        id: p.id,
                        user_id: p.user_id,
                        text: p.text,
                        image: p.image,
                    }).collect();
                Ok(result)
            }
            Err(e) => Err(e),
        }
    }

    fn get_all_posts(&self) -> Result<Vec<Post>, PostError> {
        let pool = self.conn.get().expect("could not get DB");
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

impl PostRepositoryImpl {
    fn get_conn(&self) -> Conn {
        let pool = self.conn.get().expect("could not get DB");
        let connection = Conn(pool);
        connection
    }
}
