use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use jsonwebtoken::errors::ErrorKind;
use serde::{Deserialize, Serialize};

static AUTH_SECRET: &'static str = "some_secret_key";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    username: String,
    pet_name: String,
    exp: usize,
}

pub fn authenticate(token: &String) -> bool {
    let validation = Validation { ..Validation::default() };
    let key = &DecodingKey::from_secret(AUTH_SECRET.as_ref());
    let token_data = match decode::<Claims>(&token, key, &validation) {
        Ok(c) => return true,
        Err(err) => panic!("some error"),
    };
    false
}

pub fn generate_token(username: &String, pet_name: &String) -> String {
    let claim = Claims { username: String::from(username), pet_name: String::from(pet_name), exp: 10000 };
    let token = match encode(&Header::default(), &claim, &EncodingKey::from_secret(AUTH_SECRET.as_ref())) {
        Ok(t) => t,
        Err(_) => panic!("could not generate token")
    };
    token
}