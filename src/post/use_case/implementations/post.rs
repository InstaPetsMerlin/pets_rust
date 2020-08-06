use crate::post::use_case::post::PostManager;
use crate::post::domain::Post;
use crate::post::errors::PostError;
use crate::post::repositories::post::PostRepository;

pub struct PostManagerImpl<T: PostRepository>{
    repository: T
}

impl<T: PostRepository> PostManagerImpl<T> {
    pub(crate) fn new(repository: T) -> Self{
        Self{
            repository
        }
    }
}


impl<T: PostRepository> PostManager for PostManagerImpl<T> {
    fn create_post(&self, post: Post) -> Result<Post, PostError> {
        self.repository.insert_post(post)
    }

    fn update_post(&self, post: Post) -> Result<Post, PostError> {
        self.repository.update_post(post)
    }

    fn delete_post(&self, post_id: i32) -> Result<Post, PostError> {
        self.repository.delete_post(post_id)
    }

    fn get_post_by_id(&self, id: i32) -> Result<Vec<Post>, PostError> {
        self.repository.get_post_by_id(id)
    }

    fn get_all_posts(&self) -> Result<Vec<Post>, PostError> {
        self.repository.get_all_posts()
    }
}