use crate::post::domain::Post;
use crate::post::errors::PostError;

pub trait PostRepository {
    fn insert_post(&self, post: Post) -> Result<Post, PostError>;
    fn update_post(&self, post: Post) -> Result<Post, PostError>;
    fn delete_post(&self, post_id: i32) -> Result<Post, PostError>;
    fn get_post_by_id(&self, id: i32) -> Result<Vec<Post>, PostError>;
    fn get_all_posts(&self) -> Result<Vec<Post>, PostError>;
}