use jsonwebtoken::errors::Error;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

const AUTH_SECRET: &'static [u8; 15] = b"some_secret_key";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

pub fn authenticate(token: &String) -> Result<bool, Error> {
    let validation = Validation {
        ..Validation::default()
    };
    let key = &DecodingKey::from_secret(AUTH_SECRET);
    let _token_data = match decode::<Claims>(&token, key, &validation) {
        Ok(_c) => return Ok(true),
        Err(e) => return Err(e),
    };
}

pub fn generate_token(_username: &String, _pet_name: &String) -> String {
    let claim = Claims {
        sub: "b@b.com".to_owned(),
        company: "ACME".to_owned(),
        exp: 10000000000,
    };
    let token = match encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(AUTH_SECRET),
    ) {
        Ok(t) => t,
        Err(_) => panic!("could not generate token"),
    };
    token
}
