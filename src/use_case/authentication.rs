use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use jsonwebtoken::errors::ErrorKind;
use serde::{Deserialize, Serialize};

const AUTH_SECRET: &'static [u8; 15] = b"some_secret_key";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

pub fn authenticate(token: &String) -> bool {
    let validation = Validation { ..Validation::default() };
    let key = &DecodingKey::from_secret(AUTH_SECRET);
    let token_data = match decode::<Claims>(&token, key, &validation) {
        Ok(c) => return true,
        Err(err) => panic!("some error"),
    };
    false
}

pub fn generate_token(username: &String, pet_name: &String) -> String {
    let claim = Claims { sub: "b@b.com".to_owned(), company: "ACME".to_owned(), exp: 10000000000 };
    let token = match encode(&Header::default(), &claim, &EncodingKey::from_secret(AUTH_SECRET)) {
        Ok(t) => t,
        Err(_) => panic!("could not generate token")
    };
    token
}