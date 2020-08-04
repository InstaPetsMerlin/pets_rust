use crate::post::delivery::post_response::PostResponse;
use crate::post::domain::Post;
use crate::post::errors::PostError;
use crate::post::use_case::post::PostManager;

pub struct Rest<T: PostManager> {
    manager: T
}

impl<T: PostManager> Rest<T> {
    pub fn new(manager: T) -> Self {
        Self { manager }
    }

    pub fn get_all_posts(&self) -> Result<Vec<PostResponse>, PostError> {
        match self.manager.get_all_posts() {
            Ok(posts) => {
                Ok(
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
                )
            }
            Err(e) => Err(e)
        }
    }
}