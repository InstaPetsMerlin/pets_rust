use crate::profile::delivery::profile::PresentationUser;

#[derive(Serialize, Deserialize)]
pub struct SignUpResponse {
    pub token: String,
    pub user: PresentationUser,
}
