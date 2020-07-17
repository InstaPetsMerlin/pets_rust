use crate::profile::delivery::rest_adapater::PresentationUser;

#[derive(Serialize, Deserialize)]
pub struct SignUpResponse {
    pub token: String,
    pub user: PresentationUser,
}
