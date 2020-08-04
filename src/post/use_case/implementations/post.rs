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
        self.repository.get_all_posts()
    }
}