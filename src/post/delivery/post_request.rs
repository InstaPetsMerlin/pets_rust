use crate::post::domain::Post;

#[derive(Serialize, Deserialize)]
pub struct PostRequest {
    pub user_id: i32,
    pub text: String,
    pub image: String,
}


impl PostRequest{
    pub fn to_domain(&self, id: i32) -> Post{

        Post{
            id,
            user_id: self.user_id,
            text: self.text.clone(),
            image: self.image.clone()
        }
    }
}