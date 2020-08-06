use crate::post::delivery::post_response::PostResponse;
use crate::post::domain::Post;
use crate::post::errors::PostError;
use crate::post::use_case::post::PostManager;
use crate::post::repositories::implementations::models::NewPost;
use rocket_contrib::json::Json;
use crate::post::delivery::post_request::PostRequest;

pub struct Rest<T: PostManager> {
    manager: T
}

impl<T: PostManager> Rest<T> {
    pub fn new(manager: T) -> Self {
        Self { manager }
    }

    pub fn create_post(&self, new_post: Json<PostRequest>) -> Result<PostResponse, PostError>{
        let post = new_post.to_domain(0);
        self.manager.create_post(post)
            .map(|p| PostResponse::from_domain(p))

    }

    pub fn update_post(&self, post: Json<PostRequest>, post_id: i32) -> Result<PostResponse, PostError>{
        let post = post.to_domain(post_id);
        self.manager.update_post(post)
            .map(|p| PostResponse::from_domain(p))
    }

    pub fn delete_post(&self, post_id: i32) -> Result<PostResponse, PostError>{
        self.manager.delete_post(post_id)
            .map(|p| PostResponse::from_domain(p))
    }

    pub fn get_posts_by_user_id(&self, user_id: i32) -> Result<Vec<PostResponse>, PostError> {
        match self.manager.get_post_by_id(user_id) {
            Ok(posts) => Ok(self.map_to_post_response_list(posts)),
            Err(e) => Err(e),
        }
    }

    pub fn get_all_posts(&self) -> Result<Vec<PostResponse>, PostError> {
        match self.manager.get_all_posts() {
            Ok(posts) => Ok(self.map_to_post_response_list(posts)),
            Err(e) => Err(e)
        }
    }

    fn map_to_post_response_list(&self, posts: Vec<Post>) -> Vec<PostResponse> {
        posts
            .into_iter()
            .map(|p|
                PostResponse {
                    id: p.id,
                    user_id: p.user_id,
                    text: p.text,
                    image: p.image,
                })
            .collect()
    }
}