use jsonwebtoken::errors::Error;
use rocket::http::Status;
use rocket::Outcome;
use rocket::request::{self, FromRequest, Request};

use crate::profile::use_case::authentication::authenticate;

pub struct ApiKey(pub(crate) String);

/// Returns true if `key` is a valid API key string.
pub fn is_valid(key: &str) -> Result<bool, Error> {
    authenticate(&String::from(key.split_whitespace().nth(1).unwrap()))
}

#[derive(Debug)]
pub enum ApiKeyError {
    BadCount,
    Missing,
    Invalid,
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ApiKeyError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        match keys.len() {
            0 => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            1 if is_valid(keys[0]).is_ok() => Outcome::Success(ApiKey(keys[0].to_string())),
            1 => Outcome::Failure((Status::Unauthorized, ApiKeyError::Invalid)),
            _ => Outcome::Failure((Status::Unauthorized, ApiKeyError::BadCount)),
        }
    }
}
