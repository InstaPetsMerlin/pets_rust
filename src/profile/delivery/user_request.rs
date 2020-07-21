
#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub first_name: String,
}
