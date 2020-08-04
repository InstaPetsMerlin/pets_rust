use crate::post::domain::Post;

#[derive(Serialize, Deserialize)]
pub struct PostResponse {
    pub id: i32,
    pub user_id: i32,
    pub text: String,
    pub image: String,
}


impl PostResponse{
    fn from_domain(post:Post) ->Self{
        Self{
            id: post.id,
            user_id: post.user_id,
            text: post.text,
            image: post.image
        }
    }
}